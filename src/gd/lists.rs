use crate::gd::gd_file::{Block, GDReader, GDWriter, ReadWrite};

use anyhow::{Context, Ok, Result};
use smart_default::SmartDefault;
use std::ops::{Deref, DerefMut};

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct TeleportList {
    version: u32,
    uids: [Vec<CharUID>; 3],
    #[default = 6]
    block_seq: u32,
    #[default(_code = "vec![1]")]
    supported_versions: Vec<u32>,
}

impl TeleportList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in teleport list")?;

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }

        f.read_block_end(&mut b)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct RespawnList {
    version: u32,
    uids: [Vec<CharUID>; 3],
    spawns: [CharUID; 3],
    #[default = 5]
    block_seq: u32,
    #[default(_code = "vec![1]")]
    supported_versions: Vec<u32>,
}

impl RespawnList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        for i in self.spawns.iter() {
            i.write(f)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in respawn list")?;

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }

        for i in self.spawns.iter_mut() {
            i.read(f)?;
        }

        f.read_block_end(&mut b)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct MarkerList {
    version: u32,
    uids: [Vec<CharUID>; 3],
    #[default = 7]
    block_seq: u32,
    #[default(_code = "vec![1]")]
    supported_versions: Vec<u32>,
}

impl MarkerList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in marker list")?;

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }
        f.read_block_end(&mut b)
    }
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct ShrineList {
    version: u32,
    uids: [Vec<CharUID>; 6],
    #[default = 17]
    block_seq: u32,
    #[default(_code = "vec![2]")]
    supported_versions: Vec<u32>,
}

impl ShrineList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, self.block_seq)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, self.block_seq)?;

        self.version = f
            .read_version(&self.supported_versions)
            .context("in shrine list")?;

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CharUID([u8; 16]);

impl std::fmt::Display for CharUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.map(|v| v.to_string()).join(""))
    }
}

impl Deref for CharUID {
    type Target = [u8; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharUID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ReadWrite for CharUID {
    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        for i in 0..16 {
            self[i] = f.read_byte()?;
        }
        Ok(())
    }

    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        for i in self.iter() {
            f.write_byte(*i)?;
        }
        Ok(())
    }
}
