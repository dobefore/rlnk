//! # example
//! ```
//! let target=r"C:\Users\Admin\Desktop\new aa\qiuqiu.exe";
//! let lnk=r"C:\Users\Admin\Desktop\qiuqiu.lnk";
//! let mut sl=ShellLink::new();
//! sl.set_target(target);
//! sl.create_lnk(lnk);
//! ```
use std::{
    fs,
    io::{self, Result},
    path::Path,
    process::Command,
};
/// path-related constructor
#[derive(Debug, Default)]
pub struct ShellLink {
    link_path: String,
    target: String,
    // deduce from target
    working_dir: String,
}

impl ShellLink {
    /// create an instance of struct ShellLink,same as default()
    #[cfg(windows)]
    pub fn new() -> Self {
        ShellLink::default()
    }

    /// delete shortcut.exe file from C:\Windows\Temp
    pub fn remove_scexe_from_temp(&self) -> io::Result<()> {
        let temp = r"C:\Windows\Temp\Shortcut.exe";
        if is_scexe_in_temp(temp) {
            fs::remove_file(temp)?;
        }
        Ok(())
    }
    ///  set target of link,working_dir will also be set from target
    /// ## example
    /// ```
    /// let target=r"C:\Users\Admin\Desktop\new aa\qiuqiu.exe";
    /// sl.set_target(target);
    ///```
    #[cfg(windows)]
    pub fn set_target<P: AsRef<str>>(&mut self, file_path: P) {
        self.target = file_path.as_ref().to_owned();
        let working_dir = Path::new(file_path.as_ref())
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        self.working_dir = working_dir.to_owned();
    }
    ///create lnk file(.lnk) of target
    ///
    /// return error if command(shortcut.exe) encounter false filepath
    #[cfg(windows)]
    pub fn create_lnk<P: AsRef<str>>(&self, lnk_path: P) -> io::Result<()> {
        let shortcut_exe_path = write_exe_to_temp().unwrap();
        let tgt = format!("/t:{}", self.target);
        let work_dir = format!("/w:{}", self.working_dir);
        let lnk = format!("/f:{}", lnk_path.as_ref());
        let args = &[lnk.as_str(), "/a:c", tgt.as_str(), work_dir.as_str()];
        // run cmd and capture stdoutput
        let out = Command::new(shortcut_exe_path).args(args).output().unwrap();
        let out_str = String::from_utf8_lossy(&out.stdout).trim().to_owned();
        if out_str.contains("successfully") {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "command function abnormally",
            ))
        }
    }
}
fn is_scexe_in_temp(temp_path: &str) -> bool {
    if Path::new(temp_path).exists() {
        return true;
    }
    false
}
fn write_exe_to_temp() -> Result<String> {
    let shortcut_exe = include_bytes!("bin/Shortcut.exe");
    let temp = r"C:\Windows\Temp\Shortcut.exe";
    fs::write(temp, shortcut_exe).unwrap();
    Ok(temp.to_owned())
}

#[test]
fn test_create_lnk() {
    let target = r"C:\Users\Admin\Desktop\new aa\sdr-Cleaner_win.exe";
    let lnk = r"C:\Users\Admin\Desktop\new aa\sdr-Cleaner_win.lnk";
    let mut sl = ShellLink::new();
    sl.set_target(target);
    sl.create_lnk(lnk).unwrap();
}
