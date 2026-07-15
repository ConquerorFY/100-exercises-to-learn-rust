// TODO: Use `Rc` and `RefCell` to implement `DropTracker<T>`, a wrapper around a value of type `T`
//  that increments a shared `usize` counter every time the wrapped value is dropped.

use std::cell::RefCell;
use std::rc::Rc;

pub struct DropTracker<T> {
    value: T,
    counter: Rc<RefCell<usize>>,
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: Rc<RefCell<usize>>) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        let mut a = self.counter.borrow_mut();
        *a += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let counter = Rc::new(RefCell::new(0));
        let _ = DropTracker::new((), Rc::clone(&counter));
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        let counter = Rc::new(RefCell::new(0));

        {
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }

        assert_eq!(*counter.borrow(), 2);
    }
}

// The Trace
// 1. let counter = Rc::new(RefCell::new(0));

//      - This creates one heap allocation containing a RefCell<usize> holding 0.

//      - The variable counter is an Rc pointing to that allocation. (Reference count = 1).

// 2. let _ = DropTracker::new((), Rc::clone(&counter));

//      - Rc::clone(&counter) creates a new Rc pointer that points to the same heap allocation.

//      - Now, both the counter variable in main and the counter field inside the DropTracker point to that same RefCell. (Reference count = 2).

// 3. The Statement Ends (let _ = ...;)

//      - The DropTracker was assigned to let _, which means it is immediately dropped at the end of this statement.

//      - The drop method runs: it borrows the RefCell mutably, increments the 0 to 1, and then finishes.

//      - The DropTracker is destroyed, and its internal Rc is dropped. (Reference count goes back to 1).

// 4. assert_eq!(*counter.borrow(), 1);

//      - The variable counter (the one in your test function) is still alive. It points to the same RefCell as before.

//      - It looks inside, sees the value is now 1, and the test passes.
