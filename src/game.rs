use crate::global::*;

use std::time;
use std::collections::HashSet;
use std::hash;
use std::cmp;
use std::slice;


#[derive(Debug, Clone)]
pub struct OsruGameModsActive{
    mods: HashSet<OsruGameMod>,
}
impl OsruGameModsActive {
    pub fn new() -> OsruGameModsActive{
        OsruGameModsActive{
            mods: HashSet::new(),
        }
    }
    pub fn od_multiplier(&self) -> f64{
        let mut od_mul = 1.0;
        for game_mod in self.mods.iter(){
            od_mul *= game_mod.od_multiplier;
        }
        if od_mul > 10.0{
            10.0
        } else {od_mul}
    }
    // TODO: perceived_od_mul()
    
    pub fn timing_window(&self, hit_success: OsruHitSuccess, od: OsruOD) -> GameTime{
        let (GameTime(base_timing), multiplier) = match hit_success{
            OsruHitSuccess::Great => (TIMING_WINDOW_GREAT, TIMING_WINDOW_GREAT_MULTIPLIER),
            OsruHitSuccess::Good => (TIMING_WINDOW_GOOD, TIMING_WINDOW_GOOD_MULTIPLIER),
            OsruHitSuccess::Meh => (TIMING_WINDOW_MEH, TIMING_WINDOW_MEH_MULTIPLIER),
            OsruHitSuccess::Miss => panic![],
        };
        let od_multiplier = self.od_multiplier();
        GameTime(base_timing - (self.od_multiplier() * multiplier as f64 *od.0) as usize)
    }

    pub fn enable_game_mod(&mut self, new_mod: OsruGameModName){
        let mut to_remove = vec![];
        let new_mod = OsruGameMod::new(new_mod);
        {
            for m in self.mods.iter(){
                if new_mod.eq(m){
                    return;
                }
                for exclude in m.exclusive(){
                    if new_mod.name().eq(exclude){
                        to_remove.push(OsruGameMod::new(*exclude));
                    }
                }
            }
        }
        
        for m in to_remove.iter(){
            self.mods.remove(m);
        }
        self.mods.insert(new_mod);
    }

    pub fn disable_game_mod(&mut self, mod_to_disable: OsruGameModName){
        let mod_to_disable = OsruGameMod::new(mod_to_disable);
        self.mods.remove(&mod_to_disable);
    }
}


#[derive(Debug, Clone)]
pub struct OsruGameMod{
    game_mod_name: OsruGameModName,
    exclusive: Vec<OsruGameModName>,

    ar_multiplier: f64,
    od_multiplier: f64,
    cs_multiplier: f64,
}
impl OsruGameMod{
    pub fn new(name: OsruGameModName) -> OsruGameMod{
        use OsruGameModName::*;
        let mut result = OsruGameMod::default();
        result.game_mod_name = name;
        match name{
            Easy => {
                result.exclusive.push(HardRock);
                result.od_multiplier = 0.5;
            },
            HardRock => {
                result.exclusive.push(Easy);
                result.od_multiplier = 1.4;
            }
            _ => ()
        }
        result
    }

    // TODO: other mods
    pub fn exclusive<'a>(&'a self) -> slice::Iter<'a, OsruGameModName>{
        self.exclusive.iter()
    }
    pub fn name(&self) -> OsruGameModName{
        self.game_mod_name
    }
}
impl Default for OsruGameMod{
    fn default() -> Self{
        OsruGameMod{
            game_mod_name: OsruGameModName::None,
            exclusive: vec![],
            ar_multiplier: 1.0,
            od_multiplier: 1.0,
            cs_multiplier: 1.0,
        }
    }
}
impl hash::Hash for OsruGameMod{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.game_mod_name.hash(state);
    }
}
impl cmp::PartialEq for OsruGameMod{
    fn eq(&self, other: &OsruGameMod) -> bool{
        self.game_mod_name == other.game_mod_name
    }
}
impl cmp::Eq for OsruGameMod{}