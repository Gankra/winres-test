extern crate winapi;
extern crate user32;

use winapi::wingdi::DEVMODEW;

use std::{ptr, mem};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct DisplayMode {
    freq: u32,
    width: u32,
    height: u32,
}

fn main() {
    for res in get_available_resolutions() {
        println!("{} x {} @ {}", res.width, res.height, res.freq);
    }
}

fn get_available_resolutions() -> Vec<DisplayMode> {
    unsafe {
        let mut resolutions = vec![];
        let mut idx = 0;

        let mut stats = DEVMODEW {
            dmSize: mem::size_of::<DEVMODEW>() as u16,
            .. mem::uninitialized()
        };

        while user32::EnumDisplaySettingsW(ptr::null(), idx, &mut stats) != 0 {
            if stats.dmBitsPerPel == 32 && stats.dmDisplayFrequency == 60 {
                resolutions.push(DisplayMode { 
                    width: stats.dmPelsWidth, 
                    height: stats.dmPelsHeight, 
                    freq: stats.dmDisplayFrequency 
                });
            }
            idx += 1;
        }

        resolutions.sort();
        resolutions.dedup();
        resolutions
    }
}
