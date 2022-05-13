mod greet {
    pub fn hello() {
        println!("Hello");
    }

    pub fn goodbye() {
        println!("Goodbye!");
    }
}

fn main() {
    greet::hello();
}

