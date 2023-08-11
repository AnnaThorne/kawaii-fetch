use std::{cmp, collections::HashMap};
use sysinfo::{System, SystemExt};
use unicode_width::UnicodeWidthStr;

mod config;
use config::Config;

fn main() {
    let config: Config = Config::new();

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

    let longest_ascii_line_length = KAWAII_ASCII
        .iter()
        .map(|l| UnicodeWidthStr::width(*l))
        .max()
        .unwrap_or(0);

    let longest_widget_name_length = widgets.iter().map(|w| w.len()).max().unwrap_or(0);
    let ascii_fill = " ".repeat(longest_ascii_line_length).to_owned();

    for i in 0..cmp::max(KAWAII_ASCII.len(), widgets.len()) {
        let widget = widgets.get(i).unwrap_or(&"".to_string()).to_string();

        println!(
            "{} {} {}  {}  {}",
            KAWAII_ASCII.get(i).unwrap_or(&ascii_fill.as_str()),
            " ".repeat(
                longest_ascii_line_length
                    - UnicodeWidthStr::width(*KAWAII_ASCII.get(i).unwrap_or(&ascii_fill.as_str()))
            ),
            widget.to_string()
                + " "
                    .repeat(longest_widget_name_length - widget.len())
                    .as_str(),
            if widgets.get(i).is_some() {
                separator.to_owned()
            } else {
                " ".repeat(separator.len()).to_string()
            },
            kawaii_info
                .get(widgets.get(i).unwrap_or(&"".to_string()))
                .unwrap_or(&"".to_string())
        );
    }
}

const KAWAII_ASCII: &[&str] = &[
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠛⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⢠⣦⠙⢿⡏⣼⣧⠸⣿⣿⣿⣿⡿⠟⢛⣉⣭⣭⣭⣉⡙⠛⠿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⢉⡙⢿⠘⡏⣴⡌⠃⡟⢠⣄⠻⠿⢋⣡⣐⠾⣿⣿⣭⣝⡻⠶⣤⡂⢤⣈⡙⢿⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⢸⣿⣆⠀⠃⣿⣿⡀⢠⡿⣿⣦⠰⣿⣿⣿⣿⣦⡙⢿⣿⣿⣷⣮⡙⠦⡘⢻⣿⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠈⣿⣿⣆⠀⣿⣿⡇⣼⡅⣿⣿⣷⣬⣉⠛⢿⣿⣿⣎⠻⣿⣿⣿⣿⡆⢉⠂⢻⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⢻⣿⣿⡀⣿⣿⡇⢸⣇⢹⣿⣿⣿⣿⣿⣶⣌⠻⡍⠳⠙⣿⣿⣿⣿⡈⣷⡀⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⣩⣙⠈⣿⣿⡇⣽⣿⡇⠈⢿⣦⣿⣿⣿⣿⣿⣿⣿⣦⡈⠐⣤⡙⠿⣿⣿⣧⠸⣿⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⢻⣿⣇⠸⣿⣷⡟⣉⠁⢰⡈⣿⣿⠌⠙⣛⣛⣛⠛⠻⠿⠆⠻⠈⣃⠀⢙⠻⢇⠹⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡈⢿⣿⣷⣽⣿⢰⣿⡆⡌⣧⠸⣿⣥⣤⣿⣿⣿⠿⠃⠀⢸⣿⣯⠁⠀⣿⣿⣶⣶⣬",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡈⢿⢰⣦⡈⢸⣿⡇⣧⠹⣧⡘⠻⣿⣿⣿⣯⠀⣨⠤⢼⣿⣿⡷⠶⠀⣿⣿⣿⣿",
    "⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⣛⡩⠅⡀⠤⣠⣬⣙⠛⠿⣿⣿⣿⠌⠈⣿⣿⣴⣿⠁⣿⣷⡉⠀⢡⣠⣭⣿⣿⣿⣧⡀⠀⣬⠙⣃⣠⣼⣿⣿⣿⣿",
    "⣿⣿⣿⣿⣿⣿⠟⢉⣴⡾⠋⣀⢕⣢⣵⡶⠶⣖⠲⠦⣌⠻⠇⢸⣷⡌⢿⣿⣿⡄⡸⣿⣿⡎⠌⢻⣷⣭⠉⣉⣉⣙⣁⣠⣼⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⡿⢃⡴⠟⠋⢀⡞⣱⡿⢟⣵⣾⣻⠯⠉⣉⡀⠁⠀⣀⠻⠩⣤⣼⣿⡁⢧⢻⣿⠿⡞⠈⣿⡏⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⠃⢊⠄⠄⠆⡾⣰⡟⣱⡟⣽⡟⣣⣶⣿⡿⢁⠀⠀⣿⣷⣦⣝⠻⢿⣇⢸⣇⠻⢠⢡⡀⡿⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⡏⣰⡟⢸⢸⢸⢡⣿⢱⠟⣾⡟⣰⣿⣿⣿⡇⢸⡇⠀⡿⣿⣿⣿⣿⣿⣿⡈⣿⠆⣾⡆⠁⢡⣿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣷⣿⠁⡆⡇⡟⢸⡏⣾⣼⣿⢡⣿⣿⣿⣿⠿⠘⣿⣼⣷⠸⣿⣿⣿⣿⣿⡇⠟⣰⣿⣩⣤⣶⣦⠘⣿⣦⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⡿⢸⡇⣧⡇⣿⡇⡇⠿⡟⣸⣿⣿⣇⢰⣾⣆⣿⣿⣿⡧⠘⠻⣿⣿⣿⡁⢴⣿⣿⣿⡿⣿⣿⡇⢻⣿⡀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⡇⣸⢿⣿⡇⣿⡇⢿⡆⡇⣿⣿⣿⣿⢸⣿⣿⡿⠿⢋⡀⣴⣶⣶⣦⣤⣤⣤⣭⣥⠄⣴⣿⣿⡇⢸⣿⡇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⡇⣿⢸⣿⡇⣿⣿⣸⡀⠀⣿⣿⣿⣿⢘⣿⣿⣿⣷⢸⡇⢿⣿⣿⣿⣿⣿⣿⣿⢡⣾⣿⣿⣿⡇⣼⣿⠇⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⡇⣿⢸⣿⡇⢿⢸⣿⡇⣆⠸⣿⣿⡏⣸⣿⣿⣿⣿⡘⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⣿⢠⣿⡟⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⡇⠏⢸⣿⡇⣿⠘⢼⠇⣻⣷⣿⣿⠂⣿⣿⣿⣿⣿⡇⠇⣿⣿⣿⣿⣿⣿⣿⣿⡘⣿⣿⡿⢡⣍⣋⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⠀⡜⢸⣿⡇⠉⠀⢼⠀⣿⣿⣿⣿⣧⡘⠿⣿⣿⡿⢀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣷⣬⣩⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⢃⣼⠇⣾⢧⡇⢀⡆⠏⣰⣿⣿⣿⣿⣿⣿⣷⣦⣭⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⣿⠏⣸⠟⡘⢠⣾⠇⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⣿⣿⣿⠏⠐⡁⢀⣴⣿⣿⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
    "⡟⢋⣀⣴⣮⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
];
