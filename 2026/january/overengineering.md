---
title: How to over-engineer a static site, or My Weblog's Tech Stack
slug: over-engineering
date: 2026-01-29
tags: [daily-blog]
---

I've never really had a proper website. I've had many sites, sure, on slayment.com (media servers), bwfiq.com (my old handle), even doldrive.com (purely to put a smile on my dad's face). All of these were basically CNAMES to services. I was a system administrator, but I never built a website (other than school projects).

When I finally did and threw up bwfiq.com and later rrv.sh, they were a single `index.html` that I served with Nginx. I still think this is PERFECTLY sufficient, but as I explained in [an earlier post](https://rrv.sh/2026/01/15/self-flagellation), I wanted to do things "as right as possible". This site runs on an Axum backend with templates constructed with Askama. Do I think this is necessary, or even good? ABSOLUTELY NOT!

## Why I did it anyway

I like complexity. More accurately, I like [people thinking I am smart](https://notashelf.dev/posts/the-tradeoff-trap).

There's the real reason ^. Now, I'll tell you what I told myself to convince myself it was necessary.

First, why not a static site generator? By design, SSGs are not going to let you do **anything** you want. I wanted ultimate control in case I wanted to do weird shit, and I also wanted to figure things out myself instead of it automagically rendering. Essentially, I didn't want to have to eventually migrate off of one.

So why not just a **static site**? I've done this before, and its honestly preferable and more suitable to either author my site directly in HTML or to render the markdown into HTML and manually interpolate it into the site files. Other than to make people think I am very clever, I wanted a backend so I could dynamically render content like comments and such.

Why not a separate frontend, then? I do not like separating my backend and my frontend, and I like Jinja so I went with Askama. I chose Axum because a bunch of people on Reddit said they preferred it to Actix and Rocket. I chose Rust because I like it and I want to learn more of it.

Now, why put it in a container and host it on AWS? Well, I already explained it's for my portfolio and to prove I can write an end to end deployment pipeline with everything declarative, but also... I like to tinker and this tinkering is usually done on my home servers and networks - if I self host my services and sites on this same homelab, my site would be down all the time. This has happened before many times, and I want my links to stay up. So: AWS. It costs me less than $20 a month and I have peace of mind - it's a worthy tradeoff to me. Also, now I've written the IaC, I likely never have to again.

## What's next?

I currently load the files into memory as startup to index and query them by month and date+slug. A SQL database would be preferable here, and it would allow me to add stuff like comments and live editing, should I need it.

I would also love an RSS feed, which I actually have implemented in a PR, but I broke a rule there - I used an LLM to work on something I couldn't have done myself. Here it was because I hadn't used the `rss` crate before, but I've [done it before](https://rrv.sh/2026/01/21/life-overview#aenyrathia) and didn't like the result.

Lastly, the actual goal for next month is to work through the [MDN Curriculum](https://developer.mozilla.org/en-US/curriculum/) and apply it to the site to make something I'm actually proud of. I don't think it will take me a whole month to style the site though - a stretch goal here is to make the front end also include a little interactive game in JS.

That's all! See ya tomorrow
