use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // to also cope with invalid Unicode, use args_os instead
    // collect func is one you often need to annotate types since Rust isn't able to infer the kind of collection
    let args: Vec<String> = env::args().collect();

    // the main func shouldn't contain logic parts in terms of separation of concerns
    // let query = &args[1];
    // let file_path = &args[2];

    // let config = parse_config(&args);

    // unwrap_or_else is similar to unwrap when the status is Ok, but it allows you to customize what to do when it's Err with an anonymous func that defines it accepted in its arg
    let config = Config::build(/* &args */ env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("file_path: {}", config.file_path);
    // println!("query: {}", query);
    // println!("file_path: {}", file_path);
    // dbg!(args);

    // use if let rather than unwrap_or_else like Config::build does because here you should only check whether run returns an Err val which means you don't need a val if Ok
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file!");
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file!");
    // println!("With text:\n{contents}");
}
