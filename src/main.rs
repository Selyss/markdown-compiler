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
    HeadingOne,
    HeadingTwo,
    HeadingThree,
    HeadingFour,
    HeadingFive,
    HeadingSix,
}

fn parse_markdown_file(filename: &str) -> io::Result<()> {
    let input_file = Path::new(filename);
    let file = File::open(&input_file).expect("Error: failed to read file."); // TODO: return err
    let reader = BufReader::new(file);
    let mut output_content = Vec::new();

    let mut current_element = MarkdownElement::Paragraph;

    for line in reader.lines() {
        let line = line?;

        let line_contents = if line.contains("**") || line.contains("__") {
            let bold_replaced = line
                .replace("**", "<strong>")
                .replace("__", "<strong>")
                .replace("</strong><strong>", ""); // Handle adjacent bold markers
            let parts = bold_replaced.split("<strong>").collect::<Vec<_>>();
            let mut new_line = String::new();

            for (i, part) in parts.iter().enumerate() {
                if i > 0 {
                    new_line.push_str("</strong>");
                }
                if i < parts.len() - 1 {
                    new_line.push_str(part);
                    new_line.push_str("<strong>");
                } else {
                    new_line.push_str(part);
                }
            }
            new_line
        } else {
            line
        };

        let mut chars = line_contents.chars();
        current_element = match chars.next() {
            Some('#') => match chars.take_while(|&c| c == '#').count() {
                0 => MarkdownElement::HeadingOne,
                1 => MarkdownElement::HeadingTwo,
                2 => MarkdownElement::HeadingThree,
                3 => MarkdownElement::HeadingFour,
                4 => MarkdownElement::HeadingFive,
                _ => MarkdownElement::HeadingSix, // Treat all other cases as H6
            },
            _ => MarkdownElement::Paragraph,
        };
        match current_element {
            // TODO: add === and --- heading syntax
            MarkdownElement::HeadingOne => {
                let line_output =
                    format!("<h1>{}</h1>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::HeadingTwo => {
                let line_output =
                    format!("<h2>{}</h2>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::HeadingThree => {
                let line_output =
                    format!("<h3>{}</h3>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::HeadingFour => {
                let line_output =
                    format!("<h4>{}</h4>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::HeadingFive => {
                let line_output =
                    format!("<h5>{}</h5>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::HeadingSix => {
                let line_output =
                    format!("<h6>{}</h6>", line_contents.trim_start_matches('#').trim());
                output_content.push(line_output);
            }
            MarkdownElement::Paragraph => {
                let line_output = format!("<p>{}</p>", line_contents);
                output_content.push(line_output);
            }
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
        Ok(_) => println!("File successfully parsed."),
        Err(e) => eprintln!("Error when parsing markdown: {}.", e),
    }
}
