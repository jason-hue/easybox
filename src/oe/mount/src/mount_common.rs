use std::io::{Read, Stdin};
use clap::{crate_version, Arg, Command};

pub static BASE_CMD_PARSE_ERROR: i32 = 1;

///'Config':Save arg's value.
pub struct Config{

}

///define args name
pub mod options{

}

impl Config {
    ///Parse equipment from clap
    pub fn from(options: &clap::ArgMatches) -> UResult<Self> {

    }
}
///parse cmd args to fill Config.
pub fn parse_mount_cmd_args(args: impl uucore::Args, about: &str, usage: &str) -> UResult<Config> {

}
///Define terminal tool structure and args,use uucore.
pub fn base_app<'a>(about: &'a str, usage: &'a str) -> Command<'a> {

}
///Gets the input data according to the configuration, which can be a file or stdin
pub fn get_input<'a>(config: &Config, stdin_ref: &'a Stdin) -> UResult<Box<dyn Read + 'a>> {

}
///Input data is processed,  wrapped or ignored spam characters as configured
pub fn handle_input<R: Read>(
    input: &mut R,
    format: Format,
    line_wrap: Option<usize>,
    ignore_garbage: bool,
    decode: bool,
) -> UResult<()> {}
