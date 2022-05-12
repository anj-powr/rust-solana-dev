enum Colors {
    Red,
    Green,
    Blue,
    Black
}

fn main() {

    let go_color =  Colors::Green;

    match go_color {
        Colors::Red     => println!("Red Color"),
        Colors::Green   => println!("Green Color"),
        _               => println!("Another Color")
    }

}

