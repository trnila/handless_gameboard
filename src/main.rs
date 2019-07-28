#![feature(asm)]
extern crate uinput;
extern crate serial;
mod config;

use uinput::event::keyboard::Key;

use std::time::Duration;
use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    let profiles = config::parse("sam2.txt");
    let profile = match profiles.get("sam2") {
        Some(m) => m,
        None => panic!("unknown profile")
    };


    let mut s = serial::open("/dev/ttyACM0").unwrap();
    s.reconfigure(&|conf| {
        conf.set_baud_rate(serial::Baud9600);
        Ok(())
    });
    s.set_timeout(Duration::from_secs(10000));


    let mut builder = uinput::default().unwrap()
        .name("handless gameboard").unwrap();

    for k in Key::iter_variants() {
        builder = builder.event(k).unwrap();
    }

    let mut dev = builder.create().unwrap();

    for byte in s.bytes() {
        let b = byte.unwrap();
        let btn:i32 = b as i32 & 0x7F;
        let pressed : bool = b & 0x80 > 0;

        if let Some(keys) = profile.get(&btn) {
            println!("{} {:?}", btn, keys);

            for key in keys {
                if pressed {
                    dev.press(key);
                } else {
                    dev.release(key);
                }
            }
            dev.synchronize();


        } else {
            println!("no bind");
        }
    }

    unsafe {
        asm!("
            mov $$60, %rax
            mov $$42, %rdi
            syscall
        ");
    }

}
