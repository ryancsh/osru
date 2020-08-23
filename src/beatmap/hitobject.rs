use crate::global::pixel::*;
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

pub const COLOUR_GREAT: Colour<u8> = Colour { r: 0, g: 180, b: 252, a: (u8::MAX / 2) };
pub const COLOUR_GOOD: Colour<u8> = Colour { r: 141, g: 221, b: 0, a: (u8::MAX / 2) };
pub const COLOUR_MEH: Colour<u8> = Colour { r: 255, g: 159, b: 0, a: (u8::MAX / 2) };
pub const COLOUR_MISS: Colour<u8> = Colour { r: 255, g: 39, b: 53, a: (u8::MAX / 2) };

pub enum HitObject {
   HitCircle(HitCircle),
}
impl HitObject {
   pub fn update(&mut self, update: &InputUpdate, animation_timings: &AnimationTiming) -> UpdateResult {
      match self {
         HitObject::HitCircle(hit_circle) => hit_circle.update(update, animation_timings),
      }
   }

   pub fn prepare(&mut self, viewport_size: &PixRect) {
      match self {
         HitObject::HitCircle(hit_circle) => hit_circle.prepare(viewport_size),
      }
   }

   pub fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawState {
      match self {
         HitObject::HitCircle(hit_circle) => hit_circle.draw(canvas, texture),
      }
   }
   pub fn draw_state(&self) -> DrawState {
      match self {
         HitObject::HitCircle(hit_circle) => hit_circle.draw_state(),
      }
   }

   pub fn hit_success(&self) -> HitSuccess {
      match self {
         HitObject::HitCircle(hit_circle) => hit_circle.hit_success(),
      }
   }

   pub fn time(&self) -> Duration {
      match self {
         HitObject::HitCircle(hit_circle) => hit_circle.time(),
      }
   }
}

#[derive(Debug, Clone)]
pub struct HitCircle {
   pub position: Pix2D,
   pub time: Duration,
   pub new_combo: bool,
   pub combo_colours_to_skip: u32,
   pub hitsounds: OsruHitSounds,

   pub hitsample_set: i32,
   pub hitsample_additional_set: i32,
   pub hitsample_index: i32,
   pub hitsample_volume: Volume,
   pub hitsample_filename: String,

   pub draw_state: DrawState,
   pub hit_success: HitSuccess,
   pub colour: Colour<u8>,
   pub scale: bool,
   pub circle_pos_to_window: Pix2D,
}
impl HitCircle {
   pub fn update(&mut self, update: &InputUpdate, animation_timings: &AnimationTiming) -> UpdateResult {
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
               && is_mouse_pos_in_range(&self.circle_pos_to_window, last_mouse_pos, &Pix::screen_pix(150.0))
            {
               let t = (self.time.as_micros() as i128 - current_time.as_micros() as i128).abs() as u128;
               if t < animation_timings.timing_meh.as_micros() {
                  self.hit_success = Meh;
                  self.colour = COLOUR_MEH;
                  self.scale = false;

                  if t < animation_timings.timing_good.as_micros() {
                     self.hit_success = Good;
                     self.colour = COLOUR_GOOD;

                     if t < animation_timings.timing_great.as_micros() {
                        self.hit_success = Great;
                        self.colour = COLOUR_GREAT;
                     }
                  }
                  return Success;
               }
            } else if current_time > animation_timings.timing_meh + self.time {
               self.hit_success = Miss;
               self.colour = COLOUR_MISS;
               self.scale = false;
            } else {
               let mut opacity = ((current_time + animation_timings.preempt - self.time).as_nanos() * 192)
                  / animation_timings.fade_in.as_nanos();
               if opacity > 192 {
                  opacity = 192;
               }
               self.colour.a = opacity as u8;
            }
         } else {
            if current_time > self.time {
               self.colour.a = (128
                  - ((current_time - self.time).as_nanos() * 128
                     / animation_timings.timing_meh.as_nanos()
                     / 2)) as u8;
            }
         }
      }
      return Failed;
   }

   pub fn prepare(&mut self, viewport_size: &PixRect) {
      self.circle_pos_to_window = osru_pos_to_screen_pos(&self.position, viewport_size);
   }

   pub fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawState {
      use DrawState::*;
      if self.draw_state() == Drawing {
         texture.set_alpha_mod(self.colour.a);
         texture.set_color_mod(self.colour.r, self.colour.g, self.colour.b);

         let image_size = Pix2D::new(
            Pix::screen_pix(texture.query().width as f32),
            Pix::screen_pix(texture.query().height as f32),
         );
         let viewport = circle_pos_wrt_window(
            &self.position,
            &image_size,
            &PixRect::new_from_sdl2_rect(canvas.viewport()),
            self.scale,
         );
         //println!("{:?} {:?} {:?}", image_rect, &viewport, &PixRect::new_from_sdl2_rect(canvas.viewport()));
         canvas.copy(texture, None, viewport.to_sdl2_rect()).unwrap();
      }
      self.draw_state
   }

   pub fn draw_state(&self) -> DrawState {
      self.draw_state
   }

   pub fn hit_success(&self) -> HitSuccess {
      self.hit_success
   }

   pub fn time(&self) -> Duration {
      self.time
   }
}
impl Default for HitCircle {
   fn default() -> Self {
      HitCircle {
         position: Pix2D::default_screen(),
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
         circle_pos_to_window: Pix2D::default_screen(),
      }
   }
}

/*
pub trait HitObject {
   fn update(&mut self, input_update: &InputUpdate, animation_timings: &AnimationTiming) -> UpdateResult;
   fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawState;
   fn time(&self) -> Duration;
   fn draw_state(&self) -> DrawState;
   fn to_string(&self) -> String;
   fn hit_success(&self) -> HitSuccess;
   fn prepare(&mut self, viewport_size: &PixRect);
}
impl fmt::Debug for dyn HitObject {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.to_string())
   }
}

#[derive(Debug, Clone)]
pub struct HitCircle {
   pub position: Pix2D,
   pub time: Duration,
   pub new_combo: bool,
   pub combo_colours_to_skip: u32,
   pub hitsounds: OsruHitSounds,

   pub hitsample_set: i32,
   pub hitsample_additional_set: i32,
   pub hitsample_index: i32,
   pub hitsample_volume: Volume,
   pub hitsample_filename: String,

   pub draw_state: DrawState,
   pub hit_success: HitSuccess,
   pub colour: Colour<u8>,
   pub scale: bool,
   pub circle_pos_to_window: Pix2D,
}
impl Default for HitCircle {
   fn default() -> Self {
      HitCircle {
         position: Pix2D::default_screen(),
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
         circle_pos_to_window: Pix2D::default_screen(),
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
               && is_mouse_pos_in_range(&self.circle_pos_to_window, last_mouse_pos, &Pix::screen_pix(150.0))
            {
               let t = (self.time.as_micros() as i128 - current_time.as_micros() as i128).abs() as u128;
               if t < animation_timings.timing_meh.as_micros() {
                  self.hit_success = Meh;
                  self.colour = COLOUR_MEH;
                  self.scale = false;

                  if t < animation_timings.timing_good.as_micros() {
                     self.hit_success = Good;
                     self.colour = COLOUR_GOOD;

                     if t < animation_timings.timing_great.as_micros() {
                        self.hit_success = Great;
                        self.colour = COLOUR_GREAT;
                     }
                  }
                  return Success;
               }
            } else if current_time > animation_timings.timing_meh + self.time {
               self.hit_success = Miss;
               self.colour = COLOUR_MISS;
               self.scale = false;
            } else {
               let mut opacity = ((current_time + animation_timings.preempt - self.time).as_nanos() * 192)
                  / animation_timings.fade_in.as_nanos();
               if opacity > 192 {
                  opacity = 192;
               }
               self.colour.a = opacity as u8;
            }
         } else {
            if current_time > self.time {
               self.colour.a = (128
                  - ((current_time - self.time).as_nanos() * 128
                     / animation_timings.timing_meh.as_nanos()
                     / 2)) as u8;
            }
         }
      }
      return Failed;
   }

   fn prepare(&mut self, viewport_size: &PixRect) {
      self.circle_pos_to_window = osru_pos_to_screen_pos(&self.position, viewport_size);
   }

   fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> DrawState {
      use DrawState::*;
      if self.draw_state() == Drawing {
         texture.set_alpha_mod(self.colour.a);
         texture.set_color_mod(self.colour.r, self.colour.g, self.colour.b);

         let image_size = Pix2D::new(
            Pix::screen_pix(texture.query().width as f32),
            Pix::screen_pix(texture.query().height as f32),
         );
         let viewport = circle_pos_wrt_window(
            &self.position,
            &image_size,
            &PixRect::new_from_sdl2_rect(canvas.viewport()),
            self.scale,
         );
         //println!("{:?} {:?} {:?}", image_rect, &viewport, &PixRect::new_from_sdl2_rect(canvas.viewport()));
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
   pub position: Pix2D,
   pub time: Duration,
   pub new_combo: bool,
   pub combo_colours_to_skip: u32,
   pub hitsounds: OsruHitSounds,
   pub curve_type: OsruCurveType,
   pub curve_points: Vec<Pix2D>,
   pub num_slides: u32,
   pub length_of_slider: Pix,

   pub edge_sounds: Vec<i32>,
   pub edge_sets: Vec<String>,
}

pub struct Spinner {
   pub position: Pix2D,
   pub time: Duration,
   pub new_combo: bool,
   pub combo_colours_to_skip: u32,
   pub hitsounds: OsruHitSounds,
   pub end_time: Duration,

   hit_sample: Vec<i32>,
}
*/
