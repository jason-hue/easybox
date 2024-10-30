//! This file is part of the easybox package.
//
// (c) Zhenghang <2113130664@qq.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

use std::sync::Mutex;

use crate::{
    common::util::*, test_attr::run_cmd_as_root_ignore_ci, test_hwclock::run_ucmd_as_root_ignore_ci,
};
static KEEP_SINGLE_THREAD: Mutex<bool> = Mutex::new(false);
pub const C_MOUNT_PATH: &str = "/usr/bin/mount";
pub const C_UMOUNT_PATH: &str = "/usr/bin/umount";
pub const C_MKDIR_PATH: &str = "/usr/bin/mkdir";
pub const C_DD_PATH: &str = "/usr/bin/dd";
pub const C_MKFS_PATH: &str = "/usr/bin/mkfs.ext4";
pub const C_LOSETUP_PATH: &str = "/usr/bin/losetup";
pub const TEST_MOUNT_POINT: &str = "mount_point";
pub const TEST_MOUNT_SRC: &str = "/dev/loop";

fn setup_loop_device(ts: &TestScenario) -> String {
    const TEST_TEMP_FILE:&str = "ext4.img";
    ts.cmd(C_MKDIR_PATH).arg(TEST_MOUNT_POINT).run();
    ts.cmd(C_DD_PATH).args(&["if=/dev/zero", &format!("of={}", TEST_TEMP_FILE), "bs=1M", "count=5"]).run();
    ts.cmd(C_MKFS_PATH).arg(TEST_TEMP_FILE).run();
    let losetup_res = run_cmd_as_root_ignore_ci(ts, C_LOSETUP_PATH, &["-f", "--show", TEST_TEMP_FILE]).unwrap();
    losetup_res.stdout_str().trim().to_string()
}

fn run_and_compare(ts: &TestScenario, in_args: &[&str]) {
    let _lock = KEEP_SINGLE_THREAD.lock();
    let loopdevice = &setup_loop_device(ts);
    let mut args = Vec::from(in_args);
    for i in 0..args.len() {
        if args[i] == TEST_MOUNT_SRC {
            args[i] = loopdevice;
        }
    }

    // Run C programe
    let c_res = run_cmd_as_root_ignore_ci(ts, C_MOUNT_PATH, &args).unwrap();
    let c_mount_res = ts.cmd(C_MOUNT_PATH).run();
    run_cmd_as_root_ignore_ci(ts, C_UMOUNT_PATH, &[TEST_MOUNT_POINT]).unwrap();

    // Run rust programe
    let rust_res = run_ucmd_as_root_ignore_ci(ts, &args).unwrap();
    let rust_mount_res = ts.cmd(C_MOUNT_PATH).run();
    run_cmd_as_root_ignore_ci(ts, C_UMOUNT_PATH, &[TEST_MOUNT_POINT]).unwrap();

    // Clean
    run_cmd_as_root_ignore_ci(ts, C_LOSETUP_PATH, &["-d", loopdevice]).unwrap();

    compare_mount_result(c_res, rust_res, c_mount_res, rust_mount_res);
}

fn compare_mount_result(c_res: CmdResult, rust_res: CmdResult, c_mount_res: CmdResult, rust_mount_res: CmdResult) {
    println!("c_res: {}\n{}\n{}", c_res.stdout_str(), c_res.stderr_str(), c_mount_res.stdout_str());
    println!(
        "rust_res: {}\n{}\n{}",
        rust_res.stdout_str(),
        rust_res.stderr_str(),
        rust_mount_res.stdout_str()
    );

    c_res.code_is(rust_res.code());
    c_res.stderr_is(rust_res.stderr_str());
    c_res.stdout_is(rust_res.stdout_str());
    c_mount_res.stdout_is(rust_mount_res.stdout_str());
}

#[test]
fn test_mount_print_all() {
    let _lock = KEEP_SINGLE_THREAD.lock();
    let ts = TestScenario::new(util_name!());
    let c_res = ts.cmd(C_MOUNT_PATH).run();
    let rust_res = ts.ucmd().run();
    c_res.stdout_is(rust_res.stdout_str());
}

#[test]
fn test_mount_print_all_only_types() {
    let _lock = KEEP_SINGLE_THREAD.lock();
    let ts = TestScenario::new(util_name!());
    let args = &["-t", "ext4"];
    let c_res = ts.cmd(C_MOUNT_PATH).args(args).run();
    let rust_res = ts.ucmd().args(args).run();
    c_res.stdout_is(rust_res.stdout_str());
}

#[test]
fn test_mount_verbose() {
    let ts = TestScenario::new(util_name!());
    run_and_compare(&ts, &["-v", TEST_MOUNT_SRC, TEST_MOUNT_POINT]);
}

#[test]
fn test_mount_read_only() {
    let ts = TestScenario::new(util_name!());
    run_and_compare(&ts, &["-r", TEST_MOUNT_SRC, TEST_MOUNT_POINT]);
}

#[test]
fn test_mount_all() {
    let ts = TestScenario::new(util_name!());
    run_and_compare(&ts, &["-a"]);
}

#[test]
fn test_mount_types() {
    let ts = TestScenario::new(util_name!());
    run_and_compare(&ts, &["-t", "ext4", TEST_MOUNT_SRC, TEST_MOUNT_POINT]);
}

#[test]
fn test_mount_options() {
    let ts = TestScenario::new(util_name!());
    run_and_compare(&ts, &["-o", "ro,noexec", TEST_MOUNT_SRC, TEST_MOUNT_POINT]);
}

#[test]
fn test_mount_bind() {
    let ts = TestScenario::new(util_name!());
    ts.cmd(C_MKDIR_PATH).arg("source").run();
    run_and_compare(&ts, &["--bind", "source", TEST_MOUNT_POINT]);
}

#[test]
fn test_mount_move() {
    let _lock = KEEP_SINGLE_THREAD.lock();
    let ts = &TestScenario::new(util_name!());
    const NEW_MOUNT_POINT:&str = "new_target";
    let args = &["--move", TEST_MOUNT_POINT, NEW_MOUNT_POINT];
    let loopdevice = &setup_loop_device(ts);
    ts.cmd(C_MKDIR_PATH).arg(NEW_MOUNT_POINT).run();

    ts.cmd(C_MOUNT_PATH).arg(loopdevice).arg(TEST_MOUNT_POINT).run();

    // Run C programe
    let c_res = run_cmd_as_root_ignore_ci(ts, C_MOUNT_PATH, args).unwrap();
    let c_mount_res = ts.cmd(C_MOUNT_PATH).run();
    run_cmd_as_root_ignore_ci(ts, C_UMOUNT_PATH, &[loopdevice]).unwrap();

    ts.cmd(C_MOUNT_PATH).arg(loopdevice).arg(TEST_MOUNT_POINT).run();

    // Run rust programe
    let rust_res = run_ucmd_as_root_ignore_ci(ts, args).unwrap();
    let rust_mount_res = ts.cmd(C_MOUNT_PATH).run();
    run_cmd_as_root_ignore_ci(ts, C_UMOUNT_PATH, &[loopdevice]).unwrap();
    
    // Clean
    run_cmd_as_root_ignore_ci(ts, C_LOSETUP_PATH, &["-d", loopdevice]).unwrap();
    compare_mount_result(c_res, rust_res, c_mount_res, rust_mount_res);
}

#[test]
fn test_mount_label() {
    new_ucmd!()
        .args(&["-L", "LABEL"])
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_uuid() {
    new_ucmd!()
        .args(&["-U", "UUID"])
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_no_mtab() {
    new_ucmd!()
        .arg("--no-mtab")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_invalid_option() {
    new_ucmd!()
        .arg("--invalid-option")
        .fails()
        .stderr_contains("invalid");
}

#[test]
fn test_mount_show_labels() {
    new_ucmd!()
        .arg("-l")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_no_canonicalize() {
    new_ucmd!()
        .arg("--no-canonicalize")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_fake() {
    new_ucmd!()
        .arg("-f")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_fork() {
    new_ucmd!()
        .arg("-a")
        .arg("-F")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_internal_only() {
    new_ucmd!()
        .arg("--internal-only")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_make_private() {
    new_ucmd!()
        .arg("--make-private")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_read_write() {
    new_ucmd!()
        .arg("-w")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_namespace() {
    new_ucmd!()
        .arg("-N")
        .arg("testnamespace")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_fstab_alternative() {
    new_ucmd!()
        .arg("-T")
        .arg("/etc/alt_fstab")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_rbind() {
    new_ucmd!()
        .arg("-R")
        .arg("/source")
        .arg("/target")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}
#[test]
fn test_mount_make_shared() {
    new_ucmd!()
        .arg("--make-shared")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}
