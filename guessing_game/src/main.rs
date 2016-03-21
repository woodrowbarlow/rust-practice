/* "extern crate" is like importing a package (in rust
 * terminology, a module). "use" is like importing a
 * specific object (in rust terminology, a type).
 * i need to use extern crate for modules that aren't in
 * the rust standard library.
 */

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

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

    let secret_number = rand::thread_rng().gen_range(1, 101);

    /* the loop statement creates an infinite loop. since
     * non-numeric input will throw a panic (due to the expect
     * when parsing it from a string), any non-numeric input
     * will cause the program to terminate (and, by extension,
     * break out of the loop). we've also added a break inside
     * the match case where the number was correctly guessed.
     */
    loop {
        println!("Please input your guess, between 1 and 100.");

        /* rust prefers everything to be immutable. it makes
         * things safer by design. for example, remember how
         * i had so much trouble working with `const` in C
         * because people often implemented functions that
         * accept a `char *` when they should have made it take
         * `const char *`? so we need to specify when something
         * might be mutated.
         */
        /* the double colon indicates that what follows is an
         * "associated function" of the preceding module or type.
         * an associated function is more or less like a static
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

        /* soon, we'll need to compare the guess to the our
         * secret_number. in rust, types are usually inferred;
         * nevertheless, it uses a strong static type system
         * (hooray!). we need to convert it to a number (in
         * this case, a 32-bit unsigned int), using methods
         * that are inherent to the String type.
         * like read_line, the parse method returns a Result
         * and we can use expect to throw a useful panic.
         * the colon is used when declaring a variable binding
         * in order to explicitly define a type. without it,
         * rust would attempt to infer a type. in this case,
         * though, the parse function is ambiguous.
         * in rust, you can overload a variable binding to a
         * new type. this destroys the old binding. it means
         * that i don't need to create a new binding just for
         * the integer version of guess (although i could if i
         * wanted to).
         */
        /* here we're dealing with the Result that is returned
         * by parse in a different way. rather than calling the
         * expect method, we're using the match statement to
         * control the code flow. Result is an enum just like
         * Ordering, so we can decide what to do in different
         * scenarios. i'm still hazy on what the arguments to
         * the enum values represent and how their rules
         * operate... but in the end, if it's okay, it returns
         * the number. if it's not, it prints a message and
         * continues.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        /* the "{}" is a "placeholder". it's like C's printf
         * format specifiers, but easier and smarter. it's
         * specific to the print macros.
         */
        println!("You guessed: {}", guess);
        // println!("The answer was: {}", secret_number);

        /* the Ordering type is an enum containing entries for
         * Less, Greater, and Equal. cmp is a method that can
         * be called on any type that can be compared (in java
         * terminology, we would say it "implements Comparable").
         * "match" is a "statement" that can control program flow
         * on an enum (in this case, the Ordering enum).
         * note that the cmp method takes an immutable reference
         * since it does not need to mutate its arguments.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
        }
    }
}
