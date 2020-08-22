use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, IntoEnumIterator)]
pub enum BeatmapSettings {
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

impl fmt::Display for BeatmapSettings {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
   }
}
