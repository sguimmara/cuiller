use structopt::StructOpt;

mod cuiller;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug)]
#[structopt(name = "cuiller", about = "Generate a QR code.")]
struct Cli {
    /// The text to put into a QR code
    text: String,
    // #[structopt(short="o", long="output")]
    // output: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    let source = args.text;

    match cuiller::validate_input(&source) {
        Err(e) => handle_error_and_exit(e),
        Ok(ok) => handle_valid_input(ok),
    }
}

fn handle_valid_input(ok: cuiller::InputType) {
    println!("input type is {:?}", ok);
    todo!()
}

fn handle_error_and_exit(e: cuiller::InputError) {
    eprintln!("error: {}", e);
    std::process::exit(1);
}
