# Yet Another Grim Down File Editor for Linux (yagde)

_The main purpose of this project was to learn a bit about Rust._

It's a minimalistic CLI tool for editing Grim Down save files. With `yagde`, you can use:

- [`View`] view some main info: level, attributes, skills, devotions, etc;
- [`Rename`] change character name;
- [`Clone`] clone a character with a new name;
- [`Reset`] reset deaths, skills, attributes or devotions;

---

How to run:

1. git clone repo
2. run `crago build -r`
3. run `target/release/yagde`

---

How to use:

```
Grim Dawn save file editor

Usage: yagde [OPTIONS]

Options:
  -s, --save-path <SAVE_PATH>  [default: "$HOME/.local/share/Steam/steamapps/compatdata/219990/pfx/drive_c/users/steamuser/Documents/My Games/Grim Dawn/save/main"]
  -h, --help                   Print help information
```

---

Example:

```
❯ target/release/yagde -s tests/save
> Choose character: TestMain
> Choose action: View
=================== Main stats ===================
Name:                               TestMain
Sex:                                Male
Level:                              28
Hardcore:                           1
Iron:                               94717
Max difficulty:                     Ultimate
Max crucible difficulty:            Aspirant
Experience:                         337715
Skill points:                       0
Devotion points:                    0
Total devotion points:              10
Attribute points:                   5
Physique:                           258
Cunning:                            50
Spirit:                             50
Health:                             770
Energy:                             250
===================== Stats ======================
Playtime:                           6328
Deaths:                             0
Hero kills:                         29
Champion kills:                     42
Kills:                              2918
===================== Skills =====================
Skill reclamation points used:      0
Devotion reclamation points used:   1
====================== End =======================
```
