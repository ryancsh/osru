use std::mem;
use std::ops;

pub const PIXEL_PRECISION: isize = 1024;

pub const DEFAULT_WINDOW_SIZE_X: Pix = Pix::ScreenPix(640 * PIXEL_PRECISION);
pub const DEFAULT_WINDOW_SIZE_Y: Pix = Pix::ScreenPix(480 * PIXEL_PRECISION);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pix {
   OsruPix(isize),
   ScreenPix(isize),
}
impl Pix {
   pub fn screen_pix(value: isize) -> Pix {
      Pix::screen_mpix(value * PIXEL_PRECISION as isize)
   }
   pub fn screen_mpix(value: isize) -> Pix {
      Pix::ScreenPix(value)
   }
   pub fn osru_pix(value: isize) -> Pix {
      Pix::osru_mpix(value * PIXEL_PRECISION as isize)
   }
   pub fn osru_mpix(value: isize) -> Pix {
      Pix::OsruPix(value)
   }

   pub fn get_pix_round(&self) -> isize {
      (self.get_mpix() as f32 / PIXEL_PRECISION as f32).round() as isize
   }
   pub fn get_pix_trunc(&self) -> isize {
      self.get_mpix() / PIXEL_PRECISION as isize
   }
   pub fn get_mpix(&self) -> isize {
      use Pix::*;
      match self {
         OsruPix(value) => *value,
         ScreenPix(value) => *value,
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
         _ => panic![],
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
         _ => panic![],
      }
   }
}
/*
impl ops::Mul for Pix {
   type Output = Pix;
   fn mul(self, rhs: Self) -> Self::Output {
      use Pix::*;
      match (self, rhs) {
         (OsruPix(a), OsruPix(b)) => OsruPix(a * b / PIXEL_PRECISION),
         (ScreenPix(a), ScreenPix(b)) => ScreenPix(a * b / PIXEL_PRECISION),
         _ => panic![],
      }
   }
}*/
impl ops::Mul<isize> for Pix {
   type Output = Pix;
   fn mul(self, rhs: isize) -> Self::Output {
      use Pix::*;
      match self {
         OsruPix(a) => OsruPix(a * rhs),
         ScreenPix(a) => ScreenPix(a * rhs),
      }
   }
}
/*
impl ops::Div for Pix {
   type Output = Pix;
   fn div(self, rhs: Self) -> Self::Output {
      use Pix::*;
      match (self, rhs) {
         (OsruPix(a), OsruPix(b)) => OsruPix(a * PIXEL_PRECISION / b),
         (ScreenPix(a), ScreenPix(b)) => ScreenPix(a * PIXEL_PRECISION/ b),
         _ => panic![],
      }
   }
}*/
impl ops::Div<isize> for Pix {
   type Output = Pix;
   fn div(self, rhs: isize) -> Self::Output {
      use Pix::*;
      match self {
         OsruPix(a) => OsruPix(a / rhs),
         ScreenPix(a) => ScreenPix(a / rhs),
      }
   }
}

#[derive(Clone, Debug, Copy)]
pub struct Pix2D {
   x: Pix,
   y: Pix,
}
impl Pix2D {
   fn validate(x: &Pix, y: &Pix) {
      if mem::discriminant(x) != mem::discriminant(y) {
         panic![];
      }
   }
   pub fn new(x: Pix, y: Pix) -> Pix2D {
      Pix2D::validate(&x, &y);
      Pix2D { x, y }
   }
   pub fn set_pix(&mut self, x: Pix, y: Pix) {
      Pix2D::validate(&x, &y);
      Pix2D::validate(&x, &self.x);
      self.x = x;
      self.y = y;
   }
   pub fn default_screen() -> Pix2D {
      Pix2D { x: Pix::ScreenPix(0), y: Pix::ScreenPix(0) }
   }
   pub fn default_osru() -> Pix2D {
      Pix2D { x: Pix::OsruPix(0), y: Pix::OsruPix(0) }
   }
   pub fn x(&self) -> Pix {
      self.x
   }
   pub fn y(&self) -> Pix {
      self.y
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

#[derive(Debug, Clone)]
pub struct PixRect {
   x: Pix,
   y: Pix,
   width: Pix,
   height: Pix,
}
impl PixRect {
   fn validate(x: &Pix, y: &Pix, width: &Pix, height: &Pix) {
      if !((mem::discriminant(x) == mem::discriminant(y))
         && (mem::discriminant(width) == mem::discriminant(height))
         && (mem::discriminant(x) == mem::discriminant(width)))
      {
         panic![];
      }
   }
   pub fn new(x: Pix, y: Pix, width: Pix, height: Pix) -> PixRect {
      PixRect::validate(&x, &y, &width, &height);
      PixRect { x, y, width, height }
   }

   pub fn new_from_sdl2_rect(sdl2_rect: sdl2::rect::Rect) -> PixRect {
      PixRect::new(
         Pix::screen_pix(sdl2_rect.x() as isize),
         Pix::screen_pix(sdl2_rect.y() as isize),
         Pix::screen_pix(sdl2_rect.width() as isize),
         Pix::screen_pix(sdl2_rect.height() as isize),
      )
   }

   pub fn to_sdl2_rect(&self) -> sdl2::rect::Rect {
      sdl2::rect::Rect::new(
         self.x().get_pix_round() as i32,
         self.y().get_pix_round() as i32,
         self.width().get_pix_round() as u32,
         self.height().get_pix_round() as u32,
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
