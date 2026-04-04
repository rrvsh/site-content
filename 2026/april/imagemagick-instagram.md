---
title: My ImageMagick Attempt to Post an Instagram Story
slug: imagemagick-instagram
date: 2026-04-04
tags: [daily-blog, imagemagick]
---
So I wanted to post an instagram story today of the recent Jeopardy episode with Brennan Lee Mulligan, Rebecca Black, and Monet X Change with a joke that it was a huge day for gay people. This is fully ripping off MegaMetroid2's comment on the selfsame video, by the way so credits to them. Anyway I am on NixOS and I don't have the Android app, and I encountered some issues in the process and thought it would be fun (?) to do a writeup.

Firstly, the Instagram web app on desktop does NOT have a single way to upload a new story. I found the only way to access <https://www.instagram.com/create/story/> is via `Ctrl/Cmd+Shift+M` (in Firefox) to switch into a mobile view or what they call "Responsive Design Mode".

This gave me access to upload the story, but then when I uploaded my image which is a screenshot of the [Youtube video](https://www.youtube.com/watch?v=_sQz0TyU7cE) thumbnail, it cropped it instead of letterboxing like on mobile, and gave me no option to edit it. I was too lazy to transfer the image to my phone so I decided to reach for ImageMagick instead to letterbox the image:

```
magick queer-people-parade.png -resize 1080x
magick: MissingArgument `-resize' at CLI arg 2 @ fatal/magick-cli.c/ProcessCommandOptions/676.
magick queer-people-parade.png -resize 1080x -background black
magick: MissingArgument `-background' at CLI arg 4 @ fatal/magick-cli.c/ProcessCommandOptions/676.
magick queer-people-parade.png -resize 1080x -background black -gravity center -extent 1080x1920
magick: MissingArgument `-extent' at CLI arg 8 @ fatal/magick-cli.c/ProcessCommandOptions/676.
magick queer-people-parade.png -resize 1080x -background black -gravity center -extent 1080x1920 output.png
```

As you can see it did take a few tries, but its interesting to me that the flags are interpreted procedurally, as you will see further shown in the next example, when I tried adding some text to the image.

```
magick queer-people-parade.png -resize 1080x -background black -gravity center -extent 1080x1920 -gravity south -fill white -pointsize 48 -annotate +0+100 "Your text" output.png
magick: unable to read font `(null)' @ error/annotate.c/RenderFreetype/1747.
magick queer-people-parade.png -resize 1080x -background black -gravity center -extent 1080x1920 -gravity south -fill white -pointsize 48 -annotate +0+100 "Your text" -font DejaVu-Sansoutput.png
magick: unable to read font `(null)' @ error/annotate.c/RenderFreetype/1747.
magick queer-people-parade.png -resize 1080x -background black -gravity center -extent 1080x1920 -gravity south -fill white -pointsize 48 -annotate +0+100 "Your text" -font DejaVu-Sans output.png
magick: unable to read font `(null)' @ error/annotate.c/RenderFreetype/1747.
magick queer-people-parade.png \
  -resize 1080x \
  -background black -gravity center -extent 1080x1920 \
  -font Unifont -fill white -pointsize 80 \
  -gravity south \
  -annotate +0+100 "HUGE DAY FOR GAY PEOPLE" \
  output.png
magick queer-people-parade.png \
  -resize 1080x \
  -background black -gravity center -extent 1080x1920 \
  -font Unifont -fill white -pointsize 80 \
  -gravity north \
  -annotate +0+100 "HUGE DAY FOR GAY PEOPLE" \
  output.png
magick queer-people-parade.png \
  -resize 1080x \
  -background black -gravity center -extent 1080x1920 \
  -font Unifont -fill white -pointsize 80 \
  -gravity north \
  -annotate +0+400 "HUGE DAY FOR GAY PEOPLE" \
  output.png
```

As you can see I made a mistake with the ordering of the font and the `-annotate` flag, which I attributed to just NixOS things but turns out was just my inexperience with ImageMagick. I uploaded this finished image to Instagram, but then it AGAIN got slightly cropped around the edges - I suspected some issue with mismatched aspect ratios, which I could fix by amending the image or the device chosen to emulate the aspect ratio of in Firefox, but I was lazy so I just added some padding:

```
magick output.png -bordercolor black -border 10 outpoot.png
```

and finally the story looked good! Unfortunately though I clicked "Share story" before even opening my editor to write this up, and it is still uploading until now. Guess Instagram really doesn't think my joke is that funny... Anyway see ya tomorrow (sorry for no Elric update but I'm on THE VANISHING TOWER now)
