use crate::gd::gd_file::{GDFile, GDReader, GDWriter, ReadWrite};
use crate::gd::info::{Bio, Info};
use crate::gd::inventory::*;
use crate::gd::lists::*;
use crate::gd::misc::*;
use crate::gd::skills::*;
use crate::gd::stats::*;

use std::fs::File;
use std::path::PathBuf;

use anyhow::{bail, Error, Ok, Result};
use smart_default::SmartDefault;
use strum_macros::Display;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Parse sex error: {0}")]
    ParseSexError(u8),
    #[error("Parse expansion status: {0}")]
    ParseExpansionStatusError(u8),
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct Char {
    pub header: Header,
    pub stats: Stats,
    pub bio: Bio,
    pub version: u32,
    pub uid: CharUID,
    pub info: Info,
    pub inventory: Inventory,
    pub stash: Stash,
    pub respawns: RespawnList,
    pub teleports: TeleportList,
    pub markers: MarkerList,
    pub shrines: ShrineList,
    pub skills: SkillList,
    pub notes: NoteList,
    pub factions: FactionList,
    pub ui: UI,
    pub tutorials: TutorialPages,
    pub crucible: Crucible,
    #[default(_code = "vec![6, 7, 8]")]
    supported_versions: Vec<u32>,
}

impl Char {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn rename(&mut self, new_name: &str) -> &mut Self {
        self.header.name = new_name.to_owned();
        self
    }

    pub fn reset_all(&mut self) -> &mut Self {
        self.reset_devotions()
            .reset_attributes()
            .reset_deaths()
            .reset_skills();
        self
    }

    pub fn reset_devotions(&mut self) -> &mut Self {
        self.skills.devotion_reclamation_points_used = 0;
        let pat_dev = "records/skills/devotion";
        let pat_skill = "records/skills/playerclass";

        let retained_devotion_points = self
            .skills
            .skills
            .iter()
            .filter(|&s| s.name.starts_with(pat_dev) && s.enabled == 1 && s.level == 1)
            .count() as u32;

        self.skills.skills.retain(|s| {
            !(s.devotion_level == 1
                && s.name.starts_with(pat_dev)
                && s.enabled == 1
                && s.level == 1)
        });

        for s in self.skills.skills.iter_mut() {
            if s.devotion_level > 1 {
                s.enabled = 1;
                s.level = 0;
            }
            if s.name.starts_with(pat_skill) {
                s.auto_cast_skill = String::new();
                s.auto_cast_controller = String::new();
            }
        }

        self.bio.devotion_points += retained_devotion_points;
        self.bio.total_devotion = self.bio.devotion_points;
        self
    }

    pub fn reset_attributes(&mut self) -> &mut Self {
        let spend_attibutes = (self.bio.cunning - 50.0) / 8.0
            + (self.bio.physique - 50.0) / 8.0
            + (self.bio.spirit - 50.0) / 8.0;

        self.bio.attribute_points += spend_attibutes as u32;
        self.bio.cunning = 50.0;
        self.bio.physique = 50.0;
        self.bio.spirit = 50.0;
        self
    }

    pub fn reset_skills(&mut self) -> &mut Self {
        self.skills.skill_reclamation_points_used = 0;
        let pat = "records/skills/playerclass";

        let retained_skill_points = self
            .skills
            .skills
            .iter()
            .filter(|&s| s.name.starts_with(pat) && s.enabled == 1)
            .fold(0, |acc, x| acc + x.level);

        self.skills.skills.retain(|s| !s.name.contains(pat));
        self.bio.skill_points += retained_skill_points;
        self
    }

    pub fn reset_deaths(&mut self) -> &mut Self {
        self.stats.deaths = 0;
        self
    }

    pub fn save_as(&mut self, path: &PathBuf) -> Result<()> {
        self.write(path)
    }

    pub fn print_info(&self) {
        println!("{:=^50}", " Main stats ");
        println!("{0: <35} {1}", "Name:", &self.header.name);
        println!("{0: <35} {1}", "Sex:", &self.header.sex);
        println!("{0: <35} {1}", "Level:", &self.header.level);
        println!("{0: <35} {1}", "Hardcore:", &self.header.hardcore);
        println!("{0: <35} {1}", "Iron:", &self.info.money);
        println!(
            "{0: <35} {1}",
            "Max difficulty:", &self.info.greatest_difficulty
        );
        println!(
            "{0: <35} {1}",
            "Max crucible difficulty:", &self.info.greatest_crucible_difficulty
        );
        println!("{0: <35} {1}", "Experience:", &self.bio.experience);
        println!("{0: <35} {1}", "Skill points:", &self.bio.skill_points);
        println!(
            "{0: <35} {1}",
            "Devotion points:", &self.bio.devotion_points
        );
        println!(
            "{0: <35} {1}",
            "Total devotion points:", &self.bio.total_devotion
        );
        println!(
            "{0: <35} {1}",
            "Attribute points:", &self.bio.attribute_points
        );
        println!("{0: <35} {1}", "Physique:", &self.bio.physique);
        println!("{0: <35} {1}", "Cunning:", &self.bio.cunning);
        println!("{0: <35} {1}", "Spirit:", &self.bio.spirit);
        println!("{0: <35} {1}", "Health:", &self.bio.health);
        println!("{0: <35} {1}", "Energy:", &self.bio.energy);

        println!("{:=^50}", " Stats ");
        println!("{0: <35} {1}", "Playtime:", &self.stats.playtime);
        println!("{0: <35} {1}", "Deaths:", &self.stats.deaths);
        println!("{0: <35} {1}", "Hero kills:", &self.stats.hero_kills);
        println!(
            "{0: <35} {1}",
            "Champion kills:", &self.stats.champion_kills
        );
        println!("{0: <35} {1}", "Kills:", &self.stats.kills);

        println!("{:=^50}", " Skills ");
        println!(
            "{0: <35} {1}",
            "Skill reclamation points used:", &self.skills.skill_reclamation_points_used
        );
        println!(
            "{0: <35} {1}",
            "Devotion reclamation points used:", &self.skills.devotion_reclamation_points_used
        );
        println!("{:=^50}", " End ");
        println!();
    }

    pub fn get_name(&mut self, path: &PathBuf) -> Result<String> {
        let mut f = GDFile::new(File::open(path)?);

        f.validate()?;
        self.header.read(&mut f)?;

        Ok(self.header.name.to_string())
    }

    pub fn write(&mut self, path: &PathBuf) -> Result<()> {
        let mut f = GDFile::new(File::create(path)?);

        f.write_int(1431655765)?;
        f.write_int(1480803399)?;
        self.header.write(&mut f)?;
        f.write_int(0)?;
        f.write_int(self.version)?;
        self.uid.write(&mut f)?;
        self.info.write(&mut f)?;
        self.bio.write(&mut f)?;
        self.inventory.write(&mut f)?;
        self.stash.write(&mut f)?;
        self.respawns.write(&mut f)?;
        self.teleports.write(&mut f)?;
        self.markers.write(&mut f)?;
        self.shrines.write(&mut f)?;
        self.skills.write(&mut f)?;
        self.notes.write(&mut f)?;
        self.factions.write(&mut f)?;
        self.ui.write(&mut f)?;
        self.tutorials.write(&mut f)?;
        self.stats.write(&mut f)?;
        if self.version >= 7 {
            self.crucible.write(&mut f)?;
        }

        Ok(())
    }

    pub fn read(&mut self, path: &PathBuf) -> Result<&mut Self> {
        let mut f = GDFile::new(std::fs::File::open(path)?);

        f.validate()?;
        self.header.read(&mut f)?;

        if f.next_int()? != 0 {
            bail!("next_int() != 0");
        }

        self.version = f.read_version(&self.supported_versions)?;
        self.uid.read(&mut f)?;
        self.info.read(&mut f)?;
        self.bio.read(&mut f)?;
        self.inventory.read(&mut f)?;
        self.stash.read(&mut f)?;

        self.respawns.read(&mut f)?;
        self.teleports.read(&mut f)?;
        self.markers.read(&mut f)?;
        self.shrines.read(&mut f)?;

        self.skills.read(&mut f)?;
        self.notes.read(&mut f)?;
        self.factions.read(&mut f)?;
        self.ui.read(&mut f)?;
        self.tutorials.read(&mut f)?;
        self.stats.read(&mut f)?;
        if self.version >= 7 {
            self.crucible.read(&mut f)?;
        }

        Ok(self)
    }
}

#[derive(Default, Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum Sex {
    #[default]
    Female,
    Male,
}

impl TryFrom<u8> for Sex {
    type Error = Error;
    fn try_from(t: u8) -> Result<Self, Self::Error> {
        match t {
            0 => Ok(Self::Female),
            1 => Ok(Self::Male),
            _ => bail!(ParseError::ParseSexError(t)),
        }
    }
}

#[derive(Default, Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum ExpansionStatus {
    #[default]
    Vanilla,
    Crucible,
    ForgottenGods,
    AshesOfMalmouth,
}

impl TryFrom<u8> for ExpansionStatus {
    type Error = Error;
    fn try_from(t: u8) -> Result<Self, Self::Error> {
        match t {
            0 => Ok(Self::Vanilla),
            1 => Ok(Self::AshesOfMalmouth),
            2 => Ok(Self::Crucible),
            3 => Ok(Self::ForgottenGods),
            _ => bail!(ParseError::ParseExpansionStatusError(t)),
        }
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub sex: Sex,
    pub hardcore: u8,
    pub level: u32,
    pub expansion_status: ExpansionStatus,
    version: u32,
    class_id: String,
    #[default(_code = "vec![1, 2]")]
    supported_versions: Vec<u32>,
}

impl Header {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_int(self.version)?;
        f.write_wstring(&self.name)?;
        f.write_byte(self.sex as u8)?;
        f.write_string(&self.class_id)?;
        f.write_int(self.level)?;
        f.write_byte(self.hardcore)?;
        if self.version >= 2 {
            f.write_byte(self.expansion_status as u8)?;
        }

        Ok(())
    }

    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.version = f.read_version(&self.supported_versions)?;
        self.name = f.read_wstring()?;
        self.sex = f.read_byte()?.try_into()?;
        self.class_id = f.read_string()?;
        self.level = f.read_int()?;
        self.hardcore = f.read_byte()?;

        if self.version >= 2 {
            self.expansion_status = f.read_byte()?.try_into()?;
        }

        Ok(())
    }
}
