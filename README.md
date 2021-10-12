# osru
***Rust rewrite of osu!***

Hate reading, I'd rather see videos:
https://www.youtube.com/playlist?list=PLhAHgK79Pu9tC-O8I7hwv8Fq_9q-xJaf1

**Why even bother?**
There's not much point to it, but I'm hoping to be able to figure out how to write responsive and performant applications using Rust, and writing an osu port seemed like a decent idea, at least at the start.
Osu is a game where timing matters a lot, any frame skips or inconsistency in timing becomes very noticeable really quickly. At Overall Difficulty 10, your precision has to be within 19.5 ms of the theoretical perfect timing.
I say "at the start" because I figured out along the way that it's actually way harder to write something like this from scratch than I anticipated. I don't plan on giving up though.

**Disclaimer**
I'm not in any way affiliated with osu! in any way, except for the fact that I play the game and like it.

### Current target
* Getting standard working
   * Sliders
   * Correct Circle Size
   * Proper circle position
* Scoring system
* Barebones GUI

### Low priority:
* Mods
* Online stuff
* Skins
* Beatmap Editor
* Pretty animations

### No plans to implement:
* Taiko
* Catch
* Mania

### Done
* Standard Mode:
   * Proper hit timing window
   * Hit Circles
   * Approach Circles
* Audio Loudness Normalization

## How do I ask you stuff?
Feel free to open an issue right here on github.
If there's enough interest about something, I make it happen.

## How to install
Currently there are no binary files, but if you are interested, feel free to compile it from source.
1) Install **Rust** (https://www.rust-lang.org/tools/install)
2) Install **SDL2** (https://github.com/Rust-SDL2/rust-sdl2, https://www.libsdl.org/download-2.0.php)
3) Compile and run using "**cargo run --release**"

If you run into problems on Linux or Mac OS you are on your own essentially, my pc is running windows.
