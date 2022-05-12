fn main() {

    let a = 5;

    match a {
        0       => println!("0"),
        1 | 2   => println!("1 or 2"),
        3..=4   => println!("3 to 4"),
        _       => println!("diff number")
    }
    
}



