use crate::beatmap::{
   hitobject::{self, HitObject},
   Beatmap,
};
use crate::global::pixel::*;
use crate::global::*;
use crate::time::*;

use sdl2::keyboard::{Keycode, Scancode};
use sdl2::{event::Event, EventPump};

use std::collections::HashMap;
use std::collections::VecDeque;
use std::thread;
use std::time::{self, Duration, Instant};

#[derive(Eq, PartialEq)]
pub enum PollResult {
   Success,
   Failed,
}

pub struct InputManager {
   event_pump: EventPump,
   keep_running: bool,
   reference_time: Option<ReferenceTime>,
   //prev_snapshot: InputSnapshot,
   //cur_snapshot: InputSnapshot,
   pending_snapshots: Option<VecDeque<InputSnapshot>>,
}

impl InputManager {
   pub fn new(event_pump: EventPump) -> InputManager {
      let mut pending_snapshots = VecDeque::new();
      pending_snapshots.push_back(InputSnapshot::default());
      pending_snapshots.push_back(InputSnapshot::default());
      let pending_snapshots = Some(pending_snapshots);
      let mut result = InputManager {
         event_pump,
         keep_running: true,
         reference_time: None,
         //prev_snapshot: InputSnapshot::default(),
         //cur_snapshot: InputSnapshot::default(),
         pending_snapshots,
      };
      result.set_reference_time();
      result
   }

   pub fn event_pump(&self) -> &EventPump {
      &self.event_pump
   }
   pub fn start_timer(&mut self) {
      self.reference_time.as_mut().unwrap().start();
   }

   pub fn set_reference_time(&mut self) {
      '_clear_events: for ev in self.event_pump.poll_iter() {
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

   pub fn prev_snapshot(&self) -> &InputSnapshot {
      &self.pending_snapshots.as_ref().unwrap().get(0).unwrap()
   }

   pub fn curr_snapshot(&self) -> &InputSnapshot {
      &self.pending_snapshots.as_ref().unwrap().get(1).unwrap()
   }

   pub fn reference_time(&self) -> &ReferenceTime {
      self.reference_time.as_ref().unwrap()
   }

   fn estimate_mouse_pos_from_samples(
      sample1: &InputSnapshot, sample2: &InputSnapshot, current_time: Duration,
   ) -> Pix2D {
      let x_delta = sample2.mouse_position.x().get_mpix() - sample1.mouse_position.x().get_mpix();
      let y_delta = sample2.mouse_position.y().get_mpix() - sample1.mouse_position.y().get_mpix();
      let time_delta = sample2.time.as_micros() as isize - sample1.time.as_micros() as isize;
      let time_from_last = current_time.as_micros() as isize - sample2.time.as_micros() as isize;

      let new_x = sample2.mouse_position.x().get_mpix() + (x_delta * time_from_last) / time_delta;
      let new_y = sample2.mouse_position.y().get_mpix() + (y_delta * time_from_last) / time_delta;

      Pix2D::new(Pix::screen_mpix(new_x), Pix::screen_mpix(new_y))
   }

   fn push_snapshot(&mut self, snapshot: InputSnapshot) {
      self.pending_snapshots.as_mut().unwrap().push_back(snapshot);
   }

   fn pop_snapshot(&mut self) -> Option<InputSnapshot> {
      if self.len() > 2 {
         self.pending_snapshots.as_mut().unwrap().pop_front()
      } else {
         None
      }
   }

   pub fn next_update(&mut self) -> Option<InputUpdate> {
      if let Some(snapshot) = self.pop_snapshot() {
         Some(self.last_update())
      } else {
         None
      }
   }

   pub fn force_time_update(&mut self) {
      self.poll_all();
      let mut new_snap = InputSnapshot::new_from(self.latest_snapshot());
      new_snap.time = self.reference_time().elapsed_now();
      self.push_snapshot(new_snap);
   }

   pub fn last_update(&self) -> InputUpdate {
      InputUpdate::new(self.prev_snapshot(), self.curr_snapshot())
   }

   pub fn poll_all(&mut self) {
      while self.poll_one() == PollResult::Success {}
   }

   fn len(&self) -> usize {
      self.pending_snapshots.as_ref().unwrap().len()
   }

   fn latest_snapshot(&self) -> &InputSnapshot {
      self.pending_snapshots.as_ref().unwrap().get(self.len() - 1).unwrap()
   }

   pub fn poll_one(&mut self) -> PollResult {
      if let Some(event) = self.event_pump.poll_event() {
         use Scancode::*;

         let mut new_snap = InputSnapshot::new_from(self.latest_snapshot());
         new_snap.time = self.reference_time().elapsed_now();

         match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
               self.keep_running = false;
               return PollResult::Success;
            }
            Event::MouseMotion { timestamp: t, x: x_pos, y: y_pos, .. } => {
               new_snap.mouse_position =
                  Pix2D::new(Pix::screen_pix(x_pos as isize), Pix::screen_pix(y_pos as isize))
            }
            /*
            Event::MouseButtonDown { .. } => ,
            Event::MouseButtonUp { .. } => ,
            */
            Event::KeyDown { scancode: Some(code), repeat: false, timestamp: t, .. } => match code {
               Z => new_snap.K1 = true,
               X => new_snap.K2 = true,
               _ => (),
            },
            Event::KeyUp { scancode: Some(code), repeat: false, timestamp: t, .. } => match code {
               Z => new_snap.K1 = false,
               X => new_snap.K2 = false,
               _ => (),
            },
            Event::User { timestamp: t, .. } => (),
            _ => return PollResult::Success,
         }
         self.push_snapshot(new_snap);
         PollResult::Success
      } else {
         PollResult::Failed
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
   pub mouse_position: Pix2D,
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
      self.mouse_position = other.mouse_position;
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
   pub fn mouse_position(&self) -> &Pix2D {
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
         mouse_position: Pix2D::default_screen(),
      }
   }
}

#[derive(Debug, Clone)]
pub struct InputUpdate<'a> {
   previous: &'a InputSnapshot,
   current: &'a InputSnapshot,
}
impl<'a> InputUpdate<'a> {
   pub fn new(previous: &'a InputSnapshot, current: &'a InputSnapshot) -> InputUpdate<'a> {
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

   pub fn current_mouse_pos(&self) -> &Pix2D {
      self.current.mouse_position()
   }
   pub fn previous_mouse_pos(&self) -> &Pix2D {
      self.previous.mouse_position()
   }
}
