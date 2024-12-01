// if it "one", will return "this is 1"
// if it "two", will return "this is 2"
// if it "three", will return "this is 3"

// fn main() {
//     let number: &str = "three";

//     if number == "one" {
//         println!("this is 1");
//     } else if number == "two" {
//         println!("this is 2");
//     } else if number == "three" {
//         println!("this is 3");
//     } else {
//         println!("this is not 1, 2, or 3");
//     }
// }

// fn main() {
//     let costume: &str = "superman";

//     match costume {
//         "superman" => println!("This is superman"),
//         "batman" => println!("This is batman"),
//         "spiderman" => println!("This is spiderman"),
//         _ => println!("This is not superman, batman, or spiderman"),
//     }
// }

fn main() {
    let mut wood: i32 = 0;

    loop {
        wood += 1;

        println!("Wood: {}", wood);

        if wood == 10 {
            println!(" - Done");
            break;
        }
    }
}
