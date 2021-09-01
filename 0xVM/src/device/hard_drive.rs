use crate::{cpu::HardDriveConfig, memory::{Byte, Word}};

use super::Device;

pub struct HardDrive {
    pub sector_size: Word,
    pub sector_count: Word,

    addr: Word,
    data: Vec<Byte>,
}

impl HardDrive {
    pub fn new(cfg: HardDriveConfig) -> Self {
        Self {
            sector_size: cfg.sector_size,
            sector_count: cfg.sector_count,
            addr: 0,
            data: vec![0; cfg.size as usize],
        }
    }

    pub fn from(data: Vec<Byte>, cfg: HardDriveConfig) -> Self {
        Self {
            sector_size: cfg.sector_size,
            sector_count: cfg.sector_count,
            addr: 0,
            data,
        }
    }
}

impl Device for HardDrive {
    fn get_byte(&self, _: Word) -> Byte {
        0
    }

    fn set_word(&mut self, _: Word, val: Word) -> () {
        self.addr = val;
    }

    fn get_word(&self, _: Word) -> Word {
        self.addr
    }

    fn get_range(&self, _: Word, _: Word) -> Vec<Byte> {
        let addr = self.addr * self.sector_size;
        self.data
            .get(addr as usize..addr as usize + self.sector_size as usize)
            .expect(
                format!(
                    "[HardDrive] get_range: No such sector '0x{:08X}-{:08X}'",
                    addr,
                    addr + self.sector_size
                )
                .as_str(),
            )
            .to_vec()
    }

    fn set_range(&mut self, _: Word, data: Vec<Byte>) {
        let addr = self.addr * self.sector_size;

        if data.len() != self.sector_size as usize {
            panic!(
                "[HardDrive] set_range: Data size mismatch, expected '{}' but got '{}'",
                self.sector_size,
                data.len()
            );
        }

        if addr + self.sector_size > self.data.len() as Word {
            panic!(
                "[HardDrive] set_range: No such sector '0x{:08X}-{:08X}'",
                addr,
                addr + data.len() as Word
            );
        }

        for (i, byte) in data.iter().enumerate() {
            self.data[addr as usize + i] = *byte;
        }
    }

	fn get_buffer(&self) -> Vec<Byte> {
		self.data.clone()
	}
}
