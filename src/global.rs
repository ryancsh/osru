/*
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SampleSetType{ NoCustom, normal, soft, drum}
*/
pub mod pixel;

use enum_iterator::IntoEnumIterator;
use pixel::*;
use sdl2::rect::Rect;
use std::ops;
use std::time::{Duration, SystemTime};

pub const DEFAULT_MASTER_VOLUME: f32 = 0.40;
pub const DEFAULT_TRACK_VOLUME: f32 = 0.40;
pub const AUDIO_REFERENCE_POWER: usize = 4000;
pub const AUDIO_NORMALIZE: bool = true;

pub const INTERPOLATE_MOUSE_POSITION: bool = false;

pub const TIMING_WINDOW_GREAT: Duration = Duration::from_micros(80_000);
pub const TIMING_WINDOW_GREAT_MULTIPLIER: Duration = Duration::from_micros(6_000);
pub const TIMING_WINDOW_GOOD: Duration = Duration::from_micros(140_000);
pub const TIMING_WINDOW_GOOD_MULTIPLIER: Duration = Duration::from_micros(8_000);
pub const TIMING_WINDOW_MEH: Duration = Duration::from_micros(200_000);
pub const TIMING_WINDOW_MEH_MULTIPLIER: Duration = Duration::from_micros(10_000);

pub const TIME_PER_FRAME: Duration = Duration::from_nanos(999_999_999 / 144);

pub static mut USER_EVENT_TYPE: u32 = 0;
pub static DEFAULT_ANIMATION_TIMING: AnimationTiming = AnimationTiming {
   preempt: Duration::from_millis(500),
   fade_in: Duration::from_millis(300),
   timing_great: TIMING_WINDOW_GREAT,
   timing_good: TIMING_WINDOW_GOOD,
   timing_meh: TIMING_WINDOW_MEH,
};

#[derive(Debug, Clone)]
pub struct Colour<T> {
   pub r: T,
   pub g: T,
   pub b: T,
   pub a: T,
}

#[derive(Debug, Clone)]
pub struct Bitflags(pub usize);
#[derive(Debug, Clone)]
pub struct Volume(pub f32);
impl ops::Mul for Volume {
   type Output = Volume;

   fn mul(self, rhs: Self) -> Self::Output {
      Volume(((rhs.0 - 0.5).abs() + 1.0) * self.0)
   }
}
impl Default for Volume {
   fn default() -> Volume {
      Volume(0.5)
   }
}

#[derive(Debug, Clone, IntoEnumIterator)]
pub enum OsruGameMode {
   Catch,
   Standard,
   Mania,
   Taiko,
}
#[derive(Debug, Clone, IntoEnumIterator)]
pub enum OsruCurveType {
   Bezier,
   CentripetalCatmullRom,
   Linear,
   PerfectCircle,
}
#[derive(Debug, Clone, IntoEnumIterator)]
pub enum OsruHitSuccess {
   Great,
   Good,
   Meh,
   Miss,
}

#[derive(Debug, Clone, Default)]
pub struct OsruHitSounds {
   pub normal: bool,
   pub whistle: bool,
   pub finish: bool,
   pub clap: bool,
}
impl OsruHitSounds {
   pub fn from_bitflags(flags: Bitflags) -> OsruHitSounds {
      let Bitflags(flags) = flags;
      OsruHitSounds {
         normal: flags & 0b1 == 0b1 || flags == 0b0,
         whistle: flags & 0b10 == 0b10,
         finish: flags & 0b100 == 0b100,
         clap: flags & 0b1000 == 0b1000,
      }
   }
}

#[derive(Debug, Clone)]
pub struct OsruOD(pub f64);
#[derive(Debug, Clone)]
pub struct OsruAR(pub f64);
#[derive(Debug, Clone)]
pub struct OsruCS(pub f64);

#[derive(Debug, Clone)]
pub struct AnimationTiming {
   pub preempt: Duration,
   pub fade_in: Duration,
   pub timing_great: Duration,
   pub timing_good: Duration,
   pub timing_meh: Duration,
}

pub struct TimeBarrier {}

#[derive(Debug, Clone)]
pub enum OsruType {
   Integer(isize),
   Text(String),
   Decimal(f64),
   BitFlag(usize),
   List(Vec<OsruType>),
}
impl OsruType {
   fn parse_base_type<'a>(value: &str, old_value: &OsruType) -> Option<OsruType> {
      use OsruType::*;
      match old_value {
         Integer(_) => {
            let value = value.parse::<isize>();
            if let Ok(value) = value {
               Some(Integer(value))
            } else {
               None
            }
         }
         Decimal(_) => {
            let value = value.parse::<f64>();
            if let Ok(value) = value {
               Some(Decimal(value))
            } else {
               None
            }
         }
         BitFlag(_) => {
            let value = value.parse::<usize>();
            if let Ok(value) = value {
               Some(BitFlag(value))
            } else {
               None
            }
         }
         Text(_) => {
            let value = value.parse::<String>();
            if let Ok(value) = value {
               Some(Text(value))
            } else {
               None
            }
         }
         /*
         Time(_) => {
             let value = value.parse::<isize>();
             if let Ok(value) = value{
                 let mut value = value;
                 if value < 0 {value = 0}
                 Some(Time(value as usize))
             }else {None}
         },
         */
         List(v) => panic![],
      }
   }

   pub fn parse_type(value: &str, old_value: &OsruType, separator: Option<&str>) -> Option<OsruType> {
      use OsruType::*;

      let separator = match separator {
         Some(x) => x,
         _ => ",",
      };

      match old_value {
         List(vec) => {
            if let Some(value_in_vec) = vec.get(0) {
               let mut v = vec![];
               let values_parsed = parse_list(value, separator);

               for value in values_parsed {
                  if let Some(result) = OsruType::parse_base_type(value, value_in_vec) {
                     v.push(result);
                  }
               }
               if v.len() > 0 {
                  Some(List(v))
               } else {
                  None
               }
            } else {
               panic![]
            }
         }
         _ => OsruType::parse_base_type(value, old_value),
      }
   }

   pub fn parse_as_int(&self) -> isize {
      if let OsruType::Integer(value) = self {
         *value
      } else {
         panic![]
      }
   }

   pub fn parse_as_str(&self) -> &str {
      if let OsruType::Text(value) = self {
         value
      } else {
         panic![]
      }
   }

   pub fn parse_as_dec(&self) -> f64 {
      if let OsruType::Decimal(value) = self {
         *value
      } else {
         panic![]
      }
   }

   pub fn parse_as_bitflag(&self) -> isize {
      if let OsruType::Integer(value) = self {
         *value
      } else {
         panic![]
      }
   }
   /*
   pub fn parse_as_list(&self) -> isize{
       if let OsruType::List(value) = self{
           *value
       } else{panic![]}
   }
   */
}

//------------------

pub fn parse_key_value<'a>(line: &'a str, separator: &str) -> Option<(&'a str, &'a str)> {
   let split: Vec<&str> = line.trim().split(separator).collect();
   if split.len() == 2 {
      let result = Some((split[0].trim(), split[1].trim()));
      return result;
   }
   None
}

pub fn parse_list<'a>(line: &'a str, separator: &str) -> Vec<&'a str> {
   line.trim().split(separator).map(|x| x.trim()).collect()
}

pub fn nstr(s: &str) -> String {
   String::from(s)
}

pub fn mergestr(s1: &str, s2: &str) -> String {
   let mut result = s1.to_string();
   result.push_str(s2);
   result
}

pub fn circle_pos_wrt_window(
   circle_pos: &Pix2D, image_size: &Pix2D, viewport_size: &PixRect, scale_image: bool,
) -> PixRect {
   const SCALING_RATIO_PRECISION: isize = 512;

   let x_ratio =
      (viewport_size.width().get_mpix() * SCALING_RATIO_PRECISION) / DEFAULT_WINDOW_SIZE_X.get_mpix();
   let y_ratio =
      (viewport_size.height().get_mpix() * SCALING_RATIO_PRECISION) / DEFAULT_WINDOW_SIZE_Y.get_mpix();

   let scaling_factor = {
      if x_ratio > y_ratio {
         y_ratio
      } else {
         x_ratio
      }
   };

   let new_viewport_width = (DEFAULT_WINDOW_SIZE_X * scaling_factor) / SCALING_RATIO_PRECISION;
   let new_viewport_height = (DEFAULT_WINDOW_SIZE_Y * scaling_factor) / SCALING_RATIO_PRECISION;

   let new_viewport_offset_x = (viewport_size.width() - new_viewport_width) / 2;
   let new_viewport_offset_y = (viewport_size.height() - new_viewport_height) / 2;

   let image_scaling = {
      if scale_image {
         scaling_factor
      } else {
         SCALING_RATIO_PRECISION
      }
   };

   let new_image_width = (image_size.x() * image_scaling) / SCALING_RATIO_PRECISION;
   let new_image_height = (image_size.y() * image_scaling) / SCALING_RATIO_PRECISION;

   let mut new_image_pos_x =
      (Pix::screen_mpix(circle_pos.x().get_mpix()) * scaling_factor) / SCALING_RATIO_PRECISION;
   new_image_pos_x = new_image_pos_x - new_image_width / 2 + new_viewport_offset_x;

   let mut new_image_pos_y =
      (Pix::screen_mpix(circle_pos.y().get_mpix()) * scaling_factor) / SCALING_RATIO_PRECISION;
   new_image_pos_y = new_image_pos_y - new_image_height / 2 + new_viewport_offset_y;

   PixRect::new(new_image_pos_x, new_image_pos_y, new_image_width, new_image_height)
}

pub fn convert_osru_coordinates(osru_coord: &Pix2D, viewport_size: &PixRect) -> Pix2D {
   const SCALING_RATIO_PRECISION: isize = 512;

   let x_ratio =
      (viewport_size.width().get_mpix() * SCALING_RATIO_PRECISION) / DEFAULT_WINDOW_SIZE_X.get_mpix();
   let y_ratio =
      (viewport_size.height().get_mpix() * SCALING_RATIO_PRECISION) / DEFAULT_WINDOW_SIZE_Y.get_mpix();
   let scaling_factor = {
      if x_ratio > y_ratio {
         y_ratio
      } else {
         x_ratio
      }
   };
   let new_viewport_width = (DEFAULT_WINDOW_SIZE_X * scaling_factor) / SCALING_RATIO_PRECISION;
   let new_viewport_height = (DEFAULT_WINDOW_SIZE_Y * scaling_factor) / SCALING_RATIO_PRECISION;

   let new_viewport_offset_x = (viewport_size.width() - new_viewport_width) / 2;
   let new_viewport_offset_y = (viewport_size.height() - new_viewport_height) / 2;

   let new_coord_x = (osru_coord.x() * scaling_factor).get_mpix() / SCALING_RATIO_PRECISION
      + new_viewport_offset_x.get_mpix();
   let new_coord_y = (osru_coord.y() * scaling_factor).get_mpix() / SCALING_RATIO_PRECISION
      + new_viewport_offset_y.get_mpix();

   Pix2D::new(Pix::screen_mpix(new_coord_x), Pix::screen_mpix(new_coord_y))
}

pub fn is_mouse_pos_in_range(circle_pos: &Pix2D, mouse_pos: &Pix2D, radius: &Pix) -> bool {
   let diff = *circle_pos - *mouse_pos;
   let x_sq = diff.x().get_pix_trunc().pow(2);
   let y_sq = diff.y().get_pix_trunc().pow(2);
   let r_sq = radius.get_pix_trunc().pow(2);
   if r_sq >= x_sq + y_sq {
      true
   } else {
      false
   }
}

pub fn display_background_image(
   canvas: &mut sdl2::render::WindowCanvas, texture: &mut sdl2::render::Texture, allow_letterbox: bool,
) {
   const SCALING_RATIO_PRECISION: isize = 512;

   let mut image_width = texture.query().width as isize;
   let mut image_height = texture.query().height as isize;

   let viewport_width = canvas.viewport().width() as isize;
   let viewport_height = canvas.viewport().height() as isize;

   let scaling_factor_x = (viewport_width * SCALING_RATIO_PRECISION) / image_width;
   let scaling_factor_y = (viewport_height * SCALING_RATIO_PRECISION) / image_height;

   let scaling_factor = {
      if scaling_factor_x > scaling_factor_y {
         if allow_letterbox {
            scaling_factor_y
         } else {
            scaling_factor_x
         }
      } else {
         if allow_letterbox {
            scaling_factor_x
         } else {
            scaling_factor_y
         }
      }
   };

   image_width = (image_width * scaling_factor) / SCALING_RATIO_PRECISION;
   image_height = (image_height * scaling_factor) / SCALING_RATIO_PRECISION;

   let image_offset_x = (viewport_width - image_width) / 2;
   let image_offset_y = (viewport_height - image_height) / 2;

   let dst_viewport = PixRect::new(
      Pix::screen_pix(image_offset_x),
      Pix::screen_pix(image_offset_y),
      Pix::screen_pix(image_width),
      Pix::screen_pix(image_height),
   )
   .to_sdl2_rect();

   canvas.copy(&texture, None, Some(dst_viewport)).unwrap();
}
