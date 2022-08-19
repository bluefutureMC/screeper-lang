use std::collections::{LinkedList, HashMap};

pub struct Scope<T> {
    identifier_map: HashMap<String, LinkedList<(usize, T)>>,
    current_scope: usize
}

impl<T> Scope<T> {
    pub fn new() -> Self {
        Scope {
            identifier_map: HashMap::new(),
            current_scope: 0
        }
    }

    pub fn push(&mut self, identifier: &str, value: T) {

        if let Some(value_stack) = self.identifier_map.get_mut(identifier) {
            if value_stack.front().is_none() || value_stack.front().unwrap().0 != self.current_scope {
                value_stack.push_front((self.current_scope, value));
            }
            else {
                value_stack.front_mut().unwrap().1 = value;
            }
        }
        else {
            self.identifier_map.insert(String::from(identifier), LinkedList::from([(self.current_scope, value)]));
        }

    }

    pub fn get<'a>(&'a self, key: &str) -> Option<&'a T> {
        Some(&self.identifier_map.get(key)?.front()?.1)
    }

    pub fn expand(&mut self) {
        self.current_scope += 1;
    }

    pub fn collapse(&mut self) {
        self.current_scope -= 1;
        for value_stack in self.identifier_map.values_mut() {
            if match value_stack.front() { Some(front) => front.0, None => 0 } > self.current_scope {
                value_stack.pop_front();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_values_stack() {
        let mut scope: Scope<i32> = Scope::new();

        scope.push("num1", 1);
        scope.push("num2", 2);
        
        assert_eq!(*scope.get("num1").unwrap(), 1i32);
        assert_eq!(*scope.get("num2").unwrap(), 2i32);
        assert!(scope.get("num3").is_none());

        scope.expand();

        scope.push("num3", 3);
        scope.push("num1", 4);

        assert_eq!(*scope.get("num1").unwrap(), 4i32);
        assert_eq!(*scope.get("num2").unwrap(), 2i32);
        assert_eq!(*scope.get("num3").unwrap(), 3i32);

        scope.collapse();

        assert_eq!(*scope.get("num1").unwrap(), 1i32);
        assert_eq!(*scope.get("num2").unwrap(), 2i32);
        assert!(scope.get("num3").is_none());

    }
}