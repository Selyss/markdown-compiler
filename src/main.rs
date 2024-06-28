use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser, Default, Debug)]
#[clap(author=env!("CARGO_PKG_AUTHORS"), version)]
struct Args {
    file: String,
}

enum MarkdownElement {
    Paragraph,
    Heading,
}

fn parse_markdown_file(filename: &str) -> io::Result<()> {
    let input_file = Path::new(filename);
    let file = File::open(&input_file).expect("Error: failed to read file."); // TODO: return err
    let reader = BufReader::new(file);
    let mut output_content = Vec::new();

    let mut current_element = MarkdownElement::Paragraph;

    for line in reader.lines() {
        let line_contents = line?;

        if let Some(first_char) = line_contents.chars().next() {
            match first_char {
                '#' => current_element = MarkdownElement::Heading,
                _ => current_element = MarkdownElement::Paragraph,
            }
        }
        match current_element {
            MarkdownElement::Heading => {
                let line_output = format!("<h1>{}</h1>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::Paragraph => {
                let line_output = format!("<p>{}</p>", line_contents);
                output_content.push(line_output);
            },
        }
    }

    // create an output file based on the input file, minus .md
    let output_filename = &filename[..filename.len() - 3];
    let mut output_filename = String::from(output_filename);
    output_filename.push_str(".html");

    save_to_file(&output_filename, &output_content.join("\n"))?;

    Ok(())
}

// New function to save content to a file
fn save_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut outfile =
        File::create(filename.to_string()).expect("Error: could not create output file.");
    outfile
        .write_all(content.as_bytes())
        .expect("Error: could not write to output file.");
    Ok(())
}

fn main() {
    let args = Args::parse();
    match parse_markdown_file(&args.file) {
        Ok(c) => todo!(),
        Err(e) => eprintln!("Error when parsing markdown: {}.", e),
    }
}
