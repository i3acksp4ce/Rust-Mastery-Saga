/// # Vector ใน Rust
///
/// Vector เป็นโครงสร้างข้อมูลชนิดหนึ่งใน Rust ที่สามารถเก็บข้อมูลหลายๆ ค่าในตัวแปรเดียวกัน
/// โดยค่าที่เก็บใน Vector จะต้องมีชนิดข้อมูลเดียวกันทั้งหมด
///
/// ## การสร้าง Vector
/// สามารถสร้าง Vector ได้โดยใช้คำสั่ง `vec!`
/// ```rust
/// let mut items = vec![1, 2, 3];
/// ```
///
/// ## การเพิ่มข้อมูลใน Vector
/// สามารถเพิ่มข้อมูลใน Vector ได้โดยใช้เมธอด `push`
/// ```rust
/// items.push(4);
/// ```
///
/// ## การลบข้อมูลใน Vector
/// สามารถลบข้อมูลใน Vector ได้โดยใช้เมธอด `remove`
/// ```rust
/// items.remove(1); // ลบค่าที่ตำแหน่งที่ 1
/// ```
///
/// ## การดูความยาวของ Vector
/// สามารถดูความยาวของ Vector ได้โดยใช้เมธอด `len`
/// ```rust
/// let length = items.len();
/// ```
///
/// ## การดูความจุของ Vector
/// สามารถดูความจุของ Vector ได้โดยใช้เมธอด `capacity`
/// ```rust
/// let capacity = items.capacity();
/// ```
///
/// ## การเพิ่มขึ้นของ Capacity
/// เมื่อความจุของ Vector ไม่เพียงพอที่จะเก็บข้อมูลใหม่ Rust จะเพิ่มความจุของ Vector ขึ้นเป็น 2 เท่า
/// เพื่อให้การเพิ่มข้อมูลในอนาคตมีประสิทธิภาพมากขึ้น การเพิ่มความจุเป็น 2 เท่าช่วยลดจำนวนครั้งที่ต้องจัดสรรหน่วยความจำใหม่
/// ซึ่งเป็นกระบวนการที่มีค่าใช้จ่ายสูง การเพิ่มความจุเป็น 2 เท่าจึงเป็นวิธีที่มีประสิทธิภาพในการจัดการหน่วยความจำ
fn main() {
    let mut items = vec!["gold", "silver", "bronze"];

    items.push("new item");
    items.push("new item 2");
    items.push("new item 3");
    items.push("new item 4");

    items.remove(1);

    println!("Items: {:?}", items);
    println!("Items length: {}", items.len());
    println!("Items capacity: {}", items.capacity());
}
