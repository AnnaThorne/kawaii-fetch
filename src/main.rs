use sysinfo::{System, SystemExt};
use struct_iterable::Iterable;

fn main() {
    print_kawaii_ascii();
    print_kawaii_info();
}

fn get_system_info () -> KawaiiInfo {
    // init structs
    let mut sus = System::new_all();
    let mut kawaii_info: KawaiiInfo = KawaiiInfo::new();
    sus.refresh_all();

    // system infowmation
    kawaii_info.nameu = sus.name();
    kawaii_info.os_vewsiown = sus.os_version();
    kawaii_info.howst_name = sus.host_name();
    kawaii_info.kewnel_vewsion = sus.kernel_version();

    // cpuwu infowmation
    kawaii_info.cpuwus = sus.cpus().len();

    // memowy section
    kawaii_info.totaw_memowy = sus.total_memory();
    kawaii_info.used_memowy = sus.used_memory();

    return kawaii_info;
}

fn print_kawaii_info() -> () {
    let kawaii_info: KawaiiInfo = get_system_info();
    kawaii_info.iter().for_each(|(key, value)| {
        println!("{}: {:#?}", key, value.downcast_ref::<String>());
    });

}

// print out cute ascii (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
fn print_kawaii_ascii () -> (){
    for (kawaii_line, line) in KAWAII_ASCII.iter().enumerate() {
        println!("{}{}", kawaii_line, line);
    }
}

// KawaiiInfo struct to store system infowmation
#[derive(Iterable)]
struct KawaiiInfo {
    nameu: Option<String>,
    os_vewsiown: Option<String>,
    howst_name: Option<String>,
    kewnel_vewsion: Option<String>,
    cpuwus: usize,
    totaw_memowy: u64,
    used_memowy: u64,
}

// implementation of KawaiiInfo
impl KawaiiInfo {
    fn new() -> KawaiiInfo {
        KawaiiInfo {
            nameu: Some(String::new()),
            os_vewsiown: Some(String::new()),
            howst_name: Some(String::new()),
            kewnel_vewsion: Some(String::new()),
            cpuwus: 0,
            totaw_memowy: 0,
            used_memowy: 0,
        }
    }
}

const KAWAII_ASCII: &[&str] = &[
  "  (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ "
];