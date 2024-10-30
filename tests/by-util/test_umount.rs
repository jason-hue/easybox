//! This file is part of the easybox package.
//
// (c) Zhenghang <2113130664@qq.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

use crate::common::util::*;
use std::path::Path;
#[test]
fn test_umount_all() {
    if !nix::unistd::geteuid().is_root() {
        eprintln!("Test skipped: requires root privileges.");
        return;
    }
    let test_args = &["-a"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}
#[test]
fn test_umount_all_targets() {
    let test_args = &["-A"];
    let task = TestScenario::new(util_name!());
    let result = task.ucmd().args(test_args).run();

    if result.succeeded() {
    } else {
        result.stderr_contains("No device specified for unmounting all targets");
    }
}
#[test]
fn test_umount_no_canonicalize() {
    let test_args = &["-c"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}
#[test]
fn test_umount_detach_loop() {
    let test_args = &["-d"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_force() {
    let test_args = &["-f"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_fake() {
    let test_args = &["--fake"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_internal_only() {
    let test_args = &["-i"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_lazy() {
    let test_args = &["-l"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_no_mtab() {
    let test_args = &["-n"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_namespace() {
    let namespace_path = "/run/user/1000/testns";
    if !Path::new(namespace_path).exists() {
        eprintln!("Test skipped: namespace does not exist.");
        return;
    }
    let test_args = &["-N", "testns"];
    let task = TestScenario::new(util_name!());
    let result = task.ucmd().args(test_args).run();

    if result.succeeded() {
    } else {
        result.stderr_contains("Failed to open namespace file");
    }
}
#[test]
fn test_umount_test_opts() {
    let test_args = &["-O", "testopts"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_quiet() {
    let test_args = &["-q"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_read_only() {
    let test_args = &["-r"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}

#[test]
fn test_umount_recursive() {
    let test_args = &["-R"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}
#[test]
fn test_umount_types() {
    let test_args = &["-t", "ext4"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}
#[test]
fn test_umount_verbose() {
    let test_args = &["-v"];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}
#[test]
fn test_umount_specific_target() {
    let target = "/mnt/test";
    if !Path::new(target).exists() {
        eprintln!("Test skipped: target directory does not exist.");
        return;
    }
    let test_args = &[target];
    let task = TestScenario::new(util_name!());
    task.ucmd().args(test_args).succeeds();
}
