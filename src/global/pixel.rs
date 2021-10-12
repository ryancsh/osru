use std::mem;
use std::ops;

pub const DEFAULT_WINDOW_SIZE: Pix2D = Pix2D { x: Pix::ScreenPix(640.0), y: Pix::ScreenPix(480.0) };
//pub const DEFAULT_WINDOW_SIZE_Y: Pix = Pix::ScreenPix(480.0);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Pix {
   OsruPix(f32),
   ScreenPix(f32),
}
impl Pix {
   pub fn screen_pix(value: f32) -> Pix {
      Pix::ScreenPix(value)
   }
   pub fn osru_pix(value: f32) -> Pix {
      Pix::OsruPix(value)
   }
   pub fn get_round(&self) -> i32 {
      self.get().round() as i32
   }
   pub fn get_trunc(&self) -> i32 {
      self.get() as i32
   }
   pub fn get(&self) -> f32 {
      use Pix::*;
      match self {
         OsruPix(value) => *value,
         ScreenPix(value) => *value,
      }
   }
   pub fn to_osru_pix(&self) -> Pix {
      use Pix::*;
      match self {
         ScreenPix(value) => Pix::OsruPix(*value),
         OsruPix(_) => panic!("Can't convert {:?} to osru pixels", self),
      }
   }
   pub fn to_screen_pix(&self) -> Pix {
      use Pix::*;
      match self {
         OsruPix(value) => Pix::ScreenPix(*value),
         ScreenPix(_) => panic!("Can't convert {:?} to screen pixels", self),
      }
   }
}
impl ops::Add for Pix {
   type Output = Pix;
   fn add(self, rhs: Self) -> Self::Output {
      use Pix::*;
      match (self, rhs) {
         (OsruPix(a), OsruPix(b)) => OsruPix(a + b),
         (ScreenPix(a), ScreenPix(b)) => ScreenPix(a + b),
         _ => panic!("Adding different types of pixels: {:?} {:?}", self, rhs),
      }
   }
}
impl ops::Sub for Pix {
   type Output = Pix;
   fn sub(self, rhs: Self) -> Self::Output {
      use Pix::*;
      match (self, rhs) {
         (OsruPix(a), OsruPix(b)) => OsruPix(a - b),
         (ScreenPix(a), ScreenPix(b)) => ScreenPix(a - b),
         _ => panic!("Subtracting different types of pixels: {:?} {:?}", self, rhs),
      }
   }
}
impl ops::Mul<isize> for Pix {
   type Output = Pix;
   fn mul(self, rhs: isize) -> Self::Output {
      use Pix::*;
      match self {
         OsruPix(a) => OsruPix(a * rhs as f32),
         ScreenPix(a) => ScreenPix(a * rhs as f32),
      }
   }
}
impl ops::Mul<f32> for Pix {
   type Output = Pix;
   fn mul(self, rhs: f32) -> Self::Output {
      use Pix::*;
      match self {
         OsruPix(a) => OsruPix(a * rhs),
         ScreenPix(a) => ScreenPix(a * rhs),
      }
   }
}
impl ops::Div<isize> for Pix {
   type Output = Pix;
   fn div(self, rhs: isize) -> Self::Output {
      use Pix::*;
      match self {
         OsruPix(a) => OsruPix(a / rhs as f32),
         ScreenPix(a) => ScreenPix(a / rhs as f32),
      }
   }
}
impl ops::Div<f32> for Pix {
   type Output = Pix;
   fn div(self, rhs: f32) -> Self::Output {
      use Pix::*;
      match self {
         OsruPix(a) => OsruPix(a / rhs),
         ScreenPix(a) => ScreenPix(a / rhs),
      }
   }
}

#[derive(Clone, Debug, Copy, PartialOrd, PartialEq)]
pub struct Pix2D {
   x: Pix,
   y: Pix,
}
impl Pix2D {
   fn _validate(x: &Pix, y: &Pix) {
      #[cold]
      if mem::discriminant(x) != mem::discriminant(y) {
         panic![];
      }
   }
   pub fn new(x: Pix, y: Pix) -> Pix2D {
      //Pix2D::_validate(&x, &y);
      Pix2D { x, y }
   }
   pub fn set_pix(&mut self, x: Pix, y: Pix) {
      //Pix2D::_validate(&x, &y);
      //Pix2D::_validate(&x, &self.x);
      self.x = x;
      self.y = y;
   }
   pub fn default_screen() -> Pix2D {
      Pix2D { x: Pix::ScreenPix(0.0), y: Pix::ScreenPix(0.0) }
   }
   pub fn default_osru() -> Pix2D {
      Pix2D { x: Pix::OsruPix(0.0), y: Pix::OsruPix(0.0) }
   }
   pub fn x(&self) -> Pix {
      self.x
   }
   pub fn y(&self) -> Pix {
      self.y
   }

   pub fn to_osru_pix(&self) -> Pix2D {
      Pix2D { x: self.x().to_osru_pix(), y: self.y().to_osru_pix() }
   }
   pub fn to_screen_pix(&self) -> Pix2D {
      Pix2D { x: self.x().to_screen_pix(), y: self.y().to_screen_pix() }
   }
}
impl ops::Add for Pix2D {
   type Output = Pix2D;
   fn add(self, rhs: Self) -> Self::Output {
      Pix2D { x: self.x + rhs.x, y: self.y + rhs.y }
   }
}
impl ops::Sub for Pix2D {
   type Output = Pix2D;
   fn sub(self, rhs: Self) -> Self::Output {
      Pix2D { x: self.x - rhs.x, y: self.y - rhs.y }
   }
}
impl ops::Mul<f32> for Pix2D {
   type Output = Pix2D;
   fn mul(self, rhs: f32) -> Self::Output {
      Pix2D { x: self.x * rhs, y: self.x * rhs }
   }
}
impl ops::Div<f32> for Pix2D {
   type Output = Pix2D;
   fn div(self, rhs: f32) -> Self::Output {
      Pix2D { x: self.x / rhs, y: self.x / rhs }
   }
}

#[derive(Debug, Clone)]
pub struct PixRect {
   x: Pix,
   y: Pix,
   width: Pix,
   height: Pix,
}
impl PixRect {
   fn _validate(x: &Pix, y: &Pix, width: &Pix, height: &Pix) {
      if !((mem::discriminant(x) == mem::discriminant(y))
         && (mem::discriminant(width) == mem::discriminant(height))
         && (mem::discriminant(x) == mem::discriminant(width)))
      {
         panic!(
            "Different types of pixels: x = {:?}, y = {:?}, width = {:?}, height = {:?}",
            x, y, width, height
         );
      }
   }
   pub fn new(x: Pix, y: Pix, width: Pix, height: Pix) -> PixRect {
      //PixRect::_validate(&x, &y, &width, &height);
      PixRect { x, y, width, height }
   }

   pub fn new_from_sdl2_rect(sdl2_rect: sdl2::rect::Rect) -> PixRect {
      PixRect::new(
         Pix::screen_pix(sdl2_rect.x() as f32),
         Pix::screen_pix(sdl2_rect.y() as f32),
         Pix::screen_pix(sdl2_rect.width() as f32),
         Pix::screen_pix(sdl2_rect.height() as f32),
      )
   }

   pub fn to_sdl2_rect(&self) -> sdl2::rect::Rect {
      sdl2::rect::Rect::new(
         self.x().get_round() as i32,
         self.y().get_round() as i32,
         self.width().get_round() as u32,
         self.height().get_round() as u32,
      )
   }

   pub fn x(&self) -> Pix {
      self.x
   }
   pub fn y(&self) -> Pix {
      self.y
   }
   pub fn width(&self) -> Pix {
      self.width
   }
   pub fn height(&self) -> Pix {
      self.height
   }
}
