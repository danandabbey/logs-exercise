use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let mut results = vec![];

    for line in text.split("\n") {
        if line.starts_with("ERROR") {
            results.push(String::from(line));
        }
    }
    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let results = extract_errors(text.as_str());
    fs::write("errors.txt", results.join("\n"))?;

    Ok(())

    // match fs::read_to_string("logs.txt") {
    //     Ok(value) => {
    //         let results = extract_errors(value.as_str());

    //         match fs::write("errors.txt", results.join("\n")) {
    //             Ok(..) => println!("success"),
    //             Err(value) => print!("{:#?}", value),
    //         }
    //     }
    //     Err(value) => {
    //         println!("{:#?}", value)
    //     }
    // }
}
