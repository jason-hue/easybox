use std::fs::{canonicalize, File};
use std::{fs, io};
use std::io::{BufRead, BufReader};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use nix::errno::Errno;
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
    data: Option<&str>,
    internal_only: bool
) -> nix::Result<()> {
    let result =  mount(source.map(|s| s.as_ref()), target.as_ref(), fs_type, flags, data);
    if internal_only{
        // 如果指定了 internal_only，我们只返回内核挂载的结果
        result
    }else {
        match result {
            Ok(_) => Ok(()), // 内部挂载成功
            Err(e) => {
                eprintln!("Internal mount failed: {}. Attempting external mount...", e);
                // 尝试外部挂载
                external_mount(source, target, fs_type, flags, data)
            }
        }
    }
}
fn external_mount<P: AsRef<Path>>(
    source: Option<&P>,
    target: &P,
    fs_type: Option<&str>,
    flags: MsFlags,
    data: Option<&str>
) -> nix::Result<()> {
    let mut cmd = std::process::Command::new("mount");

    if let Some(src) = source {
        cmd.arg(src.as_ref());
    }

    cmd.arg(target.as_ref());

    if let Some(fs) = fs_type {
        cmd.args(&["-t", fs]);
    }

    // 将 flags 转换为命令行选项
    if flags.contains(MsFlags::MS_RDONLY) {
        cmd.arg("-r");
    }
    if flags.contains(MsFlags::MS_NOSUID) {
        cmd.arg("-o").arg("nosuid");
    }
    if flags.contains(MsFlags::MS_NODEV) {
        cmd.arg("-o").arg("nodev");
    }
    if flags.contains(MsFlags::MS_NOEXEC) {
        cmd.arg("-o").arg("noexec");
    }
    // 可以根据需要添加更多的 flags 转换

    if let Some(d) = data {
        cmd.arg("-o").arg(d);
    }

    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(Errno::from_i32(status.code().unwrap_or(1))),
        Err(e) => Err(Errno::from_i32(e.raw_os_error().unwrap_or(1)))
    }
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
pub fn parse_fstab(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    if path.is_dir() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{:?} 是一个目录,而不是文件", path)
        )));
    }

    let file = File::open(path).map_err(|e| format!("打开 fstab 文件失败: {}", e))?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\d+)\s+(\d+)").unwrap();

    let mut fstab_vec = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("读取第 {} 行时出错: {}", index + 1, e))?;
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;  // 跳过注释和空行
        }
        if let Some(caps) = re.captures(trimmed) {
            let line_vec: Vec<String> = (1..=6).map(|i| caps[i].to_string()).collect();
            fstab_vec.push(line_vec);
        } else {
            eprintln!("警告: 第 {} 行不符合预期格式: {}", index + 1, trimmed);
        }
    }

    if fstab_vec.is_empty() {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "fstab 文件中没有找到有效条目"
        )))
    } else {
        Ok(fstab_vec)
    }
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