use crate::global::*;

pub struct EventBackground {
   pub start_time: isize,
   pub filename: String,
   pub offset_from_center: OsruPixels,
}

pub struct EventVideo {
   start_time: isize,
   offset_from_center: OsruPixels,
}

pub struct EventBreak {
   start_time: isize,
   end_time: isize,
}
