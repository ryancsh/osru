use crate::global::pixel::*;
use crate::global::*;
use crate::input::{self, InputManager, InputUpdate};

use enum_iterator::IntoEnumIterator;

pub mod hitcircle;
pub mod slider;

use hitcircle::*;
use slider::*;

use sdl2::{
   image::LoadTexture,
   rect::Rect,
   render::{Texture, WindowCanvas},
};

use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum HitSuccess {
   Meh,
   Good,
   Great,
   Miss,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum HitState {
   NotDrawing,
   Ready,
   Hit(HitSuccess),
   DoneDrawing(HitSuccess),
}
impl HitState {
   pub fn hit_success(&self) -> HitSuccess {
      use HitState::*;
      match self {
         Hit(hit_success) => *hit_success,
         DoneDrawing(hit_success) => *hit_success,
         _ => panic!["{:?} does not contain hit success", self],
      }
   }

   pub fn not_yet_drawing(&self) -> bool {
      use HitState::*;
      match self {
         NotDrawing => true,
         _ => false,
      }
   }
   pub fn is_ready(&self) -> bool {
      use HitState::*;
      match self {
         Ready => true,
         _ => false,
      }
   }

   pub fn is_drawing(&self) -> bool {
      use HitState::*;
      match self {
         NotDrawing => false,
         Ready => true,
         Hit(_) => true,
         DoneDrawing(_) => false,
      }
   }

   pub fn is_hit(&self) -> bool {
      use HitState::*;
      match self {
         Hit(_) => true,
         _ => false,
      }
   }

   pub fn is_done(&self) -> bool {
      use HitState::*;
      match self {
         DoneDrawing(_) => true,
         _ => false,
      }
   }

   pub fn to_done_drawing(&self) -> Self {
      use HitState::*;
      match self {
         Hit(v) => DoneDrawing(*v),
         _ => panic!["Trying to convert {:?} to HitState::DoneDrawing", self],
      }
   }
}
impl Default for HitState {
   fn default() -> HitState {
      HitState::NotDrawing
   }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoEnumIterator)]
pub enum UpdateResult {
   InputConsumed,
   InputNotConsumed,
}

#[derive(Debug, Copy, Clone)]
pub struct HitObjectScale(pub f32);

pub const COLOUR_GREAT: Colour<u8> = Colour { r: 0, g: 180, b: 252, a: 128 };
pub const COLOUR_GOOD: Colour<u8> = Colour { r: 141, g: 221, b: 0, a: 128 };
pub const COLOUR_MEH: Colour<u8> = Colour { r: 255, g: 159, b: 0, a: 128 };
pub const COLOUR_MISS: Colour<u8> = Colour { r: 255, g: 39, b: 53, a: 128 };

pub enum HitObject {
   HitCircle(HitCircle),
   Slider(Slider),
}
impl HitObject {
   pub fn update(&mut self, update: &InputUpdate) -> UpdateResult {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.update(update),
         Slider(slider) => slider.update(update),
      }
   }

   pub fn prepare(&mut self, viewport_size: &PixRect) {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.prepare(viewport_size),
         Slider(slider) => slider.prepare(viewport_size),
      }
   }

   pub fn draw(&self, canvas: &mut WindowCanvas, texture: &mut Texture) -> HitState {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.draw(canvas, texture),
         Slider(slider) => slider.draw(canvas, texture),
      }
   }

   pub fn hit_state(&self) -> HitState {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.hit_state(),
         Slider(slider) => slider.hit_state(),
      }
   }

   pub fn time(&self) -> Duration {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.time(),
         Slider(slider) => slider.time(),
      }
   }
}

/*

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

#[derive(Debug, Copy, Clone)]
pub struct AnimationTiming {
   preempt: Duration,
   fadein: Duration,
   timing_great: Duration,
   timing_good: Duration,
   timing_meh: Duration,
}
impl AnimationTiming {
   pub fn new(
      preempt: Duration, fadein: Duration, timing_great: Duration, timing_good: Duration,
      timing_meh: Duration,
   ) -> AnimationTiming {
      AnimationTiming { preempt, fadein, timing_great, timing_good, timing_meh }
   }
   pub fn preempt_duration(&self) -> Duration {
      self.preempt
   }
   pub fn fadein_duration(&self) -> Duration {
      self.fadein
   }
   pub fn timing_great_duration(&self) -> Duration {
      self.timing_great
   }
   pub fn timing_good_duration(&self) -> Duration {
      self.timing_good
   }
   pub fn timing_meh_duration(&self) -> Duration {
      self.timing_meh
   }
   pub fn fadein_start(&self, hit_time: Duration) -> Duration {
      hit_time - self.preempt
   }
   pub fn fadein_end(&self, hit_time: Duration) -> Duration {
      hit_time + self.fadein - self.preempt
   }
   pub fn timing_meh_start(&self, hit_time: Duration) -> Duration {
      hit_time - self.timing_meh
   }
   pub fn timing_good_start(&self, hit_time: Duration) -> Duration {
      hit_time - self.timing_good
   }
   pub fn timing_great_start(&self, hit_time: Duration) -> Duration {
      hit_time - self.timing_great
   }
   pub fn timing_great_end(&self, hit_time: Duration) -> Duration {
      hit_time + self.timing_great
   }
   pub fn timing_good_end(&self, hit_time: Duration) -> Duration {
      hit_time + self.timing_good
   }
   pub fn timing_meh_end(&self, hit_time: Duration) -> Duration {
      hit_time + self.timing_meh
   }

   pub fn is_timing_great(&self, hit_time: Duration, current_time: Duration) -> bool {
      current_time > self.timing_great_start(hit_time) && current_time < self.timing_great_end(hit_time)
   }
   pub fn is_timing_good(&self, hit_time: Duration, current_time: Duration) -> bool {
      current_time > self.timing_good_start(hit_time) && current_time < self.timing_good_end(hit_time)
   }
   pub fn is_timing_meh(&self, hit_time: Duration, current_time: Duration) -> bool {
      current_time > self.timing_meh_start(hit_time) && current_time < self.timing_meh_end(hit_time)
   }
   pub fn is_timing_miss(&self, hit_time: Duration, current_time: Duration) -> bool {
      current_time > self.timing_meh_end(hit_time)
   }
}
impl Default for AnimationTiming {
   fn default() -> AnimationTiming {
      AnimationTiming {
         preempt: Duration::from_millis(500),
         fadein: Duration::from_millis(250),
         timing_great: TIMING_WINDOW_GREAT,
         timing_good: TIMING_WINDOW_GOOD,
         timing_meh: TIMING_WINDOW_MEH,
      }
   }
}
