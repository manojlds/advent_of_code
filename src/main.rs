mod day1;

fn main() {
    if let Err(e) = day1::solve() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
