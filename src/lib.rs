type Connection<T> = Option<Box<Node<T>>>;

struct LinkedList<T>(Connection<T>);
impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }
    /// Add element to front, O(1)
    pub fn push(&mut self, data: T) -> &mut Self {
        let new_node = Node {
            data,
            next: self.0.take(),
        };
        *self = LinkedList(Some(Box::new(new_node)));
        self
    }
    /// Remove element from front, O(1)
    pub fn pop(&mut self) -> T {
        let curr = self.0.take().expect("list is not empty");
        *self = LinkedList(curr.next);
        curr.data
    }
    /// Return copy of list as Vec, O(n)
    pub fn as_vec(&self) -> Vec<T> {
        self.iter().map(|x| *x).collect()
    }
    /// Create LinkedList from vector
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = Self::new();
        vec.iter().for_each(|element| {
            list.push(*element);
        });
        list
    }
    pub fn iter(&self) -> Iter<'_, T> {
        let node = self.0.as_deref();
        Iter(node)
    }
}

struct Node<T> {
    data: T,
    next: Connection<T>,
}
pub struct Iter<'a, T>(Option<&'a Node<T>>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.0.take()?;
        self.0 = curr.next.as_deref();
        Some(&curr.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_front() {
        let mut list = LinkedList::new();
        list.push(1).push(2).push(3);
        assert_eq!(list.as_vec(), vec![3, 2, 1]);
    }
    #[test]
    fn push_pop() {
        let mut list = LinkedList::new();
        list.push(1).push(2).push(3);
        assert_eq!(list.pop(), 3);
        assert_eq!(list.pop(), 2);
        assert_eq!(list.pop(), 1);
    }
    #[test]
    fn to_from_vec() {
        let vec: Vec<i32> = vec![1, 2, 3];
        assert_eq!(
            LinkedList::from_vec(vec.clone()).as_vec(),
            vec.into_iter().rev().collect::<Vec<i32>>()
        );
    }
}
