use clap::{App, Arg};

mod code_chunker;
mod ripgrep_lines;
mod json_chunks;

use json_chunks::create_json_source;

fn main() {
    let matches = App::new("Code Chunker")
        .version("1.0")
        .author("Your Name")
        .about("Cuts .rs files into smaller chunks based on a pattern")
        .arg(
            Arg::with_name("folder")
                .help("Sets the folder to search for .rs files")
                .required(true)
                .index(1),
        )
        .get_matches();

    let folder_path = matches.value_of("folder").unwrap();

    let _ = create_json_source(folder_path);
}

