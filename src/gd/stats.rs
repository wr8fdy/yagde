use crate::gd::gd_file::{Block, FileError, GDReader, GDWriter, ReadWrite};

use anyhow::{bail, Ok, Result};
use smart_default::SmartDefault;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct SkillMap {
    skill: String,
    active: u32,
}

impl ReadWrite for SkillMap {
    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_string(&self.skill)?;
        f.write_int(self.active)
    }

    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.skill = f.read_string()?;
        self.active = f.read_int()?;

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct StatsPerDifficulty {
    greatest_monster_killed_name: String,
    greatest_monster_killed_level: u32,
    greatest_monster_killed_life_and_mana: u32,
    last_monster_hit: String,
    last_monster_hit_by: String,
    nemesis_kills: u32,
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct Stats {
    pub champion_kills: u32,
    pub deaths: u32,
    pub hero_kills: u32,
    pub kills: u32,
    pub playtime: u32,

    version: u32,
    critical_hits_inflicted: u32,
    critical_hits_received: u32,
    difficulty_skip: u8,
    endless_essence: u32,
    endless_souls: u32,
    experience_from_kills: u32,
    greatest_damage_inflicted: f32,
    greatest_damage_received: f32,
    health_potions_used: u32,
    hits_inflicted: u32,
    hits_received: u32,
    items_crafted: u32,
    last_hit: f32,
    last_hit_by: f32,
    lore_notes_collected: u32,
    mana_potions_used: u32,
    max_level: u32,
    mythical_relics_crafted: u32,
    one_shot_chests_opened: u32,
    relics_crafted: u32,
    shrines_restored: u32,
    skill_map: Vec<SkillMap>,
    stats_difficulty: [StatsPerDifficulty; 3],
    survival_defenses_built: u32,
    survival_greatest_score: u32,
    survival_greatest_wave: u32,
    survival_powerups_activated: u32,
    transcendent_relics_crafted: u32,
    unknown1: u32,
    unknown2: u32,
    #[default = 16]
    block_seq: u32,
}

impl Stats {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        f.write_int(self.playtime)?;
        f.write_int(self.deaths)?;
        f.write_int(self.kills)?;
        f.write_int(self.experience_from_kills)?;
        f.write_int(self.health_potions_used)?;
        f.write_int(self.mana_potions_used)?;
        f.write_int(self.max_level)?;
        f.write_int(self.hits_received)?;
        f.write_int(self.hits_inflicted)?;
        f.write_int(self.critical_hits_inflicted)?;
        f.write_int(self.critical_hits_received)?;
        f.write_float(self.greatest_damage_inflicted)?;

        for d in self.stats_difficulty.iter() {
            f.write_string(&d.greatest_monster_killed_name)?;
            f.write_int(d.greatest_monster_killed_level)?;
            f.write_int(d.greatest_monster_killed_life_and_mana)?;
            f.write_string(&d.last_monster_hit)?;
            f.write_string(&d.last_monster_hit_by)?;
        }

        f.write_int(self.champion_kills)?;
        f.write_float(self.last_hit)?;
        f.write_float(self.last_hit_by)?;
        f.write_float(self.greatest_damage_received)?;
        f.write_int(self.hero_kills)?;
        f.write_int(self.items_crafted)?;
        f.write_int(self.relics_crafted)?;
        f.write_int(self.transcendent_relics_crafted)?;
        f.write_int(self.mythical_relics_crafted)?;
        f.write_int(self.shrines_restored)?;
        f.write_int(self.one_shot_chests_opened)?;
        f.write_int(self.lore_notes_collected)?;

        for d in self.stats_difficulty.iter() {
            f.write_int(d.nemesis_kills)?;
        }

        if self.version >= 9 {
            f.write_int(self.survival_greatest_wave)?;
            f.write_int(self.survival_greatest_score)?;
            f.write_int(self.survival_defenses_built)?;
            f.write_int(self.survival_powerups_activated)?;
        }

        if self.version >= 11 {
            f.write_vec(&self.skill_map)?;
            f.write_int(self.endless_souls)?;
            f.write_int(self.endless_essence)?;
            f.write_byte(self.difficulty_skip)?;
        }

        f.write_int(self.unknown1)?;
        f.write_int(self.unknown2)?;

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f.read_int()?;
        if self.version != 7 && self.version != 9 && self.version != 11 {
            bail!(FileError::UnsupportedVersion(
                self.version,
                "7,9,11".to_string()
            ));
        }

        self.playtime = f.read_int()?;
        self.deaths = f.read_int()?;
        self.kills = f.read_int()?;
        self.experience_from_kills = f.read_int()?;
        self.health_potions_used = f.read_int()?;
        self.mana_potions_used = f.read_int()?;
        self.max_level = f.read_int()?;
        self.hits_received = f.read_int()?;
        self.hits_inflicted = f.read_int()?;
        self.critical_hits_inflicted = f.read_int()?;
        self.critical_hits_received = f.read_int()?;
        self.greatest_damage_inflicted = f.read_float()?;

        for i in 0..self.stats_difficulty.len() {
            self.stats_difficulty[i].greatest_monster_killed_name = f.read_string()?;
            self.stats_difficulty[i].greatest_monster_killed_level = f.read_int()?;
            self.stats_difficulty[i].greatest_monster_killed_life_and_mana = f.read_int()?;
            self.stats_difficulty[i].last_monster_hit = f.read_string()?;
            self.stats_difficulty[i].last_monster_hit_by = f.read_string()?;
        }

        self.champion_kills = f.read_int()?;
        self.last_hit = f.read_float()?;
        self.last_hit_by = f.read_float()?;
        self.greatest_damage_received = f.read_float()?;
        self.hero_kills = f.read_int()?;
        self.items_crafted = f.read_int()?;
        self.relics_crafted = f.read_int()?;
        self.transcendent_relics_crafted = f.read_int()?;
        self.mythical_relics_crafted = f.read_int()?;
        self.shrines_restored = f.read_int()?;
        self.one_shot_chests_opened = f.read_int()?;
        self.lore_notes_collected = f.read_int()?;

        for i in 0..self.stats_difficulty.len() {
            self.stats_difficulty[i].nemesis_kills = f.read_int()?;
        }

        if self.version >= 9 {
            self.survival_greatest_wave = f.read_int()?;
            self.survival_greatest_score = f.read_int()?;
            self.survival_defenses_built = f.read_int()?;
            self.survival_powerups_activated = f.read_int()?;
        }

        if self.version >= 11 {
            self.skill_map = f.read_vec()?;
            self.endless_souls = f.read_int()?;
            self.endless_essence = f.read_int()?;
            self.difficulty_skip = f.read_byte()?;
        }

        self.unknown1 = f.read_int()?;
        self.unknown2 = f.read_int()?;

        f.read_block_end(&mut b)
    }
}
