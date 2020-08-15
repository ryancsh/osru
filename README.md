# osru
***Rust rewrite of osu!***

* The issue with osu! stable: Everything happens in one thread so if you lock your fps at 60 is terrible for your accuracy.
* The issue with osu! lazer: Too many animations, also written in C# which is GC'd. Garbage Collection in my opinion in games is a terrible idea, the reason being inconsistent performance. You can drop a frame out of nowhere if the GC decides to do its thing at a bad time.

My attempt is to write a port that avoids the main issues with both osu!stable and osu!lazer.
I chose Rust because is not GC'd and actually makes it easy to write threaded applications. It's also a very interesting language in its own right.

The current idea is to have 3 threads:
1) The actual game loop where all the calculations about hit timings happen
2) The render thread, handles all the stuff that should go on the screen. This thread also currently handles event polling because rust-SDL2 doesn't support polling for events on another thread.
3) The audio thread, plays all the sounds.

### Current target
* Getting osu standard working
* Scoring system

### Low priority:
* Mods
* Online stuff
* Skins

### No plans to implement:
* osu taiko
* osu catch
* osu mania

## How to install
Currently there are no binary files, but if you are interested, feel free to compile it from source.
1) Install **Rust** (https://www.rust-lang.org/tools/install)
2) Install **SDL2** (https://github.com/Rust-SDL2/rust-sdl2, https://www.libsdl.org/download-2.0.php)
3) Compile and run using "**cargo run --release**"

If you run into problems on Linux or Mac OS you are on your own essentially, my pc is running windows.
