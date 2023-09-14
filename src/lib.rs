#[cfg(target_family = "windows")]
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
    /// use simple_home_dir::home_dir;
    ///
    /// let path = home_dir().unwrap();
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
#[cfg(target_family = "windows")]
pub use home_dir_windows::*;

#[cfg(not(target_family = "windows"))]
mod home_dir_ne_windows {
    use std::{env::var_os, path::PathBuf};

    const HOME: &'static str = "HOME";

    /// Return the user's home directory.
    ///
    /// ```
    /// use simple_home_dir::home_dir;
    ///
    /// let path = home_dir().unwrap();
    /// ```
    pub fn home_dir() -> Option<PathBuf> {
        var_os(HOME).map(Into::into)
    }
}
#[cfg(not(target_family = "windows"))]
pub use home_dir_ne_windows::*;

#[cfg(feature = "expand_tilde")]
mod expand_tilde {
    use {
        super::home_dir,
        std::path::{Path, PathBuf},
    };

    const TILDE: &'static str = "~";

    /// Expand the tilde that originates in the provided path.
    ///
    /// # Example
    ///
    /// ```
    /// use simple_home_dir::expand_tilde;
    ///
    /// let expanded = expand_tilde("~/.local").unwrap();
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
#[cfg(feature = "expand_tilde")]
pub use expand_tilde::*;

#[cfg(feature = "test")]
mod test {
    mod expand_tilde {
        #[test]
        fn expand_tilde_test() {
            let mut expected = dirs::home_dir().unwrap();
            expected.push("foo");
            let expanded = crate::expand_tilde("~/foo").unwrap();
            assert_eq!(expanded, expected)
        }
    }

    mod home_dir {
        #[test]
        fn home_dir_test() {
            let expected = dirs::home_dir().unwrap();
            let home_dir = crate::home_dir().unwrap();
            assert_eq!(home_dir, expected)
        }
    }
}
