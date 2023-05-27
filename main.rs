fn main() {
    let message = "Hello world";
    let return_message = print_welcome(message);
    println!("{}", return_message);
}

fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    return "Yello";
}
