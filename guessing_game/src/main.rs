// By default Rust brings only few types into the scope of every program,
// in the `prelude`.
// If a type you want isn't in the prelude, you'll need to explicitly import it
// using the use statement:

use std::io;

// fn defines a function.

fn main() {
    println!("Guess the number");
    println!("Please input the number for your guess");
    // Instantiate a new String variable
    // `let` allows you to bind a variable a value
    // In Rust, variables are immutable by default.
    // By specifying mut, we explicitly make the variable
    // guess mutable
    let mut guess = String::new();
    // On the other side of the equal (=)
    // guess is bound to what is returned by String::new(),
    // a functions returning a new instance of a string.
    // The `::` syntax near String indicates that new is an
    // `associated function` of the class String, aka static method

    //We now call another associated function, read_line, which belongs to the
    //class returned by `io::stdin()`
    // We call read_line by passing guess as a reference, with the & symbol:
    // A `reference` allows you to share data among different pieces of code
    // without needing to copy it.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // Actually read_line puts what the user is typing into guess, but also
    // returns an io::Result enumeration type.
    // An enumeration is a type that can have a fixed set of values, and those
    // values are called the enum's `variant`
    // For Result the variants are Ok or Err
    // Values of Result types have methods defined on them, like any other type.
    // An instance of io::Result has the expect method that you can call.
    // If the instance is an io::Error, this will make the program crash and
    // display to the user "Failed to read line"
    // If the instance is instead a io::Ok, this will make the computation
    // continue and display nothing to the user.
    //
    // Rust want you to call some method of the Result type:
    // if we omitted the .expect() call, a warning at compile time is
    // displayed
    println!("You've guessed {}", guess);
    // The brackets in the println! are placeholders for the values, in this
    // case guess.
}
