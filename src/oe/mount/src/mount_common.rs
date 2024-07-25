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
#[derive(Debug, Default)]
pub struct Config {
    // 基本选项
    pub all: bool,// 挂载 /etc/fstab 文件中提到的所有文件系统
    pub no_canonicalize: bool,//不对路径进行规范化处理
    pub fake: bool,//模拟挂载,不实际执行 mount 系统调用
    pub fork: bool,//为每个设备创建一个新进程(与 -a 一起使用)
    pub fstab: Option<OsString>,//指定替代 /etc/fstab 的文件
    pub internal_only: bool,//不调用 mount.<type> 辅助程序
    pub show_labels: bool,//显示文件系统标签
    pub no_mtab: bool,//不写入 /etc/mtab 文件
    pub verbose: bool,//显示详细的操作信息
    pub help: bool,//显示帮助信息
    pub version: bool,//显示版本信息

    // 挂载选项
    pub options: MountOptions,

    // 源和目标
    pub source: Option<Source>,//明确指定源(路径、标签、UUID)
    pub target: Option<OsString>,//明确指定挂载点
    pub target_prefix: Option<OsString>,//为所有挂载点指定路径前缀

    // 命名空间
    pub namespace: Option<OsString>,//在另一个命名空间中执行挂载

    // 操作
    pub operation: Operation,
}

#[derive(Debug, Default)]
pub struct MountOptions {
    pub mode: Option<OsString>,//指定如何处理从 fstab 加载的选项
    pub source: Option<OsString>,//指定挂载选项的来源
    pub source_force: bool,//强制使用来自 fstab/mtab 的选项
    pub options: Option<OsString>,//指定以逗号分隔的挂载选项列表
    pub test_opts: Option<OsString>,//限制文件系统集合(与 -a 选项一起使用)
    pub read_only: bool,//以只读方式挂载文件系统
    pub read_write: bool,//以读写方式挂载文件系统(默认)
    pub types: Option<OsString>,//限制文件系统类型
}

#[derive(Debug)]
pub enum Source {
    Device(OsString),//通过设备路径指定
    Label(OsString),//通过文件系统标签指定设备
    UUID(OsString),//通过文件系统 UUID 指定设备
}

#[derive(Debug, Default)]
pub enum Operation {
    #[default]
    Normal,
    Bind,//将一个子树挂载到其他位置
    Move,//将一个子树移动到其他位置
    RBind,//挂载一个子树及其所有子挂载点到其他位置
    MakeShared,//标记一个子树为共享
    MakeSlave,//标记一个子树为从属
    MakePrivate,//标记一个子树为私有
    MakeUnbindable,//标记一个子树为不可绑定
    MakeRShared,//递归地标记整个子树为共享
    MakeRSlave,//递归地标记整个子树为从属
    MakeRPrivate,//递归地标记整个子树为私有
    MakeRUnbindable,//递归地标记整个子树为不可绑定
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
    pub fn from(options: &clap::ArgMatches) -> UResult<Self> {
        Ok(Self {
            all: options.is_present(options::ALL),
            no_canonicalize: options.is_present(options::NO_CANONICALIZE),
            fake: options.is_present(options::FAKE),
            fork: options.is_present(options::FORK),
            fstab: options.value_of_os(options::FSTAB).map(OsString::from),
            internal_only: options.is_present(options::INTERNAL_ONLY),
            show_labels: options.is_present(options::SHOW_LABELS),
            no_mtab: options.is_present(options::NO_MTAB),
            verbose: options.is_present(options::VERBOSE),
            help: options.is_present(options::HELP),
            version: options.is_present(options::VERSION),

            options: MountOptions {
                mode: options.value_of_os(options::OPTIONS_MODE).map(OsString::from),
                source: options.value_of_os(options::OPTIONS_SOURCE).map(OsString::from),
                source_force: options.is_present(options::OPTIONS_SOURCE_FORCE),
                options: options.value_of_os(options::OPTIONS).map(OsString::from),
                test_opts: options.value_of_os(options::TEST_OPTS).map(OsString::from),
                read_only: options.is_present(options::READ_ONLY),
                read_write: options.is_present(options::READ_WRITE),
                types: options.value_of_os(options::TYPES).map(OsString::from),
            },

            source: Self::parse_source(options),
            target: options.value_of_os(options::TARGET)
                .or_else(|| options.value_of_os("target_positional"))
                .map(OsString::from),
            target_prefix: options.value_of_os(options::TARGET_PREFIX).map(OsString::from),

            namespace: options.value_of_os(options::NAMESPACE).map(OsString::from),

            operation: Self::parse_operation(options),
        })
    }

    fn parse_source(options: &clap::ArgMatches) -> Option<Source> {
        if let Some(label) = options.value_of_os(options::LABEL) {
            Some(Source::Label(label.to_owned()))
        } else if let Some(uuid) = options.value_of_os(options::UUID) {
            Some(Source::UUID(uuid.to_owned()))
        } else {
            options.value_of_os(options::DEVICE)
                .or_else(|| options.value_of_os(options::SOURCE))
                .map(|device| Source::Device(device.to_owned()))
        }
    }

    fn parse_operation(options: &clap::ArgMatches) -> Operation {
        if options.is_present(options::BIND) { Operation::Bind }
        else if options.is_present(options::MOVE) { Operation::Move }
        else if options.is_present(options::RBIND) { Operation::RBind }
        else if options.is_present(options::MAKE_SHARED) { Operation::MakeShared }
        else if options.is_present(options::MAKE_SLAVE) { Operation::MakeSlave }
        else if options.is_present(options::MAKE_PRIVATE) { Operation::MakePrivate }
        else if options.is_present(options::MAKE_UNBINDABLE) { Operation::MakeUnbindable }
        else if options.is_present(options::MAKE_RSHARED) { Operation::MakeRShared }
        else if options.is_present(options::MAKE_RSLAVE) { Operation::MakeRSlave }
        else if options.is_present(options::MAKE_RPRIVATE) { Operation::MakeRPrivate }
        else if options.is_present(options::MAKE_RUNBINDABLE) { Operation::MakeRUnbindable }
        else { Operation::Normal }
    }
    pub fn get_device_path(&self) -> Option<&str> {
        match &self.source {
            Some(Source::Device(device)) => Some(device.to_str().unwrap()),
            _ => None,
        }
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
    let mut cmd = Command::new(uucore::util_name())
        .version(crate_version!())
        .about(about)
        .override_usage(format_usage(usage))
        .infer_long_args(true);

    // 添加位置参数
    cmd = cmd.arg(Arg::new("target_positional").takes_value(true).help("指明挂载点").index(2).allow_invalid_utf8(true))
        .arg(Arg::new(options::DEVICE).takes_value(true).help("按路径指定设备").index(1).allow_invalid_utf8(true));

    // 添加布尔标志
    for (name, short, help) in &[
        (options::ALL, Some('a'), "挂载fstab中的所有文件系统"),
        (options::NO_CANONICALIZE, Some('c'), "不对路径规范化"),
        (options::FAKE, Some('f'), "空运行；跳过 mount(2) 系统调用"),
        (options::FORK, Some('F'), "对每个设备禁用 fork(和 -a 选项一起使用)"),
        (options::INTERNAL_ONLY, Some('i'), "不调用 mount.<type> 辅助程序"),
        (options::SHOW_LABELS, Some('l'), "也显示文件系统标签"),
        (options::NO_MTAB, Some('n'), "不写 /etc/mtab"),
        (options::OPTIONS_SOURCE_FORCE, Some('\0'), "force use of options from fstab/mtab"),
        (options::READ_ONLY, Some('r'), "以只读方式挂载文件系统(同 -o ro)"),
        (options::VERBOSE, Some('v'), "打印当前进行的操作"),
        (options::READ_WRITE, Some('w'), "以读写方式挂载文件系统(默认)"),
        (options::HELP, Some('h'), "display this help"),
        (options::VERSION, Some('V'), "display version"),
    ] {
        let arg = Arg::new(*name).long(*name).help(*help).global(true);
        cmd = cmd.arg(if let Some(s) = short { arg.short(*s) } else { arg });
    }

    // 添加带值的选项
    for (name, short, help) in &[
        (options::FSTAB, Some('T'), "/etc/fstab 的替代文件"),
        (options::OPTIONS_MODE, None, "what to do with options loaded from fstab"),
        (options::OPTIONS_SOURCE, None, "mount options source"),
        (options::OPTIONS, Some('o'), "挂载选项列表，以英文逗号分隔"),
        (options::TEST_OPTS, Some('O'), "限制文件系统集合(和 -a 选项一起使用)"),
        (options::TYPES, Some('t'), "限制文件系统类型集合"),
        (options::SOURCE, None, "指明源(路径、标签、uuid)"),
        (options::TARGET, None, "指明挂载点"),
        (options::TARGET_PREFIX, None, "specifies path used for all mountpoints"),
        (options::NAMESPACE, Some('N'), "perform mount in another namespace"),
        (options::LABEL, Some('L'), "synonym for LABEL=<label>"),
        (options::UUID, Some('U'), "synonym for UUID=<uuid>"),
    ] {
        let arg = Arg::new(*name).long(*name).help(*help).takes_value(true).allow_invalid_utf8(true);
        cmd = cmd.arg(if let Some(s) = short { arg.short(*s) } else { arg });
    }

    // 添加操作选项
    for (name, short, help) in &[
        (options::BIND, Some('B'), "挂载其他位置的子树(同 -o bind)"),
        (options::MOVE, Some('M'), "将子树移动到其他位置"),
        (options::RBIND, Some('R'), "挂载其他位置的子树及其包含的所有子挂载(submount)"),
        (options::MAKE_SHARED, None, "将子树标记为 共享"),
        (options::MAKE_SLAVE, None, "将子树标记为 从属"),
        (options::MAKE_PRIVATE, None, "将子树标记为 私有"),
        (options::MAKE_UNBINDABLE,None, "将子树标记为 不可绑定"),
        (options::MAKE_RSHARED, None, "递归地将整个子树标记为 共享"),
        (options::MAKE_RSLAVE, None, "递归地将整个子树标记为 从属"),
        (options::MAKE_RPRIVATE, None, "递归地将整个子树标记为 私有"),
        (options::MAKE_RUNBINDABLE, None, "递归地将整个子树标记为 不可绑定"),
    ] {
        let arg = Arg::new(*name).long(*name).help(*help);
        cmd = cmd.arg(if let Some(s) = short { arg.short(*s) } else { arg });
    }

    // 添加参数组
    cmd = cmd.group(ArgGroup::new("operation")
        .args(&[options::BIND, options::MOVE, options::RBIND, options::MAKE_SHARED, options::MAKE_SLAVE, options::MAKE_PRIVATE,
            options::MAKE_UNBINDABLE, options::MAKE_RSHARED, options::MAKE_RSLAVE, options::MAKE_RPRIVATE, options::MAKE_RUNBINDABLE])
        .required(false))
        .group(ArgGroup::new("source_operation")
            .args(&[options::LABEL, options::UUID, options::DEVICE, options::SOURCE])
            .required(false))
        .group(ArgGroup::new("read_write_mode")
            .args(&[options::READ_ONLY, options::READ_WRITE])
            .required(false))
        .group(ArgGroup::new("options_source")
            .args(&[options::OPTIONS_SOURCE, options::OPTIONS_SOURCE_FORCE])
            .required(false));

    cmd.trailing_var_arg(true)
}
pub struct ConfigHandler{
    config: Config
}
impl ConfigHandler{
    pub fn new(config: Config) -> ConfigHandler {
        Self{
            config,
        }
    }
    pub fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.handle_basic_options()?;
        self.handle_mount_options()?;
        self.handle_source_and_target()?;
        self.handle_namespace()?;
        self.handle_operation()?;
        Ok(())
    }
    fn handle_basic_options(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.all {
            self.mount_all_filesystems()?;
        }
        if self.config.no_canonicalize {
            println!("Path canonicalization disabled");
        }
        if self.config.fake {
            println!("Running in fake mode - no actual mounting will occur");
        }
        if self.config.fork {
            println!("Forking enabled for each device");
        }
        if let Some(fstab) = &self.config.fstab {
            self.use_alternative_fstab(fstab)?;
        }
        if self.config.internal_only {
            println!("Using internal mount helpers only");
        }
        if self.config.show_labels {
            println!("Filesystem labels will be displayed");
        }
        if self.config.no_mtab {
            println!("/etc/mtab will not be updated");
        }
        if self.config.verbose {
            println!("Verbose mode enabled");
        }
        Ok(())
    }
    fn handle_mount_options(&self) -> Result<(), Box<dyn std::error::Error>> {
        let options = &self.config.options;

        if let Some(mode) = &options.mode {
            println!("Options mode: {:?}", mode);
        }
        if let Some(source) = &options.source {
            println!("Options source: {:?}", source);
        }
        if options.source_force {
            println!("Forcing use of options from fstab/mtab");
        }
        if let Some(opts) = &options.options {
            println!("Mount options: {:?}", opts);
        }
        if let Some(test_opts) = &options.test_opts {
            println!("Test options: {:?}", test_opts);
        }
        if options.read_only {
            println!("Mounting read-only");
        }
        if options.read_write {
            println!("Mounting read-write");
        }
        if let Some(types) = &options.types {
            println!("Filesystem types: {:?}", types);
        }

        Ok(())
    }

    fn handle_source_and_target(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(source) = &self.config.source {
            match source {
                Source::Device(device) => println!("Source device: {:?}", device),
                Source::Label(label) => println!("Source label: {:?}", label),
                Source::UUID(uuid) => println!("Source UUID: {:?}", uuid),
            }
        }

        if let Some(target) = &self.config.target {
            println!("Mount target: {:?}", target);
        }

        if let Some(prefix) = &self.config.target_prefix {
            println!("Target prefix: {:?}", prefix);
        }

        Ok(())
    }

    fn handle_namespace(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ns) = &self.config.namespace {
            println!("Using namespace: {:?}", ns);
        }
        Ok(())
    }

    fn handle_operation(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.config.operation {
            Operation::Normal => self.perform_normal_mount()?,
            Operation::Bind => self.perform_bind_mount()?,
            Operation::Move => self.perform_move_mount()?,
            Operation::RBind => self.perform_rbind_mount()?,
            Operation::MakeShared => self.make_mount_shared()?,
            Operation::MakeSlave => self.make_mount_slave()?,
            Operation::MakePrivate => self.make_mount_private()?,
            Operation::MakeUnbindable => self.make_mount_unbindable()?,
            Operation::MakeRShared => self.make_mount_rshared()?,
            Operation::MakeRSlave => self.make_mount_rslave()?,
            Operation::MakeRPrivate => self.make_mount_rprivate()?,
            Operation::MakeRUnbindable => self.make_mount_runbindable()?,
        }
        Ok(())
    }
    // 辅助方法
    fn mount_all_filesystems(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Mounting all filesystems from /etc/fstab");
        // 实现挂载所有文件系统的逻辑
        Ok(())
    }
    fn use_alternative_fstab(&self, fstab: &OsString) -> Result<(), Box<dyn std::error::Error>> {
        println!("Using alternative fstab: {:?}", fstab);
        // 实现使用替代 fstab 文件的逻辑
        Ok(())
    }
    fn perform_normal_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Performing normal mount");
        // 实现正常挂载的逻辑
        Ok(())
    }

    fn perform_bind_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Performing bind mount");
        // 实现绑定挂载的逻辑
        Ok(())
    }

    fn perform_move_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Performing move mount");
        // 实现移动挂载的逻辑
        Ok(())
    }

    fn perform_rbind_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Performing rbind mount");
        // 实现递归绑定挂载的逻辑
        Ok(())
    }

    fn make_mount_shared(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount shared");
        // 实现设置共享挂载的逻辑
        Ok(())
    }

    fn make_mount_slave(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount slave");
        // 实现设置从属挂载的逻辑
        Ok(())
    }

    fn make_mount_private(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount private");
        // 实现设置私有挂载的逻辑
        Ok(())
    }

    fn make_mount_unbindable(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount unbindable");
        // 实现设置不可绑定挂载的逻辑
        Ok(())
    }

    fn make_mount_rshared(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount recursively shared");
        // 实现设置递归共享挂载的逻辑
        Ok(())
    }

    fn make_mount_rslave(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount recursively slave");
        // 实现设置递归从属挂载的逻辑
        Ok(())
    }

    fn make_mount_rprivate(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount recursively private");
        // 实现设置递归私有挂载的逻辑
        Ok(())
    }

    fn make_mount_runbindable(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making mount recursively unbindable");
        // 实现设置递归不可绑定挂载的逻辑
        Ok(())
    }
}