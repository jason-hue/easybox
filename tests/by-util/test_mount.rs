use crate::common::util::*;
#[test]
fn test_mount_verbose() {
    new_ucmd!()
        .arg("-v")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_read_only() {
    new_ucmd!()
        .arg("-r")
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_all() {
    new_ucmd!()
        .arg("-a")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_types() {
    new_ucmd!()
        .args(&["-t", "ext4"])
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_options() {
    new_ucmd!()
        .args(&["-o", "ro,noexec"])
        .arg("/dev/sda1")
        .arg("/mnt")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_bind() {
    new_ucmd!()
        .arg("--bind")
        .arg("/source")
        .arg("/target")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
}

#[test]
fn test_mount_move() {
    new_ucmd!()
        .arg("--move")
        .arg("/old")
        .arg("/new")
        .fails() // Assuming it fails because we're not root
        .stderr_contains("mount");
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
