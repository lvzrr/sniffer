use chrono::{DateTime, Local};
use matcher::{
    get_color, BLACK_OVER_WHITE, BRIGHT_MAGENTA, BRIGHT_RED, BRIGHT_WHITE, RESET, WHITE,
};
use std::{
    env::{self, current_dir},
    fmt::{format, Debug},
    fs::{self, *},
    os::windows::{fs::MetadataExt, process},
    path::{Path, PathBuf},
    process::exit,
};

mod matcher;

fn str_builder(
    name: &String,
    size: u64,
    writetime: String,
    perm: String,
    ico: String,
    color: String,
) -> String {
    return format!(
        "{:6} {:20} {:10} {} {:3}{:15} {}",
        perm,
        writetime,
        size,
        color,
        ico,
        name,
        matcher::RESET
    );
}

struct Argopts {
    hidden: bool,
    explicit_path: bool,
    exp_path: String,
}

fn get_arg_opts() -> Argopts {
    let mut opts = Argopts {
        hidden: false,
        explicit_path: false,
        exp_path: "".to_string(),
    };
    let mut argv: Vec<String> = env::args().collect();
    if argv.contains(&"-hidden".to_string()) || argv.contains(&"-h".to_string()) {
        opts.hidden = true;
    }
    if argv.contains(&"-path".to_string()) || argv.contains(&"-p".to_string()) {
        opts.explicit_path = true;
        opts.exp_path = argv.pop().unwrap().to_owned();
    }
    if argv.contains(&"-help".to_string()) || argv.contains(&"help".to_string()) {
        println!("\n{}\n", "[SNIFF ARGUMENTS SCHEMA] sniff <opts> <path>");
        println!(
            "{:15} {:15}\n{:15} {:15}\n{:15} {:15}\n",
            "-help, help",
            "Print this msg",
            "-h , -hidden",
            "Allow hidden files (dotfiles)",
            "-path , -p",
            "Path to list"
        );
        exit(0);
    }
    opts
}

fn print_headers(path: &PathBuf) {
    println!(
        "\n\n {}\n\n{}{:6} {:20} {:>10} {:3} {:15}\n{:6} {:20} {:>10} {:3} {:15}{}",
        path.to_string_lossy().to_string(),
        BRIGHT_RED,
        "PERMS",
        "LASTMODTIME",
        "SIZE",
        "",
        "NAME",
        "-----",
        "--------------------",
        "-----",
        "",
        "----------------",
        RESET,
    );
}

fn getfiles(allowhidden: bool, path: PathBuf) {
    print_headers(&path);
    let dir = fs::read_dir(path).unwrap();
    for f in dir {
        let entry = f.unwrap();
        let filesize = entry.metadata().unwrap().file_size();
        let filename = entry.file_name().to_string_lossy().to_string();
        let lastmod: String = (DateTime::from(entry.metadata().unwrap().modified().unwrap())
            as DateTime<Local>)
            .format("%H:%M:%S %d/%m/%Y")
            .to_string();
        let dir = entry.metadata().unwrap().is_dir();

        let perm = match entry.metadata().unwrap().permissions().readonly() {
            true => "r1w0".to_string(),
            false => "r1w1".to_string(),
        };
        let ico = matcher::geticon(&filename, dir.clone());

        let outfile: String;
        /*
        let mut dispname: String;
        if dir && filename.len() > 25 {
            let temp1: Vec<char> = filename.chars().collect();
            dispname = temp1[0..20].iter().collect();
            dispname += "...";
        } else {
            dispname = filename.clone();
        }
        */
        outfile = str_builder(
            &filename,
            filesize,
            lastmod,
            perm,
            ico,
            get_color(&filename, dir.clone()),
        );
        if filename.starts_with('.') == false || allowhidden {
            println!("{}", outfile);
        }
    }
}

fn main() {
    let mut path = env::current_dir().unwrap();
    let opts: Argopts = get_arg_opts();
    if opts.explicit_path {
        if opts.exp_path.starts_with(".\\") {
            path.push(opts.exp_path.strip_prefix(".\\").unwrap().to_string());
        } else {
            path.push(&opts.exp_path);
        }
    }
    match path.is_dir() {
        false => Err("Path is not a directory"),
        _ => Ok(()),
    }
    .unwrap();
    getfiles(opts.hidden, path);
    let link = "https://github.com/lvzrr";
    println!("\n\n   / \\__\n  (    @\\___  *snif\n  /         O\n /   (_____/\n/_____/        Developed by: {}",link);
}
