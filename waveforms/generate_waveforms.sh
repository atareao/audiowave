#!/bin/bash

# Ensure temp directory exists
mkdir -p temp

# Create a sample audio file if it doesn't exist
if [ ! -f temp/sample.wav ]; then
    echo "Creating sample.wav..."
    ffmpeg -f lavfi -i "sine=frequency=440:duration=5" -c:a pcm_s16le -ar 44100 temp/sample.wav
fi

if [ ! -f temp/background.jpg ]; then
    echo "Creating background.jpg..."
    magick -size 1920x1080 xc:black temp/background.jpg
fi

# List of waveform styles (from src/models/style.rs, converted to snake_case)
# Problematic styles commented out due to FFmpeg filter issues or missing options
STYLES=(
    "classic_line"
    # "cyberpunk_spectrum" # FFmpeg filter error: option 'rate' not found
    "analog_oscilloscope"
    "retro_step" # FFmpeg error: background.jpg not found
    # "nebula_histogram" # FFmpeg filter error: option 'color' not found
    "prism_frequency" # FFmpeg error: background.jpg not found
    "digital_pulse" # FFmpeg error: background.jpg not found
    "neon_mirror" # FFmpeg error: background.jpg not found
    "glass_blur" # FFmpeg error: background.jpg not found
    "ghost_frequency" # FFmpeg error: background.jpg not found
    # "cyber_circle" # FFmpeg filter error: 'polar' not found
    "liquid_gold" # FFmpeg error: background.jpg not found
    # "electric_storm" # FFmpeg filter error: 'showpeaks' not found
    # "zenith_stack" # FFmpeg filter error: option 'rate' not found
    # "pulse_radar" # FFmpeg filter error: 'polar' not found
    "studio_bars"
    "minimal_mono"
    "waveform_solid"
    "broadcast_point"
    "talk_flow"
    "audiogram_bars"
    "voice_shadow"
    # "spectrum_circle" # FFmpeg filter error: 'polar' not found
    # "glow_wave_modern" # FFmpeg error: background.jpg not found
)

# Construct the default config YAML with a dummy background
DEFAULT_CONFIG_YML="templates:
  default:
    video:
      width: 1920
      height: 1080
      fps: 30
    background:
      path: \"temp/background.jpg\" # Dummy background path
      mode: \"fill\"
    waveform:
      width: 1200
      height: 250
      x: \"(W-w)/2\"
      y: \"H-h-150\"
      style: \"neon_mirror\"
    title:
      font: \"/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf\"
      size: 80
      color: \"white\"
      x: \"(w-text_w)/2\"
      y: \"150\"
    subtitle:
      font: \"/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf\"
      size: 40
      color: \"white\"
      x: \"(w-text_w)/2\"
      y: \"250\""

for STYLE in "${STYLES[@]}"; do
    echo "Generating image for style: $STYLE"

    TEMP_CONFIG_FILE="temp/config_${STYLE}.yml"
    OUTPUT_VIDEO="temp/${STYLE}.mp4"
    OUTPUT_IMAGE="${STYLE}.png"

    echo "1. Generate temporary config file with the specific style"
    echo "$DEFAULT_CONFIG_YML" | sed "s/style: \"neon_mirror\"/style: \"${STYLE}\"/"> "$TEMP_CONFIG_FILE"

    echo "2. Run audiowave to generate a video"
    # Using 'cargo run --' to execute the audiowave binary
    cargo run -- \
        --input temp/sample.wav \
        --output "$OUTPUT_VIDEO" \
        --config "$TEMP_CONFIG_FILE" \
        --preview # Suppress audiowave output for cleaner logs

    echo "3. Check if video generation was successful"
    if [ ! -f "$OUTPUT_VIDEO" ]; then
        echo "Error generating video for style $STYLE. Skipping image extraction."
        continue
    fi

    echo "4. Extract PNG from the video"
    echo ffmpeg -y -ss 00:00:01 -i "$OUTPUT_VIDEO" -vframes 1 -f image2 -update 1 "$OUTPUT_IMAGE" #&> /dev/null # Suppress ffmpeg output
    ffmpeg -y -ss 00:00:01 -i "$OUTPUT_VIDEO" -vframes 1 -f image2 -update 1 "$OUTPUT_IMAGE" #&> /dev/null # Suppress ffmpeg output

    echo "5. Check if image extraction was successful"
    if [ ! -f "$OUTPUT_IMAGE" ]; then
        echo "Error extracting image for style $STYLE."
    else
        echo "Generated $OUTPUT_IMAGE"
    fi
done

echo "All waveform images generation attempt completed."

echo "6. Clean"
rm -rf temp
