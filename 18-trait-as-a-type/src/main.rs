/// # การใช้ Trait เป็นประเภทใน Rust
///
/// ## Generic
/// การใช้ Generic ในการกำหนดประเภทของ Trait จะช่วยให้โค้ดมีความยืดหยุ่นและสามารถทำงานกับประเภทต่าง ๆ ได้
/// โดยไม่ต้องระบุประเภทที่แน่นอนล่วงหน้า เช่น:
/// ```rust
/// fn use_gear<T: Gear>(gear: T) {
///     gear.use_gear();
/// }
/// ```
/// ข้อดี:
/// - ประสิทธิภาพสูง เนื่องจากไม่มีการใช้ Dynamic Dispatch
/// - สามารถทำงานกับประเภทที่แตกต่างกันได้
///
/// ข้อเสีย:
/// - ขนาดของไบนารีที่ใหญ่ขึ้น เนื่องจากการสร้างโค้ดสำหรับแต่ละประเภทที่ใช้
///
/// ## Box<dyn>
/// การใช้ Box<dyn Trait> จะช่วยให้สามารถเก็บ Trait Object ไว้ใน Heap ได้
/// และสามารถใช้ Dynamic Dispatch เพื่อเรียกใช้เมธอดของ Trait ได้ เช่น:
/// ```rust
/// fn use_gear2(gear: Box<dyn Gear2>) {
///     gear.use_gear();
/// }
/// ```
/// ข้อดี:
/// - ขนาดของไบนารีเล็กลง เนื่องจากใช้ Dynamic Dispatch
/// - สามารถเก็บประเภทที่แตกต่างกันในคอลเลกชันเดียวกันได้
///
/// ข้อเสีย:
/// - ประสิทธิภาพต่ำกว่าเนื่องจากการใช้ Dynamic Dispatch
///
/// ## impl Trait
/// การใช้ impl Trait จะช่วยให้สามารถกำหนดประเภทของ Trait ได้โดยไม่ต้องระบุประเภทที่แน่นอน
/// และยังคงใช้ Static Dispatch เช่น:
/// ```rust
/// fn use_gear3(gear: impl Gear3) {
///     gear.use_gear();
/// }
/// ```
/// ข้อดี:
/// - ประสิทธิภาพสูง เนื่องจากใช้ Static Dispatch
/// - โค้ดมีความกระชับและอ่านง่าย
///
/// ข้อเสีย:
/// - ไม่สามารถเก็บประเภทที่แตกต่างกันในคอลเลกชันเดียวกันได้
///
/// ## &impl Trait
/// การใช้ &impl Trait จะช่วยให้สามารถส่งผ่าน Trait Object โดยอ้างอิง (reference)
/// และยังคงใช้ Static Dispatch เช่น:
/// ```rust
/// fn use_gear4(gear: &impl Gear4) {
///     gear.use_gear();
/// }
/// ```
/// ข้อดี:
/// - ประสิทธิภาพสูง เนื่องจากใช้ Static Dispatch
/// - ลดการคัดลอกข้อมูล เนื่องจากใช้การอ้างอิง
///
/// ข้อเสีย:
/// - ต้องจัดการอายุการใช้งาน (lifetime) ของการอ้างอิง
///
/// ## เมื่อไหร่ควรใช้และไม่ควรใช้
/// - ใช้ Generic เมื่อคุณต้องการประสิทธิภาพสูงสุดและไม่กังวลเรื่องขนาดของไบนารี
/// - ใช้ Box<dyn Trait> เมื่อคุณต้องการเก็บประเภทที่แตกต่างกันในคอลเลกชันเดียวกัน
/// - ใช้ impl Trait เมื่อคุณต้องการโค้ดที่กระชับและไม่ต้องการระบุประเภทที่แน่นอน
/// - ใช้ &impl Trait เมื่อคุณต้องการลดการคัดลอกข้อมูลและยังคงใช้ Static Dispatch
trait Gear {
    fn use_gear(&self);
}

trait Gear2 {
    fn use_gear(&self);
}

trait Gear3 {
    fn use_gear(&self);
}

trait Gear4 {
    fn use_gear(&self);
}

struct Sword;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Using Sword");
    }
}

impl Gear2 for Sword {
    fn use_gear(&self) {
        println!("Using Sword from Gear 2");
    }
}

impl Gear3 for Sword {
    fn use_gear(&self) {
        println!("Using Sword from Gear 3");
    }
}

impl Gear4 for Sword {
    fn use_gear(&self) {
        println!("Using Sword from Gear 4");
    }
}

fn use_gear<T: Gear>(gear: T) {
    gear.use_gear();
}

fn use_gear2(gear: Box<dyn Gear2>) {
    gear.use_gear();
}

fn use_gear3(gear: impl Gear3) {
    gear.use_gear();
}

fn use_gear4(gear: &impl Gear4) {
    gear.use_gear();
}

fn main() {
    let my_sword = Sword;
    use_gear(my_sword);

    let my_sword2 = Box::new(Sword);
    use_gear2(my_sword2);

    let my_sword3 = Sword;
    use_gear3(my_sword3);

    let my_sword4 = Sword;
    use_gear4(&my_sword4);
}
