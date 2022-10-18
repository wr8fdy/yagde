use anyhow::{bail, Ok, Result};
use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use thiserror::Error;

pub trait ReadWrite {
    fn read(&mut self, f: &mut impl GDReader) -> Result<()>;
    fn write(&self, f: &mut impl GDWriter) -> Result<()>;
}

pub trait GDReader {
    fn read_vec<T: ReadWrite + Default>(&mut self) -> Result<Vec<T>>;
    fn read_arr<T: ReadWrite + Default>(&mut self, n: usize) -> Result<Vec<T>>;
    fn validate(&mut self) -> Result<()>;
    fn read_string(&mut self) -> Result<String>;
    fn read_wstring(&mut self) -> Result<String>;
    fn read_byte(&mut self) -> Result<u8>;
    fn read_key(&mut self) -> Result<()>;
    fn read_float(&mut self) -> Result<f32>;
    fn read_int(&mut self) -> Result<u32>;
    fn next_int(&mut self) -> Result<u32>;
    fn read_block_start(&mut self, b: &mut Block, n: u32) -> Result<()>;
    fn read_block_end(&mut self, b: &mut Block) -> Result<()>;
}

pub trait GDWriter {
    fn write_vec<T: ReadWrite + Default>(&mut self, items: &[T]) -> Result<()>;
    fn write_arr<T: ReadWrite + Default>(&mut self, items: &[T]) -> Result<()>;
    fn write_byte(&mut self, v: u8) -> Result<()>;
    fn write_string(&mut self, s: &str) -> Result<()>;
    fn write_wstring(&mut self, s: &str) -> Result<()>;
    fn write_float(&mut self, v: f32) -> Result<()>;
    fn write_int(&mut self, v: u32) -> Result<()>;
    fn write_block_start(&mut self, b: &mut Block, n: u32) -> Result<()>;
    fn write_block_end(&mut self, b: &mut Block) -> Result<()>;
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Unsupported version: {0}, expected {1}")]
    UnsupportedVersion(u32, String),
    #[error("Incorrect block end position: {0}, expected {1}")]
    IncorrectBlockEndPosition(u64, u64),
    #[error("Failed to validate block ending: {0}, expected {1}")]
    FailedToValidateBlockEnding(u32, u32),
    #[error("Failed to validate block order: {0}, expected {1}")]
    FailedToValidateBlockOrder(u32, u32),
    #[error("Failed to validate write amount: {0}, expected {1}")]
    FailedToValidateWriteAmount(usize, u32),
}

#[derive(Default, Debug)]
pub struct Block {
    len: u32,
    end: u64,
}

pub struct GDFile {
    f: File,
    key: u32,
    table: [u32; 256],
}

impl GDFile {
    pub fn new(f: File) -> Self {
        Self {
            f,
            key: 0,
            table: [0; 256],
        }
    }

    fn update_key(&mut self, val: Vec<u8>) {
        for i in val {
            self.key ^= self.table[i as usize];
        }
    }
}

impl GDReader for GDFile {
    fn read_vec<T: ReadWrite + Default>(&mut self) -> Result<Vec<T>> {
        let n = self.read_int()? as usize;
        self.read_arr(n)
    }

    fn read_arr<T: ReadWrite + Default>(&mut self, n: usize) -> Result<Vec<T>> {
        let mut items: Vec<T> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut i = T::default();
            i.read(self)?;
            items.push(i);
        }
        Ok(items)
    }

    fn validate(&mut self) -> Result<()> {
        self.f.seek(SeekFrom::End(0))?;
        let _end = self.f.stream_position()?;
        self.f.seek(SeekFrom::Start(0))?;

        self.read_key()?;

        let ret = self.read_int()?;
        if ret != 0x58434447 {
            bail!(FileError::UnsupportedVersion(ret, "0x58434447".to_string()));
        };
        Ok(())
    }

    fn read_string(&mut self) -> Result<String> {
        let len = self.read_int()?;

        let mut s = String::new();
        s.reserve(len as usize);

        for _ in 0..len {
            s.push(self.read_byte()?.into());
        }
        Ok(s)
    }

    fn read_wstring(&mut self) -> Result<String> {
        let n = self.read_int()? as usize;

        let mut s = String::new();
        s.reserve(n);

        for _ in 0..n {
            let mut c = self.read_byte()?;
            c |= self.read_byte()?.wrapping_shl(8);
            s.push(c.into());
        }
        Ok(s)
    }

    fn read_byte(&mut self) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        self.f.read_exact(&mut buf)?;
        let val = u8::from_ne_bytes(buf);
        let ret = val ^ self.key as u8;
        self.update_key(buf.to_vec());

        Ok(ret)
    }

    fn read_key(&mut self) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        self.f.read_exact(&mut buf)?;
        let mut k = u32::from_ne_bytes(buf);
        k ^= 1431655765_u32;
        self.key = k;

        for i in 0..256 {
            k = k >> 1 | k << 31;
            k = k.wrapping_mul(39916801_u32);
            self.table[i] = k;
        }

        Ok(())
    }

    fn read_float(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_int()?))
    }

    fn read_int(&mut self) -> Result<u32> {
        let mut buf: [u8; 4] = [0; 4];
        self.f.read_exact(&mut buf)?;
        let val = u32::from_ne_bytes(buf);
        let ret = val ^ self.key;
        self.update_key(buf.to_vec());

        Ok(ret)
    }

    fn next_int(&mut self) -> Result<u32> {
        let mut buf: [u8; 4] = [0; 4];
        self.f.read_exact(&mut buf)?;
        let val = u32::from_ne_bytes(buf);
        Ok(val ^ self.key)
    }

    fn read_block_start(&mut self, b: &mut Block, n: u32) -> Result<()> {
        let ret = self.read_int()?;
        b.len = self.next_int()?;
        b.end = self.f.stream_position()? + b.len as u64;

        if ret != n {
            bail!(FileError::FailedToValidateBlockOrder(ret, n))
        }
        Ok(())
    }

    fn read_block_end(&mut self, b: &mut Block) -> Result<()> {
        if b.end != self.f.stream_position()? {
            bail!(FileError::IncorrectBlockEndPosition(
                self.f.stream_position()?,
                b.end
            ));
        }

        let ni = self.next_int()?;
        if ni != 0 {
            bail!(FileError::FailedToValidateBlockEnding(ni, 0));
        }

        Ok(())
    }
}

impl GDWriter for GDFile {
    fn write_vec<T: ReadWrite + Default>(&mut self, items: &[T]) -> Result<()> {
        self.write_int(items.len() as u32)?;
        self.write_arr(items)
    }

    fn write_arr<T: ReadWrite + Default>(&mut self, items: &[T]) -> Result<()> {
        for i in items {
            i.write(self)?;
        }
        Ok(())
    }

    fn write_string(&mut self, s: &str) -> Result<()> {
        self.write_int(s.len() as u32)?;
        for c in s.as_bytes() {
            self.write_byte(*c)?;
        }
        Ok(())
    }

    fn write_wstring(&mut self, s: &str) -> Result<()> {
        self.write_int(s.chars().count() as u32)?;
        let mut b = [0; 2];
        for c in s.chars() {
            c.encode_utf8(&mut b);
            self.write_byte(b[0])?;
            self.write_byte(b[1])?;
        }
        Ok(())
    }

    fn write_byte(&mut self, v: u8) -> Result<()> {
        let ret = self.f.write(&[v; 1])?;
        if ret != 1 {
            bail!(FileError::FailedToValidateWriteAmount(ret, 1))
        }
        Ok(())
    }

    fn write_float(&mut self, v: f32) -> Result<()> {
        self.write_int(f32::to_bits(v))
    }

    fn write_int(&mut self, v: u32) -> Result<()> {
        let buf: [u8; 4] = u32::to_ne_bytes(v);
        let ret = self.f.write(&buf)?;
        if ret != 4 {
            bail!(FileError::FailedToValidateWriteAmount(ret, 4))
        }
        Ok(())
    }

    fn write_block_start(&mut self, b: &mut Block, n: u32) -> Result<()> {
        self.write_int(n)?;
        self.write_int(0)?;

        b.end = self.f.stream_position()?;

        Ok(())
    }

    fn write_block_end(&mut self, b: &mut Block) -> Result<()> {
        let pos = self.f.stream_position()?;

        self.f.seek(SeekFrom::Start(b.end - 4))?;
        self.write_int((pos - b.end).try_into()?)?;
        self.f.seek(SeekFrom::Start(pos))?;
        self.write_int(0)
    }
}
