#!/bin/bash
TITLE=""
SUBTITLE=""
WIDTH="640"
HEIGHT="480"
COLOR="yellow"
AUDIO=""

POSITIONAL=()
while [[ $# -gt 0 ]]
do
key="$1"

case $key in
    -t|--titulo)
    TITLE="$2"
    shift # past argument
    shift # past value
    ;;
    -s|--subtitulo)
    SUBTITLE="$2"
    shift # past argument
    shift # past value
    ;;
    -b|--background)
    BACKGROUND="$2"
    shift # past argument
    shift # past value
    ;;
    -a|--audio)
    AUDIO="$2"
    shift # past argument
    shift # past value
    ;;
    -w|--width)
    WIDTH="$2"
    shift # past argument
    shift # past value
    ;;
    -h|--height)
    HEIGHT="$2"
    shift # past argument
    shift # past value
    ;;
    -c|--color)
    COLOR="$2"
    shift # past argument
    shift # past value
    ;;
    --default)
    DEFAULT=YES
    shift # past argument
    ;;
    *)    # unknown option
    POSITIONAL+=("$1") # save it in an array for later
    shift # past argument
    ;;
esac
done

set -- "${POSITIONAL[@]}" # restore positional parameters

if [ ${#AUDIO} -eq 0 ] || [ ! -f $AUDIO ] 
then
    echo "Usage ./audiowave.sh [-t title] [-s subtitle] [-b background_image] -a audio [-w width] [-h height] [-c color]"
    echo "-t set the title of the video. It's optional"
    echo "-s set the subtitle of the video. It's optional"
    echo "-b set the background_image. It's optional"
    echo "-a set the audio. It's mandatory"
    echo "-w set the width of the video. It's optional, default width is 640 px"
    echo "-h set the height of the video. It's optional, default height is 480 px"
    echo "-c set the color of title and subtitle. It's optional, default color is yellow"
    exit 126
fi

if [ -f 'output.mp4' ]
then
    rm output.mp4
fi

if [ ${#BACKGROUND} -ne 0 ] && [ -f $BACKGROUND ]
then
    title_position=${HEIGHT}-80
    subtitle_position=${HEIGHT}-30
    convert $BACKGROUND -resize $WIDTHx$HEIGHT temporal.jpg
    convert temporal.jpg -font Ubuntu -weight 700 -pointsize 48 -draw "fill ${COLOR} text 10, ${title_position} '${TITLE}'" temporal.jpg
    convert temporal.jpg -font Ubuntu -weight 700 -pointsize 48 -draw "fill ${COLOR} text 10, ${subtitle_position} '${SUBTITLE}'" temporal.jpg
    ffmpeg -loop 1 -i temporal.jpg -i ${AUDIO} -c:v libx264 -tune stillimage -c:a libmp3lame -q:a 4 -pix_fmt yuv420p -shortest temporal_video.mp4
    ffmpeg -i ${AUDIO} -filter_complex "[0:a]showwaves=s=${WIDTH}x${HEIGHT}:mode=cline,format=yuv420p[v]" -map "[v]" -map 0:a -c:v libx264 -an temporal_onda.mp4
    ffmpeg -i temporal_video.mp4 -i temporal_onda.mp4 -filter_complex " \
        [0:v]setpts=PTS-STARTPTS, scale=${WIDTH}x${HEIGHT}[top]; \
        [1:v]setpts=PTS-STARTPTS, scale=${WIDTH}x${HEIGHT}, \
        format=yuva420p,colorchannelmixer=aa=0.5[bottom]; \
        [top][bottom]overlay=shortest=1" -c:a libmp3lame -c:v libx264 output.mp4
    rm temporal.jpg
    rm temporal_video.mp4
    rm temporal_onda.mp4
else
    ffmpeg -i $PWD/${AUDIO} -filter_complex "[0:a]showwaves=s=${WIDTH}x${HEIGHT}:mode=cline,format=yuv420p[v]" -map "[v]" -map 0:a -c:v libx264 -c:a libmp3lame -q:a 4 output.mp4
fi
