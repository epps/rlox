pub fn handle(line: usize, message: String) {
    report(line, String::new(), message);
}

fn report(line: usize, location: String, message: String) {
    eprintln!("[line {line}] Error {location}: {message}");
}
