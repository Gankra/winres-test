extern crate winapi;

use winapi::wingdi::DEVMODEW;

use std::{ptr, mem};

fn main() {
    unsafe {
        let mut idx = 0;

        let mut dev_mode = DEVMODEW {
            dmSize: mem::size_of::<DEVMODEW>(),
            .. mem::uninitialized()
        };

        while winapi::EnumDisplaySettingsW(ptr::null(), idx, &mut dev_mode) != 0 {
            println!("{} x {}", dev_mode.dmPelsWidth, dev_mode.dmPelsHeight);
            idx += 1;
        }
    }
}
