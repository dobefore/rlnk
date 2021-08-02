# rlnk
a Wrapper for  [Shortcut.exe](https://api.256file.com/shortcut.exe/en-download-62728.html) by Optimum X.
used to create Windows lnk files.

#example
```
use rlnk;
let target=r"C:\Users\Admin\Desktop\new aa\qiuqiu.exe";
let lnk=r"C:\Users\Admin\Desktop\qiuqiu.lnk";
let mut sl=ShellLink::new();
sl.set_target(target);
sl.create_lnk(lnk); 
```
