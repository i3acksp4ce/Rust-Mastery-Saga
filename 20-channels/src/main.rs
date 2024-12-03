/// โปรแกรมนี้ใช้ `mpsc` (multiple producer, single consumer) channel เพื่อส่งข้อมูลระหว่าง threads
///
/// - `mpsc::sync_channel(3)` สร้าง channel ที่มี buffer ขนาด 3 ซึ่งหมายความว่า channel นี้สามารถเก็บข้อมูลได้สูงสุด 3 ข้อมูลก่อนที่ผู้ส่งจะถูกบล็อก
/// - `sender` เป็นตัวส่งข้อมูล (sender) ที่ใช้ส่งข้อมูลไปยัง channel
/// - `receiver` เป็นตัวรับข้อมูล (receiver) ที่ใช้รับข้อมูลจาก channel
///
/// การใช้ `mpsc::sync_channel` เหมาะสมเมื่อ:
/// - ต้องการให้มีการควบคุมการส่งข้อมูลระหว่าง threads
/// - ต้องการให้มี buffer ขนาดจำกัดเพื่อป้องกันการใช้หน่วยความจำมากเกินไป
///
/// ไม่ควรใช้ `mpsc::sync_channel` เมื่อ:
/// - ต้องการให้มีการส่งข้อมูลแบบไม่จำกัด buffer
/// - ต้องการให้มีการส่งข้อมูลระหว่าง threads หลายตัวรับ (multiple consumers)
///
/// - `send` เป็นฟังก์ชันที่ใช้ส่งข้อมูลจาก thread หนึ่งไปยัง channel โดยจะบล็อกถ้า buffer เต็ม
/// - `recv` เป็นฟังก์ชันที่ใช้รับข้อมูลจาก channel โดยจะบล็อกถ้าไม่มีข้อมูลใน buffer
///
/// การใช้ `send` และ `recv` เหมาะสมเมื่อ:
/// - ต้องการให้ thread หนึ่งส่งข้อมูลไปยังอีก thread หนึ่ง
/// - ต้องการให้มีการควบคุมการส่งข้อมูลระหว่าง threads
///
/// ไม่ควรใช้ `send` และ `recv` เมื่อ:
/// - ต้องการให้มีการส่งข้อมูลแบบไม่บล็อก
/// - ต้องการให้มีการส่งข้อมูลระหว่าง threads หลายตัวรับ (multiple consumers)
use std::{
    sync::{mpsc, Arc},
    thread,
};

fn main() {
    let loots = vec![50, 100, 150, 200, 250];
    let mut my_gold = 0;
    let (sender, receiver) = mpsc::sync_channel(3);
    let sender_arc = Arc::new(sender);

    for loot in loots.clone().into_iter() {
        thread::spawn({
            let sender = Arc::clone(&sender_arc);
            move || {
                sender.send(loot).unwrap();
            }
        });
    }

    for _ in 0..loots.len() {
        let loot = receiver.recv().unwrap();
        my_gold += loot;
    }

    println!("My gold: {}", my_gold);
}
