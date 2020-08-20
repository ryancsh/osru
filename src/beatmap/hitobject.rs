use crate::global::*;
use crate::input::{self, InputManager, InputUpdate};

use enum_iterator::IntoEnumIterator;

use sdl2::{
  image::LoadTexture,
  rect::Rect,
  render::{Texture, WindowCanvas},
};

use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(PartialEq, Eq, Debug, Copy, Clone, IntoEnumIterator)]
pub enum HitSuccess {
  Unknown,
  Meh,
  Good,
  Great,
  Miss,
}
impl Default for HitSuccess {
  fn default() -> HitSuccess {
    HitSuccess::Unknown
  }
}
#[derive(PartialEq, Eq, Debug, Copy, Clone, IntoEnumIterator)]
pub enum DrawState {
  NotYet,
  Drawing,
  Done,
}
impl Default for DrawState {
  fn default() -> DrawState {
    DrawState::NotYet
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoEnumIterator)]
pub enum UpdateResult {
  Success,
  Failed,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoEnumIterator)]
pub enum DrawResult {
  Success,
  Failed,
}

pub trait HitObject {
  fn update(
    &mut self, input_update: &InputUpdate, animation_timings: &AnimationTiming, viewport_size: &OsruRect,
  ) -> UpdateResult;
  fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawResult;
  fn time(&self) -> Duration;
  fn draw_state(&self) -> DrawState;
  fn to_string(&self) -> String;
  fn hit_success(&self) -> HitSuccess;
}
impl fmt::Debug for dyn HitObject {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

#[derive(Debug)]
pub struct HitCircle {
  pub position: OsruPixels,
  pub time: Duration,
  pub new_combo: bool,
  pub combo_colours_to_skip: usize,
  pub hitsounds: OsruHitSounds,

  pub hitsample_set: isize,
  pub hitsample_additional_set: isize,
  pub hitsample_index: isize,
  pub hitsample_volume: Volume,
  pub hitsample_filename: String,

  pub draw_state: DrawState,
  pub hit_success: HitSuccess,
  pub colour: Colour<u8>,
}
impl Default for HitCircle {
  fn default() -> Self {
    HitCircle {
      position: OsruPixels::default(),
      time: Duration::from_secs(0),
      new_combo: false,
      combo_colours_to_skip: 0,
      hitsounds: OsruHitSounds::default(),
      hitsample_set: 0,
      hitsample_additional_set: 0,
      hitsample_index: 0,
      hitsample_volume: Volume::default(),
      hitsample_filename: nstr(""),
      draw_state: DrawState::default(),
      hit_success: HitSuccess::default(),
      colour: Colour { r: u8::MAX, g: u8::MAX, b: u8::MAX, a: u8::MAX },
    }
  }
}
impl HitObject for HitCircle {
  fn update(
    &mut self, update: &InputUpdate, animation_timings: &AnimationTiming, viewport_size: &OsruRect,
  ) -> UpdateResult {
    use DrawState::*;
    use HitSuccess::*;
    use UpdateResult::*;

    if self.draw_state == Done {
      return Failed;
    }

    let current_time = *update.current_time();

    if current_time + animation_timings.preempt < self.time {
      self.draw_state = NotYet;
    } else if current_time < self.time + animation_timings.timing_meh * 2 {
      if self.hit_success == Unknown {
        self.draw_state = Drawing;
        let last_mouse_pos = update.previous_mouse_pos();
        let circle_pos = convert_osru_coordinates(&self.position, viewport_size);

        if (update.K1_pressed() || update.K2_pressed()) && is_mouse_pos_in_range(&circle_pos, last_mouse_pos, 100.0)
        {
          // TODO: check timing for Meh
          self.hit_success = Meh;
          self.colour.r = 255;
          self.colour.g = 159;
          self.colour.b = 0;
          self.colour.a = u8::MAX / 4;

          let t = (self.time.as_secs_f64() - current_time.as_secs_f64()).abs();
          if t < animation_timings.timing_good.as_secs_f64() {
            self.hit_success = Good;
            self.colour.r = 141;
            self.colour.g = 221;
            self.colour.b = 0;

            if t < animation_timings.timing_great.as_secs_f64() {
              self.hit_success = Great;
              self.colour.r = 0;
              self.colour.g = 180;
              self.colour.b = 252;
            }
          }
          return Success;
        } else {
          let mut opacity = (current_time + animation_timings.preempt - self.time).as_secs_f64()
            / animation_timings.fade_in.as_secs_f64();
          if opacity > 1.0 {
            opacity = 1.0;
          }
          self.colour.a = (opacity * 128.0).round() as u8;
        }
      }
    } else {
      self.draw_state = Done;
    }
    //println!("current time {:?}, time {:?}, state {:?}", current_time, self.time, self.draw_state);
    Failed
  }

  fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawResult {
    use DrawResult::*;
    use DrawState::*;
    if self.draw_state() == Drawing {
      texture.set_alpha_mod(self.colour.a);
      texture.set_color_mod(self.colour.r, self.colour.g, self.colour.b);

      let image_rect = OsruRect::new(
        self.position.0,
        self.position.1,
        texture.query().width as f64,
        texture.query().height as f64,
      );
      let viewport =
        osru_pixels_to_window(&image_rect, &OsruRect::new_from_sdl2_rect(canvas.viewport()), true);
      canvas.copy(texture, None, viewport.to_sdl2_rect()).unwrap();
      Success
    } else {
      Failed
    }
  }

  fn draw_state(&self) -> DrawState {
    self.draw_state
  }

  fn hit_success(&self) -> HitSuccess {
    self.hit_success
  }

  fn time(&self) -> Duration {
    self.time
  }

  fn to_string(&self) -> String {
    format!(
      "position: {:?}, time: {:?}, new_combo: {:?}, combo_colours_to_skip: {:?}, hitsounds: {:?}\n",
      self.position, self.time, self.new_combo, self.combo_colours_to_skip, self.hitsounds
    )
  }
}

pub struct Slider {
  pub position: OsruPixels,
  pub time: Duration,
  pub new_combo: bool,
  pub combo_colours_to_skip: usize,
  pub hitsounds: OsruHitSounds,
  pub curve_type: OsruCurveType,
  pub curve_points: Vec<OsruPixels>,
  pub num_slides: usize,
  pub length_of_slider: OsruPixel,

  pub edge_sounds: Vec<isize>,
  pub edge_sets: Vec<String>,
}

pub struct Spinner {
  pub position: OsruPixels,
  pub time: Duration,
  pub new_combo: bool,
  pub combo_colours_to_skip: usize,
  pub hitsounds: OsruHitSounds,
  pub end_time: Duration,

  hit_sample: Vec<isize>,
}
