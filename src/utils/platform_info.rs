pub fn get_arch() -> String {
    #[cfg(target_arch = "x86_64")]
    return "x64".to_string();

    #[cfg(target_arch = "aarch64")]
    return "arm64".to_string();

    #[cfg(target_arch = "x86")]
    return "x86".to_string();
}

pub fn get_os() -> String {
    #[cfg(target_os = "linux")]
    return "linux".to_string();

    #[cfg(target_os = "macos")]
    return "darwin".to_string();

    #[cfg(target_os = "windows")]
    return "win".to_string();
}