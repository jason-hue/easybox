use std::io::{Read, Stdin};
use clap::{crate_version, Arg, Command};
use uucore::error::UResult;

pub static BASE_CMD_PARSE_ERROR: i32 = 1;

///保存参数
pub struct Config{
    
}

///定义参数的值
pub mod options{

}

impl Config {
    ///从clap中解析配置
    pub fn from(options: &clap::ArgMatches) -> UResult<Self> {

    }
}
///解析参数并填充Config结构体
pub fn parse_mount_cmd_args(args: impl uucore::Args, about: &str, usage: &str) -> UResult<Config> {

}
///定义命令行应用结构和参数，用uucore简化代码
pub fn base_app<'a>(about: &'a str, usage: &'a str) -> Command<'a> {

}
///根据配置获取输入数据，可以是file，也可以是 stdin
pub fn get_input<'a>(config: &Config, stdin_ref: &'a Stdin) -> UResult<Box<dyn Read + 'a>> {

}
///输入数据按照配置进行处理、包装或忽略垃圾邮件字符
pub fn handle_input<R: Read>(
    input: &mut R,
    format: Format,
    line_wrap: Option<usize>,
    ignore_garbage: bool,
    decode: bool,
) -> UResult<()> {}
