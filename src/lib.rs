/// Returns the user's home directory path.
pub fn home_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_family = "windows")]
    {
        use windows_sys::Win32::{UI::Shell::*, *};
        let mut p = std::ptr::null_mut();
        let r =
            if unsafe { SHGetKnownFolderPath(&FOLDERID_Profile, 0, std::ptr::null_mut(), &mut p) }
                == 0
            {
                let w = unsafe { core::slice::from_raw_parts(p, Globalization::lstrlenW(p) as _) };
                let o: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(w);

                Some(o.into())
            } else {
                None
            };
        unsafe { System::Com::CoTaskMemFree(p as _) }
        r
    }
    #[cfg(not(target_family = "windows"))]
    std::env::var_os("HOME").map(Into::into)
}
