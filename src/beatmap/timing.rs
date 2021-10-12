use super::*;
use std::ops;

pub const TIMING_WINDOW_GREAT: Duration = Duration::from_micros(79_500);
pub const TIMING_WINDOW_GREAT_MULTIPLIER: Duration = Duration::from_micros(6_000);
pub const TIMING_WINDOW_GOOD: Duration = Duration::from_micros(139_500);
pub const TIMING_WINDOW_GOOD_MULTIPLIER: Duration = Duration::from_micros(8_000);
pub const TIMING_WINDOW_MEH: Duration = Duration::from_micros(199_500);
pub const TIMING_WINDOW_MEH_MULTIPLIER: Duration = Duration::from_micros(10_000);

#[derive(Debug, Copy, Clone)]
pub struct AnimationTiming {
   preempt: Duration,
   fadein: Duration,
   timing_great: Duration,
   timing_good: Duration,
   timing_meh: Duration,
}
impl Default for AnimationTiming {
   fn default() -> AnimationTiming {
      AnimationTiming {
         preempt: Duration::from_millis(600),
         fadein: Duration::from_millis(300),
         timing_great: TIMING_WINDOW_GREAT,
         timing_good: TIMING_WINDOW_GOOD,
         timing_meh: TIMING_WINDOW_MEH,
      }
   }
}
impl AnimationTiming {
   pub fn new_from(od: OsruOD, ar: OsruAR) -> AnimationTiming {
      AnimationTiming {
         preempt: ar.preempt_time(),
         fadein: ar.fade_in_time(),
         timing_great: od.timing_great(),
         timing_good: od.timing_good(),
         timing_meh: od.timing_meh(),
      }
   }
   /*
   pub fn new(
      preempt: Duration, fadein: Duration, timing_great: Duration, timing_good: Duration,
      timing_meh: Duration,
   ) -> AnimationTiming {
      AnimationTiming { preempt, fadein, timing_great, timing_good, timing_meh }
   }
   */
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct OsruOD(pub f64);
impl OsruOD {
   pub fn timing_great(&self) -> Duration {
      TIMING_WINDOW_GREAT - Duration::from_secs_f64(self.0 * TIMING_WINDOW_GREAT_MULTIPLIER.as_secs_f64())
   }
   pub fn timing_good(&self) -> Duration {
      TIMING_WINDOW_GOOD - Duration::from_secs_f64(self.0 * TIMING_WINDOW_GOOD_MULTIPLIER.as_secs_f64())
   }
   pub fn timing_meh(&self) -> Duration {
      TIMING_WINDOW_MEH - Duration::from_secs_f64(self.0 * TIMING_WINDOW_MEH_MULTIPLIER.as_secs_f64())
   }
   pub fn mul_unchecked(&self, other: Self) -> Self {
      Self(self.0 * other.0)
   }
}
impl ops::Mul for OsruOD {
   type Output = Self;
   fn mul(self, rhs: Self) -> Self::Output {
      let mut result = self.0 * rhs.0;
      if result > 10.0 {
         result = 10.0
      }
      OsruOD(result)
   }
}
impl ops::Mul<f64> for OsruOD {
   type Output = Self;
   fn mul(self, rhs: f64) -> Self::Output {
      let mut result = self.0 * rhs;
      if result > 10.0 {
         result = 10.0
      }
      OsruOD(result)
   }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct OsruAR(pub f64);
impl ops::Mul for OsruAR {
   type Output = Self;
   fn mul(self, rhs: Self) -> Self::Output {
      let mut result = self.0 * rhs.0;
      if result > 10.0 {
         result = 10.0
      }
      OsruAR(result)
   }
}
impl ops::Mul<f64> for OsruAR {
   type Output = Self;
   fn mul(self, rhs: f64) -> Self::Output {
      let mut result = self.0 * rhs;
      if result > 10.0 {
         result = 10.0
      }
      OsruAR(result)
   }
}
impl OsruAR {
   pub fn preempt_time(&self) -> Duration {
      if self.0 < 5.0 {
         Duration::from_secs_f64(1.2 + 0.6 * (5.0 - self.0) / 5.0)
      } else if self.0 > 5.0 {
         Duration::from_secs_f64(1.2 - 0.75 * (self.0 - 5.0) / 5.0)
      } else {
         Duration::from_secs_f64(1.2)
      }
   }

   pub fn fade_in_time(&self) -> Duration {
      if self.0 < 5.0 {
         Duration::from_secs_f64(0.8 + 0.4 * (5.0 - self.0) / 5.0)
      } else if self.0 > 5.0 {
         Duration::from_secs_f64(0.8 - 0.5 * (self.0 - 5.0) / 5.0)
      } else {
         Duration::from_secs_f64(0.8)
      }
   }
   pub fn mul_unchecked(&self, other: Self) -> Self {
      Self(self.0 * other.0)
   }
}

/*

#[derive(Debug, Copy, Clone)]
pub struct OsruCS(pub f32);

// TODO: perceived_od_mul()
pub fn ar_multiplier(&self) -> f64 {
  let mut ar_mul = 1.0;
  for game_mod in self.mods.iter() {
    ar_mul *= game_mod.ar_multiplier;
  }
  if ar_mul > 10.0 {
    10.0
  } else {
    ar_mul
  }
}
*/

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_od() {
      println!("Testing OD timing great");
      assert_eq!(Duration::from_micros(19_500), OsruOD(10.0).timing_great());
      assert_eq!(Duration::from_micros(25_500), OsruOD(9.0).timing_great());
      assert_eq!(Duration::from_micros(31_500), OsruOD(8.0).timing_great());
      assert_eq!(Duration::from_micros(37_500), OsruOD(7.0).timing_great());
      assert_eq!(Duration::from_micros(43_500), OsruOD(6.0).timing_great());
      assert_eq!(Duration::from_micros(49_500), OsruOD(5.0).timing_great());
      assert_eq!(Duration::from_micros(55_500), OsruOD(4.0).timing_great());
      assert_eq!(Duration::from_micros(61_500), OsruOD(3.0).timing_great());
      assert_eq!(Duration::from_micros(67_500), OsruOD(2.0).timing_great());
      assert_eq!(Duration::from_micros(73_500), OsruOD(1.0).timing_great());
      assert_eq!(Duration::from_micros(79_500), OsruOD(0.0).timing_great());

      println!("Testing OD timing good");
      assert_eq!(Duration::from_micros(59_500), OsruOD(10.0).timing_good());
      assert_eq!(Duration::from_micros(67_500), OsruOD(9.0).timing_good());
      assert_eq!(Duration::from_micros(75_500), OsruOD(8.0).timing_good());
      assert_eq!(Duration::from_micros(83_500), OsruOD(7.0).timing_good());
      assert_eq!(Duration::from_micros(91_500), OsruOD(6.0).timing_good());
      assert_eq!(Duration::from_micros(99_500), OsruOD(5.0).timing_good());
      assert_eq!(Duration::from_micros(107_500), OsruOD(4.0).timing_good());
      assert_eq!(Duration::from_micros(115_500), OsruOD(3.0).timing_good());
      assert_eq!(Duration::from_micros(123_500), OsruOD(2.0).timing_good());
      assert_eq!(Duration::from_micros(131_500), OsruOD(1.0).timing_good());
      assert_eq!(Duration::from_micros(139_500), OsruOD(0.0).timing_good());

      println!("Testing OD timing meh");
      assert_eq!(Duration::from_micros(99_500), OsruOD(10.0).timing_meh());
      assert_eq!(Duration::from_micros(109_500), OsruOD(9.0).timing_meh());
      assert_eq!(Duration::from_micros(119_500), OsruOD(8.0).timing_meh());
      assert_eq!(Duration::from_micros(129_500), OsruOD(7.0).timing_meh());
      assert_eq!(Duration::from_micros(139_500), OsruOD(6.0).timing_meh());
      assert_eq!(Duration::from_micros(149_500), OsruOD(5.0).timing_meh());
      assert_eq!(Duration::from_micros(159_500), OsruOD(4.0).timing_meh());
      assert_eq!(Duration::from_micros(169_500), OsruOD(3.0).timing_meh());
      assert_eq!(Duration::from_micros(179_500), OsruOD(2.0).timing_meh());
      assert_eq!(Duration::from_micros(189_500), OsruOD(1.0).timing_meh());
      assert_eq!(Duration::from_micros(199_500), OsruOD(0.0).timing_meh());

      println!("Testing od multiplication");
      let a = OsruOD(5.54);
      let b = OsruOD(2.0);
      assert_eq!(OsruOD(10.0), a * b);
      assert_eq!(OsruOD(11.08), a.mul_unchecked(b));
   }

   #[test]
   fn test_ar() {
      use assert_approx_eq::assert_approx_eq;
      println!("Testing preempt");
      assert_approx_eq!(1.80, OsruAR(0.0).preempt_time().as_secs_f64());
      assert_approx_eq!(1.68, OsruAR(1.0).preempt_time().as_secs_f64());
      assert_approx_eq!(1.56, OsruAR(2.0).preempt_time().as_secs_f64());
      assert_approx_eq!(1.44, OsruAR(3.0).preempt_time().as_secs_f64());
      assert_approx_eq!(1.32, OsruAR(4.0).preempt_time().as_secs_f64());
      assert_approx_eq!(1.20, OsruAR(5.0).preempt_time().as_secs_f64());
      assert_approx_eq!(1.05, OsruAR(6.0).preempt_time().as_secs_f64());
      assert_approx_eq!(0.90, OsruAR(7.0).preempt_time().as_secs_f64());
      assert_approx_eq!(0.75, OsruAR(8.0).preempt_time().as_secs_f64());
      assert_approx_eq!(0.60, OsruAR(9.0).preempt_time().as_secs_f64());
      assert_approx_eq!(0.45, OsruAR(10.0).preempt_time().as_secs_f64());

      println!("Testing fadein");
      // TODO:

      println!("Testing multiplication");
      let a = OsruAR(5.0);
      assert_eq!(OsruAR(10.0), a * a);
      assert_eq!(OsruAR(25.0), a.mul_unchecked(a));
   }
}
