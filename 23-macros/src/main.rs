//! # Macros in Rust (แมโครในภาษา Rust)
//!
//! แมโครเป็นเครื่องมือที่ทรงพลังใน Rust สำหรับการสร้างโค้ดแบบอัตโนมัติในช่วง compile-time
//!
//! ## ประเภทของแมโครใน Rust
//!
//! ### 1. Declarative Macros (macro_rules!)
//! - ใช้สำหรับ pattern matching และการสร้างโค้ดแบบง่าย
//! - ตัวอย่างเช่น `println!`, `vec!`, `assert!`
//!
//! ### 2. Procedural Macros
//! #### a. Derive Macros
//! - ใช้สำหรับ auto-implement traits
//! - เช่น `#[derive(Debug, Clone)]`
//!
//! #### b. Attribute Macros
//! - ใช้สร้าง custom attributes
//! - สามารถปรับแต่งโค้ดได้ตาม attribute ที่กำหนด
//!
//! #### c. Function-like Macros
//! - ทำงานคล้ายฟังก์ชัน แต่ทำงานในช่วง compile-time
//! - สามารถรับ Rust code เป็น input ได้
//!
//! ## เมื่อไหร่ควรใช้แมโคร
//! - เมื่อต้องการลดโค้ดที่ซ้ำซ้อน (Boilerplate code)
//! - ต้องการสร้าง DSL (Domain Specific Language)
//! - ต้องการทำ Code generation ในช่วง compile-time
//! - ต้องการสร้างฟังก์ชันที่รับพารามิเตอร์แบบไม่จำกัดจำนวน
//!
//! ## เมื่อไหร่ไม่ควรใช้แมโคร
//! - เมื่อสามารถใช้ฟังก์ชันปกติได้ (แมโครดีบักยากกว่า)
//! - โค้ดที่ไม่ซับซ้อนและไม่จำเป็นต้องใช้ meta-programming
//! - เมื่อต้องการความชัดเจนในการอ่านโค้ด (แมโครอาจทำให้โค้ดอ่านยากขึ้น)
//! - เมื่อต้องการ runtime flexibility (แมโครทำงานในช่วง compile-time เท่านั้น)
//!
//!
//!

macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

macro_rules! say_something {
    ($msg:expr) => {
        println!("You said: {}", $msg);
    };
}
fn main() {
    say_hello!();
    say_something!("Hello, Rust!");
}
