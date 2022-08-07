# bliss-analyze

Simple rust app to analyze songs with [bliss-rs](https://github.com/Polochon-street/bliss-rs).


# Building

clang, pkg-config, and ffmpeg are required to build, as well as
[Rust](https://www.rust-lang.org/tools/install)

To install dependencies on a Debian system:

```
apt install -y clang libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev pkg-config
```

Build with `cargo build --release`


## Analyze a song

```
$ bliss-analyze /path/to/song.mp3
```

