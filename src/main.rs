fn main() {
    if let Err(e) = mock::run() {
        eprintln!("Error while parsing the code: {:#?}", e);
        std::process::exit(1);
    }
}
