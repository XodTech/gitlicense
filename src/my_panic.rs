pub fn panic(msg: &str) {
    println!("{}", msg);
    std::process::exit(1)
}
