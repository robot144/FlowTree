
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode{
    // content: executable, arg 1, ...
    let args: Vec<String> = env::args().collect();
    // get path
    let path = if args.len() > 1 {
      &args[1]   // user passed a path
    } else {
      "."        // default to current directory
    };
    let p = Path::new(&path);
    if !p.exists() {
        println!("path {p:?} does not exist");
        return ExitCode::FAILURE
    }
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        println!("{}", entry.file_name().to_string_lossy());
    }   
    return ExitCode::SUCCESS
}
