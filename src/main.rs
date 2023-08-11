extern crate pdf_extract;

use pdf_extract::{Extractor, Settings};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let path = "/path_to_your_pdf_file.pdf";
    let text = extract_text_from_pdf(path).unwrap_or_else(|err| {
        eprintln!("Error extracting text: {}", err);
        String::new()
    });

    display_text_word_by_word(&text, 200); // 200ms per word
}

fn extract_text_from_pdf(path: &str) -> Result<String, pdf_extract::Error> {
    let extractor = Extractor::new(path, Settings::default())?;
    let text = extractor.get_text()?;
    Ok(text)
}

fn display_text_word_by_word(text: &str, delay_ms: u64) {
    for word in text.split_whitespace() {
        print!("\r{}", word);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
}
