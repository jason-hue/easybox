use clap::Command;
use nix::mount::MsFlags;
use uucore::error::{UResult, USimpleError};
use uucore::{help_section, help_usage};
use crate::mount_common::{Config, mount_app, parse_mount_cmd_args};
use uucore::mount::{mount_fs, prepare_mount_source};

pub mod mount_common;

const ABOUT: &str = help_section!("about", "mount.md");
const USAGE: &str = help_usage!("mount.md");

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {
    let config: Config = parse_mount_cmd_args(args,ABOUT,USAGE)?;
    println!("{:?}",config);
    let source = config.device.ok_or_else(|| USimpleError::new(1, "源设备未指定"))?;
    let source = source.to_str();
    let mount_source = prepare_mount_source(source.unwrap())?;
    let mount_source = Some(mount_source.as_str());
    let target = config.target.ok_or_else(|| USimpleError::new(1, "目标路径未指定"))?;
    let target = target.to_str().unwrap();
    let fstype = Some("ext4");
    let flags = MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID;
    let data = None;
    mount_fs(mount_source.as_ref(), &target, fstype, flags, data).expect("Mount failed!");
    println!("Mount successful!");
    Ok(())
}

pub fn oe_app<'a>() -> Command<'a> {
    mount_app(ABOUT,USAGE)
}
