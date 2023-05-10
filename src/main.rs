fn main() {
    // If the cater::get_args function returns Ok(config) execute the run func from cater and check if the value returned is of type Err(e),
    // where e is some value that implements the Error trait meaning it can be printed
    if let Err(e) = cater::get_args().and_then(cater::run) {
        // eprintln! prints errors to std error STDERR
        eprintln!("{}", e);
        // Exit the program with a non zero error to indicate failure.
        std::process::exit(1);
    }
}
