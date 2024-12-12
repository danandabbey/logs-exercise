use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
    let new_text = text.split("/n");
    let mut results = vec![];

    for lines in new_text {}
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(value) => {
            println!("{:#?}", value.len())
        }
        Err(value) => {
            println!("{:#?}", value)
        }
    }
}
