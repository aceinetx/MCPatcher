use std::fs::File;
use std::io::copy;
use std::error::Error;

pub fn get_url_sys32() -> String {
    String::from("https://www.dropbox.com/scl/fi/u269g2fmyrmructoygbbp/Windows.ApplicationModel.Store32.dll?rlkey=vmym4mjjb4cc39ydmzxdgw9p7&st=9n55ey35&dl=1")
}

pub fn get_url_wow64() -> String {
    String::from("https://www.dropbox.com/scl/fi/xu57gzo4ud8gly1so9kw8/Windows.ApplicationModel.Store64.dll?rlkey=eequhuogv25wnlq1a56t62hm0&st=7i0zupmq&dl=1")
}

pub fn download(url: &str, destination: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    
    let mut dest_file = File::create(destination)?;
    
    copy(&mut response, &mut dest_file)?;
    
    Ok(())
}