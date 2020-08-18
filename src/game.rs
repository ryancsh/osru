extern crate sdl2;

use crate::global::*;

use crate::{
  audio::{self, *},
  beatmap::{self, *},
};

use sdl2::{image::LoadTexture, pixels, video::FullscreenType::Desktop};

use std::sync::mpsc;
use std::{
  cmp,
  collections::HashSet,
  hash, path, slice,
  time::{self, SystemTime},
};

pub struct Game {}
impl Game {
  pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
      .window("Osru", DEFAULT_WINDOW_SIZE.0 as u32, DEFAULT_WINDOW_SIZE.1 as u32)
      .allow_highdpi()
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.window_mut().set_fullscreen(Desktop).unwrap();

    canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let mut texture =
      texture_creator.load_texture(path::Path::new("assets/hitcircle.png")).unwrap();

    let (audio_filename, mut b) = Game::start_beatmap(
      OsruGameMode::Standard,
      "assets/beatmap/Shihori - Magic Girl !! (Frostmourne) [Lunatic].osu",
    );

    let audio = true;

    let animation_timings = AnimationTiming {
      preempt: OsruTime::s(2),
      fade_in: OsruTime::s(1),
      timing_great: OsruTime::ms(80),
      timing_good: OsruTime::ms(160),
      timing_meh: OsruTime::ms(240),
    };

    let start_time = SystemTime::now();
    /*
    let mut a = 0;
    let mut run = true;
    let mut h = hitobject::HitCircle {
        position: OsruPixels(320.0, 200.0),
        time: OsruTime::s(3),
        new_combo: false,
        combo_colours_to_skip: 0,
        hitsounds: OsruHitSounds::from_bitflags(Bitflags(0)),

        hitsample_set: 0,
        hitsample_additional_set: 0,
        hitsample_index: 0,
        hitsample_volume: Volume(0.0),
        hitsample_filename: String::from(""),

        animation_timings: &animation_timings,
        current_time: OsruTime::s(0),
        texture: &texture,
    };
    let mut h: Box<dyn hitobject::HitObject> = Box::new(h);
    */
    let mut run = true;
    let mut num_frames: u64 = 0;
    let mut hit_object_start_index = 0;
    println!("{}", b.hitobjects.len());

    let (tx, rx) = mpsc::channel();
    let (ty, ry) = mpsc::channel();

    let t = std::thread::spawn(move || {
      if audio {
        let mut audio_manager = audio::AudioManager::new();
        audio_manager.add_source(&audio_filename);
        ty.send(AudioMessage::Ready).unwrap();
        audio_manager.play_source(0);
        audio_manager.sleep_until_stop(rx);
      }
    });
    'wait_for_audio: loop {
      match ry.recv() {
        Ok(AudioMessage::Ready) => break 'wait_for_audio,
        Err(_) => panic![],
        _ => (),
      }
    }
    while run {
      use sdl2::event::Event;
      for ev in event_pump.poll_iter() {
        match ev {
          Event::Quit { .. }
          | Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
            run = false;
            tx.send(AudioMessage::Stop).unwrap();
          }
          _ => {}
        }
      }
      let current_time = SystemTime::now().duration_since(start_time).unwrap();
      let current_time = OsruTime::from_duration(current_time);
      canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
      canvas.clear();

      let mut drawed = false;
      'drawing_loop: for i in hit_object_start_index..b.hitobjects.len() {
        if let Some(h) = b.hitobjects.get_mut(i) {
          use hitobject::HitObjectDrawState::*;
          h.update(current_time);
          match h.draw(&mut canvas, &mut texture) {
            NotYet => {
              drawed = true;
              break 'drawing_loop;
            }
            Done => {
              if hit_object_start_index <= i {
                hit_object_start_index = i;
              }
            }
            Drawing => {
              drawed = true;
            }
          }
        }
      }

      canvas.present();
      run = run && drawed;
      num_frames += 1;
      //println!("frame number {}", num_frames);
    }
    let total_time = SystemTime::now().duration_since(start_time).unwrap().as_millis() as f64;
    println!("avg fps {}", num_frames as f64 / total_time * 1000.0);
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
    //println!("{}\n{}", parent_dir, audio_filename);

    //audio_manager.sleep_until_end();
    (audio_filename, b)
  }
}

#[derive(Debug, Clone)]
pub struct OsruGameModsActive {
  mods: HashSet<OsruGameMod>,
}
impl OsruGameModsActive {
  pub fn new() -> OsruGameModsActive {
    OsruGameModsActive { mods: HashSet::new() }
  }

  pub fn od_multiplier(&self) -> f64 {
    let mut od_mul = 1.0;
    for game_mod in self.mods.iter() {
      od_mul *= game_mod.od_multiplier;
    }
    if od_mul > 10.0 {
      10.0
    } else {
      od_mul
    }
  }

  // TODO: perceived_od_mul()
  pub fn ar_multiplier(&self) -> f64 {
    let mut ar_mul = 1.0;
    for game_mod in self.mods.iter() {
      ar_mul *= game_mod.ar_multiplier;
    }
    if ar_mul > 10.0 {
      10.0
    } else {
      ar_mul
    }
  }

  pub fn hit_timing_window(&self, hit_success: OsruHitSuccess, beatmap_od: OsruOD) -> OsruTime {
    let (base_timing, multiplier) = match hit_success {
      OsruHitSuccess::Great => (TIMING_WINDOW_GREAT, TIMING_WINDOW_GREAT_MULTIPLIER),
      OsruHitSuccess::Good => (TIMING_WINDOW_GOOD, TIMING_WINDOW_GOOD_MULTIPLIER),
      OsruHitSuccess::Meh => (TIMING_WINDOW_MEH, TIMING_WINDOW_MEH_MULTIPLIER),
      OsruHitSuccess::Miss => panic![],
    };
    let od_multiplier = self.od_multiplier();
    base_timing - OsruTime::us_f(self.od_multiplier() * multiplier.0 as f64 * beatmap_od.0)
  }

  pub fn preempt_time(&self, beatmap_ar: OsruAR) -> OsruTime {
    if beatmap_ar.0 == 5.0 {
      OsruTime::ms(1200)
    } else if beatmap_ar.0 < 5.0 {
      OsruTime::ms_f(1200.0 + 600.0 * (5.0 - beatmap_ar.0 * self.ar_multiplier()) / 5.0)
    } else {
      OsruTime::ms_f(1200.0 - 750.0 * (beatmap_ar.0 * self.ar_multiplier() - 5.0) / 5.0)
    }
  }

  pub fn fade_in_time(&self, beatmap_ar: OsruAR) -> OsruTime {
    if beatmap_ar.0 == 5.0 {
      OsruTime::ms(800)
    } else if beatmap_ar.0 < 5.0 {
      OsruTime::ms_f(800.0 + 400.0 * (5.0 - beatmap_ar.0 * self.ar_multiplier()) / 5.0)
    } else {
      OsruTime::ms_f(800.0 - 500.0 * (beatmap_ar.0 * self.ar_multiplier() - 5.0) / 5.0)
    }
  }

  pub fn enable_game_mod(&mut self, new_mod: OsruGameModName) {
    let mut to_remove = vec![];
    let new_mod = OsruGameMod::new(new_mod);
    {
      for m in self.mods.iter() {
        if new_mod.eq(m) {
          return;
        }
        for exclude in m.exclusive() {
          if new_mod.name().eq(exclude) {
            to_remove.push(OsruGameMod::new(*exclude));
          }
        }
      }
    }

    for m in to_remove.iter() {
      self.mods.remove(m);
    }
    self.mods.insert(new_mod);
  }

  pub fn disable_game_mod(&mut self, mod_to_disable: OsruGameModName) {
    let mod_to_disable = OsruGameMod::new(mod_to_disable);
    self.mods.remove(&mod_to_disable);
  }
}

#[derive(Debug, Clone)]
pub struct OsruGameMod {
  game_mod_name: OsruGameModName,
  exclusive: Vec<OsruGameModName>,

  ar_multiplier: f64,
  od_multiplier: f64,
  cs_multiplier: f64,
}
impl OsruGameMod {
  pub fn new(name: OsruGameModName) -> OsruGameMod {
    use OsruGameModName::*;
    let mut result = OsruGameMod::default();
    result.game_mod_name = name;
    match name {
      Easy => {
        result.exclusive.push(HardRock);
        result.ar_multiplier = 0.5;
        result.od_multiplier = 0.5;
      }
      HardRock => {
        result.exclusive.push(Easy);
        result.ar_multiplier = 1.4;
        result.od_multiplier = 1.4;
      }
      _ => (),
    }
    result
  }

  // TODO: other mods
  pub fn exclusive<'a>(&'a self) -> slice::Iter<'a, OsruGameModName> {
    self.exclusive.iter()
  }

  pub fn name(&self) -> OsruGameModName {
    self.game_mod_name
  }
}
impl Default for OsruGameMod {
  fn default() -> Self {
    OsruGameMod {
      game_mod_name: OsruGameModName::None,
      exclusive: vec![],
      ar_multiplier: 1.0,
      od_multiplier: 1.0,
      cs_multiplier: 1.0,
    }
  }
}
impl hash::Hash for OsruGameMod {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    self.game_mod_name.hash(state);
  }
}
impl cmp::PartialEq for OsruGameMod {
  fn eq(&self, other: &OsruGameMod) -> bool {
    self.game_mod_name == other.game_mod_name
  }
}
impl cmp::Eq for OsruGameMod {}
