fn main() {
    
    let square_numbers: [u8; 5] = [1, 4, 9, 16, 25];
    let vowels: [char; 5]       = ['a','e','i','o','u'];
    let colors: [&str; 3]       = ["red","green","blue"];


    println!("First square number is : {}", square_numbers[0]);
    println!("vowels array length is : {}", vowels.len());
    
    for color in &colors {
        println!("For: {}", color);
    }

    println!("Print Array : {:?}", vowels);

}
