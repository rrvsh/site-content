---
title: Ruminations on my Past Work
slug: life-overview
date: 2026-01-21
tags: [daily-blog, projects]
---
In this article, I will attempt to enumerate the various projects I have done in my life and their statuses as of today.

This is as much a reflection on my past work as it is a showcase - I am doing this from memory, and will likely not return to update this in the future.

I'll be splitting it up by time period, as I find that that is how my brain likes to organise things anyway.

## Pre-diploma

Before I learned to program, I experimented with batch scripts and did at most an hour of Minecraft modding and Skyrim modding tutorials. I credit most of my development here to simply being given (mostly) unfettered access to a computer from an early age - my father, who I credit for most of my intellectual curiosity regarding computing, did try valiantly to block access to what he considered inappropriate for his young son, but I figured out ways around most of them, primarily figuring out how to pirate games for my PC and my PSP and various Nintendo handhelds.

Around 2014, I decided I wanted to go into software engineering or IT in general as a career, and so began with simple Hello World projects in C++. I quickly jumped to the Codecademy Python course, which I remember fondly as being very Monty Python in theme. I think I tried my hand at some simple CLI text adventures, and vaguely remember trying out game engines like Game Maker, RPGMaker, and some modding tools for the Generation III Pokemon games.

In 2015, I heard about the Singapore Games Creation Competition by NYP, a short program to let secondary school students try making games. Along with a small team from my school's computer club (which I was not a part of, having already committed to the National Cadet Corps for four years), we made a simple maze game in Game Maker about the dangers of addiction where you had to avoid syringes and junkies. I think this was inspired by the game Wade Watts, the protagonist of Ready Player One, said he made as his rite of passage to becoming a gunter. Our team managed to get into the final 25 out of (I think) 200 teams, but that was it for that.

Through this competition, I learned about the Game Development & Technology diploma offered by NYP, and decided to go for it. In 2016, I applied through the Early Admissions Program, and prepared a portfolio by going through the web development course offered by freecodecamp.org. I remember the site I made was a very simple static site with a showcase of Kanye West and his music, since I had just gotten into hip-hop at the time (and he wasn't actively evil yet).

## Diploma and National Service

I spent three years from 2017 to 2020 studying game design and development with a focus on low level graphics programming with C++ and OpenGL for the majority of the course, switching to Unity towards the end. Unfortunately, I was mostly complacent and barely did anything IT, programming, or game development related other than coursework - I spent most of my time playing video games and partying. I don't regret this, obviously, but I do wish I had had more of a drive. I was also mostly carried by my natural talent here, and did not spend much time on self-improvement, even though in 2016 I was very focused on it. I graduated with a middling GPA in 2020 and spent a few months (during COVID) doing more of the same before I entered my National Service.

During my stint in the army, I did not really do any coding, but I did discover Linux through self hosting Minecraft servers, and later in 2022, Docker and Docker Compose for a NAS that ran a Jellyfin server for my friends. This is where my drive for software engineering came back - I was focused mainly on getting into university as I saw the winds changing towards the job market requiring a degree, but I spent a lot of my free time tinkering with my servers. I got into a local university to the course that most of the other diploma graduates from my batch ended up going to, but I decided to go with a remote learning degree from the University of London so I could work during my university education and so I would graduate a year early. This bet did end up paying off, but I still do think about how it could have gone differently.

## Post-NS

My ORD was on the 2nd of October 2022, and I started both my degree and my first software engineering job on the following Monday. The degree was mostly useless - I could have applied myself more, but I do not honestly think it would have helped that much. The job was making VR experiences with C# and Unity - again, I might have been able to apply myself more, but the company was poorly managed and I barely got any work done. I finally left after a year because they decided to downgrade me to part time right before my first yearly pay increment was due.

Due to my love for self hosting having developed over the past two years, I decided I wanted to go into IT rather than software engineering, after thinking that the landscape of my career would look much like the boring day in day out of the last. I worked at a small esports arena and internet cafe called Reality Rift, and loved it there. I would have gone full time, working under the main IT manager, but unfortunately the company was losing money and due to a key investor pulling out at the last second, it folded. I briefly worked as a on-site desktop technician, and I loved it enough to consider starting my own company, but unfortunately I was too lazy to continue, preferring to sit at home and tinker all day.

Eventually though, I began applying to jobs in the IT sector (still avoiding software engineering) in 2024, as me and my friends planned to get a place together in 2025 and I wanted a more stable income. This led me to working as a Technical Support Engineer for half of 2025, which sucked. The main takeaway for me, other than that Singaporeans are mostly greedy and shit people to work for, was that all the IT staff seemed not to really care about what they were doing - the fact that I was on the bottom rung and yet was one of maybe 5 people in the Singapore office who knew how to even write a Python script was frankly shocking. I realised that unfortunately, IT was not for me either. The real difference maker here was me having a whole bunch of free time since I was on shift most of my time there.

## git-livesync

My first project was `git-livesync`, a VS Code extension I made because I wanted to spend that free time writing for my DnD wiki, but the work machines didn't allow installing Obsidian and I wanted my workflow of live syncing. I really enjoyed making this project, but very quickly stopped using it because in the process of learning how to properly manage a project like this, I learned about conventional commits and general hygeine, and more importantly how to use the git CLI for (I think) the first time.

## dotfiles/pantheon/cathedral/atomic/tools

Via configuring git, I learned about dotfiles as a concept, and initialised my dotfiles repository with a one byte .gitignore a la Drew DeVault. I was still working on the code-server via my browser at the time, though, which was limiting. I wanted to try Neovim, but it required me to install it by hand into the Docker compose bind mount for code-server, and I struggled slightly with it. I was rescued, however, by finding NixOS through the developer of Lichess.org, ornicar's dotfiles repo. I put it on my desktop and renamed my repo to `pantheon`, and began adopting as terminal-native a workflow as I could. This project has gone through many iterations, changing names many times and I credit it to 90% of my learning as a software engineer - it's still ongoing, and the current things I'm working on are ZMK and my website.

## hyprcloser

I really wanted to learn Rust because it just sounded way cooler than anything else, so I looked for an excuse to try it. I speedran Rustlings and the Book in a few days, then had a false start with a CLI tool to analyse a git repo for time spent per author (the reason for failure here is that git2 is insanely hard to wrap your head around, though I did find redemption by using it for a later project). Since this was obviously out of my reach for my level, I decided to go with something similar and wrote a simple daemon that used the IPC for the desktop environment I was using at the time, Hyprland, to automatically close the clipboard manager I was using when clicking out of the window, to mimic the behaviour of the Windows 10 clipboard manager I was used to. This is abandoned now, since I wrote it specifically for that tool and I daily drive a Mac currently, but I'll likely pick it back up when I'm back on Hyprland.

## Current

At this point, I was sick and tired of IT, so I began mass applying to software engineering roles in an attempt to get out, which led me to my current company. I'm better versed in DevOps than most by virtue of my hobbies, but working here exposed my severe lack of project management and web development (especially front-end) skills. I mostly spun my wheels in the latter half of 2025 w.r.t. personal projects, mostly tinkering in my free time with my dotfiles, but have resolved to focus on improving these areas of weakness. Additionally, I love Rust and I don't want to work with anything else, so I'm throwing myself into it in hopes of getting a job coding in it.

## aenyrathia

The first serious project I worked on, in December 2025, is aenyrathia, which is a replacement for the wiki software I mentioned above. I wanted an easy way for my co-writers to edit and view the wiki while allowing me to review their changes on GitHub, which I'm used to by this point. I wrote the app with Axum and Askama, because I like SSR and Jinja and Rust. Unfortunately, I was disorganised enough at the start that I ran out of time and energy to finish it by my self-imposed deadline of the end of the year, and mostly vibe coded the front-end. Because of this, I barely have any understanding on how to modify it, and will probably be rewriting this after I get a bit more experience with working with markdown and git2 via my website project.

## rrv.sh

This is the project I'm currently working on, which is integrated into my dotfiles repo because I want to reuse the code for my scripts, and also have a framework for working with Nix, Rust, OpenTofu, and GHA. It's already live and fully managed via IaC on AWS, but the current challenge is figuring out how to index and display the markdown, as I'm essentially creating a static site generator by loading in a separate content repo and baking it into the container image at build time, then loading all the files and their content into memory at startup. I'm having fun with it, and I did meet my goal of simply having the website live for this month, but I'm planning to quickly get all the infrastructure (w.r.t. parsing, indexing, and rendering my writing) so that I can fully focus next month on the front-end. I'm still not sure if I'll be trying my hand at WASM, but I know I want to go through the MDN front end course fully next month. The goal is likely to pick a good layout and theme, and to try my hand at implementing some front-end reactivity, specifically a game if I can.

## Conclusion

I completed three projects last year, not counting my dotfiles, and I'm planning to beat that this year. Off the top of my head, I'd like to make a clone of the OpenMediaVault Docker Compose UI in Go and deploy it on some VM so I can arbitrarily set up services and I want to put all my machines on NixOS and get my NAS back up and running by April at the very least. The rest of the year will be focused on building as many hard things as I can, and contributing to as much open source as I can - this is 100% to pad my portfolio, because I have a goal to get any overseas opportunity I can for next year. That's all for today - see ya tomorrow
