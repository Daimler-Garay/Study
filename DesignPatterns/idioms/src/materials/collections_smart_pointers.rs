// https://rust-unofficial.github.io/patterns/idioms/deref.html
// Use the 'Deref' trait to treat collections like smart pointers,
// offering owning and borrowed views of data.

use std::ops::Deref;

struct Vec<T> {
    data: RawVec<T>,
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        //..
    }
}

// A 'Vec<T>' is an owning collection of 'T's, while a slice ('&[T]')
// is a borrowed collection of 'T's. Implementing 'Deref' for 'Vec'
// allows implicit derefencing from '&Vec<T>' to '&[T]' and includes
// the relationship in auto-derefencing searches. Most methods you
// might expect to be implemented for 'Vec's are instead implemented
// for slices.

// 'String' and '&str' have a similar relation.

// Most methods can be implemented only for the borrowed view, they
// are then implicitly available for the owning view. Which gives
// clients a choice between borrowing or taking ownership of data.

// Smart pointers and collections are analogous: a smart pointer
// points to a single object, whereas a collection points to many.
// From the point of view of the type system, there is little
// difference between the two.
