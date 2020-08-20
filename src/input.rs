use crate::beatmap::{
  hitobject::{self, HitObject},
  Beatmap,
};
use crate::global::*;
use crate::time::*;

use sdl2::keyboard::{Keycode, Scancode};
use sdl2::{event::Event, EventPump};

use std::collections::HashMap;
use std::thread;
use std::time::{self, Duration, Instant};

pub struct InputManager {
  event_pump: EventPump,
  keep_running: bool,
  reference_time: Option<ReferenceTime>,
  input_snapshot: InputSnapshot,
}

impl InputManager {
  pub fn new(event_pump: EventPump) -> InputManager {
    let mut result = InputManager {
      event_pump,
      keep_running: true,
      reference_time: None,
      input_snapshot: InputSnapshot::default(),
    };
    result.set_reference_time();
    result
  }

  pub fn start_timer(&mut self) {
    self.reference_time.as_mut().unwrap().start();
  }

  pub fn set_reference_time(&mut self) {
    // clear events
    for ev in self.event_pump.poll_iter() {
      match ev {
        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          self.keep_running = false;
        }
        _ => (),
      }
    }
    'create_reference_time: for ev in self.event_pump.wait_iter() {
      match ev {
        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          self.keep_running = false;
          break 'create_reference_time;
        }
        Event::KeyDown { timestamp: t, .. } => {
          let ref_time = Instant::now();
          let sdl_time = SdlTime(t as usize);
          self.reference_time = Some(ReferenceTime::new(ref_time, sdl_time));
          break 'create_reference_time;
        }
        Event::MouseMotion { timestamp: t, .. } => {
          let ref_time = Instant::now();
          let sdl_time = SdlTime(t as usize);
          self.reference_time = Some(ReferenceTime::new(ref_time, sdl_time));
          break 'create_reference_time;
        }
        _ => (),
      }
    }
  }

  pub fn last_snapshot(&self) -> &InputSnapshot {
    &self.input_snapshot
  }

  pub fn reference_time(&self) -> &ReferenceTime {
    self.reference_time.as_ref().unwrap()
  }

  fn estimate_mouse_pos_from_samples(
    sample1: &InputSnapshot, sample2: &InputSnapshot, current_time: Duration,
  ) -> Pixels {
    let x_delta = sample2.mouse_position.0 - sample1.mouse_position.0;
    let y_delta = sample2.mouse_position.1 - sample1.mouse_position.1;
    let time_delta = (sample2.time - sample1.time).as_secs_f64();
    let time_from_last = (current_time - sample2.time).as_secs_f64();

    let new_x = sample2.mouse_position.0 + x_delta * time_from_last / time_delta;
    let new_y = sample2.mouse_position.1 + y_delta * time_from_last / time_delta;

    Pixels(new_x, new_y)
  }

  pub fn next_update(&mut self) -> Option<InputUpdate> {
    let ref_time = self.reference_time().clone();
    let last_snapshot = self.last_snapshot().clone();

    'next_event: loop {
      if let Some(event) = self.event_pump.poll_event() {
        let mut new_snap = InputSnapshot::new_from(&last_snapshot);
        match event {
          Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            self.keep_running = false;
            continue 'next_event;
          }
          Event::MouseMotion { timestamp: t, x: x_pos, y: y_pos, .. } => {
            new_snap.time = ref_time.elapsed_sdl_time(SdlTime(t as usize));
            new_snap.mouse_position = Pixels(x_pos as f64, y_pos as f64);
          }
          /*
          Event::MouseButtonDown { .. } => {
            // TODO: Handle mouse clicks
          }
          Event::MouseButtonUp { .. } => {
            //
          }
          */
          Event::KeyDown { scancode: Some(code), repeat: false, timestamp: t, .. } => {
            use Scancode::*;
            let time = ref_time.elapsed_sdl_time(SdlTime(t as usize));
            match code {
              Z => {
                new_snap.time = time;
                new_snap.K1 = true;
              }
              X => {
                new_snap.time = time;
                new_snap.K2 = true;
              }
              _ => (),
            }
          }
          Event::KeyUp { scancode: Some(code), repeat: false, timestamp: t, .. } => {
            use Scancode::*;
            let time = ref_time.elapsed_sdl_time(SdlTime(t as usize));
            match code {
              Z => {
                new_snap.time = time;
                new_snap.K1 = false;
              }
              X => {
                new_snap.time = time;
                new_snap.K2 = false;
              }
              _ => (),
            }
          }
          _ => continue 'next_event,
        }
        self.input_snapshot.copy(&new_snap);
        return Some(InputUpdate::new(last_snapshot, new_snap));
      } else {
        return None;
      }
    }
  }

  pub fn is_running(&self) -> bool {
    self.keep_running
  }
}

#[derive(Debug, Clone)]
pub struct InputSnapshot {
  // true if pressed, false if not
  K1: bool,
  K2: bool,
  M1: bool,
  M2: bool,

  pub time: Duration,
  pub mouse_position: Pixels,
}
impl InputSnapshot {
  pub fn new_from(other: &InputSnapshot) -> InputSnapshot {
    other.clone()
  }
  pub fn copy(&mut self, other: &InputSnapshot) {
    self.K1 = other.K1;
    self.K2 = other.K2;
    self.M1 = other.M1;
    self.M2 = other.M2;
    self.time = other.time;
    self.mouse_position = other.mouse_position.clone();
  }
  pub fn K1(&self) -> bool {
    self.K1
  }
  pub fn K2(&self) -> bool {
    self.K2
  }
  pub fn M1(&self) -> bool {
    self.M1
  }
  pub fn M2(&self) -> bool {
    self.M2
  }
  pub fn time(&self) -> &Duration {
    &self.time
  }
  pub fn mouse_position(&self) -> &Pixels {
    &self.mouse_position
  }
}
impl Default for InputSnapshot {
  fn default() -> InputSnapshot {
    InputSnapshot {
      time: Duration::from_secs(0),
      K1: false,
      K2: false,
      M1: false,
      M2: false,
      mouse_position: Pixels::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct InputUpdate {
  previous: InputSnapshot,
  current: InputSnapshot,
}
impl InputUpdate {
  pub fn new(previous: InputSnapshot, current: InputSnapshot) -> InputUpdate {
    InputUpdate { previous, current }
  }

  pub fn K1_pressed(&self) -> bool {
    self.current.K1() && !self.previous.K1()
  }
  pub fn K2_pressed(&self) -> bool {
    self.current.K2() && !self.previous.K2()
  }
  pub fn M1_pressed(&self) -> bool {
    self.current.M1() && !self.previous.M1()
  }
  pub fn M2_pressed(&self) -> bool {
    self.current.K2() && !self.previous.K2()
  }

  pub fn K1_released(&self) -> bool {
    self.previous.K1() && !self.current.K1()
  }
  pub fn K2_released(&self) -> bool {
    self.previous.K2() && !self.current.K2()
  }
  pub fn M1_released(&self) -> bool {
    self.previous.M1() && !self.current.M1()
  }
  pub fn M2_released(&self) -> bool {
    self.previous.M2() && !self.current.M2()
  }

  pub fn current_time(&self) -> &Duration {
    self.current.time()
  }
  pub fn previous_time(&self) -> &Duration {
    self.previous.time()
  }

  pub fn current_mouse_pos(&self) -> &Pixels {
    self.current.mouse_position()
  }
  pub fn previous_mouse_pos(&self) -> &Pixels {
    self.previous.mouse_position()
  }
}
