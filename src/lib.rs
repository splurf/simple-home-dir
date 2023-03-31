use std::path::PathBuf;

#[cfg(target_os = "windows")]
use {
    core::slice::from_raw_parts,
    std::{
        ffi::{c_void, OsString},
        os::windows::prelude::OsStringExt,
        ptr::null_mut,
    },
    windows_sys::Win32::{
        Foundation::HANDLE,
        Globalization::lstrlenW,
        System::Com::CoTaskMemFree,
        UI::Shell::{FOLDERID_Profile, SHGetKnownFolderPath},
    },
};

#[cfg(target_os = "windows")]
/// Return the user's home directory.
///
/// ```
/// use simple_home_dir::*;
/// let home = home_dir().unwrap();
/// ```
pub fn home_dir() -> Option<PathBuf> {
    let mut path_ptr = null_mut();
    (unsafe { SHGetKnownFolderPath(&FOLDERID_Profile, 0, HANDLE::default(), &mut path_ptr) } == 0)
        .then_some({
            let ostr: OsString = unsafe {
                let ostr =
                    OsStringExt::from_wide(from_raw_parts(path_ptr, lstrlenW(path_ptr) as usize));
                CoTaskMemFree(path_ptr as *const c_void);
                ostr
            };
            PathBuf::from(ostr)
        })
}

#[cfg(not(target_os = "windows"))]
use std::env::var_os;

#[cfg(not(target_os = "windows"))]
/// Return the user's home directory.
///
/// ```
/// use simple_home_dir::*;
/// let home = home_dir().unwrap();
/// ```
pub fn home_dir() -> Option<PathBuf> {
    return var_os("HOME").map(PathBuf::from);
}
