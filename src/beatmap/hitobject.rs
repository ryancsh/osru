use crate::global::*;

use sdl2::{
  image::LoadTexture,
  rect::Rect,
  render::{Texture, WindowCanvas},
};

use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum HitObjectState {
  NotYet,
  Meh,
  Good,
  Great,
  Hit,
  Miss,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum HitObjectDrawState {
  NotYet,
  Drawing,
  Done,
}

pub trait HitObject {
  fn update(&mut self, current_time: OsruTime);
  fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> HitObjectDrawState;
  fn draw_state(&self) -> HitObjectDrawState;
  fn to_string(&self) -> String;
}
impl fmt::Debug for HitObject {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

#[derive(Debug)]
pub struct HitCircle<'a> {
  pub position: OsruPixels,
  pub time: OsruTime,
  pub new_combo: bool,
  pub combo_colours_to_skip: usize,
  pub hitsounds: OsruHitSounds,

  pub hitsample_set: isize,
  pub hitsample_additional_set: isize,
  pub hitsample_index: isize,
  pub hitsample_volume: Volume,
  pub hitsample_filename: String,

  pub animation_timings: &'a AnimationTiming,
  pub current_time: OsruTime,
  pub current_state: HitObjectDrawState,
}
impl<'a> HitObject for HitCircle<'a> {
  fn update(&mut self, current_time: OsruTime) {
    self.current_time.copy(current_time);
    use HitObjectDrawState::*;
    let new_state = {
      if self.current_time + self.animation_timings.preempt < self.time {
        NotYet
      } else if self.current_time > self.time + self.animation_timings.timing_meh {
        Done
      } else {
        Drawing
      }
    };
    /*if new_state != self.current_state {
      println!("{:?} -> {:?}\n{:?}", self.current_state, new_state, self)
    };*/
    self.current_state = new_state;
  }

  fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> HitObjectDrawState {
    let state = self.draw_state();
    if state == HitObjectDrawState::Drawing {
      texture.set_alpha_mod({
        if self.current_time + (self.animation_timings.preempt - self.animation_timings.fade_in)
          < self.time
        {
          let opacity = (self.current_time + self.animation_timings.preempt - self.time)
            / self.animation_timings.fade_in;
          (opacity.0 % u8::MAX as usize) as u8
        } else {
          u8::MAX
        }
      });

      let image_rect = OsruRect::new(
        self.position.0,
        self.position.1,
        texture.query().width as f64,
        texture.query().height as f64,
      );
      let viewport =
        osru_pixels_to_window(&image_rect, &OsruRect::new_from_sdl2_rect(canvas.viewport()), true);
      //canvas.draw_rect(viewport.to_sdl2_rect()).unwrap();
      //canvas.draw_rect(Rect::new(350, 400, 100, 100)).unwrap();
      canvas.copy(texture, None, viewport.to_sdl2_rect()).unwrap();
    }
    state
  }

  fn draw_state(&self) -> HitObjectDrawState {
    self.current_state
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
  pub time: OsruTime,
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
  pub time: OsruTime,
  pub new_combo: bool,
  pub combo_colours_to_skip: usize,
  pub hitsounds: OsruHitSounds,
  pub end_time: OsruTime,

  hit_sample: Vec<isize>,
}
