# audiowave

Creates a video with an image and a wave

```
Usage ./audiowave.sh [-t title] [-s subtitle] [-b background_image]
                     [-m mode] [-w width] [-h height] -a audio
                     [-ww wave_width] [-wh wave_height] [-wc wave_color]
                     [-wx wave_x_position] [-wy wave_y_position]
                     [-tx text_x_position] [-ty text_y_position]
                     [-tc text_color] [-dt distance_title_subtitle]
```

* -t set the title of the video. It's optional
* -s set the subtitle of the video. It's optional
* -b set the background_image. It's optional
* -a set the audio. It's mandatory
* -m set the mode. Default is line. Possible modes: point, line, p2p, cline
* -w set the width of the video. It's optional, default width is 640 px
* -h set the height of the video. It's optional, default height is 480 px

* -ww set the width of the wave. It's optional, default width is 640 px
* -wh set the height of the wave It's optional, default height is 480 px
* -wc set the color of wave. It's optional, default color is yellow
* -wx set the x position of wave. It's optional, default x position is in the middle
* -wy set the y position of wave. It's optional, default y position is in the middle

* -tc set the color of title and subtitle. It's optional, default color is yellow
* -tx set the x position of text. It's optional, default x position is 20 px
* -ty set the y position of text. It's optional, default y position is 100 px from bottom
* -dt set the distance between tittle and subtitle, default distance is 50px