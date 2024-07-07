use crate::gd::gd_file::{Block, GDReader, GDWriter, ReadWrite};
use crate::gd::item::Item;

use anyhow::{Context, Ok, Result};
use smart_default::SmartDefault;

#[derive(Default, Debug, Clone, PartialEq)]
struct StashItem {
    x: f32,
    y: f32,
    item: Item,
}

impl ReadWrite for StashItem {
    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        self.item.write(f)?;
        f.write_float(self.x)?;
        f.write_float(self.y)
    }

    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.item.read(f)?;
        self.x = f.read_float()?;
        self.y = f.read_float()?;
        Ok(())
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
struct StashPage {
    width: u32,
    height: u32,
    version: u32,
    items: Vec<StashItem>,
    #[default = 0]
    block_seq: u32,
}

impl StashPage {
    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        if self.version >= 6 {
            f.write_block_start(&mut b, self.block_seq)?;
        }

        f.write_int(self.width)?;
        f.write_int(self.height)?;
        f.write_vec(&self.items)?;

        if self.version >= 6 {
            f.write_block_end(&mut b)?;
        }

        Ok(())
    }

    fn read(&mut self, version: u32, f: &mut impl GDReader) -> Result<()> {
        self.version = version;

        let mut b = Block::default();
        if self.version >= 6 {
            f.read_block_start(&mut b, self.block_seq)?;
        }

        self.width = f.read_int()?;
        self.height = f.read_int()?;
        self.items = f.read_vec()?;

        if self.version >= 6 {
            f.read_block_end(&mut b)?;
        }

        Ok(())
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct Stash {
    pages: Vec<StashPage>,
    version: u32,
    num_pages: usize,
    #[default = 4]
    block_seq: u32,
    #[default(_code = "vec![5, 6]")]
    supported_versions: Vec<u32>,
}

impl Stash {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;

        f.write_int(self.version)?;

        if self.version >= 6 {
            f.write_int(self.num_pages as u32)?;
        }

        for page in self.pages.iter() {
            page.write(f)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in stash")?;
        self.num_pages = 1;
        if self.version >= 6 {
            self.num_pages = f.read_int()? as usize;
        }
        self.pages = Vec::with_capacity(self.num_pages);

        for _ in 0..self.num_pages {
            let mut page = StashPage::default();
            page.read(self.version, f)?;
            self.pages.push(page);
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct InventoryItem {
    x: u32,
    y: u32,
    item: Item,
}

impl ReadWrite for InventoryItem {
    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.item.read(f)?;
        self.x = f.read_int()?;
        self.y = f.read_int()?;
        Ok(())
    }

    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        self.item.write(f)?;
        f.write_int(self.x)?;
        f.write_int(self.y)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
struct InventorySack {
    items: Vec<InventoryItem>,
    temp_bool: u8,
    #[default = 0]
    block_seq: u32,
}

impl ReadWrite for InventorySack {
    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.temp_bool = f.read_byte()?;
        self.items = f.read_vec()?;

        f.read_block_end(&mut b)
    }

    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;

        f.write_byte(self.temp_bool)?;
        f.write_vec(&self.items)?;

        f.write_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct InventoryEquipment {
    attached: u8,
    item: Item,
}

impl InventoryEquipment {
    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.item.read(f)?;
        self.attached = f.read_byte()?;
        Ok(())
    }
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        self.item.write(f)?;
        f.write_byte(self.attached)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    version: u32,
    flag: u8,
    focused: u32,
    selected: u32,
    sacks: Vec<InventorySack>,
    use_alternate: u8,
    alternate1: u8,
    alternate2: u8,
    equipment: [InventoryEquipment; 12],
    weapon1: [InventoryEquipment; 2],
    weapon2: [InventoryEquipment; 2],
    #[default = 3]
    block_seq: u32,
    #[default(_code = "vec![4, 5]")]
    supported_versions: Vec<u32>,
}

impl Inventory {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;
        f.write_byte(self.flag)?;

        if self.flag != 0 {
            f.write_int(self.sacks.len() as u32)?;
            f.write_int(self.focused)?;
            f.write_int(self.selected)?;
            f.write_arr(&self.sacks)?;
            f.write_byte(self.use_alternate)?;

            for e in self.equipment.iter() {
                e.write(f)?;
            }

            f.write_byte(self.alternate1)?;
            for w in self.weapon1.iter() {
                w.write(f)?;
            }
            f.write_byte(self.alternate2)?;
            for w in self.weapon2.iter() {
                w.write(f)?;
            }
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in inventory")?;
        self.flag = f.read_byte()?;

        if self.flag != 0 {
            let n = f.read_int()? as usize;
            self.focused = f.read_int()?;
            self.selected = f.read_int()?;
            self.sacks = f.read_arr(n)?;

            self.use_alternate = f.read_byte()?;
            for i in 0..12 {
                self.equipment[i].read(f)?;
            }

            self.alternate1 = f.read_byte()?;
            for i in 0..2 {
                self.weapon1[i].read(f)?;
            }

            self.alternate2 = f.read_byte()?;
            for i in 0..2 {
                self.weapon2[i].read(f)?;
            }
        }
        f.read_block_end(&mut b)
    }
}
