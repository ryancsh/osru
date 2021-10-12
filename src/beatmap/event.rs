use crate::global::pixel::*;

pub struct EventBackground {
   pub start_time: i32,
   pub filename: String,
   pub offset_from_center: Pix2D,
}

pub struct EventVideo {
   start_time: i32,
   offset_from_center: Pix2D,
}

pub struct EventBreak {
   start_time: i32,
   end_time: i32,
}
