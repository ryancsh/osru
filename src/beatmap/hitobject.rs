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

pub trait HitObject {
   fn update(&mut self, input_update: &InputUpdate, animation_timings: &AnimationTiming) -> UpdateResult;
   fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawState;
   fn time(&self) -> Duration;
   fn draw_state(&self) -> DrawState;
   fn to_string(&self) -> String;
   fn hit_success(&self) -> HitSuccess;
   fn prepare(&mut self, viewport_size: &OsruRect);
}
impl fmt::Debug for dyn HitObject {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.to_string())
   }
}

#[derive(Debug, Clone)]
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
   pub scale: bool,
   pub circle_pos_to_window: Pixels,
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
         scale: true,
         circle_pos_to_window: Pixels::default(),
      }
   }
}
impl HitObject for HitCircle {
   fn update(&mut self, update: &InputUpdate, animation_timings: &AnimationTiming) -> UpdateResult {
      use DrawState::*;
      use HitSuccess::*;
      use UpdateResult::*;

      let current_time = *update.current_time();

      if self.draw_state == Done || current_time > self.time + animation_timings.timing_meh * 2 {
         self.draw_state = Done;
      } else if current_time + animation_timings.preempt < self.time {
         self.draw_state = NotYet;
      } else {
         self.draw_state = Drawing;
         if self.hit_success == Unknown {
            let last_mouse_pos = update.previous_mouse_pos();

            if (update.K1_pressed() || update.K2_pressed())
               && is_mouse_pos_in_range(&self.circle_pos_to_window.clone(), last_mouse_pos, 200.0)
            {
               let t = (self.time.as_secs_f32() - current_time.as_secs_f32()).abs();
               if t < animation_timings.timing_meh.as_secs_f32(){
                  self.hit_success = Meh;
                  self.colour.r = 255;
                  self.colour.g = 159;
                  self.colour.b = 0;
                  self.colour.a = u8::MAX / 2;
                  self.scale = false;

                  if t < animation_timings.timing_good.as_secs_f32() {
                     self.hit_success = Good;
                     self.colour.r = 141;
                     self.colour.g = 221;
                     self.colour.b = 0;

                     if t < animation_timings.timing_great.as_secs_f32() {
                        self.hit_success = Great;
                        self.colour.r = 0;
                        self.colour.g = 180;
                        self.colour.b = 252;
                     }
                  }
                  return Success;
               }
            } else if current_time > animation_timings.timing_meh + self.time {
               self.hit_success = Miss;
               self.colour.r = 255;
               self.colour.g = 39;
               self.colour.b = 53;
               self.colour.a = u8::MAX / 2;
               self.scale = false;
            } else {
               let mut opacity = (current_time + animation_timings.preempt - self.time).as_nanos() * 128
                  / animation_timings.fade_in.as_nanos();
               if opacity > 128 {
                  opacity = 128;
               }
               self.colour.a = opacity as u8;
            }
         }
         else{
            if current_time > self.time { self.colour.a = (128 - ((current_time - self.time).as_nanos() * 128 /animation_timings.timing_meh.as_nanos() / 2)) as u8;}
         }
      }
      return Failed;
   }

   fn prepare(&mut self, viewport_size: &OsruRect) {
      self.circle_pos_to_window = convert_osru_coordinates(&self.position, viewport_size);
   }

   fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawState {
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
            osru_pixels_to_window(&image_rect, &OsruRect::new_from_sdl2_rect(canvas.viewport()), self.scale);
         canvas.copy(texture, None, viewport.to_sdl2_rect()).unwrap();
      }
      self.draw_state
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
