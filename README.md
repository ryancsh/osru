**Last updated: 2021-10-18 10:24:26**

**Currently on indefinite hiatus since I'm not satisfied with the way things are atm and I'm not sure how to go about solving them:**

1. No way to tell when a screen refresh happens so the image on the screen is always pointlessly delayed unless you go for uncapped frame rates, which is a cop out and not an actual solution. If there was some way to know exactly when the last screen refresh happened, the game could predict the timing of the next one and simulate past the current game state to show the user a perfectly timed image (minus state that would be affected based on user input). For more complex games that run at lower frame rates, this might be jarring, but for simpler games like this one, it would work since the difference between what the real image should be and the shown image wouldn't be that big. In fact, the difference would be less than whatever you get when you arbitrarily lock the fps imo.
2. Polling for input is a pain because you waste tons of CPU cycles doing this in user space when the OS should be able to implement it much more efficiently. If the OS could provide timestamped input updates, that would make things run a lot faster with a lot less power.

Essentially, waiting on hardware support for something like this https://github.com/KhronosGroup/Vulkan-Docs/pull/1364

This article describes the issue in more detail: https://medium.com/@alen.ladavac/the-elusive-frame-timing-168f899aec92

## osru
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
