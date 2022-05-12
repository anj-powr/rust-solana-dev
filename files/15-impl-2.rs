fn main() {
    let f8 = Ship {
        ship_type : String::from("F8"), 
        power     : 2
    };

    f8.ship_type();
    f8.ship_power();


    let f4 = Ship::f4();
    f4.ship_type();
    f4.ship_power();
}

struct Ship {
    ship_type : String, 
    power     : i32,
}

impl Ship {
    fn ship_type(&self) {
        println!("Ship Type : {:?}", self.ship_type);
    }

    fn ship_power(&self) {
        println!("Ship Power: {:?}", self.power);
    }

    // creating a struct using function
    fn f4() -> Self {
        Self {
            ship_type   : "F4".to_owned(),
            power       : 6
        }
    }
}