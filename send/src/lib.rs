use std::path::Path;
use std::{env::current_dir, fs, path::PathBuf};
mod error;
pub use error::SendError;
use mslnk::ShellLink;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

/*
send exe to desktop and start menu
assume that the executable be located in the same directory as this bin
create a new crate with cargo new --lib to store code that send
*/
pub static LNK_NAME: &str = "ankiserver.lnk";
pub static START: &str = r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs";
#[derive(Debug, Default)]
pub struct Send {
    desktop: Option<PathBuf>,
    start: Option<PathBuf>,
}

impl Send {
    pub fn new() -> Self {
        Self::default()
    }
    /// set `Desktop` path,handled inside the function,no need for external argument
    pub fn set_desktop(&mut self) -> Result<&mut Self, SendError> {
        self.desktop = Some(desktop()?);

        Ok(self)
    }
    /// set `Start` path,handled inside the function,no need for external argument
    pub fn set_start(&mut self) -> &mut Self {
        self.start = Some(START.into());
        self
    }
    ///create shortcut in current dir
    ///
    /// this will send windows shortcut to both  `Desktop` path and `Start` path (if both specified)
    pub fn send(&self) -> Result<(), SendError> {
        // create lnk in current dir
        create_shortcut()?;
        // send lnk to dst path
        let current_dir = current_dir()?;
        let from = current_dir.join(LNK_NAME);
        let desk = self.desktop.as_ref().unwrap().join(LNK_NAME);
        let start = self.start.as_ref().unwrap().join(LNK_NAME);

        match fs::copy(&from, &desk) {
         Ok(_)=>{},
         Err(e)=> {
            println!("{}: {}",e,desk.display());
         } 
        }
        match fs::copy(&from, &start) {
            Ok(_)=>{},
            Err(e)=> {
               println!("{}: {}",e,start.display());
            } 
           }
        Ok(())
    }
}
/// create lnk in current_dir
fn create_shortcut() -> Result<(), SendError> {
    let current_dir = current_dir()?;
    let target = current_dir.join("ankiserver.exe");
    let lnk = current_dir.join(LNK_NAME);
    let sl = ShellLink::new(target)?;
    sl.create_lnk(lnk)?;
    Ok(())
}
pub fn desktop() -> Result<PathBuf, SendError> {
    let d = Path::new(&output_user_profile()?).join("Desktop");
    if !d.exists() {
        return Err(SendError::new("", "desktop not exist"));
    }
    Ok(d)
}
fn output_user_profile() -> Result<String, SendError> {
    // return
    // userprofile:C:\\Users\\Admin
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey("Volatile Environment")?;
    let userprofile: String = cur_ver.get_value("USERPROFILE")?;
    Ok(userprofile)
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
