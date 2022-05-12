fn main() {
    let f8 = Ship {
        ship_type   :   String::from("F8"), 
                        // OR "F8".to_owned;
        power       :   2
    };

    println!("Ship Type : {:?}", f8.ship_type);
    println!("Ship Power: {:?}", f8.power);
}

struct Ship {
    ship_type : String, 
    power     : i32,
}

