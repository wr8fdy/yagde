use crate::gd::gd_file::{Block, GDReader, GDWriter, ReadWrite};

use anyhow::{Ok, Result};
use smart_default::SmartDefault;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct ItemSkill {
    name: String,
    auto_cast_skill: String,
    auto_cast_controller: String,
    item_slot: u32,
    item_id: String,
}

impl ReadWrite for ItemSkill {
    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_string(&self.name)?;
        f.write_string(&self.auto_cast_skill)?;
        f.write_string(&self.auto_cast_controller)?;
        f.write_int(self.item_slot)?;
        f.write_string(&self.item_id)
    }

    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.name = f.read_string()?;
        self.auto_cast_skill = f.read_string()?;
        self.auto_cast_controller = f.read_string()?;
        self.item_slot = f.read_int()?;
        self.item_id = f.read_string()?;

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub enabled: u8,
    pub level: u32,
    pub devotion_level: u32,
    experience: u32,
    active: u32,
    unknown1: u8,
    unknown2: u8,
    pub auto_cast_skill: String,
    pub auto_cast_controller: String,
}

impl ReadWrite for Skill {
    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_string(&self.name)?;
        f.write_int(self.level)?;
        f.write_byte(self.enabled)?;
        f.write_int(self.devotion_level)?;
        f.write_int(self.experience)?;
        f.write_int(self.active)?;
        f.write_byte(self.unknown1)?;
        f.write_byte(self.unknown2)?;
        f.write_string(&self.auto_cast_skill)?;
        f.write_string(&self.auto_cast_controller)
    }

    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.name = f.read_string()?;
        self.level = f.read_int()?;
        self.enabled = f.read_byte()?;
        self.devotion_level = f.read_int()?;
        self.experience = f.read_int()?;
        self.active = f.read_int()?;
        self.unknown1 = f.read_byte()?;
        self.unknown2 = f.read_byte()?;
        self.auto_cast_skill = f.read_string()?;
        self.auto_cast_controller = f.read_string()?;

        Ok(())
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct SkillList {
    pub skill_reclamation_points_used: u32,
    pub devotion_reclamation_points_used: u32,
    pub skills: Vec<Skill>,
    version: u32,
    item_skills: Vec<ItemSkill>,
    masteries_allowed: u32,
    #[default = 8]
    block_seq: u32,
    #[default(_code = "vec![5]")]
    supported_versions: Vec<u32>,
}

impl SkillList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;

        f.write_int(self.version)?;
        f.write_vec(&self.skills)?;
        f.write_int(self.masteries_allowed)?;
        f.write_int(self.skill_reclamation_points_used)?;
        f.write_int(self.devotion_reclamation_points_used)?;
        f.write_vec(&self.item_skills)?;

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f.read_version(&self.supported_versions)?;
        self.skills = f.read_vec()?;
        self.masteries_allowed = f.read_int()?;
        self.skill_reclamation_points_used = f.read_int()?;
        self.devotion_reclamation_points_used = f.read_int()?;
        self.item_skills = f.read_vec()?;

        f.read_block_end(&mut b)
    }
}
