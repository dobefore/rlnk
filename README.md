# rlnk
a Wrapper for  [Shortcut.exe](https://api.256file.com/shortcut.exe/en-download-62728.html) by Optimum X.
used to create Windows shortcuts(lnk files).
Ispired by [A Node.js API for shortcut.exe](https://github.com/j201/windows-shortcuts)
#example
```
use rlnk::ShellLink;
let target=r"C:\Users\Admin\Desktop\new aa\qiuqiu.exe";
let lnk=r"C:\Users\Admin\Desktop\qiuqiu.lnk";
let mut sl=ShellLink::new();
sl.set_target(target);
sl.create_lnk(lnk); 
```

# How it works
Shortcut.exe as an external exec_file which should be written to Windows Temp
directory is executed with parameters to create windows shortcuts.
