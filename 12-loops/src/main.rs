fn main() {
    let boxs = ["gold", "silver", "bronze"];
    let mut energy = 10;

    for boxs in boxs.iter() {
        if energy == 0 {
            println!("You are out of energy!");
        } else if boxs == &"new item" {
            println!("You found the gold box!, Energy left: {}", energy);
        } else if boxs == &"new item 2" {
            println!("You found the silver box!, Energy left: {}", energy);
        } else if boxs == &"new item 3" {
            println!("You found the bronze box!, Energy left: {}", energy);
        }

        energy -= 1;
    }

    if energy == 0 {
        println!("You are out of energy!");
        return;
    }

    println!("You have energy left: {}", energy);
}
