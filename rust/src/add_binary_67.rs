pub fn add_binary(a: String, b: String) -> String {
    let x = isize::from_str_radix(&a, 2).unwrap();
    let y = isize::from_str_radix(&b, 2).unwrap();
    let x = x + y;
    return format!("{x:b}");
}
