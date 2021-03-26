use colored::*;
use sysinfo::{ ProcessorExt, RefreshKind, System, SystemExt };
use std::string::ToString;

const LOVE_PXART: &[&str] = &[
"        .....           .....          ",
"    ,ad8PPPP88b,     ,d88PPPP8ba,      ",
"   d8P        Y8b, ,d8P        Y8b     ",
"  dP             8a8             Yd    ",
"  8(              |              )8    ",
"  I8                             8I    ",
"   Yb,                         ,dP     ",
"     8a,                     ,a8       ",
"       8a,                 ,a8         ",
"         Yba             adP           ",
"           Y8a         a8P             ",
"             88,     ,88               ",
"               8b   d8                 ",
"                8b d8                  ",
"                 888                   ",
"                  |                    ",
"                                      ",
];

fn render(info: &[String]) {
    for (love_line, info_line) in LOVE_PXART.iter().zip(info) {
        println!("{}   {}", love_line.red(), info_line);
    }
}

fn main() {
    let sys        = System::new_with_specifics(RefreshKind::new().with_cpu().with_memory());
    let cpu        = sys.get_processors()[0].get_brand();
    let kernel     = sys.get_kernel_version().unwrap_or("Unknown".into());
    let used_ram   = sys.get_used_memory () / 1024;
    let total_ram  = sys.get_total_memory() / 1024;

    let userinfo       = format!("{}{}{}", whoami::username().bright_red().bold(), "@".bold(), whoami::hostname().bright_red().bold());
    let splitline      = "═".repeat(whoami::username().len() + whoami::hostname().len() + 1);
    let love           = format!("{}{}"      , "love for you: ".bright_red(),           "too much <3");
    let ur_love           = format!("{}{}"      , "your love for me: ".bright_red(),           "idk");
    let os             = format!("{}{}"      , "os: ".bright_red(),    whoami::distro());
    let kernel         = format!("{}{}"      , "kernel: ".bright_red(),              kernel);
    let cpu            = format!("{}{}"      , "cpu: ".bright_red(),                 cpu);
    let ram            = format!("{}{} » {}{}", "ram: ".bright_red(), used_ram, total_ram, " MB");
    
    let bright_colors = format!(
        "{}{}{}{}{}{}{}{}",
        "███".bright_red(),
        "███".bright_yellow(),
        "███".bright_green(),
        "███".bright_cyan(),
        "███".bright_blue(),
        "███".bright_magenta(),
        "███".bright_black(),
        "███".bright_white()
    );
    let dark_colors = format!(
        "{}{}{}{}{}{}{}{}",
        "███".red(),
        "███".yellow(),
        "███".green(),
        "███".cyan(),
        "███".blue(),
        "███".magenta(),
        "███".black(),
        "███".white()
    );

    render(&[
        "".to_string(),
        "".to_string(),
        userinfo,
        splitline,
        love,
        ur_love,
        os,
        kernel,
        cpu,
        ram,
        "".to_string(),
        bright_colors,
        dark_colors,
        "".to_string()
    ]);
}