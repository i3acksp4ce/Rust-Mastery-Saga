/// # คำอธิบายเกี่ยวกับ Box, Rc, และ RefCell ในภาษา Rust
///
/// ## Box
/// `Box` เป็น smart pointer ที่ใช้ในการจัดการหน่วยความจำบน heap โดยการใช้ `Box` จะช่วยให้เราสามารถสร้างค่าที่มีขนาดใหญ่หรือมีการใช้งานหน่วยความจำมากได้ง่ายขึ้น
///
/// ### เมื่อควรใช้:
/// - เมื่อเราต้องการจัดการหน่วยความจำบน heap
/// - เมื่อเราต้องการสร้างโครงสร้างข้อมูลที่มีขนาดใหญ่
/// - เมื่อเราต้องการส่งค่าที่มีขนาดใหญ่ระหว่างฟังก์ชัน
///
/// ### เมื่อไม่ควรใช้:
/// - เมื่อค่าที่เราต้องการจัดการมีขนาดเล็กและสามารถจัดการบน stack ได้
/// - เมื่อเราไม่ต้องการการจัดการหน่วยความจำที่ซับซ้อน
///
/// ## Rc
/// `Rc` (Reference Counted) เป็น smart pointer ที่ใช้ในการนับจำนวนการอ้างอิงของค่าที่ถูกแชร์ระหว่างหลายตัวแปร โดย `Rc` จะช่วยให้เราสามารถแชร์ค่าระหว่างหลายตัวแปรได้โดยไม่ต้องใช้การคัดลอกค่า
///
/// ### เมื่อควรใช้:
/// - เมื่อเราต้องการแชร์ค่าระหว่างหลายตัวแปร
/// - เมื่อเราต้องการให้ค่ามีอายุการใช้งานเท่ากับตัวแปรที่อ้างอิงถึงมัน
///
/// ### เมื่อไม่ควรใช้:
/// - เมื่อเราต้องการการเข้าถึงแบบ mutable (ใช้ `RefCell` ร่วมกับ `Rc` แทน)
/// - เมื่อเราต้องการการจัดการหน่วยความจำที่มีประสิทธิภาพสูง (ใช้ `Arc` แทนในกรณี multi-threading)
///
/// ## RefCell
/// `RefCell` เป็น smart pointer ที่ใช้ในการจัดการการเข้าถึงแบบ mutable ใน runtime โดย `RefCell` จะช่วยให้เราสามารถเปลี่ยนแปลงค่าที่ถูกแชร์ได้ใน runtime
///
/// ### เมื่อควรใช้:
/// - เมื่อเราต้องการการเข้าถึงแบบ mutable ใน runtime
/// - เมื่อเราต้องการแชร์ค่าระหว่างหลายตัวแปรและต้องการการเปลี่ยนแปลงค่า
///
/// ### เมื่อไม่ควรใช้:
/// - เมื่อเราสามารถจัดการการเข้าถึงแบบ mutable ใน compile-time ได้
/// - เมื่อเราไม่ต้องการการจัดการหน่วยความจำที่ซับซ้อน
use std::{cell::RefCell, rc::Rc};

fn main() {
    let item = Box::new(10);
    let shared_item = Rc::new(RefCell::new(item));

    **shared_item.borrow_mut() += 10;
    **shared_item.borrow_mut() += 5;

    println!("shared_item: {}", **shared_item.borrow());

    let item2 = 10;
    let shared_item2 = Rc::new(RefCell::new(item2));

    *shared_item2.borrow_mut() += 10;
    *shared_item2.borrow_mut() += 5;

    println!("shared_item2: {}", *shared_item2.borrow());
}