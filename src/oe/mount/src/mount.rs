use clap::Command;
use uucore::error::UResult;

pub mod mount_common;

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {

}

pub fn oe_app<'a>() -> Command<'a> {
}
