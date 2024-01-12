#[cfg(feature = "home_dir")]
mod home_dir {
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
        if cfg!(target_family = "windows") {
            let mut path_ptr = null_mut();
            (unsafe { SHGetKnownFolderPath(&FOLDERID_Profile, 0, 0, &mut path_ptr) } == 0)
                .then_some({
                    let wide = unsafe { from_raw_parts(path_ptr, lstrlenW(path_ptr) as usize) };
                    let ostr = OsString::from_wide(wide);
                    unsafe { CoTaskMemFree(path_ptr as *const c_void) }
                    ostr.into()
                })
        } else {
            std::env::var_os("HOME").map(Into::into)
        }
    }
}
#[cfg(feature = "home_dir")]
pub use home_dir::*;

#[cfg(feature = "expand_tilde")]
mod expand_tilde {
    /// Expand the tilde (`~`) from within the provided path.
    pub fn expand_tilde(path: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
        let p = path.as_ref();

        Some(if p.starts_with("~") {
            let mut home = super::home_dir()?;

            if !p.ends_with("~") {
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
    #[test]
    fn expand_tilde_test() {
        let expected = dirs::home_dir().unwrap().join("foo");
        let resulted = crate::expand_tilde("~/foo").unwrap();
        assert_eq!(resulted, expected)
    }

    #[test]
    fn home_dir_test() {
        let expected = dirs::home_dir().unwrap();
        let resulted = crate::home_dir().unwrap();
        assert_eq!(resulted, expected)
    }
}
