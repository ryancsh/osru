extern crate sdl2;

use crate::global::pixel::*;
use crate::global::*;
use crate::input;
use hitobject::{HitState, HitSuccess, UpdateResult};
use input::{InputSnapshot, InputUpdate};
use std::mem::discriminant;

use crate::{
   audio::{self, *},
   beatmap::{self, *},
};

use sdl2::{image::LoadTexture, pixels, rect::Rect, video::FullscreenType::Desktop};

use std::sync::mpsc;
use std::{
   cmp,
   collections::HashSet,
   hash, path,
   rc::Rc,
   slice, thread,
   time::{self, Duration, Instant},
};

pub struct Game {}
impl Game {
   pub fn start() {
      let audio = true;

      const _MAGIC: &str = "assets/beatmap/magic/Shihori - Magic Girl !! (Frostmourne) [Lunatic].osu";
      const _KOI: &str = "assets/beatmap/koi/KOTOKO - Koi Kou Enishi (Crystal) [Hard].osu";
      const _FANTASTIC: &str =
         "assets/beatmap/fantastic/Tamura Yukari - Fantastic future (TV Size) (Flask) [Hard].osu";
      let (audio_filename, background_filename, mut b) = Game::start_beatmap(OsruGameMode::Standard, _MAGIC);
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
         fn find_sdl_gl_driver() -> Option<u32> {
            for (index, item) in sdl2::render::drivers().enumerate() {
               if item.name == "opengl" {
                  return Some(index as u32);
               }
            }
            None
         }
         find_sdl_gl_driver();

         let window = video_subsystem
            .window(
               "Osru",
               DEFAULT_WINDOW_SIZE.x().get_round() as u32,
               DEFAULT_WINDOW_SIZE.y().get_round() as u32,
            )
            .opengl()
            .allow_highdpi()
            .position_centered()
            .build()
            .unwrap();
         window.into_canvas().index(find_sdl_gl_driver().unwrap()).build().unwrap()
         //
         //
      };
      canvas.window_mut().set_fullscreen(Desktop).unwrap();
      canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
      canvas.clear();
      canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
      canvas.present();

      let texture_manager = canvas.texture_creator();
      let mut texture_manager = TextureManager::new(&texture_manager);
      let mut texture_manager = &mut texture_manager;

      texture_manager.load(TextureName::HitCircle, "assets/skin/hitcircle.png");
      texture_manager.load(TextureName::Background, &background_filename);
      texture_manager.load(TextureName::ApproachCircle, "assets/skin/approachcircle.png");

      //input
      let event_subsys = sdl_context.event().unwrap();
      let event_pump = sdl_context.event_pump().unwrap();
      let mut input_manager = input::InputManager::new(event_pump);

      // other stuff

      let background_viewport = texture_manager.size(TextureName::Background);

      let viewport_size = PixRect::new_from_sdl2_rect(canvas.viewport());
      let background_texture = texture_manager.get(TextureName::Background);
      {
         canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
         canvas.clear();
         display_background_image(&mut canvas, &mut background_texture.borrow_mut(), Letterboxing::Deny);
      }
      b.prepare(&viewport_size);

      let mut run = true;
      let mut num_frames: u64 = 0;

      // wait for audio
      'wait_for_audio: loop {
         match ry.recv() {
            Ok(AudioMessage::Ready) => break 'wait_for_audio,
            Err(_) => panic![],
            _ => (),
         }
      }

      // start game
      thread::sleep(Duration::from_nanos(1));
      tx.send(AudioMessage::Play(0, BEATMAP_TIMING_OFFSET)).unwrap();
      input_manager.start_timer();

      // main loop
      'renderLoop: loop {
         let frame_start = Instant::now();

         b.full_update(&mut input_manager);
         display_background_image(&mut canvas, &mut background_texture.borrow_mut(), Letterboxing::Deny);
         b.draw(&mut canvas, &mut texture_manager, &mut input_manager);
         run = run && !b.is_done();

         input_manager.poll_all();

         //draw keypresses
         let last_snapshot = input_manager.curr_snapshot();
         canvas.set_draw_color(pixels::Color::RGBA(255, 255, 255, u8::MAX / 2));
         canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
         if last_snapshot.K1() {
            canvas.fill_rect(sdl2::rect::Rect::new(2304, 656, 128, 128)).unwrap();
         }
         if last_snapshot.K2() {
            canvas.fill_rect(sdl2::rect::Rect::new(2304, 784, 128, 128)).unwrap();
         }
         input_manager.poll_all();
         canvas.present();
         input_manager.poll_all();

         num_frames += 1;

         run = run && input_manager.is_running();
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
         //let target_frame_time = frame_start.elapsed() * 8;
         while LIMIT_FPS && frame_start.elapsed() < TIME_PER_FRAME {
            input_manager.poll_all();
            b.lazy_update(&mut input_manager);
            thread::yield_now();
         }
      }
      let total_time = input_manager.reference_time().elapsed_now().as_secs_f64();
      println!("fps: avg {}", num_frames as f64 / total_time);
      {
         use hitobject::HitState::*;
         use hitobject::HitSuccess::*;
         let mut num_great = 0;
         let mut num_good = 0;
         let mut num_meh = 0;
         let mut num_miss = 0;
         let mut num_unknown = 0;
         for hitobj in b.hitobjects {
            if let DoneDrawing(v) = hitobj.hit_state() {
               match v {
                  Great => num_great += 1,
                  Good => num_good += 1,
                  Meh => num_meh += 1,
                  Miss => num_miss += 1,
               }
            } else {
               num_unknown += 1;
            }
         }
         println!(
            "Great: {}, Good: {}, Meh: {}, Miss: {}, Unknown: {}",
            num_great, num_good, num_meh, num_miss, num_unknown
         );
         println!("Capacity: {}", input_manager.capacity());
      }
      t.join().unwrap();
   }

   pub fn start_beatmap(mode: OsruGameMode, filename: &str) -> (String, Option<String>, Beatmap) {
      use beatmap::settings::{BeatmapSettingName::*, BeatmapSettings};
      let b = beatmap::Beatmap::load(filename);

      let audio_filename = b.settings.get(&AudioFilename).unwrap().parse_as_str().to_string();
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

      (audio_filename, background_filename, b)
   }
}
