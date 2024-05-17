fn main() {
    println!("What is your name?");
    let input = read_string();
    println!("Hello: {}", input);
}

fn read_string() -> String {
    let mut eingabe = String::new();
    std::io::stdin()
        .read_line(&mut eingabe)
        .expect("can not read user eingabe");
    eingabe
}