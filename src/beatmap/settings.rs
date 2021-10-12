use super::*;
use std::fmt;

use super::timing::*;
use BeatmapSettingName::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, IntoEnumIterator)]
pub enum BeatmapSettingName {
   Unknown,
   //General
   AudioFilename,
   AudioLeadIn,
   AudioHash,
   PreviewTime,
   Countdown,
   SampleSet,
   StackLeniency,
   Mode,
   LetterboxInBreaks,
   StoryFireInFront,
   UseSkinSprites,
   AlwaysShowPlayfield,
   OverlayPosition,
   SkinPreference,
   EpilepsyWarning,
   CountdownOffset,
   SpecialStyle,
   WidescreenStoryboard,
   SamplesMatchPlaybackRate,
   //Editor
   Bookmarks,
   DistanceSpacing,
   BeatDivisor,
   GridSize,
   TimelineZoom,
   // Metadata
   Title,
   TitleUnicode,
   Artist,
   ArtistUnicode,
   Creator,
   Version,
   Source,
   Tags,
   BeatmapID,
   BeatmapSetID,
   // Difficulty
   HPDrainRate,
   CircleSize,
   OverallDifficulty,
   ApproachRate,
   SliderMultiplier,
   SliderTickRate,
}
impl BeatmapSettingName {
   pub fn from_str(name: &str) -> BeatmapSettingName {
      for beatmap_setting_name in BeatmapSettingName::into_enum_iter() {
         if beatmap_setting_name.to_string() == name {
            return beatmap_setting_name;
         }
      }
      panic!["BeatmapSettingName {:?} not found", name];
   }
}

impl fmt::Display for BeatmapSettingName {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
   }
}

#[derive(Debug, Clone)]
pub struct BeatmapSettings {
   settings: HashMap<BeatmapSettingName, OsruType>,
}

impl BeatmapSettings {
   pub fn new() -> BeatmapSettings {
      let mut s = HashMap::new();

      use BeatmapSettingName::*;
      use OsruType::*;
      //General
      s.insert(AudioFilename, Text(nstr("")));
      s.insert(AudioLeadIn, Integer(0));
      s.insert(AudioHash, Text(nstr("")));
      s.insert(PreviewTime, Integer(-1));
      s.insert(Countdown, Integer(1));
      s.insert(SampleSet, Text(nstr("Normal")));
      s.insert(StackLeniency, Decimal(0.7));
      s.insert(Mode, Integer(0));
      s.insert(LetterboxInBreaks, Integer(0));
      s.insert(StoryFireInFront, Integer(1));
      s.insert(UseSkinSprites, Integer(0));
      s.insert(AlwaysShowPlayfield, Integer(0));
      s.insert(OverlayPosition, Text(nstr("NoChange")));
      s.insert(SkinPreference, Text(nstr("")));
      s.insert(EpilepsyWarning, Integer(0));
      s.insert(CountdownOffset, Integer(0));
      s.insert(SpecialStyle, Integer(0));
      s.insert(WidescreenStoryboard, Integer(0));
      s.insert(SamplesMatchPlaybackRate, Integer(0));

      //Editor
      s.insert(Bookmarks, List(vec![Integer(0)]));
      s.insert(DistanceSpacing, Decimal(0.0));
      s.insert(BeatDivisor, Decimal(0.0));
      s.insert(GridSize, Integer(0));
      s.insert(TimelineZoom, Decimal(0.0));

      //Metadata
      s.insert(Title, Text(nstr("")));
      s.insert(TitleUnicode, Text(nstr("")));
      s.insert(Artist, Text(nstr("")));
      s.insert(ArtistUnicode, Text(nstr("")));
      s.insert(Creator, Text(nstr("")));
      s.insert(Version, Text(nstr("")));
      s.insert(Source, Text(nstr("")));
      s.insert(Tags, List(vec![Text(nstr(""))]));
      s.insert(BeatmapID, Integer(-1));
      s.insert(BeatmapSetID, Integer(-1));

      //Difficulty
      s.insert(HPDrainRate, Decimal(-1.0));
      s.insert(CircleSize, Decimal(-1.0));
      s.insert(OverallDifficulty, Decimal(-1.0));
      s.insert(ApproachRate, Decimal(-1.0));
      s.insert(SliderMultiplier, Decimal(-1.0));
      s.insert(SliderTickRate, Decimal(-1.0));

      BeatmapSettings { settings: s }
   }

   pub fn get(&self, setting_name: &BeatmapSettingName) -> Option<&OsruType> {
      if let Some(value) = self.settings.get(setting_name) {
         Some(value)
      } else {
         None
      }
   }

   pub fn set(&mut self, setting_name: &BeatmapSettingName, value: OsruType) {
      self.settings.insert(*setting_name, value);
   }

   pub fn shrink_to_fit(&mut self) {
      self.settings.shrink_to_fit();
   }

   pub fn hp_drain_rate(&self) -> f64 {
      self.get(&HPDrainRate).unwrap().parse_as_dec()
   }
   pub fn circle_size(&self) -> f64 {
      self.get(&CircleSize).unwrap().parse_as_dec()
   }
   pub fn overall_difficulty(&self) -> OsruOD {
      OsruOD(self.get(&OverallDifficulty).unwrap().parse_as_dec())
   }
   pub fn approach_rate(&self) -> OsruAR {
      OsruAR(self.get(&ApproachRate).unwrap().parse_as_dec())
   }
   pub fn slider_multiplier(&self) -> f64 {
      self.get(&SliderMultiplier).unwrap().parse_as_dec()
   }
   pub fn slider_tick_rate(&self) -> f64 {
      self.get(&SliderTickRate).unwrap().parse_as_dec()
   }
}
