use std::ops;
use std::time::{self, Duration, Instant};

#[derive(Debug, Clone)]
pub struct SdlTime(pub usize); // milliseconds
impl ops::Add for SdlTime {
  type Output = SdlTime;

  fn add(self, rhs: Self) -> Self::Output {
    SdlTime(self.0 + rhs.0)
  }
}
impl ops::Sub for SdlTime {
  type Output = SdlTime;

  fn sub(self, rhs: Self) -> Self::Output {
    SdlTime(self.0 - rhs.0)
  }
}
impl ops::Mul for SdlTime {
  type Output = SdlTime;
  fn mul(self, rhs: Self) -> Self::Output {
    SdlTime(self.0 * rhs.0)
  }
}
impl ops::Div for SdlTime {
  type Output = SdlTime;
  fn div(self, rhs: Self) -> Self::Output {
    SdlTime(self.0 / rhs.0)
  }
}

#[derive(Debug, Clone)]
pub struct ReferenceTime {
  ref_time: Instant,
  ref_sdl_time: SdlTime,
  start_time: Duration,
}
impl ReferenceTime {
  pub fn new(ref_time: Instant, ref_sdl_time: SdlTime) -> ReferenceTime {
    ReferenceTime { ref_time, start_time: Duration::from_secs(0), ref_sdl_time }
  }
  pub fn start(&mut self) {
    self.start_time = self.ref_time.elapsed();
  }
  pub fn elapsed_sdl_time(&self, current: SdlTime) -> Duration {
    let sdl_t = Duration::from_millis((current - self.ref_sdl_time.clone()).0 as u64);
    sdl_t - self.start_time
  }

  pub fn elapsed_sys_time(&self, current: Instant) -> Duration {
    current.duration_since(self.ref_time) - self.start_time
  }
}
