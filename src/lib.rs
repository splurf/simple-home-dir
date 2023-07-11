#[cfg(feature = "expand_tilde")]
mod expand_tilde {
    use {
        super::home_dir,
        std::path::{Path, PathBuf},
    };

    const TILDE: &'static str = "~";

    /// Expand the tilde that originates in the provided path.
    ///
    /// ```
    /// //  "/home/USER/local/share"
    /// let expanded = simple_home_dir::expand_tilde("~/.local/share").unwrap();
    /// ```
    pub fn expand_tilde(path: impl AsRef<Path>) -> Option<PathBuf> {
        let p = path.as_ref();

        Some(if p.starts_with(TILDE) {
            let mut home = home_dir()?;

            if !p.ends_with(TILDE) {
                let mut cmpts = p.components();
                cmpts.next()?;
                home.extend(cmpts);
            }
            home
        } else {
            p.to_path_buf()
        })
    }
}

#[cfg(target_os = "windows")]
mod home_dir_windows {
    use {
        core::slice::from_raw_parts,
        std::{
            ffi::{c_void, OsString},
            os::windows::prelude::OsStringExt,
            path::PathBuf,
            ptr::null_mut,
        },
        windows_sys::Win32::{
            Globalization::lstrlenW,
            System::Com::CoTaskMemFree,
            UI::Shell::{FOLDERID_Profile, SHGetKnownFolderPath},
        },
    };

    /// Return the user's home directory.
    ///
    /// ```
    /// //  "C:\Users\USER"
    /// let path = simple_home_dir::home_dir().unwrap();
    /// ```
    pub fn home_dir() -> Option<PathBuf> {
        let mut path_ptr = null_mut();
        (unsafe { SHGetKnownFolderPath(&FOLDERID_Profile, 0, 0, &mut path_ptr) } == 0).then_some({
            let wide = unsafe { from_raw_parts(path_ptr, lstrlenW(path_ptr) as usize) };
            let ostr = OsString::from_wide(wide);
            unsafe { CoTaskMemFree(path_ptr as *const c_void) }
            ostr.into()
        })
    }
}

#[cfg(not(target_os = "windows"))]
mod home_dir_ne_windows {
    use std::{env::var_os, path::PathBuf};

    const HOME: &'static str = "HOME";

    /// Return the user's home directory.
    ///
    /// ```
    /// //  "/home/USER"
    /// let path = simple_home_dir::home_dir().unwrap();
    /// ```
    pub fn home_dir() -> Option<PathBuf> {
        var_os(HOME).map(Into::into)
    }
}

#[cfg(target_os = "windows")]
pub use home_dir_windows::*;

#[cfg(not(target_os = "windows"))]
pub use home_dir_ne_windows::*;

#[cfg(feature = "expand_tilde")]
pub use expand_tilde::*;
