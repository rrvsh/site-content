---
title: git init ~/repos/cathedral
slug: cathedral
date: 2025-10-03
tags: [nix, site, tools]
---

okay, ive been using NixOS with this [flake](https://github.com/rrvsh/pantheon) for a while but I've kind of let it lie sallow for a while out of a kind of dissatisfaction with how I've done things. I'd love a proper rewrite with all the experience I have gained doing this.

i have moped around about doing this for months on end but it is time i cannot take not being able to deploy random shit and my servers are all down anyway IT IS TIME to go.

i broke down the issue and the number one thing that matters to me is having the product I can see and play with. To just "get there", I decided let's just start with the easiest and the edge node, the raspberry pi that will be my reverse proxy. The first step would simply be flashing a microSD card with a NixOS image with my SSH keys on there.

I almost instantly got ganked by my job (new company btw, finally SOFTWARE ENGINEERING) on the day I chose to do this small task, but managed to get er' done!

## With Some Caveats

It took 6 hours. I ran into issues with git when I was working off of two branches on pantheon. I decided on a whole new [name change](https://github.com/rrvsh/cathedral) (obviously very inspired by [CatB](http://www.catb.org/esr/faqs/hacker-howto.html)). I got distracted by a myriad other tasks, mostly also nix-related.

## Anyway

Next step is going to be getting my very static website back up on my domain:

```diff
           services = {
             ...
             nginx.enable = true;
-            nginx.virtualHosts."_".root = "/var/www/veil";
+            nginx.virtualHosts."_".root = ../www/rrv.sh;
           };
```

What I'm doing here is essentially adding [the static files](https://github.com/rrvsh/cathedral/tree/4052c606529ed90b3159977e03f2affe4f048173/www/rrv.sh) into the project and using Nix's paths to add it into the Nginx configuration.

## Detour

My ACME validation (for SSL) uses DNS-01, for which I need a Cloudflare API key, which I obviously don't want in plaintext in my repo, so it is time to add declarative secrets management with [sops-nix](https://github.com/Mic92/sops-nix).

```diff
diff --git a/docs/runbook.md b/docs/runbook.md
+ssh-to-age -private-key -i ~/.ssh/id_ed25519 > ~/.config/sops/age/keys.txt

diff --git a/flake.nix b/flake.nix
   inputs = {
+    sops-nix.url = "github:Mic92/sops-nix";
+    sops-nix.inputs.nixpkgs.follows = "nixpkgs";
   };
 }

diff --git a/nix/users.yaml b/nix/users.yaml
+rafiq:
+    password: ENC[AES256_GCM,data:8KAfatz+YSaNozd5VGo=,iv:LNRxt47iBKSWzMZuBHSxv/qDZ2h6JiTIPps7OK/o7uU=,tag:oiSfLyRVswb/wxSTE69QMA==,type:str]
+    hashedPassword: ENC[AES256_GCM,data:NogYQ3kR1TseC79HIXARrXhIncCnvxzf9zMF2QrUyTmojTffPXRGtMdjNpfMEFj5dkKfZujBL/QTIpPFFTm1py7Dreg5/9VSKQ==,iv:IwfZsrsJbLYG1ELte6aBHUtff6hIQu9rHT5tSvILIGQ=,tag:oav3paDcUY+cl4FJlZa90A==,type:str]

diff --git a/nix/veil.nix b/nix/veil.nix
   flake.nixosConfigurations.veil = lib.nixosSystem {
     modules = [
       "${inputs.nixpkgs}/nixos/modules/installer/sd-card/sd-image-aarch64.nix"
+      inputs.sops-nix.nixosModules.sops
       (
         {
           # Use the nix-community binary cache so we don't have to compile sops for aarch64-linux ourselves
+          nix.settings = {
+            substituters = [ "https://nix-community.cachix.org" ];
+            trusted-substituters = [ "https://nix-community.cachix.org" ];
+            trusted-public-keys = [ "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs=" ];
+          };
           users = {
+            mutableUsers = false;
             users.rafiq = {
+              hashedPasswordFile = config.sops.secrets."rafiq/hashedPassword".path;
             };
           };
+          sops = {
+            age.sshKeyPaths = [ "/home/rafiq/.ssh/id_ed25519" ];
+            secrets."rafiq/hashedPassword" = {
+              neededForUsers = true;
+              sopsFile = ./users.yaml;
+            };
+          };
         }
       )
     ];
```

I'm a bit out of practice since I first touched sops in [March](https://github.com/rrvsh/pantheon/commit/26ba53fee38e79ccdc438d37260f09f535b91e11), so I was a little confused why my sops config was working (which was the goal) but the `hashedPasswordFile` option wasn't kicking in. Luckily I figured out I just forgot to set `mutableUsers = false`. Now we have our password in our git repo!

## Lastly

Now we have declarative secret managment, I'll add the cloudflare key:

```diff

diff --git a/nix/keys.yaml b/nix/keys.yaml
+keys:
+    cloudflare: ENC[AES256_GCM,data:p2IISOuU/ShoifW5OFY/6Bi6PI0iIiQoBfnV512f2z84U9QS/KEhzA==,iv:5AkwtNAK8mD2DbvXCtTeNeIrpF/GIsSyOYxy8G4Jsqo=,tag:u2xJcRBR5WTMWdzupx4tbQ==,type:str]
```

Now let's use it in sops:

```diff

diff --git a/nix/veil.nix b/nix/veil.nix
           sops = {
+            secrets."keys/cloudflare".sopsFile = ./keys.yaml;
           };
```

and finally use it for our DNS-01 validation:

```diff
diff --git a/nix/veil.nix b/nix/veil.nix
           services = {
-            nginx.virtualHosts."_".root = ../www/rrv.sh;
+            nginx.virtualHosts."rrv.sh" = {
+              addSSL = true;
+              useACMEHost = "rrv.sh";
+              acmeRoot = null; # needed for DNS validation
+              locations."/".root = ../www/rrv.sh;
+            };
           };
           users = {
+            users.nginx.extraGroups = [ "acme" ];
           };
           security = {
+            acme = {
+              acceptTerms = true;
+              defaults = {
+                email = "rafiq@rrv.sh";
+                dnsProvider = "cloudflare";
+                credentialFiles."CLOUDFLARE_DNS_API_TOKEN_FILE" = config.sops.secrets."keys/cloudflare".path;
+              };
+              certs."rrv.sh".extraDomainNames = [ "*.rrv.sh" ];
+            };
           };
```

and that's it! A quick ssh and `nixos-rebuild switch --sudo --flake .` later, `veil` is up and more importantly, so is [my website](https://rrv.sh).

Here's the [repo at this point](https://github.com/rrvsh/cathedral/tree/4052c606529ed90b3159977e03f2affe4f048173). I'm trying to document any other interesting things I come up with in the process, so hopefully you see more blog posts soon. Side note: still pretty crazy that i can go from a dead Raspberry Pi to a live, SSL-secured website in a single rebuild — if that doesn't make you wanna try nix, idk what does.

Next step is getting my DnD wiki back up, which is unfortunately going to involve me learning how to declare docker compose with nix. see ya next time
