use std::fs::{canonicalize, File};
use std::{fs, io};
use std::io::{BufRead, BufReader};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use nix::mount::{mount, MsFlags};
use nix::NixPath;
use crate::error::{UResult, USimpleError};
use nix::unistd::Uid;
use regex::Regex;

pub fn mount_fs<p: AsRef<Path>>(
    source: Option<&p>,
    target: &p,
    fs_type:Option<&str>,
    flags: MsFlags,
    data: Option<&str>
) -> nix::Result<()> {
    mount(source.map(|s| s.as_ref()), target.as_ref(), fs_type, flags, data)
}
pub fn prepare_mount_source(source: &str)->UResult<String>{
    if !Uid::effective().is_root() {
        return Err(USimpleError::new(1, "需要 root 权限来挂载设备"));
    }
    let metadata = std::fs::metadata(source)
        .map_err(|e| USimpleError::new(1, format!("无法获取源文件信息: {}", e)))?;
    if metadata.file_type().is_block_device(){
        //块设备直接返回
        Ok(source.to_string())
    }else {
        //为普通文件创建循环设备
        let output = std::process::Command::new("losetup")
            .arg("-f").arg("--show").arg(source).output().map_err(|e| USimpleError::new(1, format!("创建循环设备失败: {}", e)))?;
        if !output.status.success() {
            Err(USimpleError::new(1, format!(
                "创建循环设备失败: {}",
                String::from_utf8_lossy(&output.stderr)).to_string()))
        }else {
            String::from_utf8(output.stdout)
                .map_err(|e| USimpleError::new(1, format!("解析循环设备路径失败: {}", e)))
                .map(|s| s.trim().to_string())
        }
    }
}
pub fn is_already_mounted(target: &str) -> Result<bool, Box<dyn std::error::Error>> {
    /*读取/proc/mounts来获取已挂载的设备挂载点，判断是否已挂载*/
    let file = File::open("/proc/mounts")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^\S+\s+(\S+)")?;
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            if let Some(mount_point) = caps.get(1) {
                if target == mount_point.as_str() {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}
pub fn is_swapfile(fstype: &str) -> bool {
    fstype == "swap"
}
pub fn parse_mount_options(options: &str) -> MsFlags {
    let mut flags = MsFlags::empty();
    // for option in options.split(',') {
    //     // match option {
    //     //     "noexec" => flags |= MsFlags::MS_NOEXEC,
    //     //     "nosuid" => flags |= MsFlags::MS_NOSUID,
    //     //     // 添加其他选项...
    //     //     _ => {}
    //     // }
    // }
    flags
}
pub fn parse_fstab() -> Vec<Vec<String>> {
    let file = File::open("/etc/fstab").expect("打开fstab失败!");
    let reader = BufReader::new(file);
    let re = Regex::new(r"^(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\d+)\s+(\d+)").unwrap();
    //(\S)等效于[^\s]匹配非空白符字符
    let mut fstab_vec = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue;  // 跳过注释行和空行
        }
        if let Some(caps) = re.captures(&line) {
            let line_vec: Vec<String> = (1..=6).map(|i| caps[i].to_string()).collect();
            fstab_vec.push(line_vec);

            println!("文件系统: {}", &caps[1]);
            println!("挂载点: {}", &caps[2]);
            println!("类型: {}", &caps[3]);
            println!("选项: {}", &caps[4]);
            println!("dump: {}", &caps[5]);
            println!("pass: {}", &caps[6]);
            println!("---");
        }
    }
    println!("fstab文件内容：{:?}", fstab_vec);
    fstab_vec
}
pub fn find_device_by_label(label: &str) -> Result<String, Box<dyn std::error::Error>>{
    let output = std::process::Command::new("blkid")
        .arg("-L")
        .arg(label)
        .output()?;

    if output.status.success() {
        let device = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(device)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Not found device by label").into())
    }
}
pub fn find_device_by_uuid(uuid: &str) -> Result<String,Box<dyn std::error::Error>>{
    let output = std::process::Command::new("blkid")
        .arg("-U")
        .arg(uuid)
        .output()?;

    if output.status.success() {
        let device = String::from_utf8(output.stdout)?.trim().to_string();
        println!("uuid 解析成功！");
        Ok(device)
    }else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Not found device by uuid").into())
    }

}