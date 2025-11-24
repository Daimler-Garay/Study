// https://rust-unofficial.github.io/patterns/idioms/default.html
// Many types in Rust have a constructor. However, this is specific
// to the type; Rust cannot abstract over "everything that has a 'new()'
// method". To allow this, the 'Default' trait was conceived, which can
// be used with containers and other generic types. Some containers
// already implement it where applicable.

// Constructors can take multiple arguments, while the 'default()' method
// does not. There can be multiple constructors with different names, but
// there can only be one 'Default' implementation per type.

use std::{path::PathBuf, time::Duration};

// note that we can simple auto-derive Default here.
#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    // Option defaults  to None
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

impl MyConfiguration {
    // add setters here
}

fn main() {
    // construct a new instance with default values
    let mut conf = MyConfiguration::default();
    // do something with conf here
    conf.check = true;
    println!("conf = {conf:#?}");

    // partial initialization with default values, creates the same instance
    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}
