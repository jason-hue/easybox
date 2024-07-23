use std::ffi::{CStr, CString};
use std::path::Path;
use nix::errno::Errno;
use nix::mount::{mount, MsFlags};
use nix::NixPath; 
// type Result<T> = nix::Result<T>;
// pub fn mount_fs(source: &str, target: &str, fstype: &str, flags: MsFlags, data: &str) -> Result<()> {
//     let source = MountPath::new(source);
//     let target = MountPath::new(target);
//     let fstype = MountPath::new(fstype);
//     let data = MountPath::new(data);
//     match mount(Some(&source), &target,Some(&fstype),flags,Some(&data)) {
//         Ok(()) => {
//             println!("Mount successful!");
//             Ok(())
//         },
//         Err(e) => {
//             println!("Mount failed with error: {:?}", e);
//             Err(e)
//         }
//     }
// }

pub fn mount_fs<p: AsRef<Path>>(
    source: Option<&p>,
    target: &p,
    fs_type:Option<&str>,
    flags: MsFlags,
    data: Option<&str>
) -> nix::Result<()> {
    mount(source.map(|s| s.as_ref()), target.as_ref(), fs_type, flags, data)
}

// pub struct MountPath{
//     path:  String
// }
// impl MountPath {
//     fn new(path: &str) -> MountPath {
//         MountPath {
//             path: path.to_string(),
//         }
//     }
// }
// impl NixPath for MountPath{
//     fn is_empty(&self) -> bool {
//         self.path.is_empty()
//     }
// 
//     fn len(&self) -> usize {
//         self.path.len()
//     }
// 
//     fn with_nix_path<T, F>(&self, f: F) -> nix::Result<T>
//     where
//         F: FnOnce(&CStr) -> T
//     {
//         let c_string = CString::new(self.path.clone()).map_err(|_| Errno::EINVAL)?;
//         Ok(f(&c_string))
//     }
// }