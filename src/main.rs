#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate sdl2;

mod audio;

use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};

static _TARGET_FPS: usize = 72;

const _AUDIO_FILENAME: &str = "assets/audio/magic.wav";
const _AUDIO_NORMALIZE: bool = true;

fn main() {
    //_test_concurrency();
    _audio_stuff();
}

fn _test_concurrency(){
    // test Mutex
    const ARRAY_SIZE:usize = 1_000_000;
    const LOOP_NUMBER: usize = 1_000_000;
    const LOOP_MUTEX:bool = true;
    const LOOP_MPSC:bool = true;
    if LOOP_MUTEX {
        let mut array = vec![];
        for _ in 0..4{
            array.push(Mutex::new(vec![0; ARRAY_SIZE]));
        }
        let array = Arc::new(array);
        let mut threads = vec![];

        let start_time = SystemTime::now();
        for _ in 0..2{
            let a = array.clone();
            let thread = thread::spawn( move || {
                let mut count = 0;
                for _loop_num in 0..LOOP_NUMBER{
                    'find_free_mutex: loop {
                        if let Ok(mut data) = a.get(count).unwrap().try_lock(){
                            for i in data.iter_mut(){
                                *i = (*i + 1) % 1_000_000;
                            }
                            break 'find_free_mutex;
                        }
                        count = (count + 1) % a.len();
                    }
                }
            });
            threads.push(thread);
        }

        for t in threads{
            t.join().unwrap();
        }
        println!("Time taken with mutex {}", SystemTime::now().duration_since(start_time).unwrap().as_millis());
    }

    if LOOP_MPSC{
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let array = vec![0; ARRAY_SIZE];
        tx1.send(array).unwrap();
        let array = vec![0; ARRAY_SIZE];
        tx1.send(array).unwrap();
        let array = vec![0; ARRAY_SIZE];
        tx2.send(array).unwrap();
        let array = vec![0; ARRAY_SIZE];
        tx2.send(array).unwrap();

        let rx1 = Arc::new(Mutex::new(rx1));
        let _r1 = rx1.clone();
        let rx2 = Arc::new(Mutex::new(rx2));
        let _r2 = rx2.clone();

        let start_time = SystemTime::now();
        let thread = std::thread::spawn(move || {
            for _loop_num in 0..LOOP_NUMBER{
                let mut array = rx2.lock().unwrap().recv().unwrap_or(vec![0; ARRAY_SIZE]);
                for i in array.iter_mut(){
                    *i = (*i + 1) % 1_000_000;
                }
                match tx1.send(array).unwrap(){
                    _ => {}
                }
            }
        });

        let thread2 = std::thread::spawn(move || {
            for _loop_num in 0..LOOP_NUMBER{
                let mut array = rx1.lock().unwrap().recv().unwrap_or(vec![0; ARRAY_SIZE]);
                for i in array.iter_mut(){
                    *i = (*i + 1) % 1_000_000;
                }
                match tx2.send(array).unwrap(){
                    _ => {}
                }
            }
        });

        thread.join().unwrap();
        thread2.join().unwrap();
        println!("Time taken with mpsc {}", SystemTime::now().duration_since(start_time).unwrap().as_millis());
    }
}

fn _audio_stuff(){
    use audio::AudioSource;
    use rodio::source::Source;
    use std::io::BufReader;
    use std::fs::File;
    //let source = audio::AudioFile::new("assets/audio/magic.mp3");
    //let source2 = source.clone();

    let dev = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&dev);
    //let file = File::open("assets/audio/magic.mp3").unwrap();
    //let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let mut audio_manager = audio::AudioManager::new();
    audio_manager.add_source("assets/audio/magic.mp3");
    audio_manager.play_source(0);
    audio_manager.sleep_until_end();
    
    return;
}
