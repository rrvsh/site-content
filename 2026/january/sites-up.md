---
title: My website is finally live
slug: sites-up
date: 2026-01-25
tags: [daily-blog]
---
After roughly two weeks of work, I finally have the working skeleton of my website (the one you're reading right now!).

To briefly cover the architecture, I have a [content repository](https://github.com/rrvsh/site-content) of markdown files and [app repository](https://github.com/rrvsh/tools) (under `rs/site`).

The site content is loaded as a Nix flake input into the container image at build time, and I parse each file for its frontmatter, extracting title, date, slug, and content.

I then load this into a list of months (for the blog index) and a map of date and slug to the article (for each article).

This is enough of a starting point that I feel comfortable calling this, which was my goal for January, done. I did plan out a 3 week development cycle of around a sprint a day on this, and succeeded in sticking to it.

This fourth week of January will be spent on documentation and clean up. I plan to:

- write a full session recap of my first Pinbreak TTRPG session on Thursday, including expounding on any relevant bits in other articles
- write a full test suite for the `site` package
- document it thoroughly
- install NixOS on my living room PC, desktop, NAS, and optionally a "services" host if I feel the need
- clean up and merge my zmk PR
- publish a v2026.02 release for the `tools` repo (just as a fun changelog)

It's been a good month so far, despite the very upsetting incident that happened last week - you can tell I've been having trouble sticking to my routine as a result.

I plan to take this week easy, and focus on getting the wins where I can, namely in finishing the auth migration I'm working on at $WORKPLACE and getting my daily freewriting in. I'm not too fussed about clearing any of the other goals, but it would be really nice to have the NixOS systems set up with the flake so I can easily deploy to them my services and such. I am getting pretty annoyed at my desk setup which involves a KVM to switch between my laptop and desktop - it introduces too much flakiness that I don't really want to deal with day to day. Ideally, I only have to use my desktop at my desk and my laptop when I'm not.

That's all for today - see ya tomorrow!
