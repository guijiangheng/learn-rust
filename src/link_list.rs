pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.value)
    }

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into_iter()
    }
}

impl<T> Default for LinkList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

pub struct IntoIter<T>(LinkList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for LinkList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take()?;
        self.node = node.next.as_deref();
        Some(&node.value)
    }
}

impl<'a, T> IntoIterator for &'a LinkList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            node: self.head.as_deref(),
        }
    }
}

pub struct IterMut<'a, T> {
    node: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take()?;
        self.node = node.next.as_deref_mut();
        Some(&mut node.value)
    }
}

impl<'a, T> IntoIterator for &'a mut LinkList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            node: self.head.as_deref_mut(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkList<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.iter();
        while let Some(x) = iter.next() {
            match iter.node {
                Some(_) => write!(fmt, "{:?} -> ", x)?,
                None => write!(fmt, "{:?}", x)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_list() -> LinkList<i32> {
        let mut a = LinkList::new();
        a.push_front(1);
        a.push_front(2);
        a.push_front(3);
        a
    }

    #[test]
    fn push_front() {
        let a = create_list();
        for (k, x) in a.iter().enumerate() {
            assert_eq!(*x, 3 - (k as i32));
        }
    }

    #[test]
    fn pop_front() {
        let mut a = create_list();
        let e0 = a.pop_front();
        let e1 = a.pop_front();
        let e2 = a.pop_front();
        let e3 = a.pop_front();
        assert_eq!(e0, Some(3));
        assert_eq!(e1, Some(2));
        assert_eq!(e2, Some(1));
        assert_eq!(e3, None);
    }

    #[test]
    fn debug_print() {
        let a = create_list();
        assert_eq!(format!("{:?}", a), "3 -> 2 -> 1");
    }

    #[test]
    fn iter_mut() {
        let mut a = create_list();
        for x in &mut a {
            *x += 1;
        }
        assert_eq!(format!("{:?}", a), "4 -> 3 -> 2");
    }
}
