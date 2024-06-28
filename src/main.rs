use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser, Default, Debug)]
#[clap(author=env!("CARGO_PKG_AUTHORS"), version)]
struct Args {
    file: String,
}

fn parse_markdown_file(filename: &str) {
    let input_file = Path::new(filename);

    let file = File::open(&input_file).expect("Error: failed to read file."); // TODO: return err

    let mut ptag: bool = false;
    let mut htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();

        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        // now check the first character for the octothorp (i.e., heading symbol)
        let mut s = String::new();
        let slice = &line_contents.to_string();
        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    s.push_str("</p>\n");
                }
                if htag {
                    htag = false;
                    s.push_str("</h1>\n"); // close it if we're already open
                } else {
                    htag = true;
                    s.push_str("<h1>");
                    s.push_str(&slice[2..]); // get all but the first two characters
                }
            }

            _ => {
                if htag {
                    htag = false;
                    s.push_str("</h1>\n");
                }

                if !ptag {
                    ptag = true;
                    s.push_str("<p>");
                }

                s.push_str(&slice);
            }
        };

        // at the very end, check if any of the tag bools are still open. If so,
        // close them.
        if htag {
            htag = false;
            s.push_str("</h1>\n");
        }

        if ptag {
            ptag = false;
            s.push_str("</p>\n");
        }

        // Don't push blank lines
        if s != "<p></p>\n" {
            tokens.push(s);
        }
    }

    // create an output file based on the input file, minus .md
    let output_filename = &filename[..filename.len() - 3];
    let mut output_filename = String::from(output_filename);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename.to_string()).expect("Error: could not create output file.");

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("Error: could not write to output file.");
    }
}

fn main() {
    let args = Args::parse();
    // match parse_markdown_file(&args.file) {
    //     Ok(c) => todo!(),
    //     Err(e) => eprintln!("Error when parsing markdown: {}.", e),
    // }
    parse_markdown_file(&args.file);
}
