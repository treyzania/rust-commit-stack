pub struct TxStack<T: Clone>(Vec<Vec<T>>);

impl<T: Clone> TxStack<T> {

    pub fn new() -> TxStack<T> {
        TxStack(vec![Vec::new()])
    }

    pub fn top(&self) -> &Vec<T> {
        &self.0[self.0.len() - 1]
    }

    pub fn push(&mut self, t: T) {
        let mut top = self.0.pop().unwrap(); // Never be empty.
        top.push(t);
        self.0.push(top);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut top = self.0.pop().unwrap(); // Never be empty.
        let out = top.pop();
        self.0.push(top);
        out
    }

    pub fn begin(&mut self) {
        let top = self.0.pop().unwrap(); // Never be empty.
        self.0.push(top.clone());
        self.0.push(top);
    }

    pub fn rollback(&mut self) {
        self.0.pop();
    }

    pub fn commit(&mut self) {
        let top = self.0.pop().unwrap(); // Never be empty.
        self.0.pop(); // Discard the one underneath.
        self.0.push(top);
    }

}

impl<T: Clone> Into<Vec<T>> for TxStack<T> {
    fn into(mut self) -> Vec<T> {
        if self.0.len() > 0 {
            self.0.pop().unwrap()
        } else {
            Vec::new()
        }
    }
}

impl<T: Clone> From<Vec<T>> for TxStack<T> {
    fn from(v: Vec<T>) -> TxStack<T> {
        TxStack(vec![v])
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_push_pop() {

        let mut ts = super::TxStack::new();
        ts.push(0);
        ts.push(1);

        assert_eq!(ts.pop().unwrap(), 1);
        assert_eq!(ts.pop().unwrap(), 0);

    }

    #[test]
    fn test_rollback() {

        let mut ts = super::TxStack::new();
        ts.push("foo");
        ts.push("bar");
        ts.begin();
        ts.push("baz");
        assert_eq!(ts.top().clone().pop().unwrap(), "baz");
        ts.rollback();
        assert_eq!(ts.pop().unwrap(), "bar");
        assert_eq!(ts.pop().unwrap(), "foo")

    }

    #[test]
    fn test_commit() {

        let mut ts = super::TxStack::new();
        ts.push("foo");
        ts.push("bar");
        ts.begin();
        ts.push("baz");
        assert_eq!(ts.top().clone().pop().unwrap(), "baz");
        ts.commit();
        assert_eq!(ts.pop().unwrap(), "baz");
        assert_eq!(ts.pop().unwrap(), "bar");
        assert_eq!(ts.pop().unwrap(), "foo")

    }

}
