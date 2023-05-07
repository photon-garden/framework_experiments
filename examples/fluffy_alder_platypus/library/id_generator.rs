use std::cell::RefCell;

#[derive(Clone)]
pub struct IdGenerator {
    next_id: RefCell<usize>,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator {
            next_id: RefCell::new(0),
        }
    }

    pub fn next(&self) -> usize {
        let mut current_id = self.next_id.borrow_mut();
        let current_id_copy = *current_id;
        *current_id = current_id_copy + 1;

        current_id_copy
    }
}