use super::super::timing::*;
use super::*;

#[derive(Debug, Copy, Eq, PartialEq, Clone, IntoEnumIterator)]
pub enum SliderCurveType {
   Bezier,
   CentripetalCatmullRom,
   Linear,
   PerfectCircle,
}
impl Default for SliderCurveType {
   fn default() -> Self {
      SliderCurveType::Bezier
   }
}

#[derive(Debug, Clone)]
pub struct Slider {
   pub curve_points: Vec<Pix2D>,
   pub time: Duration,
   pub new_combo: bool,
   pub combo_colours_to_skip: u32,
   pub curve_type: SliderCurveType,
   pub num_slides: u32,
   pub length_of_slider: Pix,

   //pub hitsounds: OsruHitSounds,
   //pub edge_sounds: Vec<i32>,
   //pub edge_sets: Vec<String>,
   pub colour: Colour<u8>,
   pub hit_state: HitState,
   pub curve_points_screen: Vec<Pix2D>,
   pub length_to_window: Pix,

   pub active: bool,
   pub scale: ScalingFactor,
   pub time_hit: Duration,
}
impl Slider {
   pub fn update(&mut self, update: &InputUpdate, timings: &AnimationTiming) -> UpdateResult {
      use HitState::*;
      use HitSuccess::*;
      use UpdateResult::*;

      let current_time = *update.current_time();

      if let DoneDrawing(_) = self.hit_state {
      } else if current_time < timings.fadein_start(self.time) {
         self.hit_state = NotDrawing;
      } else if current_time < timings.timing_meh_start(self.time) {
         self.colour.a = HITCIRCLE_MAX_OPACITY as u8;
         self.hit_state = Ready;
         if current_time < timings.fadein_end(self.time) {
            self.fade_in(current_time, &timings);
         }
      } else if self.active {
         if update.K1M1_held() || update.K2M2_held() {}
      } else if let Hit(_) = self.hit_state {
         self.fade_out(current_time, &timings);
      } else if !self.active {
         self.hit_state = Ready;
         self.colour.a = HITCIRCLE_MAX_OPACITY as u8;
         if timings.is_timing_meh(self.time, current_time)
            && (update.K1M1_pressed() || update.K2M2_pressed())
            && cursor_in_range(
               &self.curve_points_screen.get(0).unwrap(),
               update.current_mouse_pos(),
               &Pix::ScreenPix(150.0),
            )
         {
            self.active = true;
            self.colour = COLOUR_ACTIVE;
            self.scale = ScalingFactor(0.5);
            self.time_hit = current_time;

            if timings.is_timing_great(self.time, current_time) {
               self.hit_state = Hit(Great);
            } else if timings.is_timing_good(self.time, current_time) {
               self.hit_state = Hit(Good);
            }
            self.fade_out(current_time, &timings);
         } else if timings.is_timing_miss(self.time, current_time) {
            self.hit_state = Hit(Miss);
            self.colour = COLOUR_MISS;
            self.scale = ScalingFactor(0.5);
            self.time_hit = timings.timing_meh_end(self.time);
            self.fade_out(current_time, &timings);
         }
      }
      return InputNotConsumed;
   }

   pub fn fade_out(&mut self, current_time: Duration, timings: &AnimationTiming) {
      let num = (current_time - self.time_hit).as_micros();
      let den = (timings.timing_meh_duration() * 2).as_micros();
      if num > den {
         self.hit_state = self.hit_state.to_done_drawing();
      } else {
         self.colour.a = (HITCIRCLE_MAX_OPACITY - ((num * HITCIRCLE_MAX_OPACITY) / den)) as u8;
      }
   }

   pub fn fade_in(&mut self, current_time: Duration, timings: &AnimationTiming) {
      let num = (current_time - timings.fadein_start(self.time)).as_micros();
      let den = (timings.fadein_end(self.time) - timings.fadein_start(self.time)).as_micros();
      self.colour.a = ((num * HITCIRCLE_MAX_OPACITY) / den) as u8;
   }

   pub fn prepare(&mut self, viewport_size: &PixRect, beatmap_settings: &BeatmapSettings) {
      for curve_point in &self.curve_points {
         self.curve_points_screen.push(osru_pos_to_screen_pos(&curve_point, viewport_size));
      }

      // TODO: convert slider length to screen coordinates
   }

   pub fn draw_self(&self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager) -> DrawResult {
      use DrawResult::*;
      let texture = texture_manager.get(TextureName::HitCircle);
      let mut texture = texture.borrow_mut();
      if self.hit_state.is_drawing() {
         texture.set_alpha_mod(self.colour.a);
         texture.set_color_mod(self.colour.r, self.colour.g, self.colour.b);

         let image_size = Pix2D::new(
            Pix::screen_pix(texture.query().width as f32),
            Pix::screen_pix(texture.query().height as f32),
         );
         let viewport = calculate_texture_viewport(
            &self.curve_points_screen.get(0).unwrap(),
            &image_size,
            &PixRect::new_from_sdl2_rect(canvas.viewport()),
            self.scale,
         );
         canvas.copy(&texture, None, viewport.to_sdl2_rect()).unwrap();
         Drawed
      } else {
         NotDrawed
      }
   }

   pub fn hit_state(&self) -> HitState {
      self.hit_state
   }

   pub fn time(&self) -> Duration {
      self.time
   }

   pub fn screen_position(&self) -> Pix2D {
      *self.curve_points_screen.get(0).unwrap()
   }
}

impl Default for Slider {
   fn default() -> Self {
      Slider {
         curve_points: vec![],
         time: Duration::from_secs(0),
         new_combo: false,
         combo_colours_to_skip: 0,
         curve_type: SliderCurveType::default(),
         num_slides: 1,
         length_of_slider: Pix::OsruPix(0.0),

         colour: Colour { r: 182, g: 39, b: 246, a: 128 },
         hit_state: HitState::default(),
         curve_points_screen: vec![],
         length_to_window: Pix::OsruPix(0.0),

         active: false,
         scale: ScalingFactor(2.0),
         time_hit: Duration::default(),
      }
   }
}
