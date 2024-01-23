use arboard::Clipboard;
use std::env;
use std::io::{ self, Write };

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut clipboard = Clipboard::new().unwrap();

    match args.get(1).map(String::as_str) {
        Some("c") => {
            let mut input_text = String::new();
            print!("Enter the text to convert and copy: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_text).unwrap();
            let converted_text = convert_to_tag_chars(input_text.trim());
            clipboard.set_text(converted_text).unwrap();
            println!("Converted text copied to clipboard.");
        }
        Some("v") => {
            if let Ok(text) = clipboard.get_text() {
                let converted_back_text = convert_back_to_text(&text);
                println!("Converted back text: {}", converted_back_text);
            } else {
                println!("No text found in clipboard.");
            }
        }
        _ => {
            println!("Usage: xcv [c|v]");
        }
    }
}

fn convert_to_tag_chars(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            let code = ch as u32;
            if code < 0x20 || code > 0x7e {
                return ch;
            }
            std::char::from_u32(0xe0000 + code).unwrap_or(ch)
        })
        .collect()
}

fn convert_back_to_text(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            let code = ch as u32;
            if code < 0xe0000 || code > 0xe007f {
                return ch;
            }
            std::char::from_u32(code - 0xe0000).unwrap_or(ch)
        })
        .collect()
}
