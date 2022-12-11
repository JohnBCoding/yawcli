fn main() {
    if let Err(e) = yawcli::get_args().and_then(yawcli::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
