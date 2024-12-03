/// # คำอธิบาย
///
/// ## Arc
/// `Arc` (Atomic Reference Counted) เป็นตัวชี้วัดที่ใช้ในการแชร์ข้อมูลระหว่าง thread หลายๆ ตัว
/// โดยที่ไม่ต้องกังวลเรื่องการจัดการหน่วยความจำเอง
///
/// ## Mutex
/// `Mutex` (Mutual Exclusion) ใช้ในการป้องกันการเข้าถึงข้อมูลพร้อมกันจากหลายๆ thread
/// โดยการล็อกข้อมูลไว้เมื่อมีการใช้งาน และปลดล็อกเมื่อใช้งานเสร็จ
///
/// ## thread
/// `thread` เป็นหน่วยการทำงานที่สามารถทำงานพร้อมกันได้หลายๆ หน่วย
///
/// ## spawn
/// `spawn` ใช้ในการสร้าง thread ใหม่และเริ่มการทำงานของ thread นั้น
///
/// ## clone
/// `clone` ใช้ในการสร้างสำเนาของข้อมูลที่สามารถแชร์ระหว่าง thread ได้
///
/// ## lock
/// `lock` ใช้ในการล็อก `Mutex` เพื่อป้องกันการเข้าถึงข้อมูลพร้อมกันจากหลายๆ thread
///
/// ## unwrap
/// `unwrap` ใช้ในการดึงค่าที่อยู่ใน `Result` หรือ `Option` โดยที่ไม่ต้องตรวจสอบข้อผิดพลาด
///
/// ## move
/// `move` ใช้ในการย้ายความเป็นเจ้าของของข้อมูลไปยัง closure หรือ thread ใหม่
///
/// ## ||
/// `||` เป็นสัญลักษณ์ที่ใช้ในการสร้าง closure ซึ่งเป็นฟังก์ชันที่ไม่มีชื่อ
///
/// ## join
/// `join` ใช้ในการรอให้ thread ที่ถูกสร้างขึ้นทำงานเสร็จสิ้น
///
/// # เมื่อไหร่ควรใช้และไม่ควรใช้
///
/// ## ควรใช้
/// - เมื่อมีการแชร์ข้อมูลระหว่าง thread หลายๆ ตัว
/// - เมื่อมีการเข้าถึงข้อมูลพร้อมกันจากหลายๆ thread
/// - เมื่อมีการทำงานที่ต้องการความเร็วและประสิทธิภาพสูง
///
/// ## ไม่ควรใช้
/// - เมื่อการทำงานไม่ต้องการการทำงานพร้อมกัน
/// - เมื่อการจัดการหน่วยความจำและการซิงโครไนซ์เป็นเรื่องที่ซับซ้อนเกินไป
/// - เมื่อการทำงานไม่ต้องการความเร็วและประสิทธิภาพสูง
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let my_gold = Arc::new(Mutex::new(0));

    let loot_1 = thread::spawn({
        let my_gold = Arc::clone(&my_gold);
        move || {
            let mut my_gold = my_gold.lock().unwrap();
            *my_gold += 50;
        }
    });

    let loot_2 = thread::spawn({
        let my_gold = Arc::clone(&my_gold);
        move || {
            let mut my_gold = my_gold.lock().unwrap();
            *my_gold += 100;
        }
    });

    let loot_3 = thread::spawn({
        let my_gold = Arc::clone(&my_gold);
        move || {
            let mut my_gold = my_gold.lock().unwrap();
            *my_gold += 150;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    loot_3.join().unwrap();

    println!("Total gold: {}", *my_gold.lock().unwrap());
}
