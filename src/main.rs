extern crate pdf_extract;

use pdf_extract::{Extractor, Settings};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (text_by_page, contents) = extract_text_and_contents_from_pdf(path).unwrap_or_else(|err| {
        eprintln!("Error extracting: {}", err);
        (HashMap::new(), String::new())
    });

    // Need to add GUI intergration here
    if let Some(first_page_text) = text_by_page.get(&1) {
        display_text_word_by_word(first_page_text, 200);
    }
}

fn extract_text_and_contents_from_pdf(path: &str) -> Result<(HashMap<u32, String>, String), pdf_extract::Error> {
    let mut extractor = Extractor::new(path, Settings::default())?;
    let mut text_by_page = HashMap::new();
    let mut contents = String::new();

    for page in 1..=extractor.get_page_count()? {
        let text = extractor.get_text_from_page(page)?;
        if text.contains("Contents") || text.contains("Chapters") {
            contents = text;
        }
        text_by_page.insert(page, text);
    }

    Ok((text_by_page, contents))
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
