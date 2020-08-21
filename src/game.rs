extern crate sdl2;

use crate::global::*;
use crate::input;
use hitobject::{DrawState, UpdateResult};
use input::{InputSnapshot, InputUpdate};

use crate::{
   audio::{self, *},
   beatmap::{self, *},
};

use sdl2::{image::LoadTexture, pixels, video::FullscreenType::Desktop};

use std::sync::mpsc;
use std::{
   cmp,
   collections::HashSet,
   hash, path, slice, thread,
   time::{self, Duration, Instant},
};

pub struct Game {}
impl Game {
   pub fn start() {
      let audio = true;
      let mut target_frame_time = Duration::from_secs(0);

      let mut run = true;
      let mut num_frames: u64 = 0;
      let mut slowest_frame = Duration::from_secs(0);

      const _MAGIC:&str = "assets/beatmap/magic/Shihori - Magic Girl !! (Frostmourne) [Hard].osu";
      const _KOI: &str = "assets/beatmap/koi/KOTOKO - Koi Kou Enishi (Crystal) [Hard].osu";
      let (audio_filename, background_filename, mut b) = Game::start_beatmap(
         OsruGameMode::Standard,
         _KOI,
      );
      let background_filename = {
         if let Some(filename) = background_filename {
            filename
         } else {
            nstr("assets/black_pixel.png")
         }
      };

      println!("{}", b.hitobjects.len());
      // start audio

      let (tx, rx) = mpsc::channel();
      let (ty, ry) = mpsc::channel();

      let t = std::thread::spawn(move || {
         if audio {
            let mut audio_manager = audio::AudioManager::new();
            audio_manager.add_source(&audio_filename);
            ty.send(AudioMessage::Ready).unwrap();
            audio_manager.wait(rx);
            ty.send(AudioMessage::Done).unwrap_or(());
         }
      });

      //graphics
      let sdl_context = sdl2::init().unwrap();
      let video_subsystem = sdl_context.video().unwrap();

      let mut canvas = {
         let window = video_subsystem
            .window("Osru", DEFAULT_WINDOW_SIZE.0 as u32, DEFAULT_WINDOW_SIZE.1 as u32)
            .allow_highdpi()
            .position_centered()
            .build()
            .unwrap();
         window.into_canvas().build().unwrap()
      };
      canvas.window_mut().set_fullscreen(Desktop).unwrap();
      canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
      canvas.clear();
      canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
      canvas.present();

      let texture_creator = canvas.texture_creator();

      let mut texture = texture_creator.load_texture(path::Path::new("assets/hitcircle.png")).unwrap();
      let mut background_texture =
         texture_creator.load_texture(path::Path::new(&background_filename)).unwrap();

      let mut background_texture2 =
         texture_creator.load_texture(path::Path::new("assets/black_pixel.png")).unwrap();
      background_texture2.set_alpha_mod(u8::MAX / 4 * 3);

      //input
      let event_subsys = sdl_context.event().unwrap();
      event_subsys.register_custom_event::<TimeBarrier>().unwrap();

      let mut input_manager = input::InputManager::new(sdl_context.event_pump().unwrap());

      // other stuff
      let mut hitobj_start_i = 0;
      let mut hitobj_update_i = 0;

      let background_viewport = OsruRect::new(
         0.0,
         0.0,
         background_texture.query().width as f64,
         background_texture.query().height as f64,
      );
      let background_viewport =
         osru_pixels_to_window(&background_viewport, &OsruRect::new_from_sdl2_rect(canvas.viewport()), true);
      let viewport_size = OsruRect::new_from_sdl2_rect(canvas.viewport());
      {
         canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
         canvas.clear();
         display_background_image(&mut canvas, &mut background_texture, false);
         canvas.copy(&background_texture2, None, None).unwrap();
      }
      b.prepare(&viewport_size);

      // wait for audio
      'wait_for_audio: loop {
         match ry.recv() {
            Ok(AudioMessage::Ready) => break 'wait_for_audio,
            Err(_) => panic![],
            _ => (),
         }
      }

      // start game
      let mut frame_start = Instant::now();
      tx.send(AudioMessage::Play(0)).unwrap();
      input_manager.start_timer();

      // main loop
      'renderLoop: loop {
         //target_frame_time = frame_start.elapsed() * 15/16;
         
         // update
         {
            use hitobject::{DrawState::*, HitSuccess::*, UpdateResult::*};
            event_subsys.push_custom_event(TimeBarrier {}).unwrap();
            let mut i = hitobj_update_i;
            let mut new_update_i = None;

            'nextObj: loop {
               if i >= b.hitobjects.len(){
                  while let Some(update) = input_manager.next_update() {}
                  break 'nextObj;
               }
               let hitobj = b.hitobjects.get_mut(i).unwrap();
               match hitobj.draw_state() {
                  NotYet | Drawing if hitobj.hit_success() == Unknown => {
                     while let Some(update) = input_manager.next_update() {
                        if hitobj.update(&update, &DEFAULT_ANIMATION_TIMING) == Success {
                           i += 1;
                           if new_update_i == None {
                              new_update_i = Some(i)
                           }
                           continue 'nextObj;
                        }
                     }
                     break 'nextObj;
                  }
                  Drawing | Done => i += 1,
                  NotYet => panic![],
               }
            }
            hitobj_update_i = new_update_i.unwrap_or(hitobj_update_i);
         }
         frame_start = Instant::now();
         
         
         {
            //draw background
            canvas.set_blend_mode(sdl2::render::BlendMode::None);
            canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
            canvas.clear();
            display_background_image(&mut canvas, &mut background_texture, false);
            canvas.copy(&background_texture2, None, None).unwrap();
         }

         {
            //let time = input_manager.reference_time().elapsed_sys_time(Instant::now());
            let snap = InputSnapshot::new_from(input_manager.last_snapshot());
            let update = InputUpdate::new(&snap, &snap);
            let mut not_draw = true;
            'update_time: for i in hitobj_start_i..b.hitobjects.len() {
               let h = b.hitobjects.get_mut(i).unwrap();
               h.update(&update, &DEFAULT_ANIMATION_TIMING);
               let draw_state = h.draw(&mut canvas, &mut texture);
               if not_draw && draw_state == DrawState::Drawing {
                  not_draw = false;
                  hitobj_start_i = i;
               } else if draw_state == DrawState::NotYet {
                  break 'update_time;
               }
            }
            run = run && !not_draw && input_manager.is_running();
         }

         {
            //draw keypresses
            let snapshot = input_manager.last_snapshot();
            canvas.set_draw_color(pixels::Color::RGBA(255, 255, 255, u8::MAX / 2));
            canvas.set_blend_mode(sdl2::render::BlendMode::Add);
            if snapshot.K1() {
               canvas.fill_rect(sdl2::rect::Rect::new(2304, 656, 128, 128)).unwrap();
            }
            if snapshot.K2() {
               canvas.fill_rect(sdl2::rect::Rect::new(2304, 784, 128, 128)).unwrap();
            }
         }
         canvas.present();
         thread::yield_now();

         let current_frame_time = frame_start.elapsed();
         if slowest_frame < current_frame_time {
            slowest_frame = current_frame_time;
         }

         num_frames += 1;

         if !run {
            if !input_manager.is_running() {
               tx.send(AudioMessage::Stop).unwrap_or(());
            } else {
               tx.send(AudioMessage::Done).unwrap_or(());
            }
            match ry.try_recv() {
               Ok(AudioMessage::Done) => break 'renderLoop,
               _ => (),
            }
         }
      }
      let total_time = input_manager.reference_time().elapsed_sys_time(Instant::now()).as_millis() as f64;
      println!(
         "fps: avg {}, min {}",
         num_frames as f64 / total_time * 1000.0,
         1.0 / slowest_frame.as_secs_f64(),
      );
      {
         use hitobject::HitSuccess::*;
         let mut num_great = 0;
         let mut num_good = 0;
         let mut num_meh = 0;
         let mut num_miss = 0;
         let mut num_unknown = 0;
         for hitobj in b.hitobjects {
            match hitobj.hit_success() {
               Great => num_great += 1,
               Good => num_good += 1,
               Meh => num_meh += 1,
               Miss => num_miss += 1,
               Unknown => num_unknown += 1,
            }
         }
         println!(
            "Great: {}, Good: {}, Meh: {}, Miss: {}, Unknown: {}",
            num_great, num_good, num_meh, num_miss, num_unknown
         );
      }
      t.join().unwrap();
   }

   pub fn start_beatmap(mode: OsruGameMode, filename: &str) -> (String, Option<String>, Beatmap) {
      use beatmap::BeatmapSettings::*;
      let b = beatmap::Beatmap::load(filename);

      let audio_filename = b.get(AudioFilename).unwrap().parse_as_str().to_string();
      let parent_dir = match path::Path::new(filename).parent() {
         Some(parent_dir) => match parent_dir.to_str() {
            Some(parent_dir) => mergestr(parent_dir, "/"),
            Option::None => String::from(""),
         },
         Option::None => String::from(""),
      };
      let audio_filename = mergestr(&parent_dir, &audio_filename);

      let background_filename = {
         if let Some(ev_background) = b.event_backgrounds.get(0) {
            if ev_background.filename.len() > 0 {
               Some(mergestr(&parent_dir, &ev_background.filename))
            } else {
               None
            }
         } else {
            None
         }
      };
      println!("{:?} {:?} {:?}", parent_dir, audio_filename, background_filename);

      //audio_manager.sleep_until_end();
      (audio_filename, background_filename, b)
   }
}
