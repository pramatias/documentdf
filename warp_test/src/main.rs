mod server;

use core::load_chunks::CodeChunksJson;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read code chunk JSON data from a file and create a CodeChunksJson instance
    let filename = "source.json";
    let mut code_chunks_json = CodeChunksJson::from_file(filename)?;
    code_chunks_json.sort();

    // Display statistics based on the contents of the code chunk JSON data
    code_chunks_json.display_stats()?;

    server::serve().await;

    Ok(())
}
