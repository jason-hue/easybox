use crate::umount_common::{parse_umount_cmd_args, umount_app, Config, UmountHandler};
use clap::Command;
use uucore::error::{UResult, USimpleError};
use uucore::{help_section, help_usage};
mod umount_common;
const ABOUT: &str = help_section!("about", "umount.md");
const USAGE: &str = help_usage!("umount.md");

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {
    let config: Config = parse_umount_cmd_args(args, ABOUT, USAGE)?;
    let umount_handler = UmountHandler::new(config);
    match umount_handler.process() {
        Ok(_) => {}
        Err(e) => {
            // eprintln!("Error during mount operation: {}", e);
            return Err(USimpleError::new(
                1,
                format!("Umount operation failed: {}", e),
            ));
        }
    }
    Ok(())
}
pub fn oe_app<'a>() -> Command<'a> {
    umount_app(ABOUT, USAGE)
}
