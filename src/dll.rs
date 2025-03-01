use crate::takeown;
use crate::network;
use std::fs;

pub fn get_dll_sys32() -> String {
    return String::from("C:\\Windows\\System32\\Windows.ApplicationModel.Store.dll");
}

pub fn get_dll_wow64() -> String {
    return String::from("C:\\Windows\\SysWOW64\\Windows.ApplicationModel.Store.dll");
}

pub fn takeown_dlls() -> bool {
    println!("[ ] Updating permissions (sys32)");
    if let Err(e) = takeown::takeown(&get_dll_sys32()) {
        eprintln!("\n[-] Error: {}", e);
        return false;
    } else {
        println!("[+] Updated permissions (sys32)");
    }

    println!("[ ] Updating permissions (wow64)");
    if let Err(e) = takeown::takeown(&get_dll_wow64()) {
        eprintln!("\n[-] Error: {}", e);
        return false;
    } else {
        println!("[+] Updated permissions (wow64)");
    }
    return true;
}

pub fn rename_dlls() -> bool{
    println!("[ ] Renaming dll (sys32)");
    if let Err(e) = fs::rename(get_dll_sys32(), get_dll_sys32()+"OLD"){
        eprintln!("\n[-] Error: {}", e);
        return false;
    } else {
        println!("[+] DLL Renamed (sys32)");
    }

    println!("[ ] Renaming dll (wow64)");
    if let Err(e) = fs::rename(get_dll_wow64(), get_dll_wow64()+"OLD"){
        eprintln!("\n[-] Error: {}", e);
        return false;
    } else {
        println!("[+] DLL Renamed (wow64)");
    }
    return true;
}

pub fn download_dlls() -> bool{
    println!("[ ] Downloading DLL (sys32)");
    if let Err(e) = network::download(&network::get_url_sys32(), &get_dll_sys32()){
        eprintln!("\n[-] Error: {}", e);
        return false;
    } else {
        println!("[+] DLL Downloaded (sys32)");
    }

    println!("[ ] Downloading DLL (wow64)");
    if let Err(e) = network::download(&network::get_url_wow64(), &get_dll_wow64()){
        eprintln!("\n[-] Error: {}", e);
        return false;
    } else {
        println!("[+] DLL Downloaded (wow64)");
    }

    return true; 
}