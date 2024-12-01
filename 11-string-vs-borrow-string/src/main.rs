fn main() {
    let text = String::from("old text");
    let borrow_text = &text;
    let mut new_text = borrow_text.to_string();

    new_text.push_str(" to new text");

    println!("text: {}", new_text);
}
