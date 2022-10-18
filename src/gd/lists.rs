use crate::gd::gd_file::{Block, FileError, GDReader, GDWriter, ReadWrite};

use anyhow::{bail, Ok, Result};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TeleportList {
    version: u32,
    uids: [Vec<CharUID>; 3],
}

impl TeleportList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, 6)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, 6)?;

        self.version = f.read_int()?;
        if self.version != 1 {
            bail!(FileError::UnsupportedVersion(self.version, "1".to_string()));
        }

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RespawnList {
    version: u32,
    uids: [Vec<CharUID>; 3],
    spawns: [CharUID; 3],
}

impl RespawnList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, 5)?;
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
        f.read_block_start(&mut b, 5)?;

        self.version = f.read_int()?;
        if self.version != 1 {
            bail!(FileError::UnsupportedVersion(self.version, "1".to_string()));
        }

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }

        for i in self.spawns.iter_mut() {
            i.read(f)?;
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MarkerList {
    version: u32,
    uids: [Vec<CharUID>; 3],
}

impl MarkerList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, 7)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, 7)?;

        self.version = f.read_int()?;
        if self.version != 1 {
            bail!(FileError::UnsupportedVersion(self.version, "1".to_string()));
        }
        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }
        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ShrineList {
    version: u32,
    uids: [Vec<CharUID>; 6],
}

impl ShrineList {
    pub fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        let mut b = Block::default();
        f.write_block_start(&mut b, 17)?;
        f.write_int(self.version)?;

        for i in self.uids.iter() {
            f.write_vec(i)?;
        }

        f.write_block_end(&mut b)
    }

    pub fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        let mut b = Block::default();
        f.read_block_start(&mut b, 17)?;

        self.version = f.read_int()?;
        if self.version != 2 {
            bail!(FileError::UnsupportedVersion(self.version, "2".to_string()));
        }

        for i in self.uids.iter_mut() {
            i.extend(f.read_vec()?);
        }

        f.read_block_end(&mut b)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CharUID([u8; 16]);

impl ReadWrite for CharUID {
    fn read(&mut self, f: &mut impl GDReader) -> Result<()> {
        for i in 0..16 {
            self.0[i] = f.read_byte()?
        }
        Ok(())
    }

    fn write(&self, f: &mut impl GDWriter) -> Result<()> {
        for i in self.0.iter() {
            f.write_byte(*i)?;
        }
        Ok(())
    }
}
