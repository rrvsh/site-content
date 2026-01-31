# NixOS, Windows on another drive, systemd-boot setup plan

**Session ID:** ses_3ee2c3362ffeGxVWKK9zaMT4Kq
**Created:** 1/31/2026, 10:13:39 AM
**Updated:** 1/31/2026, 11:33:11 AM

---

## User

use the nixos tool and web search and figure out how to get windows on another drive in the same pc to show up for @nix/configs/nemesis.nix via systemd-boot

---

## Assistant

I can’t make edits in plan mode, but here’s a concrete plan based on the nixpkgs systemd-boot module and the systemd-boot man page.

**What I found**

- `systemd-boot` automatically lists Windows if its bootloader is on the same ESP it uses. If Windows is on a different drive/ESP, NixOS needs a manual entry.
- NixOS exposes `boot.loader.systemd-boot.windows` for this exact case, with a built‑in workflow to discover the Windows ESP device handle via the EDK2 UEFI shell.

**Plan**

- Confirm whether Windows uses the same ESP as NixOS. If it does, no config changes are needed; Windows should auto‑appear in the menu.
- If Windows is on another drive/ESP:
  - Temporarily enable the UEFI shell:
    - Add `boot.loader.systemd-boot.edk2-uefi-shell.enable = true;` in `nix/configs/nemesis.nix`.
  - Rebuild boot (no rebuild here yet; just the intended step), then reboot and enter “EDK2 UEFI Shell”.
  - In the shell, find the Windows ESP handle:
    - `map -c`
    - For each handle (e.g. `HD0c1`), run `ls HD0c1:\EFI` and look for `Microsoft`.
    - Test with `HD0c1:\EFI\Microsoft\Boot\Bootmgfw.efi` to confirm it boots.
  - Add a Windows entry in `nix/configs/nemesis.nix` using the found handle:
    ```nix
    boot.loader.systemd-boot.windows."11" = {
      title = "Windows 11";
      efiDeviceHandle = "HD0c1";
      sortKey = "z_windows";
    };
    ```
  - Optionally disable the UEFI shell afterward.

**Why this works**

- NixOS’ `boot.loader.systemd-boot.windows` builds a loader entry that chains into the Windows boot manager on the other ESP, using the device handle discovered via the EDK2 shell.

Question (so I can prepare the exact change): do you already know the Windows ESP device handle (like `HD0c1` or `FS1`)? If yes, tell me the handle and the title you want; if not, I can walk you through the shell step-by-step.

---

## User

i dont - is it possible to detect it from the command line wihtout having to boot into the uefi shell?

---

## Assistant

Short answer: there isn’t a reliable way to get the `HD0c1` / `FS1` handle from Linux alone. The NixOS `boot.loader.systemd-boot.windows` option is designed around the EDK2 UEFI Shell `map -c` output, so that’s the canonical way.

What you _can_ do from Linux is confirm which partition is the Windows ESP:

- Find all ESPs (vfat with `EFI` folder), mount read‑only, and look for `EFI/Microsoft/Boot/Bootmgfw.efi`.
- `efibootmgr -v` can show the firmware’s Windows boot entry and its device path, but it does not map cleanly to the `HD0c1` style handle that the NixOS module expects.

If you want to avoid UEFI shell entirely, the other workaround is to copy `Bootmgfw.efi` onto your NixOS ESP and add a normal `systemd-boot` entry via `boot.loader.systemd-boot.extraEntries`, but that _does_ require copying files onto `/boot` and keeping them in sync.

If you want, I can outline the exact Linux steps to identify the Windows ESP and then decide whether you’d prefer:

1. quick UEFI shell mapping (recommended), or
2. copy + `extraEntries` approach.

---

## User

okay im finding the esp handle now, in the meantime could you help me describe exactly what edk2 is and what the edk2 uefi shell is? also, whats an efi device handle, and how does the .efi file work?

---

## Assistant

Here’s a clear, practical explanation in the context of what you’re doing:

**EDK2**

- EDK2 is the open‑source reference implementation of UEFI firmware (originally “TianoCore”).
- It provides firmware components and tools, including a UEFI Shell that runs _before_ any OS, directly in UEFI.

**EDK2 UEFI Shell**

- A tiny interactive environment that runs as a UEFI application (from your ESP).
- Think of it as a pre‑OS command line with built‑in commands like `map`, `ls`, and the ability to launch other EFI apps.
- systemd‑boot can offer it as a boot menu entry so you can inspect devices and launch Windows’ bootloader directly.

**EFI device handle (e.g., `HD0c1`, `FS1`)**

- A UEFI firmware identifier for a specific partition/volume.
- In the shell, `map -c` prints consistent handles for disks/partitions; `HD0c1` is “hard disk 0, partition 1”, `FS1` is a filesystem alias.
- NixOS’s `boot.loader.systemd-boot.windows` option wants this handle so it can tell the UEFI shell to launch the Windows bootloader from the correct ESP on another drive.

**What a `.efi` file is**

- A `.efi` file is a UEFI executable (PE/COFF format), similar to a Windows `.exe` but for firmware.
- `Bootmgfw.efi` is the Windows Boot Manager. When run by firmware or a boot manager, it finds and boots Windows.

So the flow is: systemd‑boot → (EDK2 shell entry) → use the handle to locate the Windows ESP → run `Bootmgfw.efi` to confirm → use that handle in NixOS so systemd‑boot can launch it directly.

If you want, once you get the handle, tell me the exact string (like `HD0c1`) and I’ll give you the minimal NixOS snippet for `nix/configs/nemesis.nix`.

---

## User

fuck the map -c only shows the linux drive

---

## Assistant

Got it. If `map -c` only shows the Linux drive, the firmware isn’t exposing the Windows disk/ESP to the UEFI shell. That usually means one of these is true:

- The Windows drive isn’t being detected by firmware at all (power/data issue, BIOS/UEFI storage config).
- Windows was installed in **Legacy/CSM** mode (no UEFI bootloader/ESP to discover).
- The drive is behind a controller mode or RAID/Intel RST setting that the UEFI shell can’t see.
- The Windows ESP is on a different disk that’s disabled in BIOS/UEFI boot settings.

Try these checks in firmware (no OS changes needed):

- Ensure the Windows drive appears in BIOS/UEFI storage list.
- Disable CSM/Legacy boot (force UEFI-only).
- Switch storage mode to AHCI (if safe for your Windows install; only do this if you know Windows already supports it).
- Make sure the Windows disk is not disabled in boot order or storage settings.

If you can access Windows, confirm whether it’s UEFI:

- In Windows: `msinfo32` → “BIOS Mode” should be **UEFI**.
- If it says **Legacy**, systemd-boot won’t see a UEFI bootloader on that drive. You’d need to convert Windows to UEFI (mbr2gpt) or use GRUB instead.

If you _do_ want to keep systemd-boot and Windows is UEFI but the handle isn’t showing, you can also:

- Use the firmware’s own boot menu (not systemd-boot) to see if it lists a “Windows Boot Manager” entry. If it does, the disk is visible to firmware; the shell should also see it once mapping is refreshed (try `map -r`).

Question (so I can narrow this down): does `msinfo32` in Windows show **BIOS Mode: UEFI** or **Legacy**? If Windows is Legacy, I’ll outline the conversion or an alternative bootloader path.

---

## User

okay here's what i found out - my windows drive is in 3 partitions, 2 of them ntfs (1 system volume info etc 2 is C drive). the p1 is empty with no format - im downloading the windows 11 install iso to try and boot into it to get the esp partition back.

---

## Assistant

Understood. Given that layout (p1 empty/unformatted, p2+p3 NTFS), Windows likely lost or never had its ESP, or it was deleted. In UEFI mode, Windows _must_ have a FAT32 ESP with `EFI/Microsoft/Boot/Bootmgfw.efi`. Without it, systemd-boot can’t discover anything.

Here’s a safe, read-only plan you can follow while you boot the Windows 11 installer ISO:

**Plan**

- Boot the Windows 11 ISO in UEFI mode.
- Use its recovery tools to recreate the ESP and restore the Windows bootloader.
- After Windows boots again, confirm it’s UEFI (`msinfo32` → BIOS Mode).
- Then retry the UEFI shell `map -c` to get the handle and add the NixOS `systemd-boot.windows` entry.

**Windows installer recovery steps (high level)**

- Choose “Repair your computer” → Troubleshoot → Command Prompt.
- Identify disks and partitions, create a new EFI System Partition (FAT32, ~100–500MB), assign a letter.
- Run `bcdboot` to install boot files onto that ESP.

I can give exact commands once you confirm:

1. Which disk is Windows on (disk number), and
2. If you want to reuse the empty p1 as the ESP (recommended).

If you want exact steps, tell me the disk number and size from Windows `diskpart`, and I’ll write the precise command sequence.

---

## User

Use its recovery tools to recreate the ESP and restore the Windows bootloader.

can you look up how to do this exactly? is there a way that windows installer can do it for me or do i need to manually do it

---

## Assistant

Yes, you can do it manually from the Windows installer’s recovery environment. There isn’t a reliable one‑click “fix ESP” button—Windows “Startup Repair” sometimes recreates it, but it often fails when the ESP is missing entirely. The most dependable approach is manual using `diskpart` + `bcdboot`.

Here’s the exact, standard sequence (you’ll run these from the Windows installer ISO → “Repair your computer” → Troubleshoot → Command Prompt):

**Manual ESP recreate + bootloader restore**

```bat
diskpart
list disk
select disk <WINDOWS_DISK_NUMBER>
list partition

# If p1 is the empty partition you want to use:
select partition <P1_NUMBER>
# If it is not already FAT32, format it:
format fs=fat32 quick label=ESP
assign letter=S
exit

# Identify the Windows partition letter
diskpart
list volume
# note the volume that contains your Windows install (usually large NTFS)
exit

# Assume Windows volume is C: (replace if different)
bcdboot C:\Windows /s S: /f UEFI
```

**Notes**

- If the empty p1 is not large enough (100–500MB is typical), you may need to shrink the NTFS partition and create a new FAT32 ESP.
- If `bcdboot` completes successfully, you should see `S:\EFI\Microsoft\Boot\Bootmgfw.efi`.
- Then reboot, enter BIOS/UEFI, and set “Windows Boot Manager” as an option (it should now appear).

**Optional “try first”**

- In the Windows installer, you can run **Startup Repair**. If it doesn’t fix it, go manual.

If you want, tell me:

1. the disk number for Windows,
2. the size of the empty p1, and
3. which volume letter Windows is showing in `diskpart`,

and I’ll sanity‑check the exact commands for your layout.

---

## User

okay im in the windows installer, ive launched diskpart. select disk 0 list part says 3 partitions, 1 is reserved 16mb, part 2 is primary 1.8tb, part 3 is recovery 730mb. sel disk 1 list part says 3 partitions, part 3 is system 487 mb, part 1 is unkown 1855 gb, part 2 is unkown 7GB. disk 1 should be nixos right based on @nix/configs/nemesis.nix ? how can we verify?

---

## Assistant

Based on what you see, **Disk 0 is almost certainly Windows** and **Disk 1 is almost certainly NixOS**.

Why:

- Disk 0 has the classic Windows GPT layout: `16MB Reserved (MSR) + large Primary (C:) + Recovery`. That’s a standard Windows install pattern.
- Disk 1 has a `System 487MB` partition (likely the NixOS ESP) plus two “Unknown” partitions (Windows installer can’t read ext4, so it shows as Unknown). That matches a typical Linux disk: ESP + ext4 root + maybe something else.

How to verify in DiskPart (no risk, read-only):

1. `list volume`
2. Look for:
   - A **FAT32** volume ~487MB with a label (often “EFI” or “BOOT”) → that’s your NixOS ESP on Disk 1.
   - A **large NTFS** volume ~1.8TB with your Windows files → that’s Disk 0 partition 2.
3. You can also do:
   - `select disk 1`
   - `detail disk` (shows which volumes live on the disk)

If you want to be 100% sure, paste the output of `list volume` and I’ll map it precisely.

---

## User

cant paste, the windows installer is not networked lol. anyway list volume says volume 1 is the 2tb ntfs, volume 2 is the 730mb ntfs. volume 3 is the linux esp, volume 0,4,5 is my usb drive. this verifies that the windows esp is gone right

---

## Assistant

Yes — that layout confirms the Windows ESP is missing.

- Disk 0 (Windows) only shows a large NTFS volume and the recovery NTFS volume.
- There is no FAT32 “System” / “EFI” volume on Disk 0.
- The only FAT32/“System” volume you saw (487MB) is on Disk 1, which matches your NixOS ESP.

So Windows has no ESP right now.

Next step is to recreate an ESP on Disk 0 and run `bcdboot` to install the Windows boot files onto it. If you want, I can give you the exact command sequence tailored to Disk 0 (using free space or shrinking the Windows partition if needed) — just tell me whether you want to reuse existing free space or are OK shrinking the 2TB NTFS partition by ~300–500MB.

---

## User

okay i know that i have to run bcdboot, but i need to assign a drive letter to the windows partition and the (as yet uncreated) esp first yes? how do i find these and assign drive letters to them

---

## Assistant

Yes — you need drive letters for both the Windows partition and the new ESP before running `bcdboot`.

Here’s exactly how to do it from the Windows installer command prompt:

**1) Identify the Windows partition and give it a letter**

```bat
diskpart
list volume
```

- Find the large NTFS volume (your Windows partition).
- Then:

```bat
select volume <WINDOWS_VOLUME_NUMBER>
assign letter=C
exit
```

(Use `C` if it’s free; otherwise pick any free letter.)

**2) Create the ESP and assign a letter**
If you already have unallocated space on Disk 0 (recommended 300–500MB):

```bat
diskpart
select disk 0
create partition efi size=500
format fs=fat32 quick label=ESP
assign letter=S
exit
```

If you _don’t_ have unallocated space, you must shrink the large NTFS partition first (from DiskPart):

```bat
diskpart
select disk 0
select partition 2   # (your big Windows partition)
shrink desired=500
create partition efi size=500
format fs=fat32 quick label=ESP
assign letter=S
exit
```

**3) Install Windows boot files onto the ESP**

```bat
bcdboot C:\Windows /s S: /f UEFI
```

After that, reboot, and Windows Boot Manager should appear in BIOS/UEFI and be detectable.

If you want, tell me:

- whether Disk 0 has unallocated space already, and
- whether you successfully assigned letters (and which),  
  and I’ll confirm the exact next command.

---

## User

okay i assigned A to the windows volume (c was taken up). ive selected disk - and lsit part says part 1 is reserved 16mb, part2 is primary 1862gb, part 3 is recovery 730mb. is it safe to create partition efi size=500 now? also, what sizes should i consider?

---

## Assistant

It’s safe to create the ESP **only if there is unallocated space on Disk 0**. From what you described, Disk 0 currently has only the MSR, the large primary, and the recovery partition — so unless you already shrank the primary partition, there’s no free space yet. Creating the ESP would fail unless you first shrink partition 2.

**Sizes to use**

- Minimum: 100 MB (Windows can work with this, but it’s tight).
- Safer: 260–300 MB.
- Comfortable: 500 MB (what you suggested) — plenty of room and common.

**Recommended approach**

1. Shrink the big primary partition by 300–500 MB.
2. Create the ESP in the newly freed space.
3. Format it FAT32 and assign a letter (e.g. `S`).
4. Run `bcdboot A:\Windows /s S: /f UEFI`.

Do you already have unallocated space on Disk 0? If not, do you want to shrink by 300 MB or 500 MB? I can give you the exact DiskPart commands once you pick a size.

---

## User

hold on how to check if i have unallocated space? it only shows the 3 partitions. i think the reserved partition is the old esp that got unformatted during linux install.

---

## Assistant

Unallocated space won’t show up as a partition. If `list partition` only shows those 3, then there’s **no unallocated space** right now.

That 16 MB “Reserved” partition is the **MSR** (Microsoft Reserved), _not_ the ESP. The ESP would be a **FAT32 System** partition, typically 100–500 MB.

How to verify:

- In DiskPart:
  - `select disk 0`
  - `list partition`
  - If there’s no “EFI” or “System” partition and no unallocated space, you must **shrink** the large primary partition to create space.

If you want, tell me how much you want to shrink (300 MB or 500 MB are both fine), and I’ll give you the exact DiskPart commands.

---

## User

i really dont want to shrink a partition if i dont have to. how can i see if i have unallocated space?

---

## Assistant

You can see unallocated space in DiskPart by switching to the disk view (not partitions):

```
diskpart
list disk
select disk 0
list partition
```

If there’s unallocated space, DiskPart will show a “\*” row like:

```
  Partition 1  Reserved   16 MB
  Partition 2  Primary    1862 GB
  Partition 3  Recovery   730 MB
  *  Unallocated  500 MB
```

If you only see the three partitions and no “Unallocated” row, then there is **no free space** on that disk.

You can also run:

```
detail disk
```

That will show total size and used space, but `list partition` is the clearest view for unallocated.

If there’s no unallocated space, the only way to create an ESP is to shrink the large primary partition. If you do see unallocated space, you can create the ESP there without shrinking.

---

## User

okay wait before i shrink it - if i try to create partition efi size=500 it will fail if there is not enough unallocated space right? is there any risk to running ti?

---

## Assistant

If there’s no unallocated space, `create partition efi size=500` will fail with “not enough free space” and **it won’t change anything**. It doesn’t delete or shrink existing partitions on its own. So the only “risk” is that it just errors out.

If you want the safest possible check first, `list partition` is still the best way to confirm unallocated space.

---

## User

okay yeah it errors out. is shrinking the partition really the only way? I dont want data loss if possible

---

## Assistant

Yes — if there’s no unallocated space on Disk 0, **shrinking the Windows partition is the only non-destructive way** to create a new ESP. There’s no other place to put it. The ESP must be a small FAT32 partition on the same disk, and Windows won’t boot in UEFI mode without it.

Good news: shrinking is safe and non‑destructive when done properly. You’re only reducing the partition size by a few hundred MB at the end of the disk.

If you want the lowest‑risk path:

- Shrink by **300 MB** (enough for ESP).
- Create the ESP in the newly freed space.
- Run `bcdboot`.

If you want, I can give you the exact DiskPart commands to shrink by 300 MB and create the ESP, using your current letters (`A:` for Windows).

---

## User

okay yeah explain

---

## Assistant

Here’s the safest, non‑destructive plan to restore the Windows ESP from the installer, using your current drive letters (Windows partition is `A:`):

**Plan**

1. Shrink the Windows partition by 300–500 MB (this creates unallocated space).
2. Create a new EFI System Partition (ESP) in that space.
3. Format it FAT32 and assign a drive letter (e.g., `S:`).
4. Run `bcdboot` to install the Windows bootloader onto the new ESP.

**Exact commands (DiskPart + bcdboot)**

```bat
diskpart
select disk 0
select partition 2
shrink desired=300
create partition efi size=300
format fs=fat32 quick label=ESP
assign letter=S
exit

bcdboot A:\Windows /s S: /f UEFI
```

**Notes**

- `shrink desired=300` is in MB. You can use 500 instead if you want more space:
  - `shrink desired=500`
  - `create partition efi size=500`
- This is safe: it reduces the end of the Windows partition but doesn’t delete data.
- After `bcdboot`, the Windows Boot Manager should appear in BIOS/UEFI again.

If you want, tell me whether you prefer **300 MB** or **500 MB**, and I’ll confirm the exact command sequence accordingly.

---

## User

how to ls?

---

## Assistant

If you’re still in DiskPart, the equivalent of “ls” is:

```bat
list disk
list partition
list volume
```

Those show disks, partitions, and volumes.

If you meant in the UEFI shell, the command is just:

```
ls
```

Tell me which environment you’re in (DiskPart or UEFI shell) and I’ll tailor the exact command.

---

## User

nah in windows install shell, not in diskpart

---

## Assistant

In the Windows installer command prompt (not DiskPart), use:

```
dir
```

Example:

```
dir A:\
dir S:\
```

That’s the equivalent of “ls” there.

---

## User

IT WORKS thanks lumen

---

## Assistant

Glad it worked.

If you want, next steps are:

1. Reboot into Windows and confirm `msinfo32` shows **BIOS Mode: UEFI**.
2. Reboot into NixOS, enable the UEFI shell entry, and use `map -c` again to find the Windows ESP handle.
3. Add the `boot.loader.systemd-boot.windows` entry once you have the handle.

Say the word and I’ll help with the exact NixOS snippet.

---
