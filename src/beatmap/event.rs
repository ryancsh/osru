use crate::global::pixel::*;

pub struct EventBackground {
   pub start_time: isize,
   pub filename: String,
   pub offset_from_center: Pix2D,
}

pub struct EventVideo {
   start_time: isize,
   offset_from_center: Pix2D,
}

pub struct EventBreak {
   start_time: isize,
   end_time: isize,
}
