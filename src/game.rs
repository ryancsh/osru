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
    let animation_timings: AnimationTiming = AnimationTiming {
      preempt: Duration::from_millis(500),
      fade_in: Duration::from_millis(250),
      timing_great: TIMING_WINDOW_GREAT,
      timing_good: TIMING_WINDOW_GOOD,
      timing_meh: TIMING_WINDOW_MEH,
    };

    let audio = true;

    let mut run = true;
    let mut num_frames: u64 = 0;

    // start audio
    let (audio_filename, mut b) = Game::start_beatmap(
      OsruGameMode::Standard,
      "assets/beatmap/Shihori - Magic Girl !! (Frostmourne) [Hard].osu",
    );

    println!("{}", b.hitobjects.len());

    let (tx, rx) = mpsc::channel();
    let (ty, ry) = mpsc::channel();

    let t = std::thread::spawn(move || {
      if audio {
        let mut audio_manager = audio::AudioManager::new();
        audio_manager.add_source(&audio_filename);
        ty.send(AudioMessage::Ready).unwrap();
        audio_manager.wait(rx);
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
      texture_creator.load_texture(path::Path::new("assets/beatmap/magic girl.jpg")).unwrap();

    let mut background_texture2 =
      texture_creator.load_texture(path::Path::new("assets/black_pixel.png")).unwrap();
    background_texture2.set_alpha_mod(u8::MAX / 4 * 3);

    //input
    let mut input_manager = input::InputManager::new(sdl_context.event_pump().unwrap());

    // other stuff
    let mut hitobj_start_i = 0;

    // wait for audio
    'wait_for_audio: loop {
      match ry.recv() {
        Ok(AudioMessage::Ready) => break 'wait_for_audio,
        Err(_) => panic![],
        _ => (),
      }
    }

    // start game
    tx.send(AudioMessage::Play(0)).unwrap();
    input_manager.start_timer();

    while run {
      {
        //draw background
        let background_viewport = OsruRect::new(
          0.0,
          0.0,
          background_texture.query().width as f64,
          background_texture.query().height as f64,
        );
        let background_viewport =
          osru_pixels_to_window(&background_viewport, &OsruRect::new_from_sdl2_rect(canvas.viewport()), true);
        canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        display_background_image(&mut canvas, &mut background_texture, false);
        canvas.copy(&background_texture2, None, None).unwrap();
        canvas.set_draw_color(pixels::Color::RGBA(255, 255, 255, 255));
      }

      // update
      let viewport_size = OsruRect::new_from_sdl2_rect(canvas.viewport());
      'nextUpdate: while let Some(update) = input_manager.next_update() {
        println!("update {:?}", update);
        for i in hitobj_start_i..b.hitobjects.len() {
          println!("hitobj index {}", i);
          let hitobj = b.hitobjects.get_mut(i).unwrap();
          let update_result = hitobj.update(&update, &animation_timings, &viewport_size);
          if update_result == UpdateResult::Success || hitobj.draw_state() == DrawState::NotYet {
            continue 'nextUpdate;
          }
        }
      }
      {
        let time = input_manager.reference_time().elapsed_sys_time(Instant::now());
        let mut snap = InputSnapshot::default();
        snap.time = time;
        let update = InputUpdate::new(snap.clone(), snap);
        'update_time: for i in hitobj_start_i .. b.hitobjects.len(){
          let h = b.hitobjects.get_mut(i).unwrap();
          h.update(&update, &animation_timings, &viewport_size);
          if h.draw_state() == DrawState::NotYet{
            break 'update_time;
          }
        }
      }
      

      if !input_manager.is_running() {
        tx.send(AudioMessage::Stop).unwrap_or(());
        run = false;
      }

      let mut start_i = None;
      '_drawing_loop: for i in hitobj_start_i..b.hitobjects.len() {
        use hitobject::DrawState::*;
        let hitobj = b.hitobjects.get(i).unwrap();
        match hitobj.draw_state() {
          NotYet => {
            if start_i == None {
              start_i = Some(i)
            }
            break '_drawing_loop;
          }
          Drawing => {
            if start_i == None {
              start_i = Some(i)
            }
            hitobj.draw(&mut canvas, &mut texture);
          }
          _ => (),
        }
      }

      {
        //draw keypresses
        let snapshot = input_manager.last_snapshot();
        canvas.set_draw_color(pixels::Color::RGBA(255, 255, 255, u8::MAX / 2));
        if snapshot.K1() {
          canvas.fill_rect(sdl2::rect::Rect::new(2304, 656, 128, 128)).unwrap();
        }
        if snapshot.K2() {
          canvas.fill_rect(sdl2::rect::Rect::new(2304, 784, 128, 128)).unwrap();
        }
      }

      canvas.present();
      num_frames += 1;

      if let Some(i) = start_i {
        hitobj_start_i = i;
      } else {
        run = false
      }
      if !run {
        tx.send(AudioMessage::Done).unwrap_or(());
      }
    }
    let total_time = input_manager.reference_time().elapsed_sys_time(Instant::now()).as_millis() as f64;
    println!("avg fps {}", num_frames as f64 / total_time * 1000.0);
    tx.send(AudioMessage::Done).unwrap_or(());
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

  pub fn start_beatmap(mode: OsruGameMode, filename: &str) -> (String, Beatmap) {
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

    //audio_manager.sleep_until_end();
    (audio_filename, b)
  }
}
