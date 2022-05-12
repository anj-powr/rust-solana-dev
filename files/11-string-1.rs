fn print_data(data : &str) {
    println!("{}",data);
}

fn main() {
    print_data("this is a string");

    let owned_string = "owned string".to_owned();
    let owned_another = String::from("another owned string");

    print_data(&owned_string);
    print_data(&owned_another);
}