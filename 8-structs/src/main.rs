struct Crabby {
    name: String,
    health: u8, // max 100
}

impl Crabby {
    fn take_damanage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal > 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
    }
}

fn main() {
    let mut crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
    };

    crabby.take_damanage(100);
    println!("{} health: {}", crabby.name, crabby.health);

    crabby.take_damanage(50);
    println!("{} health: {}", crabby.name, crabby.health);

    crabby.healing(60);
    println!("{} health: {}", crabby.name, crabby.health);
}
