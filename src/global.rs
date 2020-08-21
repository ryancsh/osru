/*
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SampleSetType{ NoCustom, normal, soft, drum}
*/
use enum_iterator::IntoEnumIterator;
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

//pub const TIME_PER_FRAME: Duration = Duration::from_nanos(999_999_999 / 576);

pub const DEFAULT_WINDOW_SIZE: (usize, usize) = (640, 480);

pub static mut USER_EVENT_TYPE: u32 = 0;
pub static DEFAULT_ANIMATION_TIMING: AnimationTiming = AnimationTiming {
   preempt: Duration::from_millis(500),
   fade_in: Duration::from_millis(400),
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
pub struct OsruPixel(pub f64);
#[derive(Debug, Clone, Default)]
pub struct OsruPixels(pub f64, pub f64);
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pixels(pub f64, pub f64);
impl ops::Add for Pixels {
   type Output = Pixels;
   fn add(self, rhs: Self) -> Self::Output {
      Pixels(self.0 + rhs.0, self.1 + rhs.1)
   }
}
impl ops::Sub for Pixels {
   type Output = Pixels;
   fn sub(self, rhs: Self) -> Self::Output {
      Pixels(self.0 - rhs.0, self.1 - rhs.1)
   }
}

#[derive(Debug, Clone)]
pub struct OsruRect {
   pub x: f64,
   pub y: f64,
   pub width: f64,
   pub height: f64,
}
impl OsruRect {
   pub fn new(x: f64, y: f64, width: f64, height: f64) -> OsruRect {
      OsruRect { x, y, width, height }
   }

   pub fn new_from_sdl2_rect(sdl2_rect: sdl2::rect::Rect) -> OsruRect {
      OsruRect::new(
         sdl2_rect.x() as f64,
         sdl2_rect.y() as f64,
         sdl2_rect.width() as f64,
         sdl2_rect.height() as f64,
      )
   }

   pub fn to_sdl2_rect(&self) -> sdl2::rect::Rect {
      sdl2::rect::Rect::new(
         self.x.round() as i32,
         self.y.round() as i32,
         self.width.round() as u32,
         self.height.round() as u32,
      )
   }
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

pub fn osru_pixels_to_window(image: &OsruRect, viewport_size: &OsruRect, scale_image: bool) -> OsruRect {
   let x_ratio = viewport_size.width / DEFAULT_WINDOW_SIZE.0 as f64;
   let y_ratio = viewport_size.height / DEFAULT_WINDOW_SIZE.1 as f64;

   let scaling_factor = {
      if x_ratio > y_ratio {
         y_ratio
      } else {
         x_ratio
      }
   };
   //TODO fix letter boxing
   let mut new_image_width = image.width as f64;
   let mut new_image_height = image.height as f64;

   if scale_image {
      new_image_width *= scaling_factor;
      new_image_height *= scaling_factor;
   }

   let new_viewport_width = DEFAULT_WINDOW_SIZE.0 as f64 * scaling_factor;
   let new_viewport_height = DEFAULT_WINDOW_SIZE.1 as f64 * scaling_factor;

   let new_viewport_offset_x = (viewport_size.width - new_viewport_width) / 2.0;
   let new_viewport_offset_y = (viewport_size.height - new_viewport_height) / 2.0;

   let mut new_image_offset_x = image.x * scaling_factor + new_viewport_offset_x;
   let mut new_image_offset_y = image.y * scaling_factor + new_viewport_offset_y;

   new_image_offset_x -= new_image_width / 2.0;
   new_image_offset_y -= new_image_height / 2.0;

   OsruRect::new(new_image_offset_x, new_image_offset_y, new_image_width, new_image_height)
}

pub fn convert_osru_coordinates(osru_coord: &OsruPixels, viewport_size: &OsruRect) -> Pixels {
   let x_ratio = viewport_size.width / DEFAULT_WINDOW_SIZE.0 as f64;
   let y_ratio = viewport_size.height / DEFAULT_WINDOW_SIZE.1 as f64;
   let scaling_factor = {
      if x_ratio > y_ratio {
         y_ratio
      } else {
         x_ratio
      }
   };
   let new_viewport_width = DEFAULT_WINDOW_SIZE.0 as f64 * scaling_factor;
   let new_viewport_height = DEFAULT_WINDOW_SIZE.1 as f64 * scaling_factor;

   let new_viewport_offset_x = (viewport_size.width - new_viewport_width) / 2.0;
   let new_viewport_offset_y = (viewport_size.height - new_viewport_height) / 2.0;

   let new_coord_x = osru_coord.0 * scaling_factor + new_viewport_offset_x;
   let new_coord_y = osru_coord.1 * scaling_factor + new_viewport_offset_y;

   Pixels(new_coord_x, new_coord_y)
}

pub fn is_mouse_pos_in_range(circle_pos: &Pixels, mouse_pos: &Pixels, radius: f64) -> bool {
   let x_sq = (circle_pos.0 - mouse_pos.0).powi(2);
   let y_sq = (circle_pos.1 - mouse_pos.1).powi(2);
   let r_sq = radius * radius;
   if r_sq >= x_sq + y_sq {
      true
   } else {
      false
   }
}

pub fn display_background_image(
   canvas: &mut sdl2::render::WindowCanvas, texture: &mut sdl2::render::Texture, allow_letterbox: bool,
) {
   let mut image_width = texture.query().width as f64;
   let mut image_height = texture.query().height as f64;

   let viewport_width = canvas.viewport().width() as f64;
   let viewport_height = canvas.viewport().height() as f64;

   let scaling_factor_x = viewport_width / image_width;
   let scaling_factor_y = viewport_height / image_height;

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

   image_width *= scaling_factor;
   image_height *= scaling_factor;

   let image_offset_x = (viewport_width - image_width) / 2.0;
   let image_offset_y = (viewport_height - image_height) / 2.0;

   canvas
      .copy(
         &texture,
         None,
         OsruRect::new(image_offset_x, image_offset_y, image_width, image_height).to_sdl2_rect(),
      )
      .unwrap();
}
