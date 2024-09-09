use std::path::Path;
use nix::errno::Errno;
use nix::mount::{MntFlags, umount, umount2};
use nix::unistd::Uid;
use crate::error::{UResult, USimpleError};

pub fn umount_fs<p: AsRef<Path>>(target: p, flags: MntFlags, internal_only: bool) ->nix::Result<()>{
    let result = if flags == MntFlags::empty() {
        umount(target.as_ref())
    } else {
        umount2(target.as_ref(), flags)
    };    if internal_only {
        result
    }else {
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Internal mount failed: {}. Attempting external mount...", e);
                internal_umount(&target,flags)
            }
        }
    }
}

fn internal_umount<p: AsRef<Path>>(target: &p, flags: MntFlags) -> nix::Result<()> {
    let mut cmd = std::process::Command::new("umount");

    // 添加 flags
    if flags.contains(MntFlags::MNT_FORCE) {
        cmd.arg("-f");
    }
    if flags.contains(MntFlags::MNT_DETACH) {
        cmd.arg("-l");
    }
    // 可以根据需要添加更多的 flags 转换

    cmd.arg(target.as_ref());

    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(Errno::from_i32(status.code().unwrap_or(1))),
        Err(e) => Err(Errno::from_i32(e.raw_os_error().unwrap_or(1)))
    }
}
pub fn prepare_umount_target(target: &str)->UResult<String>{
    if !Uid::effective().is_root() {
        return Err(USimpleError::new(1, "需要 root 权限来挂载设备"));
    }else {
        Ok("".to_string())
    }
}