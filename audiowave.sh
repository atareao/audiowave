#!/bin/bash
TITLE=""
SUBTITLE=""
WIDTH="640"
HEIGHT="480"
MODE="line" # point, line, p2p, cline
WAVE_WIDTH="640"
WAVE_HEIGHT="480"
WAVE_COLOR="yellow"
WAVE_X=-1
WAVE_Y=-1
TEXT_COLOR="yellow"
TEXT_X=-1
TEXT_Y=-1
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
    -ww|--wave_width)
    WAVE_WIDTH="$2"
    shift # past argument
    shift # past value
    ;;
    -wh|--wave_height)
    WAVE_HEIGHT="$2"
    shift # past argument
    shift # past value
    ;;
    -wc|--wave_color)
    WAVE_COLOR="$2"
    shift # past argument
    shift # past value
    ;;
    -wx|--wave_x)
    WAVE_X="$2"
    shift # past argument
    shift # past value
    ;;
    -wy|--wave_y)
    WAVE_Y="$2"
    shift # past argument
    shift # past value
    ;;
    -tc|--text_color)
    TEXT_COLOR="$2"
    shift # past argument
    shift # past value
    ;;
    -tx|--text_x)
    TEXT_X="$2"
    shift # past argument
    shift # past value
    ;;
    -ty|--text_y)
    TEXT_Y="$2"
    shift # past argument
    shift # past value
    ;;
    -m|--mode)
    MODE="$2"
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

if [ $WAVE_X -eq -1 ]
then
    WAVE_X="$((($WIDTH-$WAVE_WIDTH)/2))"
fi
if [ $WAVE_Y -eq -1 ]
then
    WAVE_Y="$((($HEIGHT-$WAVE_HEIGHT)/2))"
fi
echo $TEXT_X $TEXT_Y
if [ $TEXT_X -eq -1 ]
then
    TEXT_X=20
fi
if [ $TEXT_Y -eq -1 ]
then
    TEXT_Y="$(($HEIGHT-100))"
fi
echo $TEXT_X $TEXT_Y
TEXT_Y2="$(($TEXT_Y + 50))"
if [ ${#AUDIO} -eq 0 ] || [ ! -f $AUDIO ] 
then
    echo $AUDIO
    echo "Usage ./audiowave.sh [-t title] [-s subtitle] [-b background_image] -a audio [-w width] [-h height] [-c color]"
    echo "-t set the title of the video. It's optional"
    echo "-s set the subtitle of the video. It's optional"
    echo "-b set the background_image. It's optional"
    echo "-a set the audio. It's mandatory"
    echo "-m set the mode. Default is line. Possible modes: point, line, p2p, cline"
    echo "-w set the width of the video. It's optional, default width is 640 px"
    echo "-h set the height of the video. It's optional, default height is 480 px"
    echo "-ww set the width of the wave. It's optional, default width is 640 px"
    echo "-wh set the height of the wave It's optional, default height is 480 px"
    echo "-wc set the color of wave. It's optional, default color is yellow"
    echo "-wx set the x position of wave. It's optional, default x position is in the middle"
    echo "-wy set the y position of wave. It's optional, default y position is in the middle"
    echo "-tc set the color of title and subtitle. It's optional, default color is yellow"
    echo "-tx set the x position of text. It's optional, default x position is 20 px"
    echo "-ty set the y position of text. It's optional, default y position is 100 px from bottom"
    exit 126
fi

if [ -f 'output.mp4' ]
then
    rm output.mp4
fi


if [ ${#BACKGROUND} -ne 0 ] && [ -f $BACKGROUND ]
then
    ffmpeg -y -i ${AUDIO} -loop 1 -i $BACKGROUND -filter_complex "\
        [0:a]showwaves=size=${WAVE_WIDTH}x${WAVE_HEIGHT}:mode=${MODE}:colors=${WAVE_COLOR}:rate=30,format=rgba[top];\
        [1:v]setpts=PTS-STARTPTS, scale=${WIDTH}x${HEIGHT}, format=rgba[bottom];\
        [bottom][top]overlay=format=auto:y=${WAVE_Y}:x=${WAVE_X}[temporal];\
        [temporal]drawtext=text='${TITLE}':fontcolor=${TEXT_COLOR}:fontsize=30:font=Ubuntu:x=${TEXT_X}:y=${TEXT_Y}[l1];\
        [l1]drawtext=text='${SUBTITLE}':fontcolor=${TEXT_COLOR}:fontsize=30:font=Ubuntu:x=${TEXT_X}:y=${TEXT_Y2}[output]\
        " -map "[output]" -pix_fmt yuv420p -map 0:a -c:v libx264 -c:a libmp3lame -shortest output.mp4
else
    ffmpeg -y -i ${AUDIO} -filter_complex "\
        [0:a]showwaves=size=${WAVE_WIDTH}x${WAVE_HEIGHT}:mode=${MODE}:colors=${WAVE_COLOR}:rate=30,format=rgba[top];\
        [top]drawtext=text='${TITLE}':fontcolor=${TEXT_COLOR}:fontsize=30:font=Ubuntu:x=${TEXT_X}:y=${TEXT_Y}[l1];\
        [l1]drawtext=text='${SUBTITLE}':fontcolor=${TEXT_COLOR}:fontsize=30:font=Ubuntu:x=${TEXT_X}:y=${TEXT_Y2}[output]\
        " -map "[output]" -pix_fmt yuv420p -map 0:a -c:v libx264 -c:a libmp3lame -shortest output.mp4
fi

