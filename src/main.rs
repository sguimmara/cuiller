use std::{
    fs,
    io::{self, Write},
};

use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug)]
#[structopt(name="cuiller", about = "Generate a QR code.")]
struct Cli {
    /// The text to put into a QR code
    text: String,

    // #[structopt(short="o", long="output")]
    // output: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    // let mut stdout = io::stdout();
    let source = args.text;

    println!("generating QR code for string '{}'...", source);
    // let contents = fs::read_to_string(&args.path)
    //     .expect("Could not read file.");
    // (Buf) Wraps stdout in a buffer.
    // let mut stdout = BufWriter::new(stdout);

    // for (line_no, line) in contents.lines().enumerate() {
    //     if line.contains(&args.pattern) {
    //         writeln!(stdout, "{}: {}", line_no + 1, line);
    //     }
    // }
}

