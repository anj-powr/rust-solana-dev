trait Details  {
    fn description(&self)   -> String;
    fn age_of_car(&self)    -> i32;
}

enum BodyType {
    Hatch,
    Sedan,
    Coupe,
    Suv
}

struct Car {
    make    : String,
    model   : String,
    bought  : i32,
    body    : BodyType
}

impl Details for Car {
    fn description(&self) -> String {
        return format! ("I got a {} {} !", self.make, self.model);
    }

    fn age_of_car(&self) -> i32 {
        return 2022 - self.bought;
    }
}


fn main() {
    let first_car = Car {
        make    : String::from("Nissan"),
        model   : String::from("Silvia"),
        bought  : 2010,
        body    : BodyType::Coupe
    };

    println!("{}",first_car.description());
    println!("The car is {} years old",first_car.age_of_car());
}

