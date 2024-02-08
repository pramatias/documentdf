// use crate::code_chunk::CodeChunk;
use crate::json_chunks::read_chunks_from_file;
use thirtyfour::{prelude::*, By};

// Update the import to use thirtyfour::By
pub fn connect() {
    let file_path = "./doc/source.json";

    // Read the chunks from the JSON file
    match read_chunks_from_file(file_path) {
        Ok(read_chunks) => {
            // Print each read chunk
            for chunk in &read_chunks {
                println!("{}", chunk);
            }
        }
        Err(err) => {
            eprintln!("Error reading chunks from file: {}", err);
            // Handle the error as needed
        }
    }

    // Perform automation with Firefox.
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        if let Err(e) = automate_firefox().await {
            eprintln!("Error automating Firefox: {}", e);
        }
    });
}

pub async fn automate_firefox() -> WebDriverResult<()> {
    // Create a new WebDriver session for Firefox.
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?; // Remove the reference to `&caps`

    // Navigate to chat.openai.com.
    driver.get("https://chat.openai.com").await?;

    // Find the text area element and type some text.
    let text_area = driver.find(By::Css("textarea")).await?;
    text_area.send_keys("Hello, world!").await?;

    // You can add more automation tasks here.

    // Close the WebDriver session.
    driver.quit().await?;

    Ok(())
}

