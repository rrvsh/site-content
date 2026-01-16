---
slug: firefox-conform-fullscreen
tags: [til, firefox]
---
# Make Firefox's fullscreen conform to the window size instead of screen size

Go to `about:config` and search for `full-screen-api.ignore-widgets`. Set the value to `true`.

This makes any fullscreen i.e. double clicking on a Youtube video only take up the size and location where the video already was instead of taking up the entire screen.
