// https://rust-unofficial.github.io/patterns/idioms/ctor.html
// Rust does not have constructors as a language construct. Instead,
// the convention is to use an associated function 'new' to create an
// object:

pub struct Second {
    value: u64,
}

impl Second {
    // Constructs a new instance of ['Second'].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    // Returns the value in seconds
    pub fn value(&self) -> u64 {
        self.value
    }
}

// Rust also supports default constructors via 'Default' trait:

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

// 'Default' can also be derived if all the fields in the type
// implement 'Default'

// #[derive(Default)]
pub struct Second {
    value: u64, // u64 already implements 'Default'
}

// It is expected of types to implement both 'Default' and an empty 'new'
// constructor. 'new' is the constructor convention in Rust.
// Another advantage of implementing or deriving 'Default' is that
// your type can now be used where a 'Default' implementation is
// required - any of the '*or_default' functions in the std library.
