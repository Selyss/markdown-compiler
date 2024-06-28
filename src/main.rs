use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author=env!("CARGO_PKG_AUTHORS"), version)]
struct Args {
    file: String,

}

fn parse_markdown_file() {
    todo!()
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
