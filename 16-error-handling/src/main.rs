use core::panic;

/// # Option และ Result ใน Rust
///
/// ในภาษา Rust มีชนิดข้อมูลสองชนิดที่ใช้ในการจัดการกับค่าที่อาจจะมีหรือไม่มี และการจัดการกับข้อผิดพลาด
///
/// ## Option
/// `Option` เป็น enum ที่ใช้ในการแทนค่าที่อาจจะมีหรือไม่มี โดยมีสองค่าคือ:
/// - `Some(T)`: แทนค่าที่มีอยู่
/// - `None`: แทนค่าที่ไม่มี
///
/// ตัวอย่างการใช้งาน:
/// ```rust
/// let some_value: Option<i32> = Some(10);
/// let no_value: Option<i32> = None;
/// ```
///
/// ## Result
/// `Result` เป็น enum ที่ใช้ในการจัดการกับข้อผิดพลาด โดยมีสองค่าคือ:
/// - `Ok(T)`: แทนค่าที่สำเร็จ
/// - `Err(E)`: แทนค่าที่เกิดข้อผิดพลาด
///
/// ตัวอย่างการใช้งาน:
/// ```rust
/// let success: Result<i32, &str> = Ok(10);
/// let error: Result<i32, &str> = Err("เกิดข้อผิดพลาด");
/// ```
///
/// ## panic!
/// `panic!` เป็นมาโครที่ใช้ในการหยุดการทำงานของโปรแกรมเมื่อเกิดข้อผิดพลาดที่ไม่สามารถจัดการได้
///
/// ตัวอย่างการใช้งาน:
/// ```rust
/// panic!("เกิดข้อผิดพลาดที่ไม่คาดคิด!");
/// ```

fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a treasure!".to_string())
    }
}

fn open_door(is_trap: bool) -> Result<String, String> {
    if is_trap {
        Err("You found a trap!".to_string())
    } else {
        Ok("You found a key!".to_string())
    }
}

fn main() {
    let chest_res = match open_chest(true) {
        Some(item) => item,
        None => "You found nothing!".to_string(),
    };

    println!("{}", chest_res);

    let door_res = match open_door(true) {
        Ok(item) => item,
        Err(err) => panic!("{}", err),
    };

    println!("{}", door_res);
}
