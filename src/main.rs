fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value)
}
