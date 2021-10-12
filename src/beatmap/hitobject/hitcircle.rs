use super::super::timing::*;
use super::*;

#[derive(Debug, Clone)]
pub struct HitCircle {
   pub position: Pix2D,
   pub time: Duration,
   pub new_combo: bool,
   pub combo_colours_to_skip: u32,
   pub hitsounds: OsruHitSounds,

   pub hitsample_set: i32,
   pub hitsample_additional_set: i32,
   pub hitsample_index: i32,
   pub hitsample_volume: Volume,
   pub hitsample_filename: String,

   pub hit_state: HitState,
   pub colour: Colour<u8>,
   pub scale: ScalingFactor,
   pub screen_position: Pix2D,
   pub time_hit: Duration,
   pub current_time: Duration,
}
impl HitCircle {
   pub fn update(&mut self, update: &InputUpdate, timings: &AnimationTiming) -> UpdateResult {
      use HitState::*;
      use HitSuccess::*;
      use UpdateResult::*;

      self.current_time = *update.current_time();

      if let DoneDrawing(_) = self.hit_state {
      } else if self.current_time < timings.fadein_start(self.time) {
         self.hit_state = NotDrawing;
      } else if self.current_time < timings.timing_meh_start(self.time) {
         self.colour.a = HITCIRCLE_MAX_OPACITY as u8;
         self.hit_state = Ready;
         if self.current_time < timings.fadein_end(self.time) {
            self.fade_in(self.current_time, &timings);
         }
      } else if let Hit(_) = self.hit_state {
         self.fade_out(&timings);
      } else {
         self.hit_state = Ready;
         self.colour.a = HITCIRCLE_MAX_OPACITY as u8;
         if timings.is_timing_meh(self.time, self.current_time)
            && (update.K1M1_pressed() || update.K2M2_pressed())
            && cursor_in_range(&self.screen_position, update.current_mouse_pos(), &Pix::ScreenPix(150.0))
         {
            self.hit_state = Hit(Meh);
            self.colour = COLOUR_MEH;
            self.scale = ScalingFactor(0.5);
            self.time_hit = self.current_time;

            if timings.is_timing_great(self.time, self.current_time) {
               self.hit_state = Hit(Great);
               self.colour = COLOUR_GREAT;
            } else if timings.is_timing_good(self.time, self.current_time) {
               self.hit_state = Hit(Good);
               self.colour = COLOUR_GOOD;
            }
            self.fade_out(&timings);
            return InputConsumed;
         } else if timings.is_timing_miss(self.time, self.current_time) {
            self.hit_state = Hit(Miss);
            self.colour = COLOUR_MISS;
            self.scale = ScalingFactor(0.5);
            self.time_hit = timings.timing_meh_end(self.time);
            self.fade_out(&timings);
         }
      }
      return InputNotConsumed;
   }

   pub fn fade_out(&mut self, timings: &AnimationTiming) {
      let num = (self.current_time - self.time_hit).as_micros();
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
      self.screen_position = osru_pos_to_screen_pos(&self.position, viewport_size);
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
            &self.screen_position,
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
      self.screen_position
   }

   // fn reset()
}
impl Default for HitCircle {
   fn default() -> Self {
      HitCircle {
         position: Pix2D::default_osru(),
         time: Duration::from_secs(0),
         new_combo: false,
         combo_colours_to_skip: 0,
         hitsounds: OsruHitSounds::default(),
         hitsample_set: 0,
         hitsample_additional_set: 0,
         hitsample_index: 0,
         hitsample_volume: Volume::default(),
         hitsample_filename: nstr(""),
         hit_state: HitState::default(),
         colour: Colour { r: u8::MAX, g: u8::MAX, b: u8::MAX, a: 128 },
         scale: ScalingFactor(2.0),
         screen_position: Pix2D::default_screen(),
         time_hit: Duration::default(),
         current_time: Duration::default(),
      }
   }
}
