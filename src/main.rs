use clap::{App, Arg};

mod code_chunker;
mod ripgrep_lines;
mod code_chunk;
use code_chunker::process_folder;

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
    
    process_folder(folder_path);
}

