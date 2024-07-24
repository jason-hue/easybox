use std::os::unix::fs::FileTypeExt;
use std::path::Path;
use nix::mount::{mount, MsFlags};
use crate::error::{UResult, USimpleError};
use nix::unistd::Uid;
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