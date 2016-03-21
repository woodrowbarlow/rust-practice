use std::io;

fn main() {

    /* the "!" shows that this is a macro rather than a
     * function. macros are built at compile-time, whereas
     * functions are at run-time. macros are implemented
     * as "compiler plugins", which are sort of like macros
     * in C's preprocessor.
     * question: can a module or type have a macro, or are
     * they all at the "global" level?
     * answer: they're never owned by a type... e.g., you
     * will never have `something.macro!()``. i'm not sure if
     * they can be owned by a module... e.g., you might have
     * `io::macro!()`.
     */
    println!("Guess the number!");
    println!("Please input your guess.");

    /* the double colon indicates that what follows is an
     * "associated function" of the preceding type. an
     * associated function is more or less like a static
     * method of a Java class.
     */
    let mut guess = String::new();

    /* the period indicates a "method" call. methods are
     * specific to an instance of a type -- similar to
     * non-static methods of a java class.
     */
    /* the "&mut guess" part is still a little hazy to me.
     * as i understand it, everything is passed by reference,
     * and references are immutable by default. since the
     * type signature for read_line says is needs a mutable
     * reference, we need to declare it as mutable.
     * question: if read_line did not expect a mutable ref,
     * would i pass "&guess" rather than "&mut guess"?
     * answer: yes.
     * even though the binding was declared as mutable, we
     * need to explicitly give a mutable reference to it b/c
     * it's possible to pass an immutable reference to mutable
     * data.
     * one of the tenets of rust's references is that if you
     * hold a mutable reference to data, then it is gauranteed
     * that you are the only one who can mutate its data, and
     * that if you hold an immutable reference to data, then
     * it is gauranteed that no one will mutate the data while
     * you hold it.
     */
    /* the read_line function returns a built-in type called
     * a result. results have a method called "expect" which
     * can throw a sort of "exception" (i don't know the
     * terminology for that yet) if the result couldn't be
     * loaded.
     * EDIT: it's called a "panic!".
     * the expect method is optional, but the compiler will
     * throw a warning if it isn't included. it's like the
     * -Wunused-value flag in C.
     */
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    /* the "{}" is a "placeholder". it's like C's printf
     * format specifiers, but easier and smarter. it's
     * specific to the print macros.
     */
    println!("You guessed: {}", guess);
}

/*
four kinds of types in rust:
 * primitives
 * products, aka structs, which let you bundle stuff together
 * sums, aka enums, which let you have one of a collection of types
 * derived, which include parameterized structs and enums
all types can have methods
*/
