use crate::global::*;

pub struct EventBackground {
  start_time: isize,
  filename: String,
  offset_from_center: OsruPixels,
}

pub struct EventVideo {
  start_time: isize,
  offset_from_center: OsruPixels,
}

pub struct EventBreak {
  start_time: isize,
  end_time: isize,
}
