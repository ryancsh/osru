#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod audio;
mod beatmap;
mod game;
mod global;
mod input;
mod time;

use enum_dispatch::enum_dispatch;
use global::*;
use sdl2::event::*;

use std::{
   fs::File,
   io::prelude::*,
   sync::{mpsc, Arc, Mutex},
   thread,
   time::{Duration, Instant},
};

fn main() {
   let g = game::Game::start();
}
