// Rust is an "ahead of time" compiled language
// i.e you can compile a program and give the executable to
// someone else; and they can run it without having rust installed


// This is just like C. This is the main program
fn main() {
    println!("Hello, world");  // println! is a Rust macro

    let answer = 20;
    println!("Answer: {}", answer);
    assert_eq!(answer, 20);
    // println!("{} -> {}", "18*2", "36");
    // println!("Base 2 representation:  {:b}", 42);
    // println!("My name is {0}, {1}", "ade");
    
    // loop1();
    add_loop();
    println!("Square: {}", sqr(2.0));
}

fn sqr(x: f64) -> f64 {
    x * x
}

fn add_loop() {
    let mut _sum = 0.0;
    for i in 0..5 {
        _sum = _sum + i as f32; // we are type-casting `i` here.
    }
    println!("Sum: {}", _sum);
}

// fn loop1() {
//     for i in 0..5 {
//         let even_odd = if i%2 == 0 {"even"} else {"odd"};
//         println!("{} {}", even_odd, i);
//     }
// }

// fn loop2() {
//     for i in 0..5 {
//         println!("Hello {}", i);
//         if i%2 == 0 {
//             println!("Even {}", i);
//         } else {
//             println!("Odd {}", i);
//         }
//     }
// }

