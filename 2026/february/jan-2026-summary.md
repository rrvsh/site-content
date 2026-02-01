---
title: Month in Review - January 2026
slug: jan-2026-review
date: 2026-02-01
tags: [daily-blog]
---

It's officially the last day of the month (I know it's february but I count by four week blocks), so here's what I did this month:

For $WORKPLACE, I:
- set up Github Actions workflows to post `tofu plan` output on PRs and safely trigger reviewed `tofu apply` runs per environment.
- added an inline edit mode to a list of client documents (HTMX + Alpinejs!).
- wrote [an article on code review](https://rrv.sh/2026/01/02/code-review) for the engineering retrospective.
- wrote an end to end inhouse auth implementation to replace Supabase Auth including:
    - migrating existing user credentials without any user-facing changes
    - register/login with email
    - login with Google OAuth
    - email verification and password reset with signed JWT tokens with expiry times

I also set up my website, which is an Axum and Askama Rust app packaged with Nix as a container image and deployed to AWS ECS Fargate with OpenTofu, all automated with Github Actions including pulling in a separate content repo of markdown files, baking it into the image, and parsing + indexing them on startup. This was done in my [`tools`](https://github.com/rrvsh/tools) monorepo, where I also (re)installed NixOS on my desktop and integrated it into my flake, setting up my `home-manager` config and declaring my user with `sops-nix`

As for media, I managed to watch seasons one through five of Game of Thrones (though I am not continuing), the new Fallout season, A Knight of The Seven Kingdoms, and completed Bendy 1 and 2 as well as Routine, also managing to watch my roommates play through Outer Wilds and the Fallout New Vegas DLC Dead Money. I also played a bit of Prey, but only up to watching the protagonist's full message.

I also wrote and ran the first session of my new TTRPG campaign Pinbreak, wrote 23 (including this) blog posts, spring cleaned the house (happy Lunar New Year!) and went to the gym four times running a bastardised ME method.

Writing this has officially tired me out. This was a great month, despite the bad stuff that happened, and I look forward to the next - I plan to work through the [MDN curriculum](https://developer.mozilla.org/en-US/curriculum/) next month to get better at front-end and design this website the way I want it to look. See you tomorrow!
