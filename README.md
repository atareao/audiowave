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
      --wave-color <WAVE_COLOR> Waveform color (e.g., 'red', '#FF0000', '0xFF0000')
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

### Analog oscilloscope

![waveforms/analog_oscilloscope.png](waveforms/analog_oscilloscope.png)

### Audiogram bars

![waveforms/audiogram_bars.png](waveforms/audiogram_bars.png)

### Broadcast point

![waveforms/broadcast_point.png](waveforms/broadcast_point.png)

### Circular nebula

![waveforms/circular_nebula.png](waveforms/circular_nebula.png)

### Circular wave

![waveforms/circular_wave.png](waveforms/circular_wave.png)

### Classic line

![waveforms/classic_line.png](waveforms/classic_line.png)

### Cyber ghost

![waveforms/cyber_ghost.png](waveforms/cyber_ghost.png)

### Cyber reflex

![waveforms/cyber_reflex.png](waveforms/cyber_reflex.png)

### Digital pulse

![waveforms/digital_pulse.png](waveforms/digital_pulse.png)

### Equalizer10 bands

![waveforms/equalizer10_bands.png](waveforms/equalizer10_bands.png)

### Equalizer128 bands

![waveforms/equalizer128_bands.png](waveforms/equalizer128_bands.png)

### Equalizer32 bands

![waveforms/equalizer32_bands.png](waveforms/equalizer32_bands.png)

### Ghost frequency

![waveforms/ghost_frequency.png](waveforms/ghost_frequency.png)

### Glow wave modern

![waveforms/glow_wave_modern.png](waveforms/glow_wave_modern.png)

### Liquid gold

![waveforms/liquid_gold.png](waveforms/liquid_gold.png)

### Minimal mono

![waveforms/minimal_mono.png](waveforms/minimal_mono.png)

### Neon mirror

![waveforms/neon_mirror.png](waveforms/neon_mirror.png)

### Prism frequency

![waveforms/prism_frequency.png](waveforms/prism_frequency.png)

### Professional neon

![waveforms/professional_neon.png](waveforms/professional_neon.png)

### Retro step

![waveforms/retro_step.png](waveforms/retro_step.png)

### Smooth line

![waveforms/smooth_line.png](waveforms/smooth_line.png)

### Studio bars

![waveforms/studio_bars.png](waveforms/studio_bars.png)

### Talk flow

![waveforms/talk_flow.png](waveforms/talk_flow.png)

### Toxic pulse

![waveforms/toxic_pulse.png](waveforms/toxic_pulse.png)

### Vapor wave mirror

![waveforms/vapor_wave_mirror.png](waveforms/vapor_wave_mirror.png)

### Voice shadow

![waveforms/voice_shadow.png](waveforms/voice_shadow.png)

### Waveform solid

![waveforms/waveform_solid.png](waveforms/waveform_solid.png)


---

## License
This project is licensed under the [MIT License](LICENSE).

