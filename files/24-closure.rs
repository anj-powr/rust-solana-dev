// fn add(a:i32, b:i32) -> i32 {
//     a + b
// }

fn main() {

    // pipe sign are the closure
    // instead of having a function, we can use like this
    let add = |a:i32, b:i32| -> i32 {
        a + b
    };

    // or the short form of the closure
    // let add = |a,b| a+b;

    let sum = add(1,1);
    println!("{:?}",sum);

}