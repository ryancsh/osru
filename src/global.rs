/*
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SampleSetType{ NoCustom, normal, soft, drum}
*/

use std::ops;

pub const DEFAULT_MASTER_VOLUME: f32 = 0.40;
pub const DEFAULT_TRACK_VOLUME: f32 = 0.40;
pub const AUDIO_REFERENCE_POWER: usize = 4000;

pub const TIMING_WINDOW_GREAT: OsruTime = OsruTime(79_500);
pub const TIMING_WINDOW_GREAT_MULTIPLIER: OsruTime = OsruTime(6_000);
pub const TIMING_WINDOW_GOOD: OsruTime = OsruTime(139_500);
pub const TIMING_WINDOW_GOOD_MULTIPLIER: OsruTime = OsruTime(8_000);
pub const TIMING_WINDOW_MEH: OsruTime = OsruTime(199_500);
pub const TIMING_WINDOW_MEH_MULTIPLIER: OsruTime = OsruTime(10_000);

pub const DEFAULT_WINDOW_SIZE: (usize, usize) = (640, 480);

#[derive(Debug, Clone, Copy)]
pub struct Colour<T>(pub T, pub T, pub T, pub T);
#[derive(Debug, Clone, Copy)]
pub struct Bitflags(pub usize);
#[derive(Debug, Clone, Copy)]
pub struct Volume(pub f32);
impl ops::Mul for Volume {
  type Output = Volume;

  fn mul(self, rhs: Self) -> Self::Output {
    Volume(((rhs.0 - 0.5).abs() + 1.0) * self.0)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum OsruGameMode {
  Catch,
  Standard,
  Mania,
  Taiko,
}
#[derive(Debug, Clone, Copy)]
pub enum OsruCurveType {
  Bezier,
  CentripetalCatmullRom,
  Linear,
  PerfectCircle,
}
#[derive(Debug, Clone, Copy)]
pub enum OsruHitSuccess {
  Great,
  Good,
  Meh,
  Miss,
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum OsruGameModName {
  None,
  Easy,
  HardRock,
  DoubleTime,
  HalfTime,
  NoFail,
  SuddenDeath,
  Perfect,
  Hidden,
  FlashLight,
  //Scoring
  ScoreOsru,
  ScoreV1,
  ScoreV2,
  //Special
  Relax,
  AutoPilot,
  SpunOut,
  Auto,
}

#[derive(Debug, Clone, Copy)]
pub struct OsruPixel(pub f64);
#[derive(Debug, Clone, Copy)]
pub struct OsruPixels(pub f64, pub f64);
#[derive(Debug, Clone, Copy)]
pub struct Pixels(pub usize, pub usize);
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct OsruOD(pub f64);
#[derive(Debug, Clone, Copy)]
pub struct OsruAR(pub f64);
#[derive(Debug, Clone, Copy)]
pub struct OsruCS(pub f64);

#[derive(Debug, Clone, Copy, Default)]
pub struct AnimationTiming {
  pub preempt: OsruTime,
  pub fade_in: OsruTime,
  pub timing_great: OsruTime,
  pub timing_good: OsruTime,
  pub timing_meh: OsruTime,
}

#[derive(Debug, Clone, Copy, Default, Ord, Eq, PartialOrd, PartialEq)]
pub struct OsruTime(pub usize); //micro seconds
impl OsruTime {
  pub fn us(us: usize) -> OsruTime {
    OsruTime(us)
  }

  pub fn ms(ms: usize) -> OsruTime {
    OsruTime::us(ms * 1000)
  }

  pub fn s(s: usize) -> OsruTime {
    OsruTime::ms(s * 1000)
  }

  pub fn us_f(us: f64) -> OsruTime {
    OsruTime(us as usize)
  }

  pub fn ms_f(ms: f64) -> OsruTime {
    OsruTime::us_f(ms * 1000.0)
  }

  pub fn s_f(s: f64) -> OsruTime {
    OsruTime::ms_f(s * 1000.0)
  }

  pub fn from_duration(d: std::time::Duration) -> OsruTime {
    OsruTime::us(d.as_micros() as usize)
  }

  pub fn copy(&mut self, other: OsruTime) {
    self.0 = other.0
  }
}
impl ops::Add for OsruTime {
  type Output = OsruTime;

  fn add(self, rhs: Self) -> Self::Output {
    OsruTime(self.0 + rhs.0)
  }
}
impl ops::Sub for OsruTime {
  type Output = OsruTime;

  fn sub(self, rhs: Self) -> Self::Output {
    OsruTime(self.0 - rhs.0)
  }
}
impl ops::Mul for OsruTime {
  type Output = OsruTime;
  fn mul(self, rhs: Self) -> Self::Output {
    OsruTime(self.0 * rhs.0)
  }
}
impl ops::Div for OsruTime {
  type Output = OsruTime;
  fn div(self, rhs: Self) -> Self::Output {
    OsruTime(self.0 / rhs.0)
  }
}

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

  pub fn parse_type(
    value: &str, old_value: &OsruType, separator: Option<&str>,
  ) -> Option<OsruType> {
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

pub fn osru_pixels_to_window(
  image: &OsruRect, viewport_size: &OsruRect, scale_image: bool,
) -> OsruRect {
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

pub fn display_background_image(
  canvas: &mut sdl2::render::WindowCanvas, texture: &mut sdl2::render::Texture,
  allow_letterbox: bool,
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
