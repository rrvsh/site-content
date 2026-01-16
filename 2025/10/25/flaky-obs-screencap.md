---
slug: flaky-obs-screencap-macos
tags: [til, macos]
---
# How to Fix Flaky macOS Screen Capture on OBS

On macOS, when switching scenes in OBS, if you move from a scene with a Display Capture source to one without it, macOS stops the screen capture session. When you switch back, the display stays frozen until you re-enable it in the source properties.

The workaround is to include the Display Capture source in every scene, even the ones that don’t use it. Just hide it by clicking the eye icon or placing it below another layer. This keeps the capture session active and prevents interruptions when switching scenes.

You may still encounter issues where the macOS Screen Capture source still keeps freezing and requires manual restarting in the properties of the source. The best (for now) fix for this is to use [this script](https://github.com/yayuanli/OBS_Restart_Capture_Stuck_Source) which will monitor the source and restart it if it's frozen.

I have a Nix Flake setup [here](https://github.com/rrvsh/OBS_Restart_Capture_Stuck_Source) if you prefer that (or have to like me).
