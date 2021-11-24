use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::memory::{HalfWord, Word};

#[derive(Clone)]
pub struct Config {
    pub ram_size: Word,
    pub stack_size: Word,

    pub hd_cfg: HardDriveConfig,
    pub hd_file: String,
    pub load_hd: bool,
    pub enable_hd: bool,

    pub screen_cfg: ScreenConfig,
    pub enable_screen: bool,

    pub program_file: String,
    pub debug_mode: bool,
}

#[derive(Clone, Copy)]
pub struct HardDriveConfig {
    pub sector_size: Word,
    pub sector_count: Word,
    pub size: Word,
}

#[derive(Clone, Copy)]
pub struct ScreenConfig {
    pub width: HalfWord,
    pub height: HalfWord,
    pub size: Word,
}

impl Config {
    pub fn default() -> Self {
        Self {
            ram_size: 0,
            stack_size: 0,

            hd_cfg: HardDriveConfig {
                sector_size: 0,
                sector_count: 0,
                size: 0,
            },
            hd_file: String::new(),
            load_hd: false,
            enable_hd: false,

            screen_cfg: ScreenConfig {
                width: 0,
                height: 0,
                size: 0,
            },
            enable_screen: false,

            program_file: String::new(),
            debug_mode: false,
        }
    }
}

fn base10_string_to_word(s: &str) -> Word {
    Word::from_str_radix(s, 10).expect(format!("[Config] failed to parse '{}' to u32", s).as_str())
}

fn base10_string_to_half_word(s: &str) -> HalfWord {
    HalfWord::from_str_radix(s, 10)
        .expect(format!("[Config] failed to parse '{}' to u16", s).as_str())
}

fn hex_string_to_word(s: &str) -> Word {
    Word::from_str_radix(s, 16).expect(format!("[Config] failed to parse '{}' to u32", s).as_str())
}

fn hex_string_to_half_word(s: &str) -> HalfWord {
    HalfWord::from_str_radix(s, 16)
        .expect(format!("[Config] failed to parse '{}' to u16", s).as_str())
}

fn string_to_word(s: &str) -> Word {
    if s.starts_with("0x") {
        hex_string_to_word(&s[2..])
    } else {
        base10_string_to_word(s)
    }
}

fn string_to_half_word(s: &str) -> HalfWord {
    if s.starts_with("0x") {
        hex_string_to_half_word(&s[2..])
    } else {
        base10_string_to_half_word(s)
    }
}

pub fn generate_config(cfg_path: &String) -> Config {
    let mut cfg_flags = [false; 10];
    let mut cfg = Config::default();

    if let Ok(f) = File::open(cfg_path) {
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();

            if l.is_empty() {
                continue;
            }

            let (name, mut val) = l.split_once(":").expect("[Config] failed to split line");
            val = val.trim();
            match name.trim() {
                "ram_size" => {
                    cfg.ram_size = string_to_word(val);
                    cfg_flags[0] = true;
                }
                "stack_size" => {
                    cfg.stack_size = string_to_word(val);
                    cfg_flags[1] = true;
                }

                "hard_drive_config" => {
                    let (size, count) = val.split_once(",").expect("[Config] failed to split line");

                    cfg.hd_cfg.sector_size = string_to_word(size.trim());
                    cfg.hd_cfg.sector_count = string_to_word(count.trim());

                    cfg.hd_cfg.size = cfg.hd_cfg.sector_size * cfg.hd_cfg.sector_count;

                    cfg_flags[2] = true;
                }
                "hard_drive_file" => {
                    cfg.hd_file = val.to_string();
                    cfg_flags[3] = true;
                }
                "load_hard_drive" => {
                    cfg.load_hd = val
                        .parse::<bool>()
                        .expect(format!("[Config] failed to parse '{}' as bool", val).as_str());
                    cfg_flags[4] = true;
                }
                "enable_hard_drive" => {
                    cfg.enable_hd = val
                        .parse::<bool>()
                        .expect(format!("[Config] failed to parse '{}' as bool", val).as_str());
                    cfg_flags[5] = true;
                }

                "screen_config" => {
                    let (width, height) = val.split_once(",").expect("[Config] failed to split line");

                    cfg.screen_cfg.width = string_to_half_word(width.trim());
                    cfg.screen_cfg.height = string_to_half_word(height.trim());

                    cfg.screen_cfg.size =
                        cfg.screen_cfg.width as Word * cfg.screen_cfg.height as Word;

                    cfg_flags[6] = true;
                }
                "enable_screen" => {
                    cfg.enable_screen = val
                        .parse::<bool>()
                        .expect(format!("[Config] failed to parse '{}' as bool", val).as_str());
                    cfg_flags[7] = true;
                }

                "program_file" => {
                    cfg.program_file = val.to_string();
                    cfg_flags[8] = true;
                }
                "debug_mode" => {
                    cfg.debug_mode = val
                        .parse::<bool>()
                        .expect(format!("[Config] failed to parse '{}' as bool", val).as_str());
                    cfg_flags[9] = true;
                }
                _ => panic!("[Config] invalid setting '{}'", name),
            }
        }
    }

    if cfg_flags.iter().any(|&f| !f) {
        panic!("[Config] missing required setting");
    }

    cfg
}
