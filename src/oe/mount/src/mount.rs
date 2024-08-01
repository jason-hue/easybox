use clap::Command;
use nix::mount::MsFlags;
use uucore::error::{UResult, USimpleError};
use uucore::{help_section, help_usage};
use crate::mount_common::{Config, ConfigHandler, mount_app, parse_mount_cmd_args, Source};
use uucore::mount::{mount_fs, prepare_mount_source};

pub mod mount_common;

const ABOUT: &str = help_section!("about", "mount.md");
const USAGE: &str = help_usage!("mount.md");

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {
    let config: Config = parse_mount_cmd_args(args, ABOUT, USAGE)?;
    println!("{:#?}", config);

    let config_handler = ConfigHandler::new(config);
    match config_handler.process() {
        Ok(_) => {
            println!("Mount operation completed successfully.");
        },
        Err(e) => {
            eprintln!("Error during mount operation: {}", e);
            return Err(USimpleError::new(1, format!("Mount operation failed: {}", e)));
        }
    }

    Ok(())
}

pub fn oe_app<'a>() -> Command<'a> {
    mount_app(ABOUT,USAGE)
}
