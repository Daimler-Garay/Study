// https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
// Rust does not provide the equivalent to 'finally' blocks - code that
// will be executed no matter how a function is exited. Instead, an
// objects destructor can be used to run code that must be run before
// exit.

fn baz() -> Result<(), ()> {
    // some code
}

fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The dtor of _exit will run however the function 'bar' is exited.
    let _exit = Foo;
    // Implicit return with '?' operator.
    baz()?;
    // Normal return.
    Ok(())
}

// If a function has multiple return points, then executing code
// on exit becomes difficult and repetitive (and thus bug-prone).
// This is especially the case where return is implicit due to a
// macro. A common case is '?' operator which returns if the result
// is an 'Err', but continues if it is 'Ok'. '?' is used as an
// exception handling mechanism, but unlike Java (which has 'finally'),
// there is no way to schedule code to run in both the normal and exceptional
// cases. Panicking will also exit a funciton early.

// Code in destructors will (nearly) always be run - copes with panics,
// early returns, etc.

// It is not guaranteed destructors will run. For example, if there is an
// infinite loop in a function or if running a function crashes before exit.
// Destructors are also not run in the case of a panic in an already panicking
// thread. Therefor, destructors cannot be relied on as a finalizers where it
// is absolutely essential that finalization happens.

// This pattern introduces some hard to notice, implicit code. Reading a
// function gives no clear indication of destructors to be run on exit.
// This can make debugging tricky.

// Requiring an object and 'Drop' impl just for finalization is heavy on
// boilerplate.

// An object used as a finalizer must be kept alive until the end of the function
// and must then be destroyed. The object must always be a value or unique owned
// pointer (e.g., 'Box<Foo>'). If a shraed pointer ('Rc') is used, then the finalizer
// can be kept alive beyond the lifetime of the function. For similar reasons, the
// finalizer should not be moved or returned.

// The finalizer must be assigned into a variable, else it will be destroyed immediately,
// rather than when it goes out of scope. The variable name should start with '_' if the
// variable is only used as a finalizer, otherwise the compiler will warn that the finalizer
// is never used.
