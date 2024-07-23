use std::any::Any;
use std::ffi::OsString;
use std::io::{Read, Stdin};
use clap::{crate_version, Arg, Command, ArgGroup};
use clap::error::ContextValue::String;
use uucore::error::UResult;
use uucore::format_usage;
use crate::mount_common::options::OPTIONS_SOURCE;

pub static BASE_CMD_PARSE_ERROR: i32 = 1;

///保存参数
#[derive(Debug)]
pub struct Config{
    pub all: bool,                   // -a, --all
    pub no_canonicalize: bool,       // -c, --no-canonicalize
    pub fake: bool,                  // -f, --fake
    pub fork: bool,                  // -F, --fork
    pub fstab: Option<OsString>,     // -T, --fstab <路径>
    pub internal_only: bool,         // -i, --internal-only
    pub show_labels: bool,           // -l, --show-labels
    pub no_mtab: bool,               // -n, --no-mtab
    pub options_mode: Option<OsString>,// --options-mode <mode>
    pub options_source: Option<OsString>, // --options-source <source>
    pub options_source_force: bool,  // --options-source-force
    pub options: Option<OsString>,     // -o, --options <列表>
    pub test_opts: Option<OsString>,   // -O, --test-opts <列表>
    pub read_only: bool,             // -r, --read-only
    pub types: Option<OsString>,       // -t, --types <列表>
    pub source: Option<OsString>,    // --source <源>
    pub target: Option<OsString>,    // --target <目标>
    pub target_prefix: Option<OsString>, // --target-prefix <path>
    pub verbose: bool,               // -v, --verbose
    pub read_write: bool,            // -w, --rw, --read-write
    pub namespace: Option<OsString>, // -N, --namespace <ns>
    pub help: bool,                  // -h, --help
    pub version: bool,               // -V, --version

    // Source
    pub label: Option<OsString>,     // -L, --label <label>
    pub uuid: Option<OsString>,      // -U, --uuid <uuid>
    pub device: Option<OsString>,    // <设备>

    // 操作
    pub bind: bool,                  // -B, --bind
    pub move_: bool,                 // -M, --move
    pub rbind: bool,                 // -R, --rbind
    pub make_shared: bool,           // --make-shared
    pub make_slave: bool,            // --make-slave
    pub make_private: bool,          // --make-private
    pub make_unbindable: bool,       // --make-unbindable
    pub make_rshared: bool,          // --make-rshared
    pub make_rslave: bool,           // --make-rslave
    pub make_rprivate: bool,         // --make-rprivate
    pub make_runbindable: bool,      // --make-runbindable
}


///定义参数的值
pub mod options{
    pub static ALL: &str = "all";                       // -a, --all
    ///
    pub static NO_CANONICALIZE: &str = "no-canonicalize"; // -c, --no-canonicalize
    ///
    pub static FAKE: &str = "fake";                     // -f, --fake
    ///
    pub static FORK: &str = "fork";                     // -F, --fork
    ///
    pub static FSTAB: &str = "fstab";                   // -T, --fstab
    ///
    pub static INTERNAL_ONLY: &str = "internal-only";   // -i, --internal-only
    ///
    pub static SHOW_LABELS: &str = "show-labels";       // -l, --show-labels
    ///
    pub static NO_MTAB: &str = "no-mtab";               // -n, --no-mtab
    ///
    pub static OPTIONS_MODE: &str = "options-mode";     // --options-mode
    ///
    pub static OPTIONS_SOURCE: &str = "options-source"; // --options-source
    ///
    pub static OPTIONS_SOURCE_FORCE: &str = "options-source-force"; // --options-source-force
    ///
    pub static OPTIONS: &str = "options";               // -o, --options
    ///
    pub static TEST_OPTS: &str = "test-opts";           // -O, --test-opts
    ///
    pub static READ_ONLY: &str = "read-only";           // -r, --read-only
    ///
    pub static TYPES: &str = "types";                   // -t, --types
    ///
    pub static SOURCE: &str = "source";                 // --source
    ///
    pub static TARGET: &str = "target";                 // --target
    ///
    pub static TARGET_PREFIX: &str = "target-prefix";   // --target-prefix
    ///
    pub static VERBOSE: &str = "verbose";               // -v, --verbose
    ///
    pub static READ_WRITE: &str = "read-write";         // -w, --rw, --read-write
    ///
    pub static NAMESPACE: &str = "namespace";           // -N, --namespace
    ///
    pub static HELP: &str = "help";                     // -h, --help
    ///
    pub static VERSION: &str = "version";               // -V, --version

    // Source
    ///
    pub static LABEL: &str = "label";                   // -L, --label
    ///
    pub static UUID: &str = "uuid";                     // -U, --uuid
    ///
    pub static DEVICE: &str = "device";                 // <设备>

    // 操作
    ///
    pub static BIND: &str = "bind";                     // -B, --bind
    ///
    pub static MOVE: &str = "move";                     // -M, --move
    ///
    pub static RBIND: &str = "rbind";                   // -R, --rbind
    ///
    pub static MAKE_SHARED: &str = "make-shared";       // --make-shared
    ///
    pub static MAKE_SLAVE: &str = "make-slave";         // --make-slave
    ///
    pub static MAKE_PRIVATE: &str = "make-private";     // --make-private
    ///
    pub static MAKE_UNBINDABLE: &str = "make-unbindable"; // --make-unbindable
    ///
    pub static MAKE_RSHARED: &str = "make-rshared";     // --make-rshared
    ///
    pub static MAKE_RSLAVE: &str = "make-rslave";       // --make-rslave
    ///
    pub static MAKE_RPRIVATE: &str = "make-rprivate";   // --make-rprivate
    ///
    pub static MAKE_RUNBINDABLE: &str = "make-runbindable"; // --make-runbindable

}

impl Config {
    ///从clap中解析配置
    pub fn from(options: &clap::ArgMatches) -> UResult<Self> {
        Ok(Self{
            all: options.is_present(options::ALL),
            no_canonicalize: options.is_present(options::NO_CANONICALIZE),
            fake: options.is_present(options::FAKE),
            fork: options.is_present(options::FORK),
            fstab: options.value_of_os(options::FSTAB).map(OsString::from),
            internal_only: options.is_present(options::INTERNAL_ONLY),
            show_labels: options.is_present(options::SHOW_LABELS),
            no_mtab: options.is_present(options::NO_MTAB),
            options_mode: options.value_of_os(options::OPTIONS_MODE).map(OsString::from),
            options_source: options.value_of_os(options::OPTIONS_SOURCE).map(OsString::from),
            options_source_force: options.is_present(options::OPTIONS_SOURCE_FORCE),
            options: options.value_of_os(options::OPTIONS).map(OsString::from),
            test_opts: options.value_of_os(options::TEST_OPTS).map(OsString::from),
            read_only: options.is_present(options::READ_ONLY),
            types: options.value_of_os(options::TYPES).map(OsString::from),
            source: options.value_of_os(options::SOURCE).map(OsString::from),
            target: options.value_of_os(options::TARGET).or_else(||options.value_of_os("target_positional")).map(OsString::from),
            target_prefix: options.value_of_os(options::TARGET_PREFIX).map(OsString::from),
            verbose: options.is_present(options::VERBOSE),
            read_write: options.is_present(options::READ_WRITE),
            namespace: options.value_of_os(options::NAMESPACE).map(OsString::from),
            help: options.is_present(options::HELP),
            version: options.is_present(options::VERSION),
            label: options.value_of_os(options::LABEL).map(OsString::from),
            uuid: options.value_of_os(options::UUID).map(OsString::from),
            device: options.value_of_os(options::DEVICE).or_else(||options.value_of_os(options::SOURCE)).map(OsString::from).map(OsString::from),
            bind: options.is_present(options::BIND),
            move_: options.is_present(options::MOVE),
            rbind: options.is_present(options::RBIND),
            make_shared: options.is_present(options::MAKE_SHARED),
            make_slave: options.is_present(options::MAKE_SLAVE),
            make_private: options.is_present(options::MAKE_PRIVATE),
            make_unbindable: options.is_present(options::MAKE_UNBINDABLE),
            make_rshared: options.is_present(options::MAKE_RSHARED),
            make_rslave: options.is_present(options::MAKE_RSLAVE),
            make_rprivate: options.is_present(options::MAKE_RPRIVATE),
            make_runbindable: options.is_present(options::MAKE_RUNBINDABLE),
        })



    }
}
///解析参数并填充Config结构体
pub fn parse_mount_cmd_args(args: impl uucore::Args, about: &str, usage: &str) -> UResult<Config> {
    let command = mount_app(about,usage);
    let args_list = args.collect_lossy();
    match command.try_get_matches_from(args_list) {
        Ok(matches) => Config::from(&matches),
        Err(e) => Err(uucore::error::USimpleError::new(BASE_CMD_PARSE_ERROR,e.to_string()))
    }
}
///定义命令行应用结构和参数，用uucore简化代码
pub fn mount_app<'a>(about: &'a str, usage: &'a str) -> Command<'a> {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(about)
        .override_usage(format_usage(usage))
        .infer_long_args(true)
        .arg(
            Arg::new("target_positional").takes_value(true).help("指明挂载点").index(2).allow_invalid_utf8(true)
        )
        .arg(
            Arg::new(options::ALL).short('a').long(options::ALL).help("挂载fstab中的所有文件系统")
        ).arg(
            Arg::new(options::NO_CANONICALIZE).short('c').long(options::NO_CANONICALIZE).help("不对路径规范化")
        ).arg(
            Arg::new(options::FAKE).short('f').long(options::FAKE).help("空运行；跳过 mount(2) 系统调用")
        ).arg(
            Arg::new(options::FORK).short('F').long(options::FORK).help("对每个设备禁用 fork(和 -a 选项一起使用)")
        ).arg(
            Arg::new(options::FSTAB).short('T').long(options::FSTAB).takes_value(true).help("/etc/fstab 的替代文件")
        ).arg(
            Arg::new(options::INTERNAL_ONLY).short('i').long(options::INTERNAL_ONLY).help("不调用 mount.<type> 辅助程序")
        ).arg(
            Arg::new(options::SHOW_LABELS).short('l').long(options::SHOW_LABELS).help("也显示文件系统标签")
        ).arg(
            Arg::new(options::NO_MTAB).short('n').long(options::NO_MTAB).help("不写 /etc/mtab")
        ).arg(
            Arg::new(options::OPTIONS_MODE).long(options::OPTIONS_MODE).takes_value(true).help("what to do with options loaded from fstab")
        ).arg(
            Arg::new(options::OPTIONS_SOURCE).long(options::OPTIONS_SOURCE).takes_value(true).help(" mount options source")
        ).arg(
            Arg::new(options::OPTIONS_SOURCE_FORCE).long(options::OPTIONS_SOURCE_FORCE).help("force use of options from fstab/mtab")
        ).arg(
            Arg::new(options::OPTIONS).short('o').long(options::OPTIONS).takes_value(true).help("挂载选项列表，以英文逗号分隔")
        ).arg(
            Arg::new(options::TEST_OPTS).short('O').long(options::TEST_OPTS).takes_value(true).help("限制文件系统集合(和 -a 选项一起使用)")
        ).arg(
            Arg::new(options::READ_ONLY).short('r').long(options::READ_ONLY).help("以只读方式挂载文件系统(同 -o ro)")
        ).arg(
            Arg::new(options::TYPES).short('t').long(options::TYPES).takes_value(true).help("限制文件系统类型集合")
        ).arg(
            Arg::new(options::SOURCE).long(options::SOURCE).takes_value(true).help("指明源(路径、标签、uuid)")
        ).arg(
            Arg::new(options::TARGET).long(options::TARGET).takes_value(true).help("指明挂载点")
        ).arg(
            Arg::new(options::TARGET_PREFIX).long(options::TARGET_PREFIX).takes_value(true).help("specifies path used for all mountpoints")
        ).arg(
            Arg::new(options::VERBOSE).short('v').long(options::VERBOSE).help("打印当前进行的操作")
        ).arg(
            Arg::new(options::READ_WRITE).short('w').long(options::READ_WRITE).help("以读写方式挂载文件系统(默认)")
        ).arg(
            Arg::new(options::NAMESPACE).short('N').long(options::NAMESPACE).takes_value(true).help("perform mount in another namespace")
        ).arg(
            Arg::new(options::HELP).short('h').long(options::HELP).help("display this help")
        ).arg(
            Arg::new(options::VERSION).short('V').long(options::VERSION).help("display version")
        ).arg(
            Arg::new(options::LABEL).short('L').long(options::LABEL).takes_value(true).help("synonym for LABEL=<label>")
        ).arg(
            Arg::new(options::UUID).short('U').long(options::UUID).takes_value(true).help("synonym for UUID=<uuid>")
        ).arg(
            Arg::new(options::DEVICE).takes_value(true).help(" 按路径指定设备").index(1).allow_invalid_utf8(true)
        ).arg(
            Arg::new(options::BIND).short('B').long(options::BIND).help("挂载其他位置的子树(同 -o bind)")
        ).arg(
            Arg::new(options::MOVE).short('M').long(options::MOVE).help("将子树移动到其他位置")
        ).arg(
            Arg::new(options::RBIND).short('R').long(options::RBIND).help("挂载其他位置的子树及其包含的所有子挂载(submount)")
        ).arg(
            Arg::new(options::MAKE_SHARED).long(options::MAKE_SHARED).help("将子树标记为 共享")
        ).arg(
            Arg::new(options::MAKE_SLAVE).long(options::MAKE_SLAVE).help("将子树标记为 从属")
        ).arg(
            Arg::new(options::MAKE_PRIVATE).long(options::MAKE_PRIVATE).help("将子树标记为 私有")
        ).arg(
            Arg::new(options::MAKE_UNBINDABLE).long(options::MAKE_UNBINDABLE).help("将子树标记为 不可绑定")
        ).arg(
            Arg::new(options::MAKE_RSHARED).long(options::MAKE_RSHARED).help("递归地将整个子树标记为 共享")
        ).arg(
            Arg::new(options::MAKE_RSLAVE).long(options::MAKE_RSLAVE).help("递归地将整个子树标记为 从属")
        ).arg(
            Arg::new(options::MAKE_RPRIVATE).long(options::MAKE_RPRIVATE).help("递归地将整个子树标记为 私有")
        ).arg(
            Arg::new(options::MAKE_RUNBINDABLE).long(options::MAKE_RUNBINDABLE).help("递归地将整个子树标记为 不可绑定")
        ).group(
        ArgGroup::new("operation")
            .args(&[options::BIND, options::MOVE, options::RBIND,options::MAKE_SHARED, options::MAKE_SLAVE, options::MAKE_PRIVATE,
                options::MAKE_UNBINDABLE, options::MAKE_RSHARED, options::MAKE_RSLAVE, options::MAKE_RPRIVATE, options::MAKE_RUNBINDABLE])
            .required(false)
        )
        .group(
            ArgGroup::new("source_operation")
                .args(&[options::LABEL,options::UUID, options::DEVICE,options::SOURCE])
                .required(false)
        )
        .group(
            ArgGroup::new("read_write_mode")
                .args(&[options::READ_ONLY, options::READ_WRITE])
                .required(false)
        )
        .group(
            ArgGroup::new("options_source")
                .args(&[options::OPTIONS_SOURCE, options::OPTIONS_SOURCE_FORCE])
                .required(false)
        )
        .trailing_var_arg(true)
}