fn main() {
    if let Err(e) = mock::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
