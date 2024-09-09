use std::ffi::OsString;
use std::fs::File;
use std::{fs, io};
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::os::fd::AsRawFd;
use std::path::Path;
use clap::{crate_version, Arg, Command, ArgGroup};
use nix::mount::{MntFlags, MsFlags};
use uucore::error::{UResult, USimpleError};
use uucore::format_usage;
use nix::sched::{setns, CloneFlags};
use uucore::umount::umount_fs;
use std::sync::Mutex;
use once_cell::sync::Lazy;
pub static BASE_CMD_PARSE_ERROR: i32 = 1;

#[derive(Debug, Default)]
pub struct Config {
    pub all: bool,
    pub all_targets: bool,
    pub no_canonicalize: bool,
    pub detach_loop: bool,
    pub fake: bool,
    pub force: bool,
    pub internal_only: bool,
    pub no_mtab: bool,
    pub lazy: bool,
    pub recursive: bool,
    pub read_only: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub help: bool,
    pub version: bool,

    pub test_opts: Option<OsString>,
    pub types: Option<OsString>,
    pub namespace: Option<OsString>,

    pub target: Option<OsString>,
}

pub mod options {
    pub static ALL: &str = "all";
    pub static ALL_TARGETS: &str = "all-targets";
    pub static NO_CANONICALIZE: &str = "no-canonicalize";
    pub static DETACH_LOOP: &str = "detach-loop";
    pub static FAKE: &str = "fake";
    pub static FORCE: &str = "force";
    pub static INTERNAL_ONLY: &str = "internal-only";
    pub static NO_MTAB: &str = "no-mtab";
    pub static LAZY: &str = "lazy";
    pub static TEST_OPTS: &str = "test-opts";
    pub static RECURSIVE: &str = "recursive";
    pub static READ_ONLY: &str = "read-only";
    pub static TYPES: &str = "types";
    pub static VERBOSE: &str = "verbose";
    pub static QUIET: &str = "quiet";
    pub static NAMESPACE: &str = "namespace";
    pub static HELP: &str = "help";
    pub static VERSION: &str = "version";
}

impl Config {
    pub fn from(options: &clap::ArgMatches) -> UResult<Self> {
        Ok(Self {
            all: options.is_present(options::ALL),
            all_targets: options.is_present(options::ALL_TARGETS),
            no_canonicalize: options.is_present(options::NO_CANONICALIZE),
            detach_loop: options.is_present(options::DETACH_LOOP),
            fake: options.is_present(options::FAKE),
            force: options.is_present(options::FORCE),
            internal_only: options.is_present(options::INTERNAL_ONLY),
            no_mtab: options.is_present(options::NO_MTAB),
            lazy: options.is_present(options::LAZY),
            recursive: options.is_present(options::RECURSIVE),
            read_only: options.is_present(options::READ_ONLY),
            verbose: options.is_present(options::VERBOSE),
            quiet: options.is_present(options::QUIET),
            help: options.is_present(options::HELP),
            version: options.is_present(options::VERSION),

            test_opts: options.value_of_os(options::TEST_OPTS).map(OsString::from),
            types: options.value_of_os(options::TYPES).map(OsString::from),
            namespace: options.value_of_os(options::NAMESPACE).map(OsString::from),

            target: options.value_of_os("target").map(OsString::from),
        })
    }
}

pub fn parse_umount_cmd_args(args: impl uucore::Args, about: &str, usage: &str) -> UResult<Config> {
    let command = umount_app(about, usage);
    let args_list = args.collect_lossy();
    match command.try_get_matches_from(args_list) {
        Ok(matches) => Config::from(&matches),
        Err(e) => Err(USimpleError::new(BASE_CMD_PARSE_ERROR, e.to_string()))
    }
}

pub fn umount_app<'a>(about: &'a str, usage: &'a str) -> Command<'a> {
    let mut cmd = Command::new(uucore::util_name())
        .version(crate_version!())
        .about(about)
        .override_usage(format_usage(usage))
        .infer_long_args(true);

    cmd = cmd.arg(Arg::new("target").help("指定要卸载的目标").index(1).allow_invalid_utf8(true));

    for (name, short, help) in &[
        (options::ALL, Some('a'), "卸载所有文件系统"),
        (options::ALL_TARGETS, Some('A'), "卸载当前名字空间内指定设备对应的所有挂载点"),
        (options::NO_CANONICALIZE, Some('c'), "不对路径规范化"),
        (options::DETACH_LOOP, Some('d'), "若挂载了回环设备，也释放该回环设备"),
        (options::FAKE, None, "空运行；跳过 umount(2) 系统调用"),
        (options::FORCE, Some('f'), "强制卸载(遇到不响应的 NFS 系统时)"),
        (options::INTERNAL_ONLY, Some('i'), "不调用 umount.<类型> 辅助程序"),
        (options::NO_MTAB, Some('n'), "不写 /etc/mtab"),
        (options::LAZY, Some('l'), "立即断开文件系统，清理以后执行"),
        (options::RECURSIVE, Some('R'), "递归卸载目录及其子对象"),
        (options::READ_ONLY, Some('r'), "若卸载失败，尝试以只读方式重新挂载"),
        (options::VERBOSE, Some('v'), "打印当前进行的操作"),
        (options::QUIET, Some('q'), "suppress 'not mounted' error messages"),
        (options::HELP, Some('h'), "display this help"),
        (options::VERSION, Some('V'), "display version"),
    ] {
        let arg = Arg::new(*name).long(*name).help(*help);
        cmd = cmd.arg(if let Some(s) = short { arg.short(*s) } else { arg });
    }

    for (name, short, help, value_name) in &[
        (options::TEST_OPTS, Some('O'), "限制文件系统集合(和 -a 选项一起使用)", "列表"),
        (options::TYPES, Some('t'), "限制文件系统集合", "列表"),
        (options::NAMESPACE, Some('N'), "perform umount in another namespace", "ns"),
    ] {
        let arg = Arg::new(*name).long(*name).help(*help).value_name(*value_name).takes_value(true).allow_invalid_utf8(true);
        cmd = cmd.arg(if let Some(s) = short { arg.short(*s) } else { arg });
    }

    cmd
}

pub struct UmountHandler {
    config: Config
}
static MTAB_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

impl UmountHandler {
    pub fn new(config: Config) -> UmountHandler {
        Self { config }
    }

    pub fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.handle_namespace()?;
        self.handle_basic_options()?;
        self.handle_umount_options()?;
        self.handle_target()?;
        Ok(())
    }

    fn handle_namespace(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ns) = &self.config.namespace {
            self.verbose_print(&format!("Using namespace: {:?}", ns));
            // Implement namespace switching logic here
            self.enter_namespace()?;
        }
        Ok(())
    }

    fn handle_basic_options(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.all {
            self.umount_all_filesystems()?;

        }
        if self.config.all_targets {
            self.umount_all_targets()?;
        }
        if self.config.no_canonicalize {
            self.verbose_print("Path canonicalization disabled");
        }
        if self.config.fake {
            self.verbose_print("Running in fake mode - no actual unmounting will occur");
        }
        Ok(())
    }

    fn handle_umount_options(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.force {
            self.verbose_print("Force unmount enabled");
        }
        if self.config.lazy {
            self.verbose_print("Lazy unmount enabled");
        }
        if self.config.recursive {
            self.verbose_print("Recursive unmount enabled");
        }
        if self.config.read_only {
            self.verbose_print("Read-only remount on failure enabled");
        }
        Ok(())
    }

    fn handle_target(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(target) = &self.config.target {
            let target_path = if self.config.no_canonicalize {
                target.to_string_lossy().into_owned()
            } else {
                self.canonicalize_path(target.to_str().unwrap())?
            };
            if self.config.recursive {
                self.umount_recursive(&target_path)?;
            } else {
                self.umount_single_target(&target_path)?;
            }
        }
        Ok(())
    }

    fn umount_all_filesystems(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose_print("Unmounting all filesystems");
        // Implement logic to unmount all filesystems
        if !self.config.fake {
            let mounts = fs::read_to_string("/proc/mounts")?;
            let mut mounted = Vec::new();

            for line in mounts.lines().rev() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 3 {
                    let mount_point = fields[1];
                    let fs_type = fields[2];

                    if self.should_umount(mount_point, fs_type) {
                        mounted.push(mount_point);
                    }
                }
            }

            for mount_point in mounted {
                self.umount_single_target(mount_point)?;
            }
        }
        Ok(())
    }
    fn should_umount(&self, mount_point: &str, fs_type: &str) -> bool {
        if let Some(types) = &self.config.types {
            let types_str = types.to_str().unwrap_or_else(|| {
                log::warn!("无法将文件系统类型转换为字符串，使用空字符串");
                ""
            });
            let allowed_types: HashSet<_> = types_str.split(',').collect();
            if !allowed_types.contains(fs_type) {
                return false;
            }
        }

        if let Some(test_opts) = &self.config.test_opts {
            let test_opts_str = test_opts.to_str().unwrap_or_else(|| {
                log::warn!("无法将测试选项转换为字符串，使用空字符串");
                ""
            });
            let mount_opts = self.get_mount_options(mount_point);
            let required_opts: HashSet<_> = test_opts_str.split(',').collect::<HashSet<_>>();
            if !required_opts.iter().all(|opt| mount_opts.contains(*opt)) {
                return false;
            }
        }
        true
    }

    fn get_mount_options(&self, mount_point: &str) -> HashSet<String> {
        let mut options = HashSet::new();
        if let Ok(file) = File::open("/proc/mounts") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 4 && fields[1] == mount_point {
                        options = fields[3].split(',').map(String::from).collect();
                        break;
                    }
                }
            }
        }
        options
    }
    fn umount_all_targets(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose_print("Unmounting all targets for the specified device");
        // Implement logic to unmount all targets for a device
        // 读取 /proc/mounts 文件获取所有挂载点信息
        let mounts = fs::read_to_string("/proc/mounts")?;
        let device_to_unmount = self.config.target.as_ref()
            .ok_or("No device specified for unmounting all targets")?;

        // 遍历所有挂载点，找到匹配的设备并卸载
        for line in mounts.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 2 && fields[0] == device_to_unmount {
                let mount_point = fields[1];
                self.verbose_print(&format!("Unmounting target: {} for device: {:?}", mount_point, device_to_unmount.to_str()));

                if !self.config.fake {
                    self.umount_single_target(mount_point)?;
                }
            }
        }

        Ok(())
    }

    fn umount_single_target(&self, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose_print(&format!("Unmounting target: {:?}", target));
        // Implement logic to unmount a single target
        let loop_device = self.get_loop_device(target);
        if !self.config.fake {
            if !nix::unistd::geteuid().is_root() {
                return Err("需要 root 权限来卸载文件系统".into());
            }
            // 这里使用 umount_fs 函数来实际执行卸载操作
            let result = if self.config.force || self.config.lazy {
                let mut flags = MntFlags::empty();
                if self.config.force {
                    flags |= MntFlags::MNT_FORCE;
                }
                if self.config.lazy {
                    flags |= MntFlags::MNT_DETACH;
                }
                umount_fs::<&str>(target.as_ref(), flags, self.config.internal_only)
            } else {
                let mut flags = MntFlags::empty();
                umount_fs::<&str>(target.as_ref(), flags, self.config.internal_only)
            };
            match result {
                Ok(_) => {
                    self.verbose_print(&format!("Successfully unmounted {}", target));
                    if self.config.detach_loop {
                        if let Ok(device) = loop_device {
                            match self.detach_loop_device(&device) {
                                Ok(_) => self.verbose_print(&format!("Successfully detached loop device {}", device)),
                                Err(e) => self.verbose_print(&format!("Failed to detach loop device {}: {}", device, e)),
                            }
                        } else {
                            self.verbose_print("No loop device found to detach");
                        }
                    }
                    if !self.config.no_mtab {
                        self.update_mtab(target)?;
                    }
                },
                Err(e) => {
                    if self.config.read_only {
                        self.verbose_print(&format!("Unmount failed, attempting read-only remount for {}", target));
                        self.remount_read_only(target)?;
                    } else if !self.config.quiet{
                        eprintln!("卸载 {} 失败: {}", target, e);
                        return Err(Box::new(e));
                    }
                }
            }
        }
        Ok(())
    }

    fn verbose_print(&self, message: &str) {
        if self.config.verbose && !self.config.quiet {
            println!("详细信息: {}", message);
        }
    }

    fn enter_namespace(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ns) = &self.config.namespace {
            self.verbose_print(&format!("正在进入命名空间: {:?}", ns));

            let ns_file = File::open(ns).map_err(|e| format!("打开命名空间文件失败: {}", e))?;

            let _guard = scopeguard::guard(ns_file, |f| drop(f));

            setns(_guard.as_raw_fd(), CloneFlags::CLONE_NEWNS)
                .map_err(|e| format!("进入命名空间失败: {}", e))?;


            self.verbose_print("成功进入指定的命名空间");
        }
        Ok(())
    }
    fn detach_loop_device(&self, device: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose_print(&format!("Attempting to detach loop device for {}", device));
        // 打开设备文件
        let file = File::open(&device)?;
        let fd = file.as_raw_fd();

        // LOOP_CLR_FD 的 ioctl 请求码
        const LOOP_CLR_FD: nix::libc::c_ulong = 0x4C01;

        // 执行 ioctl 调用
        unsafe {
            if nix::libc::ioctl(fd, LOOP_CLR_FD, 0) == -1 {
                return Err(Box::new(std::io::Error::last_os_error()));
            }
        }
        Ok(())
    }
    fn get_loop_device(&self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.verbose_print(&format!("Attempting to find loop device for target: {}", target));
        let target_path = Path::new(target).canonicalize()?;
        // 方法1: 检查 /proc/mounts
        let mounts = fs::read_to_string("/proc/mounts")?;
        for line in mounts.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() > 1 {
                let mount_point = Path::new(fields[1]).canonicalize().unwrap_or_else(|_| Path::new(fields[1]).to_path_buf());
                if mount_point == target_path && fields[0].starts_with("/dev/loop") {
                    self.verbose_print(&format!("Found loop device in /proc/mounts: {}", fields[0]));
                    return Ok(fields[0].to_string());
                }
            }
        }


        // 方法2: 使用 losetup 命令
        let output = std::process::Command::new("losetup")
            .arg("-a")
            .output()?;
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                let device = parts[0];
                let file_path = parts[1].trim_start_matches('(').trim_end_matches(')');
                if Path::new(file_path).canonicalize()? == target_path {
                    self.verbose_print(&format!("Found loop device using losetup: {}", device));
                    return Ok(device.to_string());
                }
            }
        }
        self.verbose_print("No loop device found for the given target");
        Err("No loop device found for the given target".into())
    }
    fn remount_read_only(&self, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose_print(&format!("Remounting {} as read-only", target));
        let path = Path::new(target);
        nix::mount::mount(
            None::<&str>,
            path,
            None::<&str>,
            MsFlags::MS_REMOUNT | MsFlags::MS_RDONLY,
            None::<&str>,
        )?;
        Ok(())
    }
    fn canonicalize_path(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let canonical_path = fs::canonicalize(path)?;
        Ok(canonical_path.to_string_lossy().into_owned())
    }
    fn umount_recursive(&self, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(target);
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Err(e) = self.umount_recursive(path.to_str().ok_or("无效路径")?) {
                        log::warn!("递归卸载 {} 时出错: {}", path.display(), e);
                    }
                }
            }
        }
        self.umount_single_target(target)
    }

    fn update_mtab(&self, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose_print(&format!("更新 /etc/mtab，移除 {}", target));

        if fs::symlink_metadata("/etc/mtab")?.file_type().is_symlink() {
            self.verbose_print("/etc/mtab 是符号链接，不需要更新");
            return Ok(());
        }

        let _lock = MTAB_LOCK.lock().unwrap();
        let content = fs::read_to_string("/etc/mtab")?;
        let updated_content: String = content
            .lines()
            .filter(|line| !line.split_whitespace().nth(1).map_or(false, |mp| mp == target))
            .collect::<Vec<&str>>()
            .join("\n");
        fs::write("/etc/mtab", updated_content)?;
        Ok(())
    }
}