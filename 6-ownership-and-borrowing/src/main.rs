fn main() {
    let mut owner = String::from("admin");
    // multiple owner borrow immutably

    let owner1 = &owner;
    let owner2 = &owner;

    println!("Owner 1 can be: {}", owner1);
    println!("Owner 2 can be: {}", owner2);

    // multiple owner borrow mutably

    let trusted_owner = &mut owner;

    trusted_owner.push_str(" and trusted admin");
    println!("Trusted admin updates: {}", trusted_owner);

    println!("Owner transform to: {}", owner);
}
