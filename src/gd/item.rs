use crate::gd::gd_file::{GDReader, GDWriter};

use anyhow::{Ok, Result};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Item {
    id: String,
    prefix_id: String,
    suffix_id: String,
    modifier_id: String,
    transmute_id: String,
    seed: u32,
    component_id: String,
    component_bonus: String,
    component_seed: u32,
    augment_id: String,
    augment_seed: u32,
    unknown: u32,
    var1: u32,
    stack_count: u32,
    container_type: u32,
}

impl Item {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        f.write_string(&self.id)?;
        f.write_string(&self.prefix_id)?;
        f.write_string(&self.suffix_id)?;
        f.write_string(&self.modifier_id)?;
        f.write_string(&self.transmute_id)?;
        f.write_int(self.seed)?;
        f.write_string(&self.component_id)?;
        f.write_string(&self.component_bonus)?;
        f.write_int(self.component_seed)?;
        f.write_string(&self.augment_id)?;
        f.write_int(self.unknown)?;
        f.write_int(self.augment_seed)?;
        f.write_int(self.var1)?;
        f.write_int(self.stack_count)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        self.id = f.read_string()?;
        self.prefix_id = f.read_string()?;
        self.suffix_id = f.read_string()?;
        self.modifier_id = f.read_string()?;
        self.transmute_id = f.read_string()?;
        self.seed = f.read_int()?;
        self.component_id = f.read_string()?;
        self.component_bonus = f.read_string()?;
        self.component_seed = f.read_int()?;
        self.augment_id = f.read_string()?;
        self.unknown = f.read_int()?;
        self.augment_seed = f.read_int()?;
        self.var1 = f.read_int()?;
        self.stack_count = f.read_int()?;
        Ok(())
    }
}
