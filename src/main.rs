pub mod dll;
pub mod network;
pub mod takeown;
pub mod util;
pub mod hash;
pub mod console;

use getch_rs::{Getch, Key};

fn main() {
    let g = Getch::new();
    let mut do_clear = true;

    loop {
        if do_clear { 
            console::clear();
            do_clear = false;
        }

        println!("[ ] ---------------------------------");
        println!("[ ] MCPatcher by aceinetx (2022-2025)");
        println!("[ ] What would you like to do?");
        println!("[p] Patch minecraft");
        println!("[s] Check patch status");
        println!("[q] Quit");
        println!("[c] Clear screen");
        println!("[ ]");
        match g.getch() {
            Ok(Key::Char('q')) => break,
            Ok(Key::Char('p')) => {
                if dll::takeown_dlls() {
                    if !dll::download_dlls() {
                        println!("[-] Download, cannot proceed");
                    }
                } else {
                    println!("[-] Takeown failed, cannot proceed");
                }

                println!("[+] Patched successfully!");
            },
            Ok(Key::Char('s')) => {
                println!("[ ] Checking status (sys32)");

                let mut hash_sys32 = String::new();
                match hash::hash_file_sha256(&dll::get_dll_sys32()) {
                    Ok(hash_s) => {hash_sys32 = hash_s.to_string()},
                    Err(_) => {},
                }
                
                println!("[ ] Checking status (wow64)");

                let mut hash_wow64 = String::new();
                match hash::hash_file_sha256(&dll::get_dll_wow64()) {
                    Ok(hash_s) => {hash_wow64 = hash_s.to_string()},
                    Err(_) => {},
                }

                let mut hash_matches: u8 = 0;
                if hash_sys32 == "c1469dea551c95d2c68eb42ceb37f020cb5b75d777e7083f24bf2e54ae2e4f55"{
                    hash_matches += 1;
                    println!("[+] ({}/2) System32 Patch is installed!", hash_matches);
                }

                if hash_wow64 == "ceae86e550dc1daa1b364be1ac195dd5dd9eaea8bfdf1875a4ae832c3e1a42a2"{
                    hash_matches += 1;
                    println!("[+] ({}/2) SysWOW64 Patch is installed!", hash_matches)
                }

                if hash_matches == 2 {
                    println!("[+] All patches are installed! You're good to go!")
                } else {
                    println!("[-] Some of the patches are not installed")
                }
            },
            Ok(Key::Char('c')) => {
                do_clear = true;
            },
            Ok(_) => {},
            Err(e) => println!("{}", e),
        }
        println!("[ ]");
    }
}
