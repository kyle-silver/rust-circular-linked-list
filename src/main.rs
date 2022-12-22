use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct CllNode<T> {
    value: T,
    prev: u128,
    next: u128,
}

#[derive(Debug)]
struct CircularLinkedList<T> {
    count: u128,
    head: Option<u128>,
    map: HashMap<u128, CllNode<T>>,
}

impl<T> CircularLinkedList<T> {
    fn new() -> Self {
        CircularLinkedList {
            count: 0,
            head: None,
            map: HashMap::new(),
        }
    }

    fn push(&mut self, value: T) {
        if let Some(head) = self.head {
            // get the head node
            let front = self.map.get(&head).unwrap();

            // create the node that we need to insert
            let to_insert = CllNode {
                value,
                prev: front.prev,
                next: head,
            };

            // update the tail node to point to our new node
            self.map
                .entry(to_insert.prev)
                .and_modify(|node| node.next = self.count);

            // update the head node to have our new node as its prev
            self.map
                .entry(head)
                .and_modify(|node| node.prev = self.count);

            // insert the new node
            self.map.insert(self.count, to_insert);

            // increment the count for next time
            self.count += 1;
        } else {
            self.head = Some(self.count);
            let to_insert = CllNode {
                value,
                prev: self.count,
                next: self.count,
            };
            self.map.insert(self.count, to_insert);
            self.count += 1;
        }
    }

    fn iter(&self) -> CllIterator<'_, T> {
        if let Some(head) = self.head {
            let first = self.map.get(&head).unwrap();
            let tail = first.prev;
            CllIterator {
                current_forward: Some(head),
                current_backward: Some(tail),
                list: &self,
            }
        } else {
            CllIterator {
                current_forward: None,
                current_backward: None,
                list: &self,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct CllIterator<'a, T> {
    current_forward: Option<u128>,
    current_backward: Option<u128>,
    list: &'a CircularLinkedList<T>,
}

impl<'a, T> Iterator for CllIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current_forward {
            let node = self.list.map.get(&current).unwrap();
            if current == self.current_backward.unwrap() {
                self.current_forward = None;
                self.current_backward = None;
            } else {
                self.current_forward = Some(node.next);
            }
            Some(&node.value)
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for CllIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current_backward {
            let node = self.list.map.get(&current).unwrap();
            if current == self.current_forward.unwrap() {
                self.current_forward = None;
                self.current_backward = None;
            } else {
                self.current_backward = Some(node.prev);
            }
            Some(&node.value)
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a CircularLinkedList<T> {
    type Item = &'a T;

    type IntoIter = CllIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> FromIterator<T> for CircularLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut cll = CircularLinkedList::new();
        for item in iter {
            cll.push(item);
        }
        cll
    }
}

#[test]
fn populate_empty_list() {
    let mut cll = CircularLinkedList::new();
    assert!(cll.map.is_empty());

    cll.push("Hello");
    let (index, f) = cll.map.iter().next().unwrap();
    assert_eq!(0, *index);
    assert_eq!(0, f.prev);
    assert_eq!(0, f.next);
    assert_eq!("Hello", f.value);
}

#[test]
fn insert_items() {
    // insert some data
    let mut cll = CircularLinkedList::new();
    cll.push("hello");
    cll.push("world");
    cll.push("foobar");

    // manually grab everything out of the map
    // we can maybe rewrite this once we have iterators
    let (_, a) = cll.map.iter().find(|(i, _)| **i == 0).unwrap();
    let (_, b) = cll.map.iter().find(|(i, _)| **i == 1).unwrap();
    let (_, c) = cll.map.iter().find(|(i, _)| **i == 2).unwrap();

    // first node
    assert_eq!(2, a.prev);
    assert_eq!(1, a.next);
    assert_eq!("hello", a.value);

    // second node
    assert_eq!(0, b.prev);
    assert_eq!(2, b.next);
    assert_eq!("world", b.value);

    // third node
    assert_eq!(1, c.prev);
    assert_eq!(0, c.next);
    assert_eq!("foobar", c.value);

    println!("{cll:?}");
}

#[test]
fn test_iter() {
    let mut cll = CircularLinkedList::new();
    cll.push(1);
    cll.push(2);
    cll.push(3);

    let forward: Vec<_> = cll.iter().collect();
    assert_eq!(vec![&1, &2, &3], forward);

    let backward: Vec<_> = cll.iter().rev().collect();
    assert_eq!(vec![&3, &2, &1], backward);
}

#[test]
fn double_ended_iter_doc_test() {
    let numbers: CircularLinkedList<_> = [1, 2, 3, 4, 5, 6].into_iter().collect();

    let mut iter = numbers.iter();

    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&6), iter.next_back());
    assert_eq!(Some(&5), iter.next_back());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&4), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
}
