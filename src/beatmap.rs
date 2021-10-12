use crate::*;

pub mod event;
pub mod hitobject;
pub mod settings;
pub mod timing;

use event::*;
use global::pixel::*;
use hitobject::*;
use input::*;
use settings::*;
use timing::*;

use std::time::{Duration, SystemTime};
use std::{any, clone, cmp, collections::HashMap, fmt, fs, slice};

use sdl2::render::{Texture, WindowCanvas};

pub struct Beatmap {
   pub settings: BeatmapSettings,

   pub timing_points: Vec<TimingPoint>,
   pub hitobjects: Vec<HitObject>,
   pub event_backgrounds: Vec<EventBackground>,
   /*
   pub event_backgrounds : Vec<EventBackground>,
   pub event_videos: Vec<EventVideo>,
   pub event_breaks: Vec<EventBreak>,
   // storyboard
   pub beatmap_colours: Vec<BeatmapColour>,
   */
   update_start_index: usize,
   draw_start_index: usize,
   draw_end_index: usize,
   animation_timings: AnimationTiming,
}
impl Beatmap {
   fn new() -> Beatmap {
      Beatmap {
         settings: BeatmapSettings::new(),

         timing_points: vec![],
         hitobjects: vec![],
         event_backgrounds: vec![],
         /*
         event_videos: vec![],
         event_breaks: vec![],
         beatmap_colours: vec![],
         //hitobjects
         */
         update_start_index: 0,
         draw_start_index: 0,
         draw_end_index: 0,
         animation_timings: AnimationTiming::default(),
      }
   }

   pub fn load(filename: &str) -> Beatmap {
      let mut beatmap = Beatmap::new();
      let file = fs::read_to_string(filename).unwrap();
      let mut last_hitobj_pos_x = -50;
      let mut last_hitobj_pos_y = -50;

      use BeatmapSection::*;
      let mut section = General;
      'next_line: for line in file.lines() {
         let line = line.trim();
         if line.starts_with("//") || line.len() == 0 {
            continue 'next_line;
         }
         if line.contains("[") && line.contains("]") && line.len() > 2 {
            let line = line.trim_start_matches("[");
            let line = line.trim_end_matches("]");

            for v in BeatmapSection::into_enum_iter() {
               if line == v.to_string() {
                  section = v;
                  continue 'next_line;
               }
            }
         } else if section == General || section == Editor || section == Metadata || section == Difficulty {
            if let Some((k, value)) = parse_key_value(line, ":") {
               let mut key = BeatmapSettingName::from_str(k);

               if let Some(old_value) = beatmap.settings.get(&key) {
                  if section == Metadata && key == BeatmapSettingName::Tags {
                     if let Some(result) = OsruType::parse_type(value, old_value, Some(" ")) {
                        beatmap.settings.set(&key, result);
                     }
                  } else if let Some(result) = OsruType::parse_type(value, old_value, None) {
                     beatmap.settings.set(&key, result);
                  }
               }
            } else {
               continue 'next_line;
            }
         } else if section == Events {
            let line = parse_list(line, ",");
            if line.len() >= 3 {
               if line[0] == "0" {
                  let start_time = line[1].trim().parse().unwrap_or_default();
                  let filename: String = line[2].trim().parse().unwrap_or_default();
                  let filename = filename.trim_end_matches("\"");
                  let filename = filename.trim_start_matches("\"");
                  let filename = nstr(filename);
                  let mut offset_from_center = Pix2D::new(Pix::osru_pix(0.0), Pix::osru_pix(0.0));
                  if line.len() >= 5 {
                     let x = line[3].trim().parse().unwrap_or_default();
                     let y = line[4].trim().parse().unwrap_or_default();
                     offset_from_center.set_pix(Pix::osru_pix(x), Pix::osru_pix(y));
                  }
                  beatmap.event_backgrounds.push(EventBackground {
                     start_time,
                     filename,
                     offset_from_center,
                  });
               }
               // TODO: OTHER EVENTS
            }
         } else if section == TimingPoints {
            let line = parse_list(line, ",");
            if line.len() >= 8 {
               let t = TimingPoint {
                  start_time: line[0].trim().parse().unwrap_or_default(),
                  beat_length: line[1].trim().parse().unwrap_or_default(),
                  meter: line[2].trim().parse().unwrap_or_default(),
                  sample_set: line[3].trim().parse().unwrap_or_default(),
                  sample_index: line[4].trim().parse().unwrap_or_default(),
                  volume: line[5].trim().parse().unwrap_or_default(),
                  uninherited: line[6].trim().parse().unwrap_or_default(),
                  effects: line[7].trim().parse().unwrap_or_default(),
               };
               beatmap.timing_points.push(t);
            }
         } else if section == Colours {
            ///////////////////////////////////////////////////////////////////////////
         } else if section == HitObjects {
            let line = parse_list(line, ",");
            if line.len() >= 4 {
               let position = {
                  let mut x: i32 = line[0].trim().parse().unwrap_or_default();
                  let mut y: i32 = line[1].trim().parse().unwrap_or_default();
                  if ((x - last_hitobj_pos_x).pow(2) + (y - last_hitobj_pos_y).pow(2)) < 25 {
                     x += 5;
                     y += 5;
                  }
                  last_hitobj_pos_x = x;
                  last_hitobj_pos_y = y;
                  Pix2D::new(Pix::osru_pix(x as f32), Pix::osru_pix(y as f32))
               };
               let time = Duration::from_millis(line[2].trim().parse::<u64>().unwrap_or_default())
                  + BEATMAP_TIMING_OFFSET;
               let type_bitflags = line[3].trim().parse::<u32>().unwrap_or_default();
               let hitsound_bitflags = line[4].trim().parse::<u32>().unwrap_or_default();
               let hitsounds = OsruHitSounds::from_bitflags(Bitflags(hitsound_bitflags));
               let new_combo = type_bitflags & 0b100 == 0b100;
               let combo_colours_to_skip = (type_bitflags & 0b1110000) >> 4;

               if type_bitflags & 0b1 == 0b1 {
                  //hitcircle
                  let hitcircle = hitcircle::HitCircle {
                     position,
                     time,
                     new_combo,
                     combo_colours_to_skip,
                     hitsounds,
                     ..Default::default()
                  };
                  beatmap.hitobjects.push(HitObject::HitCircle(hitcircle));
               } else if type_bitflags & 0b10 == 0b10 {
                  //slider
                  use slider::SliderCurveType::{self, *};
                  let curve = line[5].trim().parse::<String>().unwrap_or_default();
                  let curve = curve.trim();
                  let curve = parse_list(curve, "|");
                  let mut iter = curve.iter();
                  let mut curve_type = SliderCurveType::default();
                  let mut curve_points = vec![position];
                  if let Some(c) = iter.next() {
                     curve_type = match *c {
                        "C" => CentripetalCatmullRom,
                        "L" => Linear,
                        "P" => PerfectCircle,
                        _ => Bezier,
                     };

                     for pt in iter {
                        let split = parse_list(pt, ":");
                        if split.len() == 2 {
                           let x = split[0].trim().parse().unwrap_or_default();
                           let y = split[1].trim().parse().unwrap_or_default();
                           let curve_point = Pix2D::new(Pix::osru_pix(x), Pix::osru_pix(y));
                           curve_points.push(curve_point);
                        }
                     }
                     curve_points.shrink_to_fit();
                  }
                  let num_slides = line[6].trim().parse::<u32>().unwrap_or(1);
                  let length_of_slider =
                     Pix::OsruPix(line[7].trim().parse::<f32>().unwrap_or_default() / 10.0);
                  let mut edge_sounds = vec![];
                  if line.len() >= 9 {
                     let line8 = parse_list(line[8], "|");
                     for sound in line8 {
                        edge_sounds.push(sound.trim().parse::<i32>().unwrap_or(0));
                     }
                  }
                  //let mut edge_sets = vec![];
                  if line.len() >= 10 {
                     let line9 = parse_list(line[9], "|");
                     for sound in line9 {
                        //edge_sounds.push(sound.trim().parse::<isize>().unwrap_or(0));
                        // TODO: deal with hitsound bitflag
                     }
                  }

                  let slider = slider::Slider {
                     curve_points,
                     time,
                     new_combo,
                     combo_colours_to_skip,
                     curve_type,
                     num_slides,
                     length_of_slider,
                     ..Default::default()
                  };
               //beatmap.hitobjects.push(HitObject::Slider(slider));

               //println!("slider {:?}", line);
               } else if type_bitflags & 0b1000 == 0b1000 {
                  // TODO: spiner
                  //println!("spinner {:?}", line);
               } else if (type_bitflags & 0b1000_0000) == 0b1000_0000 {
                  // LOW PRIORITY: mania hold
                  println!("mania {:?}", line);
               } else {
                  println!("unknown {:?}", line);
               }
            }
         }
      }
      //println!("settings {:?}", beatmap.settings);
      //println!("timing points {:?}", beatmap.timing_points);
      beatmap.hitobjects.shrink_to_fit();
      beatmap.event_backgrounds.shrink_to_fit();
      beatmap.timing_points.shrink_to_fit();
      beatmap.settings.shrink_to_fit();
      beatmap
   }

   pub fn prepare(&mut self, viewport_size: &PixRect) {
      self.animation_timings =
         AnimationTiming::new_from(self.settings.overall_difficulty(), self.settings.approach_rate());
      for hitobj in self.hitobjects.iter_mut() {
         hitobj.prepare(viewport_size, &self.settings);
      }
   }

   pub fn lazy_update(&mut self, input_manager: &mut InputManager) {
      use hitobject::UpdateResult::*;

      input_manager.poll_all();

      'nextObj: while self.update_start_index < self.hitobjects.len() {
         input_manager.poll_one();
         let hitobj = self.hitobjects.get_mut(self.update_start_index).unwrap();
         if hitobj.hit_state().is_ready() || hitobj.hit_state().not_yet_drawing() {
            if let Some(update) = input_manager.next_update() {
               if hitobj.update(&update, &self.animation_timings) == InputConsumed {
                  self.update_start_index += 1;
               }
            } else {
               break 'nextObj;
            }
         } else {
            self.update_start_index += 1;
         }
      }
   }

   pub fn full_update(&mut self, input_manager: &mut InputManager) {
      input_manager.force_time_update();
      self.lazy_update(input_manager);

      let update = InputUpdate::new(input_manager.curr_snapshot(), input_manager.curr_snapshot());

      let draw_start_index = self.draw_start_index;
      self.draw_start_index = usize::MAX;

      for i in draw_start_index..self.hitobjects.len() {
         let hitobj = self.hitobjects.get_mut(i).unwrap();
         hitobj.update(&update, &self.animation_timings);
         if !hitobj.hit_state().is_done() {
            if i < self.draw_start_index {
               self.draw_start_index = i;
            }
            if i > self.draw_end_index {
               self.draw_end_index = i;
            }
            if hitobj.hit_state().not_yet_drawing() {
               break;
            }
         }
      }
   }

   pub fn draw(
      &mut self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager,
      input_manager: &mut InputManager,
   ) {
      use DrawResult::*;

      for i in self.draw_start_index..self.draw_end_index {
         input_manager.poll_one();
         self.hitobjects.get(i).unwrap().draw_self(canvas, texture_manager);
      }
      for i in self.draw_start_index..self.draw_end_index {
         input_manager.poll_one();
         self.hitobjects.get(i).unwrap().draw_approach_circle(
            canvas,
            texture_manager,
            &self.animation_timings,
            *input_manager.curr_snapshot().time(),
         );
      }
   }

   pub fn is_done(&self) -> bool {
      self.draw_start_index >= self.hitobjects.len() - 1
         && self.hitobjects.get(self.hitobjects.len() - 1).unwrap().hit_state().is_done()
   }
}

#[derive(Debug, Copy, Clone)]
pub struct TimingPoint {
   start_time: isize,
   beat_length: f64,
   meter: isize,
   sample_set: isize,
   sample_index: isize,
   volume: isize,
   uninherited: isize,
   effects: usize,
}

pub struct BeatmapColour {
   combo_colour: Colour<u8>,
   slider_track_override_colour: Colour<u8>,
   slider_border_colour: Colour<u8>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoEnumIterator)]
pub enum BeatmapSection {
   General,
   Editor,
   Metadata,
   Difficulty,
   Events,
   TimingPoints,
   Colours,
   HitObjects,
}
impl BeatmapSection {
   pub fn eq(&self, other: &str) -> bool {
      self.to_string() == other
   }
}
impl fmt::Display for BeatmapSection {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
   }
}
