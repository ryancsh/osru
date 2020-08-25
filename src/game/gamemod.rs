
#[derive(Debug, Clone)]
pub struct OsruGameModsActive {
  mods: HashSet<OsruGameMod>,
}
impl OsruGameModsActive {
  pub fn new() -> OsruGameModsActive {
    OsruGameModsActive { mods: HashSet::new() }
  }

  pub fn enable_game_mod(&mut self, new_mod: OsruGameModName) {
    let mut to_remove = vec![];
    let new_mod = OsruGameMod::new(new_mod);
    {
      for m in self.mods.iter() {
        if new_mod.eq(m) {
          return;
        }
        for exclude in m.exclusive() {
          if new_mod.name().eq(exclude) {
            to_remove.push(OsruGameMod::new(*exclude));
          }
        }
      }
    }

    for m in to_remove.iter() {
      self.mods.remove(m);
    }
    self.mods.insert(new_mod);
  }

  pub fn disable_game_mod(&mut self, mod_to_disable: OsruGameModName) {
    let mod_to_disable = OsruGameMod::new(mod_to_disable);
    self.mods.remove(&mod_to_disable);
  }
}

#[derive(Debug, Clone)]
pub struct OsruGameMod {
  game_mod_name: OsruGameModName,
  exclusive: Vec<OsruGameModName>,

  ar_multiplier: f64,
  od_multiplier: f64,
  cs_multiplier: f64,
}
impl OsruGameMod {
  pub fn new(name: OsruGameModName) -> OsruGameMod {
    use OsruGameModName::*;
    let mut result = OsruGameMod::default();
    result.game_mod_name = name;
    match name {
      Easy => {
        result.exclusive.push(HardRock);
        result.ar_multiplier = 0.5;
        result.od_multiplier = 0.5;
      }
      HardRock => {
        result.exclusive.push(Easy);
        result.ar_multiplier = 1.4;
        result.od_multiplier = 1.4;
      }
      _ => (),
    }
    result
  }

  // TODO: other mods
  pub fn exclusive<'a>(&'a self) -> slice::Iter<'a, OsruGameModName> {
    self.exclusive.iter()
  }

  pub fn name(&self) -> OsruGameModName {
    self.game_mod_name
  }
}
impl Default for OsruGameMod {
  fn default() -> Self {
    OsruGameMod {
      game_mod_name: OsruGameModName::None,
      exclusive: vec![],
      ar_multiplier: 1.0,
      od_multiplier: 1.0,
      cs_multiplier: 1.0,
    }
  }
}
impl hash::Hash for OsruGameMod {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    self.game_mod_name.hash(state);
  }
}
impl cmp::PartialEq for OsruGameMod {
  fn eq(&self, other: &OsruGameMod) -> bool {
    self.game_mod_name == other.game_mod_name
  }
}
impl cmp::Eq for OsruGameMod {}

#[derive(Debug, Clone, Hash, Eq, PartialEq, IntoEnumIterator)]
pub enum OsruGameModName {
  None,
  Easy,
  HardRock,
  DoubleTime,
  HalfTime,
  NoFail,
  SuddenDeath,
  Perfect,
  Hidden,
  FlashLight,
  //Scoring
  ScoreOsru,
  ScoreV1,
  ScoreV2,
  //Special
  Relax,
  AutoPilot,
  SpunOut,
  Auto,
}
