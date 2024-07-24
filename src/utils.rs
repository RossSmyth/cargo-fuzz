use std::ffi::OsString;

/// The default target to pass to cargo, to workaround issue #11.
pub fn default_target() -> &'static str {
    current_platform::CURRENT_PLATFORM
}

/// Gets the path to the asan DLL required for the asan instrumented binary to run.
#[cfg(target_env = "msvc")]
pub fn get_asan_path() -> Option<std::path::PathBuf> {
    use std::path::Path;

    #[cfg(target_pointer_width = "64")]
    const DLL_NAME: &str = "clang_rt.asan_dynamic-x86_64";

    #[cfg(target_pointer_width = "32")]
    const DLL_NAME: &str = "clang_rt.asan_dynamic-i386";

    for entry in walkdir::WalkDir::new(r"C:\Program Files (x86)\Microsoft Visual Studio")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(name) = entry.file_name().to_str() {
            if name.contains(DLL_NAME) {
                return entry.path().parent().map(Path::to_owned);
            }
        }
    }

    None
}

/// Append a value to the PATH variable
#[cfg(target_env = "msvc")]
pub fn append_to_pathvar(path: std::path::PathBuf) -> Option<OsString> {
    use std::env;

    if let Some(current) = env::var_os("PATH") {
        let mut current = env::split_paths(&current).collect::<Vec<_>>();
        current.push(path);
        return env::join_paths(current).ok();
    }

    return None;
}
