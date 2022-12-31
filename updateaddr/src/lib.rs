use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
mod error;
use async_std::io::prelude::BufReadExt;
use async_std::io::{BufReader, WriteExt};
use async_std::prelude::*;
pub use error::UpdateAddrError;
use std::env;
use std::net::UdpSocket;
use std::path::{Path, PathBuf};
fn output_user_profile() -> Result<String, UpdateAddrError> {
    // return
    // userprofile:C:\\Users\\Admin
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey("Volatile Environment")?;
    let userprofile: String = cur_ver.get_value("USERPROFILE")?;
    Ok(userprofile)
}
pub fn addon_ankisyncd_dir() -> Result<PathBuf, UpdateAddrError> {
    let usrname_profile = output_user_profile()?;
    let addon_dir = Path::new(&usrname_profile).join(r"AppData\Roaming\Anki2\addons21");
    // give notice and return err when Anki not installed.
    if addon_dir.exists() {
        let ankisyncd_dir = addon_dir.join("ankisyncd");
        Ok(ankisyncd_dir)
    } else {
        Err(UpdateAddrError::NotFound(
            "Anki addon path not found".into(),
        ))
    }
}
/// copy files from current path to pc anki folder path
///
/// create dir if ankisyncd dir not exist in pc anki folder path
async fn copy_addon(ankisyncd_dir: &PathBuf) -> Result<(), UpdateAddrError> {
    let cwd = env::current_dir()?;

    // create dir if not exist
    if !ankisyncd_dir.exists() {
        async_std::fs::create_dir(&ankisyncd_dir).await?;
        // read files names from deployer's addon dir
        let server_addon_dir = cwd.join("ankisyncd");
        let mut entries = async_std::fs::read_dir(&server_addon_dir).await?;
        while let Some(res) = entries.next().await {
            let entry = res?.path();
            let f = entry.file_name();
            let dst_path = ankisyncd_dir.join(f.unwrap());

            async_std::fs::copy(entry, &dst_path).await?;
        }
    }
    Ok(())
}
// /// https not finished
// async fn set_pcip(ankisyncd_dir: PathBuf, ipaddr: &str) -> Result<(), UpdateAddrError> {
//     let conf_file = ankisyncd_dir.join("config.json");
//     let b = Vec::new();
//     let f = async_std::fs::File::open(conf_file.clone()).await?;
//     let mut lines = BufReader::new(f).lines();
// let mut ip_changed=false;
//     let mut buf = BufWriter::new(b);
//     while let Some(line) = lines.next().await {
//         let l = line?;
//         let cont = if l.contains("syncaddr") {
//             if !l.contains(ipaddr){
//                 ip_changed=true;
//             }
//             format!("\"syncaddr\":\"https://{}:27701/\"", &ipaddr)
//         } else {
//             l
//         };

//         if ip_changed {
//             buf.write(cont.as_bytes()).await?;
//             println!("检测到IP地址发生改变，将下面的地址填写到安卓Ankidroid相应界面，电脑Anki重新打开");
//             println!("同步地址：\n https://{}:27701",ipaddr);
//             println!("媒体文件同步地址：\n https://{}:27701/msync",ipaddr);
//         }

//     }
//     Ok(())
// }
///http print addr in console
async fn set_pcip_http(ankisyncd_dir: PathBuf, ipaddr: &str) -> Result<(), UpdateAddrError> {
    let conf_file = ankisyncd_dir.join("config.json");
    let f = async_std::fs::File::open(conf_file.clone()).await?;
    let mut lines = BufReader::new(f).lines();
    let mut ip_changed = false;
    let mut file_string = String::new();
    // let mut buf = BufWriter::new(b);
    while let Some(line) = lines.next().await {
        let l = line?;
        let cont = if l.contains("syncaddr") {
            if !l.contains(ipaddr) {
                ip_changed = true;
            }
            format!("\"syncaddr\":\"http://{}:27701/\"", &ipaddr)
        } else {
            l
        };
        // buf.write(cont.as_bytes()).await?;
        file_string.push_str(&format!("{}\n", &cont));
    }
    let mut f = async_std::fs::File::create(conf_file).await?;
    f.write(file_string.as_bytes()).await?;
    // f.write(&buf.buffer());
    if ip_changed {
        println!("检测到IP地址发生改变，将下面的地址填写到安卓Ankidroid相应界面，电脑Anki重新打开");
        println!("同步地址：\n http://{}:27701/sync/", ipaddr);
        println!("媒体文件同步地址：\n http://{}:27701/msync/", ipaddr);
    } else {
        println!("将下面的地址填写到安卓Ankidroid相应界面，电脑Anki重新打开");
        println!("同步地址：\n http://{}:27701/sync/", ipaddr);
        println!("媒体文件同步地址：\n http://{}:27701/msync/", ipaddr);
    }
    Ok(())
}
/// lookup ip lan addr
fn lookup_ip() -> Result<String, UpdateAddrError> {
    // look up local ipaddr

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .connect("8.8.8.8:80")
        .expect("Couldn't connect to the server...");
    let ipaddr = socket.local_addr().unwrap().ip();
    let ipaddr_str = format!("{}", ipaddr);

    Ok(ipaddr_str)
}
/// cp addon dir and set pc anki addr
/// updateaddr::update_syncaddr().await.unwrap();
#[cfg(target_family = "windows")]
pub async fn update_syncaddr() -> Result<(), UpdateAddrError> {
    let ipaddr = lookup_ip()?;
    // handle err,if err,print info and skip copy_addon
    match addon_ankisyncd_dir() {
        Ok(dir) => {
            copy_addon(&dir).await?;
            set_pcip_http(dir, &ipaddr).await?;
        }
        Err(_) => {
            println!("没有检测到电脑有安装Anki");
            // ,print addr info in set_pcip_http
            println!("将下面的地址填写到安卓Ankidroid相应界面，电脑Anki重新打开");
            println!("同步地址：\n http://{}:27701/sync/", ipaddr);
            println!("媒体文件同步地址：\n http://{}:27701/msync/", ipaddr);
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}

    #[test]
    fn test_contain_addr() {
        let s = "https://192.0.0.1:27701";
        let newaddr = "192.0.0.1";
        println!("{}", s.contains(newaddr));
    }
}
