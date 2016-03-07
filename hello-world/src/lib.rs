pub fn hello(name: Option<&str>) -> String {
    let mut hello: String = "Hello, ".to_string();
    let name_s = match name {
        Some(n) => n,
        None => "World",
    };
    hello.push_str(name_s);
    hello.push_str("!");

    return hello;
}
