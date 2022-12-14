fn hello(name: String) -> String {
    return "hello ".to_owned() + &name
}

fn main() {
    let output = hello("world".to_string());
    print!("{}", output)
}