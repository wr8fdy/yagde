use std::vec;

use crate::gd::gd_file::{Block, GDReader, GDWriter, ReadWrite};

use anyhow::{Context, Ok, Result};
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct Crucible {
    version: u32,
    pub tokens_per_difficulty: [Vec<String>; 3],
    #[default = 10]
    block_seq: u32,
    #[default(_code = "vec![2]")]
    supported_versions: Vec<u32>,
}

impl Crucible {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        for t in self.tokens_per_difficulty.iter() {
            f.write_int(t.len() as u32)?;

            for s in t.iter() {
                f.write_string(s)?;
            }
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in crucible")?;

        for i in 0..self.tokens_per_difficulty.len() {
            for _ in 0..f.read_int()? {
                let s = f.read_string()?;

                if let Some(data) = self.tokens_per_difficulty.get_mut(i) {
                    if data.contains(&s) {
                        data.push(s);
                    }
                }
            }
        }

        f.read_block_end(&mut b)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct TutorialPages {
    version: u32,
    pages: Vec<u32>,
    #[default = 15]
    block_seq: u32,
    #[default(_code = "vec![1]")]
    supported_versions: Vec<u32>,
}

impl TutorialPages {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        f.write_int(self.pages.len() as u32)?;
        for p in self.pages.iter() {
            f.write_int(*p)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in tutorial pages")?;
        let n = f.read_int()? as usize;
        self.pages = Vec::with_capacity(n);
        for _ in 0..n {
            let page = f.read_int()?;
            self.pages.push(page);
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UISkillSet {
    id: u32,
    slots: Vec<UISlot>,
}

impl UISkillSet {
    pub fn with_slots(cap: usize) -> Self {
        Self {
            id: 0,
            slots: Vec::with_capacity(cap),
        }
    }

    pub fn write(&self, version: u32, f: &mut impl GDWriter) -> Result<()> {
        if version >= 7 {
            f.write_int(self.id)?;
        }

        for s in self.slots.iter() {
            s.write(f)?;
        }

        Ok(())
    }

    pub fn read(&mut self, version: u32, f: &mut impl GDReader) -> Result<()> {
        if version >= 7 {
            self.id = f.read_int()?;
        }

        for _ in 0..self.slots.capacity() {
            let mut slot = UISlot::default();
            slot.read(f)?;
            self.slots.push(slot);
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UISlot {
    bitmap_down: String,
    bitmap_up: String,
    equip_location: u32,
    is_item_skill: u8,
    item: String,
    label: String,
    skill: String,
    slot_type: u32,
}

impl UISlot {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_int(self.slot_type)?;

        if self.slot_type == 0 {
            f.write_string(&self.skill)?;
            f.write_byte(self.is_item_skill)?;
            f.write_string(&self.item)?;
            f.write_int(self.equip_location)?;
        } else if self.slot_type == 4 {
            f.write_string(&self.item)?;
            f.write_string(&self.bitmap_up)?;
            f.write_string(&self.bitmap_down)?;
            f.write_wstring(&self.label)?;
        }

        Ok(())
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.slot_type = f.read_int()?;

        if self.slot_type == 0 {
            self.skill = f.read_string()?;
            self.is_item_skill = f.read_byte()?;
            self.item = f.read_string()?;
            self.equip_location = f.read_int()?;
        } else if self.slot_type == 4 {
            self.item = f.read_string()?;
            self.bitmap_up = f.read_string()?;
            self.bitmap_down = f.read_string()?;
            self.label = f.read_wstring()?;
        }

        Ok(())
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct UI {
    version: u32,
    unknown1: u8,
    unknown2: u32,
    unknown3: u8,
    unknown4: [String; 5],
    unknown5: [String; 5],
    unknown6: [u8; 5],
    camera_distance: f32,
    num_skill_set: u32,
    slots_per_set: u32,
    skill_sets: Vec<UISkillSet>,
    #[default = 14]
    block_seq: u32,
    #[default(_code = "vec![4, 5, 6, 7]")]
    supported_versions: Vec<u32>,
}

impl UI {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        f.write_byte(self.unknown1)?;
        f.write_int(self.unknown2)?;
        f.write_byte(self.unknown3)?;

        for i in 0..self.unknown4.len() {
            f.write_string(&self.unknown4[i])?;
            f.write_string(&self.unknown5[i])?;
            f.write_byte(self.unknown6[i])?;
        }

        if self.version >= 7 {
            f.write_int(self.num_skill_set)?;
            f.write_int(self.slots_per_set)?;
        }

        for s in self.skill_sets.iter() {
            s.write(self.version, f)?;
        }

        f.write_float(self.camera_distance)?;

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f.read_version(&self.supported_versions).context("in UI")?;
        self.unknown1 = f.read_byte()?;
        self.unknown2 = f.read_int()?;
        self.unknown3 = f.read_byte()?;

        for i in 0..self.unknown4.len() {
            self.unknown4[i] = f.read_string()?;
            self.unknown5[i] = f.read_string()?;
            self.unknown6[i] = f.read_byte()?;
        }

        if self.version >= 7 {
            self.num_skill_set = f.read_int()?;
            self.slots_per_set = f.read_int()?;
        } else {
            self.num_skill_set = 1;
            if self.version >= 6 {
                self.slots_per_set = 47;
            } else if self.version == 5 {
                self.slots_per_set = 46;
            } else {
                self.slots_per_set = 36;
            }
        }

        for _ in 0..self.num_skill_set {
            let mut skill_set = UISkillSet::with_slots(self.slots_per_set as usize);
            skill_set.read(self.version, f)?;
            self.skill_sets.push(skill_set);
        }

        self.camera_distance = f.read_float()?;

        f.read_block_end(&mut b)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct FactionList {
    pub factions: Vec<Faction>,
    version: u32,
    faction: u32,
    #[default = 13]
    block_seq: u32,
    #[default(_code = "vec![5]")]
    supported_versions: Vec<u32>,
}

impl FactionList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;

        f.write_int(self.version)?;
        f.write_int(self.faction)?;
        f.write_vec(&self.factions)?;

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in faction list")?;
        self.faction = f.read_int()?;
        self.factions = f.read_vec()?;

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Faction {
    pub value: f32,
    modified: u8,
    unlocked: u8,
    positive_boost: f32,
    negative_boost: f32,
}

impl ReadWrite for Faction {
    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_byte(self.modified)?;
        f.write_byte(self.unlocked)?;
        f.write_float(self.value)?;
        f.write_float(self.positive_boost)?;
        f.write_float(self.negative_boost)
    }

    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.modified = f.read_byte()?;
        self.unlocked = f.read_byte()?;
        self.value = f.read_float()?;
        self.positive_boost = f.read_float()?;
        self.negative_boost = f.read_float()?;

        Ok(())
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct NoteList {
    version: u32,
    notes: Vec<String>,
    #[default = 12]
    block_seq: u32,
    #[default(_code = "vec![1]")]
    supported_versions: Vec<u32>,
}

impl NoteList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        f.write_int(self.notes.len() as u32)?;
        for s in self.notes.iter() {
            f.write_string(s)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in note list")?;
        let n = f.read_int()?;
        for _ in 0..n {
            let note = f.read_string()?;
            self.notes.push(note);
        }

        f.read_block_end(&mut b)
    }
}
