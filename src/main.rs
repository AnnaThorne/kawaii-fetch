use std::collections::HashMap;
use sysinfo::{System, SystemExt};

fn main() {
    print_kawaii_ascii();
    print_kawaii_info();
}

fn get_system_info () -> HashMap<String, String> {
    // init structs
    let mut sus = System::new_all();
    sus.refresh_all();
    let mut kawaii_info: HashMap<String, String> = HashMap::new();

    // system infowmation
    kawaii_info.insert("nameu".to_string(), sus.name().unwrap());
    kawaii_info.insert("os_vewsiown".to_string(), sus.os_version().unwrap());
    kawaii_info.insert("howst_name".to_string(), sus.host_name().unwrap());
    kawaii_info.insert("kewnel_vewsion".to_string(), sus.kernel_version().unwrap());

    // cpuwu infowmation
    kawaii_info.insert("cpuwu".to_string(), sus.cpus().len().to_string());

    // memowy section
    kawaii_info.insert("totaw_memowy".to_string(), sus.total_memory().to_string());
    kawaii_info.insert("used_memowy".to_string(), sus.used_memory().to_string());

    return kawaii_info;
}

fn print_kawaii_info() -> () {
    let kawaii_info: HashMap<String, String> = get_system_info();
    
    for (kawaii_key, kawaii_value) in kawaii_info {
        println!("{}{:#?}", kawaii_key, kawaii_value);
    }

}

// print out cute ascii (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
fn print_kawaii_ascii () -> (){
    for (kawaii_line, line) in KAWAII_ASCII.iter().enumerate() {
        println!("{}{}", kawaii_line, line);
    }
}

const KAWAII_ASCII: &[&str] = &[
  "  (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ "
];