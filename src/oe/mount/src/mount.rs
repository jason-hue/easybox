use std::env::args;
use clap::Command;
use uucore::error::UResult;
use uucore::{help_section, help_usage};
use crate::mount_common::{Config, mount_app, parse_mount_cmd_args};

pub mod mount_common;

const ABOUT: &str = help_section!("about", "mount.md");
const USAGE: &str = help_usage!("mount.md");

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {
    let config: Config = parse_mount_cmd_args(args,ABOUT,USAGE)?;
    println!("{:?}",config);
    Ok(())
}

pub fn oe_app<'a>() -> Command<'a> {
    mount_app(ABOUT,USAGE)
}
