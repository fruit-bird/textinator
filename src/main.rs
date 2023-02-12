mod textinator;

use clap::Parser;
use textinator::Textinator;

fn main() {
    let cli = Textinator::parse();
    cli.run();
}
