// 9ms - 6.1 MB
struct MinStack {
    v: Vec<i32>,
    // min is also a stack to store the minimum value at a point of the main stack
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            v: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.v.push(val);

        let last = self.min.last();
        match last {
            None => self.min.push(val),
            Some(x) => {
                if *x > val {
                    self.min.push(val);
                } else {
                    self.min.push(*x);
                }
            }
        }
    }

    fn pop(&mut self) {
        self.v.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        let last = self.v.last();
        match last {
            None => -1,
            Some(x) => *x,
        }
    }

    fn get_min(&self) -> i32 {
        let last = self.min.last();
        match last {
            None => -1,
            Some(x) => *x,
        }
    }
}
