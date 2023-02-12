fn just_hello() -> String {
    String::from("Hello")
}

fn hello_world() -> String {
    "Hello, world!".to_string()
}

fn main() {
    println!("{}", just_hello());
    println!("{}", hello_world());
}

#[test]
fn test_hello_world() {
    assert_eq!(hello_world(), "Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::hello_world;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}