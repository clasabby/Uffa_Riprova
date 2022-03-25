use clap::Parser;
use untitled2::capitalize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short, long)]
    stringa: String,
}
fn main() {
    let args = Args::parse();

    println!("Input -> {}", args.stringa);

    println!("Result -> {}", capitalize(&args.stringa));
    //println!("Result -> {}", capitalize("prova"));
}
