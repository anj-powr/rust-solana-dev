fn main() {
    let mut vec = vec![1,2,4,5];

    for v in &vec {
        println!("{}",v);
    }

    println!("Vector Length : {}",vec.len());
    println!("Position 0 : {}",vec[0]);


    vec.push(6);
    vec.remove(1);

    println!("New Values : {:?}",vec);
}

