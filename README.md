# Blissify-analyse

Simple rust app to analyse songs with bliss-rs.


# Building

clang, pkg-config, and ffmpeg are required to build, as well as
[Rust](https://www.rust-lang.org/tools/install)

To install dependencies on a Debian system:

```
apt install -y clang libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev pkg-config
```

Build with `cargo build --release`


## Analyse a song

```
$ bliss-analyse /path/to/song.mp3
```

