use crate::gd::gd_file::{Block, FileError, GDReader, GDWriter};
use anyhow::{bail, Result};
use strum_macros::Display;

#[derive(Default, Debug, Display, Clone, PartialEq, Eq)]
pub enum Difficulty {
    #[default]
    Normal,
    Elite,
    Ultimate,
}

impl From<u8> for Difficulty {
    fn from(t: u8) -> Self {
        match t {
            1 => Self::Elite,
            2 => Self::Ultimate,
            _ => Self::Normal,
        }
    }
}

impl From<Difficulty> for u8 {
    fn from(t: Difficulty) -> u8 {
        match t {
            Difficulty::Normal => 0,
            Difficulty::Elite => 1,
            Difficulty::Ultimate => 2,
        }
    }
}

#[derive(Default, Debug, Display, Clone, PartialEq, Eq)]
pub enum CrucibleDifficulty {
    #[default]
    Aspirant,
    Challenger,
    Gladiator,
}

impl From<u8> for CrucibleDifficulty {
    fn from(t: u8) -> Self {
        match t {
            1 => Self::Challenger,
            2 => Self::Gladiator,
            _ => Self::Aspirant,
        }
    }
}

impl From<CrucibleDifficulty> for u8 {
    fn from(t: CrucibleDifficulty) -> u8 {
        match t {
            CrucibleDifficulty::Aspirant => 0,
            CrucibleDifficulty::Challenger => 1,
            CrucibleDifficulty::Gladiator => 2,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub greatest_difficulty: Difficulty,
    pub greatest_crucible_difficulty: CrucibleDifficulty,
    pub money: u32,
    difficulty: Difficulty,
    alternate_config: u8,
    alternate_config_enabled: u8,
    compass_state: u8,
    current_tribute: u32,
    has_been_in_game: u8,
    is_in_main_quest: u8,
    loot_filters: Vec<u8>,
    loot_mode: u32,
    skill_window_show_help: u8,
    texture: String,
    version: u32,
}

impl Info {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, 1)?;

        f.write_int(self.version)?;

        f.write_byte(self.is_in_main_quest)?;
        f.write_byte(self.has_been_in_game)?;
        f.write_byte(self.difficulty.clone().into())?;
        f.write_byte(self.greatest_difficulty.clone().into())?;
        f.write_int(self.money)?;

        if self.version >= 4 {
            f.write_byte(self.greatest_crucible_difficulty.clone().into())?;
            f.write_int(self.current_tribute)?;
        }

        f.write_byte(self.compass_state)?;

        if self.version >= 2 && self.version <= 4 {
            f.write_int(self.loot_mode)?;
        }

        f.write_byte(self.skill_window_show_help)?;
        f.write_byte(self.alternate_config)?;
        f.write_byte(self.alternate_config_enabled)?;
        f.write_string(&self.texture)?;

        if self.version >= 5 {
            f.write_int(self.loot_filters.len() as u32)?;

            for i in self.loot_filters.iter() {
                f.write_byte(*i)?;
            }
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, 1)?;

        self.version = f.read_int()?;
        if !(3..=5).contains(&self.version) {
            bail!(FileError::UnsupportedVersion(
                self.version,
                "3..=5".to_string()
            ));
        }

        self.is_in_main_quest = f.read_byte()?;
        self.has_been_in_game = f.read_byte()?;
        self.difficulty = f.read_byte()?.into();
        self.greatest_difficulty = f.read_byte()?.into();
        self.money = f.read_int()?;
        if self.version >= 4 {
            self.greatest_crucible_difficulty = f.read_byte()?.into();
            self.current_tribute = f.read_int()?;
        }
        self.compass_state = f.read_byte()?;
        if self.version >= 2 && self.version <= 4 {
            self.loot_mode = f.read_int()?;
        }
        self.skill_window_show_help = f.read_byte()?;
        self.alternate_config = f.read_byte()?;
        self.alternate_config_enabled = f.read_byte()?;
        self.texture = f.read_string()?;

        if self.version >= 5 {
            let size = f.read_int()?;
            self.loot_filters = vec![0; size as usize];

            for i in 0..self.loot_filters.len() {
                self.loot_filters[i] = f.read_byte()?;
            }
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bio {
    pub experience: u32,
    pub skill_points: u32,
    pub physique: f32,
    pub cunning: f32,
    pub spirit: f32,
    pub attribute_points: u32,
    pub devotion_points: u32,
    pub health: f32,
    pub energy: f32,
    pub total_devotion: u32,
    level: u32,
    version: u32,
}

impl Bio {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, 2)?;

        f.write_int(8)?;
        f.write_int(self.level)?;
        f.write_int(self.experience)?;
        f.write_int(self.attribute_points)?;
        f.write_int(self.skill_points)?;
        f.write_int(self.devotion_points)?;
        f.write_int(self.total_devotion)?;
        f.write_float(self.physique)?;
        f.write_float(self.cunning)?;
        f.write_float(self.spirit)?;
        f.write_float(self.health)?;
        f.write_float(self.energy)?;

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();

        f.read_block_start(&mut b, 2)?;

        self.version = f.read_int()?;
        if self.version != 8 {
            bail!(FileError::UnsupportedVersion(self.version, "8".to_string()));
        }

        self.level = f.read_int()?;
        self.experience = f.read_int()?;
        self.attribute_points = f.read_int()?;
        self.skill_points = f.read_int()?;
        self.devotion_points = f.read_int()?;
        self.total_devotion = f.read_int()?;
        self.physique = f.read_float()?;
        self.cunning = f.read_float()?;
        self.spirit = f.read_float()?;
        self.health = f.read_float()?;
        self.energy = f.read_float()?;

        f.read_block_end(&mut b)
    }
}
