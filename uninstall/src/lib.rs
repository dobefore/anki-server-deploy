use std::{fs, path};

use send::{START, LNK_NAME, desktop};
use updateaddr::addon_ankisyncd_dir;

// Note the lack of the `#[link]` attribute. We’re delegating the responsibility
// of selecting what to link over to the build script rather than hard-coding
// it in the source file.
extern { fn press(); }

// remove anki add-on dir
// remove shortcut in start menu
// remove shortcut in desktop
pub fn uninstall() {
    let addon_dir=addon_ankisyncd_dir().unwrap();
    if addon_dir.exists() {
 fs::remove_dir_all(addon_dir).unwrap();
println!("已删除电脑Anki插件ankisyncd，重启Anki生效");
       
    }

let start=path::Path::new(START).join(LNK_NAME);
let desk=desktop().unwrap().join(LNK_NAME);
if start.exists() {
 fs::remove_file(start).unwrap();
println!("已删除开始菜单快捷方式");   
}
if desk.exists() {
 fs::remove_file(desk).unwrap();
println!("已删除电脑桌面快捷方式");   
}

println!("按任意键退出...");   
    unsafe { press(); }
}

