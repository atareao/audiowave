# audiowave

This project provides two ways to generate videos with an image and a waveform from an audio file:
1.  **`audiowave.sh`**: A shell script that uses `ffmpeg` directly.
2.  **Rust Binary**: A Rust application that can be run via `cargo`.

---

## `audiowave.sh` Usage

Creates a video with an image and a wave using `ffmpeg`.

```
Usage ./audiowave.sh [-t title] [-s subtitle] [-b background_image]
                     [-m mode] [-w width] [-h height] -a audio
                     [-ww wave_width] [-wh wave_height] [-wc wave_color]
                     [-wx wave_x_position] [-wy wave_y_position]
                     [-tx text_x_position] [-ty text_y_position]
                     [-tc text_color] [-dt distance_title_subtitle]
```

*   `-t` set the title of the video. It's optional
*   `-s` set the subtitle of the video. It's optional
*   `-b` set the background_image. It's optional
*   `-a` set the audio. It's mandatory
*   `-m` set the mode. Default is line. Possible modes: point, line, p2p, cline
*   `-w` set the width of the video. It's optional, default width is 640 px
*   `-h` set the height of the video. It's optional, default height is 480 px

*   `-ww` set the width of the wave. It's optional, default width is 640 px
*   `-wh` set the height of the wave It's optional, default height is 480 px
*   `-wc` set the color of wave. It's optional, default color is yellow
*   `-wx` set the x position of wave. It's optional, default x position is in the middle
*   `-wy` set the y position of wave. It's optional, default y position is in the middle

*   `-tc` set the color of title and subtitle. It's optional, default color is yellow
*   `-tx` set the x position of text. It's optional, default x position is 20 px
*   `-ty` set the y position of text. It's optional, default y position is 100 px from bottom
*   `-dt` set the distance between tittle and subtitle, default distance is 50px

---

## Rust Binary Usage

The Rust application provides more advanced features, including template support and metadata extraction.

To run the Rust binary directly, use `cargo run -- [OPTIONS]`.

```
Usage: cargo run -- [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>          Input MP3 file
  -t, --template <TEMPLATE>    Name of the template defined in the YAML [default: default]
  -m, --title <TITLE>          Video title (overwrites metadata or YAML)
  -s, --subtitle <SUBTITLE>    Video subtitle
  -o, --output <OUTPUT>        Output MP4 file [default: output.mp4]
  -c, --config <CONFIG>        YAML configuration file
      --preview                Preview mode: generates only the first 5 seconds
      --debug                  Activate DEBUG mode
  -h, --help                   Print help
  -V, --version                Print version
```

**Example:**

```bash
cargo run -- -i /path/to/your/audio.mp3 -o my_waveform_video.mp4 --title "My Awesome Podcast" --subtitle "Episode 1"
```

---

## Waveform Styles

### Classic Line
![Classic Line](assets/waveforms/classic_line.png)

### Studio Bars
![Studio Bars](assets/waveforms/studio_bars.png)

### Minimal Mono
![Minimal Mono](assets/waveforms/minimal_mono.png)

### Waveform Solid
![Waveform Solid](assets/waveforms/waveform_solid.png)

### Broadcast Point
![Broadcast Point](assets/waveforms/broadcast_point.png)

### Talk Flow
![Talk Flow](assets/waveforms/talk_flow.png)

### Audiogram Bars
![Audiogram Bars](assets/waveforms/audiogram_bars.png)

### Voice Shadow
![Voice Shadow](assets/waveforms/voice_shadow.png)
