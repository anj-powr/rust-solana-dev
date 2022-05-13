enum Position {
    Manager,
    Supervisor,
    Worker
}

struct Employee  {
    position : Position,
    work_hours : i64
}

fn main() {

    let me = Employee {
        position : Position::Worker,
        work_hours : 40
    };

    match me.position {
        Position::Manager => println!("Manager"),
        Position::Supervisor => println!("Supervisor"),
        Position::Worker => println!("Worker"),
    }
    
}


