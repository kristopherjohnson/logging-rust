fn main() {
    logging::init_logging().expect("Unable to initialize logging");
    logging::test_logging();
}
