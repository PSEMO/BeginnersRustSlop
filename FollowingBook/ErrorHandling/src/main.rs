fn main() {
    match last_char_of_first_line("asd") {
        Some(val) => println!("some"),
        None => println!("does not have last char"),
    };
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    let output = text.lines().next()?.chars().last();
    println!("Found: {:?}", output);
    output
}
