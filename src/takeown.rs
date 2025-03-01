use std::process::{Command, Stdio};
use std::io;

pub fn takeown(file: &str) -> io::Result<()> {
    let takeown_status = Command::new("takeown")
        .arg("/F")
        .arg(file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    
    if !takeown_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("takeown command failed with status: {:?}", takeown_status),
        ));
    }
    
    let icacls_status = Command::new("icacls")
        .arg(file)
        .arg("/grant")
        .arg("Everyone:F")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    
    if !icacls_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("icacls command failed with status: {:?}", icacls_status),
        ));
    }
    
    Ok(())
}