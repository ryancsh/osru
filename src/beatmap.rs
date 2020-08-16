#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use crate::audio;
use crate::global::*;


use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::cmp;
use std::slice;
use std::clone;
use std::any;

#[derive(Debug)]
pub struct Beatmap{
    pub settings: HashMap<BeatmapSettings, OsruType>,

    pub timing_points: Vec<TimingPoint>,
    /*
    pub event_backgrounds : Vec<EventBackground>,
    pub event_videos: Vec<EventVideo>,
    pub event_breaks: Vec<EventBreak>,
    // storyboard
    pub beatmap_colours: Vec<BeatmapColour>,
    //pub hit_objects: Vec<HitObject>,
    */
}
impl Beatmap{
    fn new() -> Beatmap{
        let mut s = HashMap::new();
        {
            use BeatmapSettings::*;
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
        }

        Beatmap{
            settings: s,
            
            timing_points: vec![],
            /*
            event_backgrounds: vec![],
            event_videos: vec![],
            event_breaks: vec![],
            beatmap_colours: vec![],
            //hitobjects
            */
        }
    }

    pub fn load(filename: &str){
        let mut beatmap = Beatmap::new();
        let file = fs::read_to_string(filename).unwrap();

        use BeatmapSection::*;
        let mut section = General;
        'next_line: for line in file.lines(){
            let line = line.trim();
            if line.starts_with("//") || line.len() == 0{
                continue 'next_line;
            }
            if line.contains("[") && line.contains("]") && line.len() > 2{
                let line = line.trim_start_matches("[");
                let line = line.trim_end_matches("]");

                for v in BeatmapSection::iter(){
                    if line == v.to_string(){
                        section = *v;
                        continue 'next_line;
                    }
                }
            }
            else if section == General || section == Editor || section == Metadata || section == Difficulty{
                if let Some((k, value)) = parse_key_value(line, ":"){
                    let mut key = BeatmapSettings::None;
                    for val in BeatmapSettings::iter(){
                        if val.to_string() == k{
                            key = *val;
                        }
                    }
                    if let Some(old_value) = beatmap.settings.get(&key){
                        if section == Metadata && key == BeatmapSettings::Tags{
                            if let Some(result) = parse_osru_type(value, old_value, Some(" ")){
                                beatmap.settings.insert(key, result);
                            }
                        }
                        else if let Some(result) = parse_osru_type(value, old_value, None){
                            beatmap.settings.insert(key, result);
                        }
                    }
                }
                else {continue 'next_line;}
            }
            else if section == Events{
                let line = parse_list(line, ",");
                if line.len() >= 3{
                    if line[0] == "0" {
                        ////////////////////////////
                    }

                }
            }
            else if section == TimingPoints{
                let line = parse_list(line, ",");
                if line.len() >= 8{
                    let t = TimingPoint{
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
            }
            else if section == Colours{
                ///////////////////////////////////////////////////////////////////////////
            }
            else if section == HitObjects{
                let line = parse_list(line, ",");
                if line.len() >= 4{
                    let x = line[0].trim().parse().unwrap_or_default();
                    let y = line[1].trim().parse().unwrap_or_default();
                    let position = OsruPixels(x, y);
                    let time:isize = line[2].trim().parse().unwrap_or_default();
                    let type_bitflags = line[3].trim().parse::<isize>().unwrap_or_default();
                    let hitsound_bitflags = line[4].trim().parse::<isize>().unwrap_or_default();
                    let hitsound = convert_hitsound_bitflag(hitsound_bitflags);
                    if type_bitflags & 0b1 == 0b1 {
                        //hitcircle
                        //println!("hit circle {:?}", line);
                    }
                    else if type_bitflags & 0b10 == 0b10 {
                        //slider
                        use OsruCurveType::*;
                        let curve = line[5].trim().parse::<String>().unwrap_or_default();
                        let curve = curve.trim();
                        let curve = parse_list(curve, "|");
                        let mut iter = curve.iter();
                        let mut curve_type = Bezier;
                        let mut curve_list = vec![];
                        if let Some(c) = iter.next(){
                            curve_type = match *c {
                                "C" => CentripetalCatmullRom,
                                "L" => Linear,
                                "P" => PerfectCircle,
                                _ => Bezier,
                            };

                            for pt in iter{
                                let split = parse_list(pt, ":");
                                if split.len() == 2{
                                    let x = split[0].trim().parse().unwrap_or_default();
                                    let y = split[1].trim().parse().unwrap_or_default();
                                    let curve_point = OsruPixels(x, y);
                                    curve_list.push(curve_point);
                                }
                            }
                        }
                        let num_slides = line[6].trim().parse::<isize>().unwrap_or(1);
                        let length_of_slider = line[7].trim().parse::<f64>().unwrap_or_default();
                        let mut edge_sounds = vec![];
                        if line.len() >= 9{
                            let line8 = parse_list(line[8], "|");
                            for sound in line8{
                                edge_sounds.push(sound.trim().parse::<isize>().unwrap_or(0));
                            }
                        }
                        //let mut edge_sets = vec![];
                        if line.len() >= 10{
                            let line9 = parse_list(line[9], "|");
                            for sound in line9{
                                //edge_sounds.push(sound.trim().parse::<isize>().unwrap_or(0));
                                // TODO: deal with hitsound bitflag
                            }
                        }



                        //println!("slider {:?}", line);
                    }
                    else if type_bitflags & 0b1000 == 0b1000{
                        // TODO: spiner
                        //println!("spinner {:?}", line);
                    }
                    else if (type_bitflags & 0b1000_0000) == 0b1000_0000{
                        // LOW PRIORITY: mania hold
                        println!("mania {:?}", line);
                    }
                    else{
                        println!("unknown {:?}", line);
                    }

                    if type_bitflags & 0b0100 == 0b0100{
                        //new combo
                    }
                    let combo_colours_to_skip = (type_bitflags & 0b1110000) >> 4;
                    
                }
            }
        }
        //println!("settings {:?}", beatmap.settings);
        //println!("timing points {:?}", beatmap.timing_points);
    }

    
}


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum BeatmapSettings{
    None,

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
impl BeatmapSettings{
    fn iter() -> slice::Iter<'static, BeatmapSettings>{
        use BeatmapSettings::*;
        static VALUES: [BeatmapSettings; 40] = [
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
        ];
        VALUES.iter()
    }
}
impl fmt::Display for BeatmapSettings{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

pub struct EventBackground{
    start_time: isize,
    filename: String,
    offset_from_center: OsruPixels,
}

pub struct EventVideo{
    start_time: isize,
    offset_from_center: OsruPixels,
}

pub struct EventBreak{
    start_time: isize,
    end_time: isize,
}

#[derive(Debug, Copy, Clone)]
pub struct TimingPoint{
    start_time: isize,
    beat_length: f64,
    meter: isize,
    sample_set: isize,
    sample_index: isize,
    volume: isize,
    uninherited: isize,
    effects: usize,
}

pub struct BeatmapColour{
    combo_colour: Colour<u8>,
    slider_track_override_colour: Colour<u8>,
    slider_border_colour: Colour<u8>,
}

pub struct HitCircle{
    position: OsruPixels,
    time: isize,
    type_bitflags: u8,
    hitsound_bitflags: u8,
    hitsample_set: isize,
    hitsample_additional_set: isize,
    hitsample_index: isize,
    hitsample_volume: f32,
    hitsample_filename: String,
}

pub struct Slider{
    position: OsruPixels,
    time: isize,
    type_bitflags: u8,
    hitsound_bitflags: u8,
    curve_type: char,
    curve_points: Vec<OsruPixels>,
    num_slides: isize,
    length_of_slider: OsruPixels,
    edge_sounds: Vec<isize>,
    edge_sets: Vec<String>,
}

pub struct Spinner{
    position: OsruPixels,
    time: isize,
    type_bitflags: u8,
    hitsound_bitflags: u8,
    end_time: isize,
    hit_sample: Vec<isize>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BeatmapSection{General, Editor, Metadata, Difficulty, Events, TimingPoints, Colours, HitObjects}
impl BeatmapSection{
    pub fn eq(&self, other: &str) -> bool{
        self.to_string() == other
    }
    pub fn iter() -> slice::Iter<'static, BeatmapSection>{
        use BeatmapSection::*;
        static VALUES: [BeatmapSection; 8] = [General, Editor, Metadata, Difficulty, Events, TimingPoints, Colours, HitObjects];
        VALUES.iter()
    }
}
impl fmt::Display for BeatmapSection{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
}