/* TODO :
 *          - tree method
 *          - Improve coloring
 */
/*
 *
 *NOTE      - I want this to be more or less the logics of user input,
 *          - dont let it fucking evolve into a convoluted piece of ass
 *
 */
use std::{env, path::PathBuf};
use toolkit::{get_arg_opts, print_headers, show_files, Argopts};
fn main() {
    let opts: Argopts = get_arg_opts();
    let mut path = PathBuf::from(env::current_dir().unwrap());
    if opts.explicit_path && opts.exp_path.starts_with(".\\") {
        path.push(opts.exp_path.strip_prefix(".\\").unwrap().to_string());
    } else {
        path.push(&opts.exp_path);
    }
    if path.is_dir() {
        print_headers(&path);
        match opts.tree {
            true => show_files(opts.hidden, path, 0, true),
            false => show_files(opts.hidden, path, 0, false),
        }
        let link = "https://github.com/lvzrr";
        println!("\n\n   / \\__\n  (    @\\___  *snif\n  /         O\n /   (_____/\n/_____/        Developed by: {}",link);
    }
}
