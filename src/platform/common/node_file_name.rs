
pub struct NodeFileName {
    name: String = "node".to_string(),
    version: String ,
    platform: String,
    arch: String,
    file_type: String = "tar.xz".to_string(),
    file_name: String = format!("{}-{}-{}-{}.{}", name, version, platform, arch, file_type),
}

impl NodeFileName {
    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
}


