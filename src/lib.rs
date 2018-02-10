#[macro_export]
macro_rules! stack {
    ($($x:expr),* ) => {
        {
            let temp_stack = Stack::new();
            $(
                let temp_stack = temp_stack.push($x);
            )*
            temp_stack
        }
    }
}

#[derive(Debug)]
pub struct Stack {
    values: Vec<i32>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }
    pub fn push(self, val: i32) -> Stack {
        let mut new_values = self.values.clone();
        new_values.push(val);

        Stack { values: new_values }
    }
    pub fn pop(self) -> Stack {
        let mut new_values = self.values.clone();
        new_values.pop();
        Stack { values: new_values }
    }
    pub fn get(&self) -> Option<&i32> {
        self.values.last()
    }
    pub fn add(self) -> Stack {
        let mut new_values = self.values.clone();
        let a = new_values.pop().unwrap();
        let b = new_values.pop().unwrap();
        new_values.push(a + b);
        Stack { values: new_values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_values() {
        let result = Stack::new().push(1).push(2).push(3).add();
        assert_eq!(*result.get().unwrap(), 5);

        let s: Stack = stack![1, 2, 3];
        let s = s.push(4);
        assert_eq!(s.values, vec![1, 2, 3, 4]);
    }
}
