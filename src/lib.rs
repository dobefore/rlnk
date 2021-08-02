use std::{fs, io::Result, path::Path, process::Command};
#[derive(Debug, Default)]
pub struct ShellLink {
    link_path: String,
    target: String,
    // deduce from target
    working_dir: String,
}

impl ShellLink {
    /// create an instance of struct ShellLink,same as default()
    pub fn new() -> Self {
        ShellLink::default()
    }
    ///  set target of link,working_dir will also be set from target
    /// ## example
    /// ```
    /// let target=r"C:\Users\Admin\Desktop\new aa\qiuqiu.exe";
    /// sl.set_target(target);
    ///```
    pub fn set_target<P: AsRef<str>>(&mut self, file_path: P) {
        self.target = file_path.as_ref().to_owned();
        let working_dir = Path::new(file_path.as_ref())
            .ancestors()
            .next()
            .unwrap()
            .to_str()
            .unwrap();
        self.working_dir = working_dir.to_owned();
    }
    ///create lnk file of target
    ///## example
    ///```
    /// let target=r"C:\Users\Admin\Desktop\new aa\qiuqiu.exe";
    ///    let lnk=r"C:\Users\Admin\Desktop\qiuqiu.lnk";
    ///     let mut sl=ShellLink::new();
    ///     sl.set_target(target);
    /// sl.create_lnk(lnk);
    /// ```
    pub fn create_lnk<P: AsRef<str>>(&self, lnk_path: P) {
        let shortcut_exe_path = write_exe_to_temp().unwrap();
        let tgt = format!("/t:\"{}\"", self.target);
        let lnk = format!("/f:{}", lnk_path.as_ref());
        let args = &[lnk.as_str(), "/a:c", tgt.as_str()];
        Command::new(shortcut_exe_path).args(args).status().unwrap();
    }
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
    let lnk = r"C:\Users\Admin\Desktop\sdr-Cleaner_win.lnk";
    let mut sl = ShellLink::new();
    sl.set_target(target);
    sl.create_lnk(lnk);
}
