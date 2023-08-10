use std::collections::HashMap;
use sysinfo::{System, SystemExt};

mod config;
use config::Config;

fn main() {
    let config: Config = Config::new();

    print_kawaii_ascii();
    print_kawaii_info(config.widgets, config.separator);
}

fn get_system_info() -> HashMap<String, String> {
    // init structs
    let mut sus = System::new_all();
    sus.refresh_all();
    let mut kawaii_info: HashMap<String, String> = HashMap::new();

    // system infowmation
    kawaii_info.insert("nameu".to_string(), sus.name().unwrap_or_default());
    kawaii_info.insert(
        "os_vewsiown".to_string(),
        sus.os_version().unwrap_or_default(),
    );
    kawaii_info.insert(
        "howst_name".to_string(),
        sus.host_name().unwrap_or_default(),
    );
    kawaii_info.insert(
        "kewnel_vewsion".to_string(),
        sus.kernel_version().unwrap_or_default(),
    );

    // cpuwu infowmation
    kawaii_info.insert("cpuwu".to_string(), sus.cpus().len().to_string());

    // memowy section
    kawaii_info.insert(
        "totaw_memowy".to_string(),
        format!("{:.2} GiB", (sus.total_memory() as f64) * 1e-9).to_string(),
    );
    kawaii_info.insert(
        "used_memowy".to_string(),
        format!("{:.2} GiB", (sus.used_memory() as f64) * 1e-9).to_string(),
    );

    kawaii_info
}

fn print_kawaii_info(widgets: Vec<String>, separator: String) {
    let kawaii_info: HashMap<String, String> = get_system_info();

    for widget in widgets {
        if kawaii_info.contains_key(&widget) {
            println!("{}: {}", widget, kawaii_info.get(&widget).unwrap());
        }
    }
}

// print out cute ascii (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
fn print_kawaii_ascii() {
    for (_line, kawaii_line) in KAWAII_ASCII.iter().enumerate() {
        println!("{}", kawaii_line);
    }
}

const KAWAII_ASCII: &[&str] = &["(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ "];
