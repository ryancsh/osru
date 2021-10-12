use crate::global::*;

use rodio::{source::Source, Device, Sink};
use std::sync::mpsc;
use std::{
   cell::RefCell,
   fs,
   io::BufReader,
   rc::Rc,
   sync::{Arc, Mutex},
   thread,
   time::{Duration, Instant},
};

pub enum AudioMessage {
   Ready,
   Stop,
   Play(usize, Duration),
   Done,
}

pub struct AudioManager {
   _device: Device,
   sinks: Vec<SinkWrapper>,
   sources: Vec<AudioSource>,
}

impl AudioManager {
   pub fn new() -> AudioManager {
      let _device = rodio::default_output_device().unwrap();
      let sinks = vec![];
      let sources = vec![];
      let mut audio_manager = AudioManager { _device, sinks, sources };
      audio_manager.new_sink();
      audio_manager
   }

   pub fn new_sink(&mut self) {
      let mut new_sink = SinkWrapper::new(&self._device);
      new_sink.set_track_volume(DEFAULT_TRACK_VOLUME);
      self.sinks.push(new_sink);
   }

   pub fn add_source(&mut self, filename: &str) -> usize {
      let id = self.sources.len();
      self.sources.push(AudioSource::new(filename));
      id
   }

   pub fn normalize_volume(data: &Vec<i16>) -> f32 {
      let mut samples_count: [usize; 32769] = [0; 32769]; // (i16::MIN.abs() == 32768) > (i16::MAX == 32767)
      let mut samples_total_count: usize = 0;
      for sample in data {
         let sample = (*sample as i32).abs() as usize;
         samples_count[sample] += 1;
         samples_total_count += 1;
      }

      samples_total_count -= samples_count[0];
      if samples_total_count == 0 {
         return DEFAULT_TRACK_VOLUME;
      }

      let count_skip = samples_total_count / 4;
      let mut count_limit = samples_total_count / 2;
      if samples_total_count < 2 {
         count_limit = samples_total_count;
      }

      let mut current_count: usize = 0;
      let mut sum: u128 = 0;
      'find_start: for i in 1..samples_count.len() {
         current_count += samples_count[i];
         if current_count >= count_skip {
            if current_count >= count_limit {
               current_count -= samples_count[i];
               let difference = count_limit - count_skip;
               current_count += difference;
               sum += (difference * i) as u128;
               break 'find_start;
            } else {
               current_count -= count_skip;
               sum += (current_count * i) as u128;
               '_add_rest: for j in i + 1..samples_count.len() {
                  current_count += samples_count[j];
                  if current_count >= count_limit {
                     current_count -= samples_count[j];
                     let difference = count_limit - current_count;
                     current_count += difference;
                     sum += (difference * j) as u128;
                     break 'find_start;
                  } else {
                     sum += (samples_count[j] * j) as u128;
                  }
               }
            }
            panic!["Should never be here"];
         }
      }

      (current_count as f64 * AUDIO_REFERENCE_POWER as f64 / sum as f64) as f32 * DEFAULT_TRACK_VOLUME
   }

   pub fn play_source(&mut self, audio_source_id: usize) {
      let audio_source = self.get_audio_source(audio_source_id);
      for sink in self.sinks.iter_mut() {
         if sink.empty() {
            sink.append(audio_source);
            return;
         }
      }
      self.new_sink();
      self.play_source(audio_source_id);
   }

   pub fn get_audio_source(&self, audio_source_id: usize) -> AudioSource {
      self.sources.get(audio_source_id).unwrap().clone()
   }

   pub fn track_volume(&self, id: usize) -> f32 {
      self.sources.get(id).unwrap().track_volume()
   }

   pub fn sleep_until_end(&self) {
      for sink in self.sinks.iter() {
         sink.sleep_until_end();
      }
   }

   pub fn isPlaying(&self) -> bool {
      let mut result = false;
      for sink in self.sinks.iter() {
         result = result || !sink.empty();
      }
      result
   }

   pub fn wait(&mut self, rx: mpsc::Receiver<AudioMessage>) {
      let mut wait_for_end = false;
      'running: loop {
         match rx.try_recv() {
            Ok(AudioMessage::Stop) => break 'running,
            Ok(AudioMessage::Done) => wait_for_end = true,
            Ok(AudioMessage::Play(id, sleep)) => {
               let now = Instant::now();
               while now.elapsed() < sleep {
                  thread::yield_now()
               }
               self.play_source(id)
            }
            _ => (),
         }
         if wait_for_end && !self.isPlaying() {
            break 'running;
         }
         thread::yield_now();
      }
   }

   pub fn get_channel_volume(&self, channel_id: usize) -> f32 {
      self.sinks.get(channel_id).unwrap().volume()
   }

   pub fn set_master_volume(&mut self, value: f32) {
      for sink in self.sinks.iter_mut() {
         sink.set_master_volume(value);
      }
   }

   pub fn master_volume(&self) -> f32 {
      self.sinks[0].master_volume()
   }

   pub fn stop_playing(&self) {
      for sink in self.sinks.iter() {
         sink.stop();
      }
   }
}

pub struct AudioSource {
   audiofile: Arc<AudioFile>,
   current_pos: usize,
}

impl AudioSource {
   pub fn new(filename: &str) -> AudioSource {
      AudioSource { audiofile: Arc::new(AudioFile::new(filename)), current_pos: 0 }
   }

   pub fn track_volume(&self) -> f32 {
      self.audiofile.volume()
   }

   pub fn channels(&self) -> u16 {
      self.audiofile.channels()
   }

   pub fn sample_rate(&self) -> u32 {
      self.audiofile.sample_rate()
   }

   pub fn sample_at(&self, position: usize) -> Option<i16> {
      self.audiofile.sample_at(position)
   }

   pub fn len(&self) -> usize {
      self.audiofile.len()
   }

   pub fn current_pos(&self) -> usize {
      self.current_pos
   }

   pub fn set_current_pos(&mut self, value: usize) {
      self.current_pos = value
   }
}

impl Clone for AudioSource {
   fn clone(&self) -> AudioSource {
      AudioSource { audiofile: Arc::clone(&self.audiofile), current_pos: 0 }
   }
}

impl Iterator for AudioSource {
   type Item = i16;

   fn next(&mut self) -> Option<i16> {
      let result = self.sample_at(self.current_pos());
      if let Some(_) = result {
         self.set_current_pos(self.current_pos() + 1);
      }
      result
   }
}

impl Source for AudioSource {
   fn current_frame_len(&self) -> Option<usize> {
      Some(self.len() - self.current_pos())
   }

   fn channels(&self) -> u16 {
      self.channels()
   }

   fn sample_rate(&self) -> u32 {
      self.sample_rate()
   }

   fn total_duration(&self) -> Option<Duration> {
      let sample_per_channel = ((self.len() - self.current_pos()) / self.channels() as usize) as u128;
      let duration_ns = sample_per_channel * 1_000_000_000 / self.sample_rate() as u128;
      let duration_s = duration_ns / 1_000_000_000;
      let duration_ns = duration_ns - (duration_s * 1_000_000_000);
      Some(Duration::new(duration_s as u64, duration_ns as u32))
   }
}

pub struct AudioFile {
   samples: Vec<i16>,
   volume: f32,
   channels: u16,
   sample_rate: u32,
}

impl AudioFile {
   pub fn new(filename: &str) -> AudioFile {
      let file = fs::File::open(filename).unwrap();
      let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
      let channels = source.channels();
      let sample_rate = source.sample_rate();
      let mut v = vec![];

      for sample in source {
         v.push(sample);
      }

      let volume = AudioManager::normalize_volume(&v);

      v.shrink_to_fit();
      AudioFile { samples: v, volume, channels, sample_rate }
   }

   pub fn volume(&self) -> f32 {
      self.volume
   }

   pub fn channels(&self) -> u16 {
      self.channels
   }

   pub fn sample_rate(&self) -> u32 {
      self.sample_rate
   }

   pub fn sample_at(&self, position: usize) -> Option<i16> {
      if let Some(num) = self.samples.get(position) {
         return Some(*num);
      }
      None
   }

   pub fn len(&self) -> usize {
      self.samples.len()
   }
}

struct SinkWrapper {
   sink: Sink,
   master_volume: f32,
   track_volume: f32,
}

impl SinkWrapper {
   pub fn new(device: &Device) -> SinkWrapper {
      SinkWrapper {
         sink: Sink::new(device),
         master_volume: DEFAULT_MASTER_VOLUME,
         track_volume: DEFAULT_TRACK_VOLUME,
      }
   }

   pub fn append(&mut self, source: AudioSource) {
      self.set_track_volume(source.track_volume());
      self.sink.append(source);
   }

   pub fn master_volume(&self) -> f32 {
      self.master_volume
   }

   pub fn volume(&self) -> f32 {
      self.sink.volume()
   }

   pub fn set_track_volume(&mut self, value: f32) {
      let value = {
         if value > 1.0 {
            1.0
         } else {
            value
         }
      };
      self.track_volume = value;
      self.update_sink_volume();
   }

   pub fn track_volume(&self) -> f32 {
      self.track_volume
   }

   pub fn set_master_volume(&mut self, value: f32) {
      let value = {
         if value > 1.0 {
            1.0
         } else {
            value
         }
      };
      self.master_volume = value;
      self.update_sink_volume();
   }

   fn update_sink_volume(&self) {
      self.sink.set_volume(self.master_volume() * self.track_volume());
   }

   pub fn play(&self) {
      self.sink.play()
   }

   pub fn pause(&self) {
      self.sink.pause()
   }

   pub fn is_paused(&self) -> bool {
      self.sink.is_paused()
   }

   pub fn stop(&self) {
      self.sink.stop()
   }

   pub fn sleep_until_end(&self) {
      self.sink.sleep_until_end()
   }

   pub fn empty(&self) -> bool {
      self.sink.empty()
   }

   pub fn len(&self) -> usize {
      self.sink.len()
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use rand::prelude::*;
   #[test]
   #[ignore]
   fn test_normalize_volume() {
      fn test_vol(message: &str, samples: &Vec<i16>) {
         println!("test_normalize_volume {}", message);
         let mut data2 = Vec::with_capacity(samples.len());
         for i in samples.iter() {
            if *i != 0 {
               data2.push((*i as i32).abs() as usize);
            }
         }
         data2.sort();

         let mut sum = 0;
         let mut count = data2.len() / 2;
         if count < 1 {
            count = data2.len();
         }
         if count == 0 {
            assert_eq!(DEFAULT_TRACK_VOLUME, AudioManager::normalize_volume(samples));
            return;
         }
         for i in data2.len() / 4..data2.len() / 4 + count {
            sum += data2.get(i).unwrap();
         }
         println!("test sum {}, count {}, data2.len {}", sum, count, data2.len());
         assert_eq!(
            (AUDIO_REFERENCE_POWER as f64 * count as f64 / sum as f64) as f32 * DEFAULT_TRACK_VOLUME,
            AudioManager::normalize_volume(samples)
         );
      }

      let mut data = vec![];
      data.push(0);
      test_vol("0", &data);

      data.clear();
      test_vol("empty", &data);

      data.push(0);
      data.push(3000);
      test_vol("0, 3000", &data);

      data.push(4000);
      test_vol("0, 3000, 4000", &data);

      data.push(5000);
      test_vol("0, 3000, 4000, 5000", &data);

      data.push(6000);
      test_vol("0, 3000, 4000, 5000, 6000", &data);

      data.push(7000);
      test_vol("0, 3000, 4000, 5000, 6000, 7000", &data);

      data.clear();
      data.push(-3000);
      test_vol("-3000, 4000, 5000, 6000", &data);

      data.push(-4000);
      test_vol("-3000, -4000", &data);

      data.push(-5000);
      test_vol("-3000, -4000, -5000", &data);

      data.push(-6000);
      test_vol("-3000, -4000, -5000, -6000", &data);

      data.push(-7000);
      test_vol("-3000, -4000, -5000, -6000, -7000", &data);

      data.clear();
      for i in 0..i16::MAX {
         data.push(i);
      }
      test_vol("0 .. i16::MAX", &data);

      data.clear();
      for i in i16::MIN..0 {
         data.push(i);
      }
      test_vol("i16::MIN .. 0", &data);

      data.clear();
      for i in i16::MIN..i16::MAX {
         data.push(i);
      }
      test_vol("i16::MIN .. i16::MAX", &data);

      data.clear();
      let mut rng = rand::thread_rng();
      for _ in 0..10_000_000 {
         data.push(rng.gen());
      }
      test_vol("10,000,000 random i16", &data);
   }
}
