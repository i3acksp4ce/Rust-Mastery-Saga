fn main() {
    let number_1: i32 = 1;
    let number_2: f64 = 0.5;

    let result: i32 = number_1 + number_2 as i32;

    let msg: String = String::from("Message");
    let msg_2: String = "Message".to_string();
    let msg_3: &str = "Message";
    let msg_4: String = format!("Number: {}, {}", number_1, number_2);

    println!("Result: {}", msg_4);
}
