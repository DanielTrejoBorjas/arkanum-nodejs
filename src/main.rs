use arkanum_nodejs::utils::platform_info;
use reqwest::blocking::get;
use serde::Deserialize;
use arkanum_nodejs::platform::common::node_file_name::NodeFileName;

#[derive(Deserialize, Debug)]
struct NodeJson {
    version: String,
}





fn get_lasted_version() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://nodejs.org/dist/index.json";
    let resp = get(url)?.text()?;
    let versiones: Vec<NodeJson> = serde_json::from_str(&resp)?;

    // Asumiendo que el JSON viene ordenado de más reciente a más antigua
    let ultima_version = versiones.first()
        .ok_or("No se encontraron versiones")?
        .version
        .clone();

    Ok(ultima_version)
}


fn set_file_name() -> String {
    let os: String = platform_info::get_os();
    let arch: String = platform_info::get_arch();
    let node_version = get_lasted_version().unwrap_or_else(|_| "unknown".to_string());

    format!("args: node-{}-{}-{}.tar.xz", node_version, os, arch)
}

fn main() {
    /* 
    match obtener_version_mas_reciente() {
        Ok(v) => println!("La versión más reciente de Node.js es {}", v),
        Err(e) => eprintln!("Error: {}", e),
    }
    */
    let node_file = NodeFileName::new();



    println!("{}", node_file.get_file_name());

    println!("{}", set_file_name());
}
