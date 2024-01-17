#[cfg(target_family = "windows")]
use {
    core::slice::from_raw_parts,
    std::{
        ffi::{c_void, OsString},
        os::windows::prelude::OsStringExt,
        ptr::null_mut,
    },
    windows_sys::Win32::{
        Globalization::lstrlenW,
        System::Com::CoTaskMemFree,
        UI::Shell::{FOLDERID_Profile, SHGetKnownFolderPath},
    },
};

/// Return the path of the user's home directory.
pub fn home_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_family = "windows")]
    {
        let mut path_ptr = null_mut();
        (unsafe { SHGetKnownFolderPath(&FOLDERID_Profile, 0, 0, &mut path_ptr) } == 0).then_some({
            let wide = unsafe { from_raw_parts(path_ptr, lstrlenW(path_ptr) as usize) };
            let ostr = OsString::from_wide(wide);
            unsafe { CoTaskMemFree(path_ptr as *const c_void) }
            ostr.into()
        })
    }

    #[cfg(not(target_family = "windows"))]
    std::env::var_os("HOME").map(Into::into)
}
