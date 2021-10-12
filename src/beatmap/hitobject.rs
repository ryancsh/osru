pub mod hitcircle;
pub mod slider;

use super::*;
use crate::global::pixel::*;
use crate::global::*;
use crate::input::{self, InputManager, InputUpdate};

use enum_iterator::IntoEnumIterator;

use hitcircle::*;
use slider::*;

use sdl2::{
   image::LoadTexture,
   rect::Rect,
   render::{Texture, WindowCanvas},
};

use std::fmt;
use std::time::{Duration, SystemTime};

pub const COLOUR_GREAT: Colour<u8> = Colour { r: 0, g: 180, b: 252, a: 128 };
pub const COLOUR_GOOD: Colour<u8> = Colour { r: 141, g: 221, b: 0, a: 128 };
pub const COLOUR_MEH: Colour<u8> = Colour { r: 255, g: 159, b: 0, a: 128 };
pub const COLOUR_MISS: Colour<u8> = Colour { r: 255, g: 39, b: 53, a: 128 };
pub const COLOUR_ACTIVE: Colour<u8> = Colour { r: 230, g: 230, b: 250, a: 128 };

pub const APPROACH_CIRCLE_MAX_SCALING: f32 = 6.0;

pub const HITCIRCLE_DEFAULT_SCALING: f32 = 2.0;
pub const HITCIRCLE_MAX_OPACITY: u128 = 128;

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoEnumIterator)]
pub enum UpdateResult {
   InputConsumed,
   InputNotConsumed,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoEnumIterator)]
pub enum DrawResult {
   Drawed,
   NotDrawed,
}
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

#[derive(Debug, Clone)]
pub enum HitObject {
   HitCircle(HitCircle),
   Slider(Slider),
}
impl HitObject {
   pub fn update(&mut self, update: &InputUpdate, timing: &AnimationTiming) -> UpdateResult {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.update(update, timing),
         Slider(slider) => slider.update(update, timing),
      }
   }

   pub fn prepare(&mut self, viewport_size: &PixRect, beatmap_settings: &BeatmapSettings) {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.prepare(viewport_size, beatmap_settings),
         Slider(slider) => slider.prepare(viewport_size, beatmap_settings),
      }
   }

   // fn reset()

   pub fn draw_self(&self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager) -> DrawResult {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.draw_self(canvas, texture_manager),
         Slider(slider) => slider.draw_self(canvas, texture_manager),
      }
   }

   pub fn draw_approach_circle(
      &self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager, timings: &AnimationTiming,
      current_time: Duration,
   ) {
      use HitObject::*;

      let self_time = self.time();
      if current_time < self_time {
         let texture = texture_manager.get(TextureName::ApproachCircle);
         let mut texture = texture.borrow_mut();

         let mut opacity = (self_time - current_time).as_secs_f32() / timings.fadein_duration().as_secs_f32();
         if opacity > 1.0 {
            opacity = 1.0;
         }
         texture.set_alpha_mod(((1.0 - opacity) * 128.0).round() as u8);

         let scaling = (self_time - current_time).as_secs_f32() / timings.preempt_duration().as_secs_f32()
            * APPROACH_CIRCLE_MAX_SCALING
            + 2.0;

         let image_size = Pix2D::new(
            Pix::screen_pix(texture.query().width as f32),
            Pix::screen_pix(texture.query().height as f32),
         );
         let viewport = calculate_texture_viewport(
            &self.screen_position(),
            &image_size,
            &PixRect::new_from_sdl2_rect(canvas.viewport()),
            ScalingFactor(scaling),
         );
         canvas.copy(&texture, None, viewport.to_sdl2_rect()).unwrap();
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

   pub fn screen_position(&self) -> Pix2D {
      use HitObject::*;
      match self {
         HitCircle(hit_circle) => hit_circle.screen_position(),
         Slider(slider) => slider.screen_position(),
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
