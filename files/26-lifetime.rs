#[derive(Debug)]
enum Answer {
    Yes,
    No
}

#[derive(Debug)]
// <'a> is the lifetime. 
// can be <'abc> as well. 
// usually indicating from a single character
struct Form<'abc> {
    question: &'abc Answer
}

fn main() {
    let answer = Answer::Yes;
    let form = Form {
        question: &answer
    };

    println!("{:?}", form);
}

