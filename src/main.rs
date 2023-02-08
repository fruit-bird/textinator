mod mocker;

use mocker::Mocker;
use std::process;
use text_converter::TextConverterCLI;

fn main() {
    if let Err(e) = Mocker::run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
