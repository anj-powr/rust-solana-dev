fn main() {
    let square_numbers: [u8; 5] = [1, 4, 9, 16, 25];

    let slice = &square_numbers[1 .. 3]; // slicing
    get_slice(square_numbers, slice);

}

fn get_slice(array: [u8; 5], slice: &[u8]) {

    println!("{:?}", array);
    println!("{:?}", slice);
    println!("Slice Length : {:?}", slice.len());
    println!("Slices #1 is {},  #2 is {}", slice[0], slice[1]);

}
