use std::collections::{LinkedList, HashMap};

/// A structure that keeps track of values of identifiers that exist in cascading scopes
/// 
/// # Example
/// ```
/// use screeper::Scope;
/// 
/// let mut scope: Scope<i32> = Scope::new();
/// 
/// scope.push("number", 1);
/// assert_eq!(*scope.get("number").unwrap(), 1);
/// assert!(scope.get("bla").is_none());
/// 
/// scope.expand();
/// 
/// scope.push("number", 2);
/// scope.push("number", 3);
/// scope.push("bla", 100);
/// 
/// assert_eq!(*scope.get("number").unwrap(), 3);
/// assert_eq!(*scope.get("bla").unwrap(), 100);
/// 
/// scope = scope.collapse();
/// 
/// assert_eq!(*scope.get("number").unwrap(), 1);
/// assert!(scope.get("bla").is_none());
/// ```
pub struct Scope<T> {
    identifier_map: HashMap<String, LinkedList<(usize, T)>>,
    current_scope: usize
}

impl<T> Scope<T> {

    /// Constructs a new Scope instance
    pub fn new() -> Self {
        Scope {
            identifier_map: HashMap::new(),
            current_scope: 0
        }
    }

    /// Sets the given `identifier`'s value for the current scope to `value`.
    /// When the current scope ends, this value is going to be dropped and the identifier will return to it's previous value or will be dropped completely.
    /// 
    /// # Example
    /// ```
    /// use screeper::Scope;
    /// 
    /// let mut scope: Scope<i32> = Scope::new();
    /// 
    /// scope.push("number", 1);
    /// assert_eq!(*scope.get("number").unwrap(), 1);
    /// assert!(scope.get("bla").is_none());
    /// 
    /// scope.expand();
    /// 
    /// scope.push("number", 2);
    /// scope.push("number", 3);
    /// scope.push("bla", 100);
    /// 
    /// assert_eq!(*scope.get("number").unwrap(), 3);
    /// assert_eq!(*scope.get("bla").unwrap(), 100);
    /// 
    /// scope = scope.collapse();
    /// 
    /// assert_eq!(*scope.get("number").unwrap(), 1);
    /// assert!(scope.get("bla").is_none());
    /// ```
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

    /// Gets the given `identifier`'s value for the current scope.
    /// 
    /// # Example
    /// ```
    /// use screeper::Scope;
    /// 
    /// let mut scope: Scope<i32> = Scope::new();
    /// 
    /// scope.push("number", 1);
    /// assert_eq!(*scope.get("number").unwrap(), 1);
    /// assert!(scope.get("bla").is_none());
    /// 
    /// scope.expand();
    /// 
    /// scope.push("number", 2);
    /// scope.push("number", 3);
    /// scope.push("bla", 100);
    /// 
    /// assert_eq!(*scope.get("number").unwrap(), 3);
    /// assert_eq!(*scope.get("bla").unwrap(), 100);
    /// 
    /// scope = scope.collapse();
    /// 
    /// assert_eq!(*scope.get("number").unwrap(), 1);
    /// assert!(scope.get("bla").is_none());
    /// ```
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a T> {
        Some(&self.identifier_map.get(key)?.front()?.1)
    }

    /// Expands the current scope. Any values pushed after this method is executed will be dropped after the corrosponding collapse.
    /// 
    /// # Example
    /// ```
    /// use screeper::Scope;
    /// 
    /// let mut scope: Scope<i32> = Scope::new();
    /// 
    /// scope.push("num", 1);
    /// 
    /// scope.expand();
    /// 
    /// scope.push("num", 2);
    /// scope.push("num", 3);
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 3);
    /// 
    /// scope.expand();
    /// 
    /// scope.push("num", 4);
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 4);
    /// 
    /// scope = scope.collapse();
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 3);
    /// 
    /// scope = scope.collapse();
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 1);
    /// ```
    pub fn expand(&mut self) {
        self.current_scope += 1;
    }

    /// Collapses the current scope. Any values pushed during the current scope will be reverted to their state in the previous scope.
    /// This method consumes the scope instance and returns a new one to force the lifetime of any references to previous values to end.
    /// 
    /// # Example
    /// ```
    /// use screeper::Scope;
    /// 
    /// let mut scope: Scope<i32> = Scope::new();
    /// 
    /// scope.push("num", 1);
    /// 
    /// scope.expand();
    /// 
    /// scope.push("num", 2);
    /// scope.push("num", 3);
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 3);
    /// 
    /// scope.expand();
    /// 
    /// scope.push("num", 4);
    /// 
    /// let current_num: &i32 = scope.get("num").unwrap();
    /// 
    /// assert_eq!(*current_num, 4);
    /// 
    /// scope = scope.collapse();
    /// 
    /// // assert_eq!(*current_num, 3); X
    /// // wont compile because current_num is no longer a valid reference
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 3);
    /// 
    /// scope = scope.collapse();
    /// 
    /// assert_eq!(*scope.get("num").unwrap(), 1);
    /// ```
    pub fn collapse(mut self) -> Self {
        self.current_scope -= 1;
        for value_stack in self.identifier_map.values_mut() {
            if match value_stack.front() { Some(front) => front.0, None => 0 } > self.current_scope {
                value_stack.pop_front();
            }
        }

        self
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
        
        assert_eq!(*scope.get("num1").unwrap(), 1);
        assert_eq!(*scope.get("num2").unwrap(), 2);
        assert!(scope.get("num3").is_none());

        scope.expand();

        scope.push("num3", 3);
        scope.push("num1", 4);

        assert_eq!(*scope.get("num1").unwrap(), 4);
        assert_eq!(*scope.get("num2").unwrap(), 2);
        assert_eq!(*scope.get("num3").unwrap(), 3);

        scope = scope.collapse();

        assert_eq!(*scope.get("num1").unwrap(), 1);
        assert_eq!(*scope.get("num2").unwrap(), 2);
        assert!(scope.get("num3").is_none());

    }
}