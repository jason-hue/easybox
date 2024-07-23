use clap::Command;
use nix::mount::MsFlags;
use uucore::error::UResult;
use uucore::{help_section, help_usage};
use crate::mount_common::{Config, mount_app, parse_mount_cmd_args};
use uucore::mount::mount_fs;

pub mod mount_common;

const ABOUT: &str = help_section!("about", "mount.md");
const USAGE: &str = help_usage!("mount.md");

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {
    let config: Config = parse_mount_cmd_args(args,ABOUT,USAGE)?;
    println!("{:?}",config);
    let source = config.device.unwrap();
    let source = source.to_str();
    let target = config.target.unwrap();
    let target = target.to_str().unwrap();
    let fstype = Some("ext4");
    let flags = MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID;
    let data = None;
    mount_fs(source.as_ref(), &target, fstype, flags, data).expect("Mount failed!");
    println!("Mount successful!");
    
    Ok(())
}

pub fn oe_app<'a>() -> Command<'a> {
    mount_app(ABOUT,USAGE)
}
