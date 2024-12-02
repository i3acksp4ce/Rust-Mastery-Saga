/// `HashMap` ใน Rust เป็นโครงสร้างข้อมูลที่ใช้เก็บคู่ของคีย์และค่า
/// โดยคีย์แต่ละตัวจะมีค่าที่ไม่ซ้ำกัน และสามารถใช้คีย์ในการเข้าถึงค่าได้อย่างรวดเร็ว
///
/// คุณสมบัติของ `HashMap`:
/// - การเพิ่มข้อมูล: สามารถเพิ่มคู่คีย์-ค่าใหม่ลงใน `HashMap` ได้โดยใช้เมธอด `insert`
/// - การเข้าถึงข้อมูล: สามารถเข้าถึงค่าที่เก็บอยู่ใน `HashMap` ได้โดยใช้คีย์ผ่านเมธอด `get` หรือ `get_mut`
/// - การลบข้อมูล: สามารถลบคู่คีย์-ค่าออกจาก `HashMap` ได้โดยใช้เมธอด `remove`
/// - การปรับปรุงข้อมูล: สามารถปรับปรุงค่าที่เก็บอยู่ใน `HashMap` ได้โดยใช้เมธอด `entry` และ `or_insert`
///
/// `HashMap` เป็นโครงสร้างข้อมูลที่มีประสิทธิภาพสูงในการจัดการข้อมูลที่ต้องการการเข้าถึงอย่างรวดเร็ว
/// และมีการใช้งานที่หลากหลายในโปรแกรมต่าง ๆ
use std::collections::HashMap;

fn main() {
    let mut items = HashMap::new();

    items.insert("item1", 1);
    items.insert("item2", 2);
    items.insert("item3", 3);

    println!("{:?}", items);

    if let Some(value) = items.get_mut("item2") {
        println!("item2: {}", value);

        *value += 10;

        println!("item2: {}", value);
    }

    items.remove("item3");
    println!("{:?}", items);

    // modify item 3
    let item3 = items.entry("item3").or_insert(0);
    *item3 += 100;
    println!("{:?}", items);
}
