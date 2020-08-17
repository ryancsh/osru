/*
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SampleSetType{ NoCustom, normal, soft, drum}
*/

pub const DEFAULT_MASTER_VOLUME: f32 = 0.50;
pub const DEFAULT_TRACK_VOLUME: f32 = 0.50;
pub const AUDIO_REFERENCE_POWER: usize = 3000;

pub const TIMING_WINDOW_GREAT: GameTime = GameTime(79_500);
pub const TIMING_WINDOW_GREAT_MULTIPLIER: usize = 6_000;
pub const TIMING_WINDOW_GOOD: GameTime = GameTime(139_500);
pub const TIMING_WINDOW_GOOD_MULTIPLIER: usize = 8_000;
pub const TIMING_WINDOW_MEH: GameTime = GameTime(199_500);
pub const TIMING_WINDOW_MEH_MULTIPLIER: usize = 10_000;

#[derive(Debug, Clone, Copy)]
pub enum OsruCurveType{ Bezier, CentripetalCatmullRom, Linear, PerfectCircle}
#[derive(Debug, Clone, Copy)]
pub enum OsruHitSuccess{Great, Good, Meh, Miss}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum OsruGameModName{
    None,
    Easy, HardRock, DoubleTime, HalfTime, NoFail, SuddenDeath, Perfect, Hidden, FlashLight, 
    //Scoring
    ScoreOsru, ScoreV1, ScoreV2,
    //Special
    Relax, AutoPilot, SpunOut, Auto, 
}

#[derive(Debug, Clone, Copy)]
pub struct OsruPixel(pub isize);
#[derive(Debug, Clone, Copy)]
pub struct OsruPixels(pub isize, pub isize);

#[derive(Debug, Clone, Copy)]
pub struct OsruCustomHitSound{pub normal: bool, pub whistle: bool, pub finish: bool, pub clap: bool }

#[derive(Debug, Clone, Copy)]
pub struct Colour<T>(pub T, pub T, pub T, pub T);

#[derive(Debug, Clone, Copy)]
pub struct OsruOD(pub f64);
#[derive(Debug, Clone, Copy)]
pub struct OsruAR(pub f64);
#[derive(Debug, Clone, Copy)]
pub struct OsruCS(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct GameTime(pub usize);


#[derive(Debug, Clone)]
pub enum OsruType{
    Integer(isize),
    Text(String),
    Decimal(f64),
    BitFlag(usize),
    List(Vec<OsruType>),
}

pub fn nstr(s: &str) -> String{
    String::from(s)
}

pub fn parse_key_value<'a>(line: &'a str, separator: &str) -> Option<(&'a str, &'a str)>{
    let split: Vec<&str> = line.trim().split(separator).collect();
    if split.len() == 2{
        let result = Some((split[0].trim(), split[1].trim()));
        return result;
    }
    None
}

pub fn parse_list<'a>(line: &'a str, separator: &str) -> Vec<&'a str>{
    line.trim().split(separator).map(|x| x.trim()).collect()
}

fn parse_osru_base_type<'a>(value: &str, old_value: &OsruType) -> Option<OsruType>{
    use OsruType::*;
    match old_value{
        Integer(_) => {
            let value = value.parse::<isize>();
            if let Ok(value) = value{
                Some(Integer(value))
            }
            else {None}
        },
        Decimal(_) => {
            let value = value.parse::<f64>();
            if let Ok(value) = value{
                Some(Decimal(value))
            }else {None}
        },
        BitFlag(_) => {
            let value = value.parse::<usize>();
            if let Ok(value) = value{
                Some(BitFlag(value))
            }else{None}
        },
        Text(_) => {
            let value = value.parse::<String>();
            if let Ok(value) = value{
                Some(Text(value))
            }else {None}
        },
        /*
        Time(_) => {
            let value = value.parse::<isize>();
            if let Ok(value) = value{
                let mut value = value;
                if value < 0 {value = 0}
                Some(Time(value as usize))
            }else {None}
        },
        */
        List(v) => {
            panic![]
        }
    }
}

pub fn parse_osru_type(value: &str, old_value: &OsruType, separator: Option<&str>) -> Option<OsruType> {
    use OsruType::*;

    let separator = match separator{
        Some(x) => x,
        _ => ",",
    };

    match old_value{
        List(vec) => {
            if let Some(value_in_vec) = vec.get(0){
                let mut v = vec![];
                let values_parsed = parse_list(value, separator);

                for value in values_parsed{
                    if let Some(result) = parse_osru_base_type(value, value_in_vec){
                        v.push(result);
                    }
                }
                if v.len() > 0{
                    Some(List(v))
                }
                else {None}
            }
            else {panic![]}
        }
        _ => parse_osru_base_type(value, old_value),
    }
}

pub fn convert_hitsound_bitflag(hitsound_bitflags: isize) -> OsruCustomHitSound{
    OsruCustomHitSound{
        normal: hitsound_bitflags & 0b1 == 0b1,
        whistle: hitsound_bitflags & 0b10 == 0b10,
        finish: hitsound_bitflags & 0b100 == 0b100,
        clap: hitsound_bitflags & 0b1000 == 0b1000,
    }
}