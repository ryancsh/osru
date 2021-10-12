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
pub const AUDIO_REFERENCE_POWER: u32 = 4000;
pub const AUDIO_NORMALIZE: bool = true;

pub const INTERPOLATE_MOUSE_POSITION: bool = false;

pub const BEATMAP_TIMING_OFFSET: Duration = Duration::from_secs(2);

pub const LIMIT_FPS: bool = true;
pub const TIME_PER_FRAME: Duration = Duration::from_nanos(999_999_999 / (144 * 3));

pub static mut USER_EVENT_TYPE: u32 = 0;

#[derive(Debug, Copy, Clone)]
pub struct Colour<T> {
   pub r: T,
   pub g: T,
   pub b: T,
   pub a: T,
}

#[derive(Debug, Clone)]
pub struct Bitflags(pub u32);
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

#[derive(Debug, Copy, Clone)]
pub struct ScalingFactor(pub f32);

#[derive(Debug, Clone, IntoEnumIterator)]
pub enum OsruGameMode {
   Catch,
   Standard,
   Mania,
   Taiko,
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
pub enum OsruType {
   Integer(i32),
   Text(String),
   Decimal(f64),
   BitFlag(u32),
   List(Vec<OsruType>),
}
impl OsruType {
   fn parse_base_type<'a>(value: &str, old_value: &OsruType) -> Option<OsruType> {
      use OsruType::*;
      match old_value {
         Integer(_) => {
            let value = value.parse::<i32>();
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
            let value = value.parse::<u32>();
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

   pub fn parse_as_int(&self) -> i32 {
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

   pub fn parse_as_bitflag(&self) -> i32 {
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

#[derive(Eq, PartialEq)]
pub enum Letterboxing {
   Allow,
   Deny,
}

pub fn scaling_factor(assumed_size: &Pix2D, screen_viewport: &PixRect, letterboxing: Letterboxing) -> f32 {
   let x_ratio = screen_viewport.width().get() / assumed_size.x().get();
   let y_ratio = screen_viewport.height().get() / assumed_size.y().get();

   use Letterboxing::*;
   if x_ratio > y_ratio {
      if letterboxing == Allow {
         y_ratio
      } else {
         x_ratio
      }
   } else {
      if letterboxing == Allow {
         x_ratio
      } else {
         y_ratio
      }
   }
}

pub fn calculate_texture_viewport(
   screen_pos: &Pix2D, texture_size: &Pix2D, screen_viewport: &PixRect, image_scaling: ScalingFactor,
) -> PixRect {
   let image_size = *texture_size * image_scaling.0;
   let new_pos_x = screen_pos.x() - image_size.x() / 2;
   let new_pos_y = screen_pos.y() - image_size.y() / 2;
   PixRect::new(new_pos_x, new_pos_y, image_size.x(), image_size.y())
}

pub fn osru_pos_to_screen_pos(osru_coord: &Pix2D, viewport_size: &PixRect) -> Pix2D {
   let scaling_factor = scaling_factor(&DEFAULT_WINDOW_SIZE, viewport_size, Letterboxing::Allow);

   let new_viewport_width = DEFAULT_WINDOW_SIZE.x() * scaling_factor;
   let new_viewport_height = DEFAULT_WINDOW_SIZE.y() * scaling_factor;

   let new_viewport_offset_x = (viewport_size.width() - new_viewport_width) / 2;
   let new_viewport_offset_y = (viewport_size.height() - new_viewport_height) / 2;

   let osru_coord = osru_coord.to_screen_pix();
   let new_coord_x = osru_coord.x() * scaling_factor + new_viewport_offset_x;
   let new_coord_y = osru_coord.y() * scaling_factor + new_viewport_offset_y;

   Pix2D::new(new_coord_x, new_coord_y)
}

pub fn cursor_in_range(circle_pos: &Pix2D, cursor_pos: &Pix2D, radius: &Pix) -> bool {
   let diff = *circle_pos - *cursor_pos;
   let x_sq = diff.x().get().powi(2);
   let y_sq = diff.y().get().powi(2);
   let r_sq = radius.get().powi(2);
   if r_sq >= x_sq + y_sq {
      true
   } else {
      false
   }
}

pub fn display_background_image(
   canvas: &mut sdl2::render::WindowCanvas, texture: &mut sdl2::render::Texture, letterboxing: Letterboxing,
) {
   let screen_viewport = PixRect::new_from_sdl2_rect(canvas.viewport());

   let image_size =
      Pix2D::new(Pix::ScreenPix(texture.query().width as f32), Pix::ScreenPix(texture.query().height as f32));
   let scaling_factor = scaling_factor(&image_size, &screen_viewport, letterboxing);

   let image_size = Pix2D::new(image_size.x() * scaling_factor, image_size.y() * scaling_factor);

   let image_offset_x = (screen_viewport.width() - image_size.x()) / 2;
   let image_offset_y = (screen_viewport.height() - image_size.y()) / 2;

   let dst_viewport =
      PixRect::new(image_offset_x, image_offset_y, image_size.x(), image_size.y()).to_sdl2_rect();

   canvas.set_blend_mode(sdl2::render::BlendMode::None);
   canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
   canvas.clear();
   canvas.copy(&texture, None, Some(dst_viewport)).unwrap();
   canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
   canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, u8::MAX / 4 * 3));
   canvas.fill_rect(canvas.viewport()).unwrap();
}

/////////////////////////////

//use enum_iterator::IntoEnumIterator;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, IntoEnumIterator, Eq, PartialEq, Hash)]
pub enum TextureName {
   Background,
   ApproachCircle,
   HitCircle,
}

pub struct TextureManager<'a> {
   texture_creator: &'a TextureCreator<WindowContext>,
   textures: HashMap<TextureName, Rc<RefCell<Texture<'a>>>>,
}

impl<'s> TextureManager<'s> {
   pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> TextureManager<'a> {
      TextureManager { texture_creator, textures: HashMap::new() }
   }

   pub fn load(&mut self, name: TextureName, filename: &str) {
      if let Ok(texture) = self.texture_creator.load_texture(Path::new(filename)) {
         self.textures.insert(name, Rc::new(RefCell::new(texture)));
      } else {
         panic![];
      }
   }

   pub fn unload_all(&mut self) {
      self.textures.clear();
   }

   pub fn get(&self, name: TextureName) -> Rc<RefCell<Texture<'s>>> {
      Rc::clone(self.textures.get(&name).unwrap())
   }

   pub fn size(&self, name: TextureName) -> Pix2D {
      let texture = self.get(name);
      let texture_size = texture.borrow().query();
      Pix2D::new(Pix::ScreenPix(texture_size.width as f32), Pix::ScreenPix(texture_size.height as f32))
   }
}
