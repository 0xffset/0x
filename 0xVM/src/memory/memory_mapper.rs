use crate::device::Device;

use super::{Byte, Word};

pub struct MemoryMapper {
    pub regions: Vec<Region>,
}

#[derive()]
pub struct Region {
    pub device: Box<dyn Device>,
    pub start: Word,
    pub end: Word,
    pub remap: bool,
}

impl MemoryMapper {
    pub fn new() -> MemoryMapper {
        MemoryMapper {
            regions: Vec::new(),
        }
    }

    fn find_region(&self, address: Word) -> usize {
        for (i, region) in self.regions.iter().enumerate() {
            if region.start <= address && address < region.end {
                return i;
            }
        }
        panic!("[MEMORY MAPPER] No such region: '0x{:08X}'", address);
    }

    fn get_region_and_address(&self, address: Word) -> (usize, Word) {
        let region_index = self.find_region(address);
        let final_address = if self.regions[region_index].remap {
            address - self.regions[region_index].start
        } else {
            address
        };

        (region_index, final_address)
    }

    pub fn get_word(&self, address: Word) -> Word {
        let (region_index, final_address) = self.get_region_and_address(address);

        self.regions[region_index].device.get_word(final_address)
    }

    pub fn get_byte(&self, address: Word) -> Byte {
        let (region_index, final_address) = self.get_region_and_address(address);

        self.regions[region_index].device.get_byte(final_address)
    }

    pub fn set_word(&mut self, address: Word, value: Word) {
        let (region_index, final_address) = self.get_region_and_address(address);

        self.regions[region_index]
            .device
            .set_word(final_address, value);
    }

    pub fn set_byte(&mut self, address: Word, value: Byte) {
        let (region_index, final_address) = self.get_region_and_address(address);

        self.regions[region_index]
            .device
            .set_byte(final_address, value);
    }

    pub fn map(&mut self, device: Box<dyn Device>, start: Word, end: Word, remap: bool)
    // -> Box<dyn Fn(&mut MemoryMapper)> {
    {
        let region = Region {
            device,
            start,
            end,
            remap,
        };

        self.regions.insert(0, region);

        /*
        Box::new(move |this: &mut MemoryMapper| {
            for (i, r) in this.regions.iter().enumerate() {
                if *r == region {
                    this.regions.remove(i);
                    break;
                }
            }
        })
        */
    }
}
