use std::io;

fn main() {
    println!("Guess the number: ");
    // `let` is used to create variables;
    // let apples = 5;
    //     In rust, variables are immutable by default
    // let mut apples = 5;
    //     This makes `apples` mutable.
    //
    //  ::new in String::new indicates that new is an
    //  associated function of the String type (associated
    //  functions are implemented on a type)
    let mut guess = String::new();

    // '&' is a reference; a way to let multiple parts of code
    // access one piece of data without needing to copy the data
    // into memory multiple times.
    //
    // `.readline` returns a Result (variant --> "Ok" and "Err")
    // which is an enumeration, enum, (which can be in one of multiple
    // possible states).
    // NB: each possible state is a variant
    // Results encode error-handling information.
    //   : 0k --> successful
    //   : Err --> unsuccessful
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
