fn just_hello() -> String {
    String::from("Hello")
}

fn hello_world() -> String {
    "Hello, world!".to_string()
}

// fn convert_color(color: String) -> [u8; 3] {

//     // convert hex to rgb in rust
//     [0,1,2]
// }

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');

    let (r, g, b) = if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).unwrap();
        let g = u8::from_str_radix(&hex[1..2], 16).unwrap();
        let b = u8::from_str_radix(&hex[2..3], 16).unwrap();
        (r * 16 + r, g * 16 + g, b * 16 + b)
    } else {
        panic!("Invalid hex color string");
    };

    (r, g, b)
}

fn main() {
    println!("{}", just_hello());
    println!("{}", hello_world());
    // println!("{}", convert_colors());
}

#[test]
fn test_hello_world() {
    assert_eq!(hello_world(), "Hello, world!");
}

// #[test]
// fn test_convert_colors() {
//     assert_eq!(convert_color(color: "FFFFFF"), [255,255,255]);
// }

#[test]
fn test_hex_to_rgb() {
    let cases: [(&str, (u8,u8,u8));7] = [
        ("#000000", (0, 0, 0)),
        ("#ffffff", (255, 255, 255)),
        ("#FF0000", (255, 0, 0)),
        ("#00FF00", (0, 255, 0)),
        ("#0000FF", (0, 0, 255)),
        ("#F0F0F0", (240, 240, 240)),
        ("#abcdef", (171, 205, 239)),
    ];

    for case in cases.iter() {
        let (hex, expected) = case;
        let result = hex_to_rgb(hex);
        assert_eq!(result, *expected, "hex: {}", hex);


        // assert_eq!(result, expected, "hex: {}", hex);
    }
}


#[cfg(test)]
mod tests {
    use super::hello_world;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}