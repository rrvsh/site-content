---
title: Manually setting up a Windows EFI System Partition
slug: binbows-rescue
date: 2026-01-31
tags: [daily-blog]
---

As mentioned previously, I reinstalled NixOS on one of the drives in my desktop PC. I say one of because the other drive still has Windows on it, because when I wanna play games I don't really want to faff about with Proton and such. This morning, I wanted to play some Crusader Kings 3, so I went into UEFI - curiously though, Windows Boot Manager wasn't there.

For context, since I had two drives each with their own EFI System Partition (ESP), which is where the bootloader lives for each operating system, I should have a "Linux Boot Manager" and a "Windows Boot Manager". Not only was the latter not showing up, the drive itself wasn't either.

At this point, I figured that the NixOS install probably just overwrote all the motherboard boot entries. I figured it would be easiest to boot into NixOS and just add a boot entry to `systemd-boot` (my Linux bootloader of choice) pointing to the other drive. I found that to achieve this, I had to install and boot into an EDK2 UEFI Shell, extract the EFI device handle of my Windows install and add it to `systemd-boot`. This is trivial with NixOS:

```nix
boot.loader.systemd-boot.edk2-uefi-shell.enable = true;
boot.loader.systemd-boot.windows."11-pro" = {
    title = "Windows 11 Pro";
    efiDeviceHandle = "HD0d";
};
```

> [!NOTE]
> For anyone already lost, don't worry - a shell is just a way to interact with a computer and this shell is a way to bypass the operating system and run commands on the underlying hardware.

But this was unfortunately (or fortunately if you consider I got a blog post out of it) not the end of the story. When I booted into the UEFI shell, I ran `map -c` to see the available device handles. Curiously, there was only one - my Linux disk.

It's alright, I thought - just boot into Linux and figure out why it isn't showing. At this point, I already suspected what had happened, but we'll get to that in a minute. `lsblk -f` confirmed it though: my Windows drive showed only three partitions - the Microsoft reserved partition, my actual Windows NTFS partition, and a system information partition. The ESP was completely gone.

## WTF

Unfortunately, despite trying my best to mount the partitions, I could not find any evidence to point to the ESP still being there. So, I went to my one true love Microsoft's website and downloaded a fresh W11 ISO to put on my Ventoy disk, cursing myself for deleting it **literally** the day before. In the meantime, I began googling to try and figure out what I had to do next.

Here's the plan I came up with:

1. Use `bcdboot` to copy the boot environment files from the Windows partition to the ESP.

Yeah, I have basically no experience with recovering Windows installs. I knew I had to actually create the ESP, but I didn't know how and the documentation for this is fucking horrid. Credit where credit's due, though - the documentation for [bcdboot](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/bcdboot-command-line-options-techref-di?view=windows-11) itself was phenomenal.

Anyway, the install ISO was finally done copying to my Ventoy drive, so I switched my KVM to my desktop and prayed for the best.

## How to manually configure a Windows EFI System Partition

Booting into the install media, I navigated to the Command Prompt and went into `diskpart`. At this point I knew roughly that I had to create the partition, but listing the existing partitions showed that there was no unallocated space. Somehow, the NixOS install had not only removed the ESP, but the other partitions on the device had grown to full the space it took up.

The way to handle this is to shrink a partition so I would have space for the new one. I was very very nervous - I had never done this and have always heard of the dangers of fucking with existing partitions. Luckily, this didn't go too wrong. I selected the partition in `diskpart`, shrank it by 300MB, and added a new FAT32 partition. I then used `bcdboot` and copied over the boot env files from the existing partition.

I did have one scare - while trying to format the new partition, I hit `format<Enter>` thinking it would fail with a helpful message. It said "format successful" and my heart dropped between my balls - I thought I had formatted the windows partition and lost all my data. Thankfully a `detail part` showed all was well.

Regardless, here's the commands:

```bat
diskpart
list disk
select disk <disk> # the windows one
list partition
select partition <partition> # the windows one (should be the biggest)
assign letter=A # whatever letter is free (so we can refer to it)
shrink desired=300
create partition efi size=300
format fs=fat32 quick label=ESP
assign letter=S # whatever letter is free (so we can refer to it)
exit

bcdboot A:\Windows /s S:
```

A quick reboot later and I was back in my Windows install with both boot managers showing in UEFI. Success!

## Doing what I set out to do in the first place

I booted back into the EDK2 UEFI shell and ran `map -c` again, and thankfully this time it showed up as `HD0d`. I added it to my NixOS configuration with:

```nix
boot.loader.systemd-boot.windows."11-pro" = {
    title = "Windows 11 Pro";
    efiDeviceHandle = "HD0d";
};
```

and rebooted, seeing it appear in the `systemd-boot` list. Very nice!

## What can we learn from this?

I'm not sure! See you tomorrow
