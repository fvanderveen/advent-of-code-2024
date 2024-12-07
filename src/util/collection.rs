// Allow dead_code since this is a util file copied across years. Later in the AoC we might use everything, or not.
#![allow(dead_code)]

pub trait CollectionExtension<T> {
    fn deduplicate(&self) -> Self;
    fn union(&self, other: &Self) -> Self;
    fn except(&self, item: &T) -> Self;
    fn prepend_item(&self, item: &T) -> Self;
    fn append_item(&self, item: &T) -> Self;
    fn push_all(&mut self, other: &Self);
    fn map<U, F>(&self, mapper: F) -> Vec<U> where F: Fn(&T) -> U;
}

impl<T> CollectionExtension<T> for Vec<T> where T: Clone + Eq {
    fn deduplicate(&self) -> Self {
        let mut result = vec![];
        for item in self {
            if !result.contains(item) { result.push(item.clone()) }
        }
        result
    }

    fn union(&self, other: &Self) -> Self {
        self.iter().cloned().filter(|v| other.contains(v)).collect()
    }

    fn except(&self, item: &T) -> Self {
        self.iter().cloned().filter(|v| v.ne(item)).collect()
    }

    fn prepend_item(&self, item: &T) -> Self {
        vec![item.clone()].into_iter().chain(self.clone()).collect()
    }

    fn append_item(&self, item: &T) -> Self {
        self.iter().cloned().chain(vec![item.clone()].into_iter()).collect()
    }

    fn push_all(&mut self, other: &Self) {
        for value in other {
            self.push(value.clone());
        }
    }

    fn map<U, F>(&self, mapper: F) -> Vec<U>
        where F: Fn(&T) -> U {
        self.iter().map(mapper).collect()
    }
}

pub trait VecToString {
    fn to_string(&self) -> Vec<String>;
}

impl<T> VecToString for Vec<T> where T : ToString {
    fn to_string(&self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}