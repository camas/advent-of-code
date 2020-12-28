use std::iter;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let cup_values = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let highest_val = *cup_values.iter().max().unwrap();
    assert_eq!(highest_val, cup_values.len());
    let first_cup = cup_values[0];

    let mut cups = vec![0; cup_values.len() + 1];
    for (a, b) in cup_values.iter().zip(cup_values.iter().skip(1)) {
        cups[*a] = *b;
    }
    cups[*cup_values.last().unwrap()] = cup_values[0];
    let result1 = cup_game(cups.clone(), 100, first_cup);
    let mut part1_digits = Vec::new();
    let mut index = 1;
    for _ in 0..(highest_val - 1) {
        index = result1[index];
        part1_digits.push(index);
    }
    let part1 = part1_digits
        .into_iter()
        .map(|d| format!("{}", d))
        .collect::<String>();

    let mut cups = cups
        .into_iter()
        .chain((highest_val + 2)..=1_000_000)
        .chain(iter::once(cup_values[0]))
        .collect::<Vec<_>>();
    cups[*cup_values.last().unwrap()] = 10;
    let result2 = cup_game(cups, 10_000_000, first_cup);
    let part2 = result2[1] * result2[result2[1]];

    (part1, part2)
}

fn cup_game(initial_cups: Vec<usize>, rounds: usize, first_cup: usize) -> Vec<usize> {
    let mut cups = initial_cups;
    let highest = cups.len() - 1;
    let mut index = first_cup;
    for _ in 0..rounds {
        let start = index;
        let a = cups[start];
        let b = cups[a];
        let c = cups[b];
        let end = cups[c];
        let mut after = start - 1;
        if after == 0 {
            after = highest;
        }
        while after == a || after == b || after == c {
            after -= 1;
            if after == 0 {
                after = highest;
            }
        }
        let temp = cups[after];
        cups[index] = end;
        cups[after] = a;
        cups[c] = temp;

        index = cups[index];
    }
    cups
}

// fn cup_game(mut cups: LinkedList<u32>, rounds: usize, highest_val: u32) -> LinkedList<u32> {
//     for _ in 0..rounds {
//         let start = cups.pop_front().unwrap();
//         let next3 = (0..3)
//             .map(|_| cups.pop_front().unwrap())
//             .collect::<Vec<_>>();
//         let mut dest = start - 1;
//         if dest == 0 {
//             dest = highest_val;
//         }
//         while next3.contains(&dest) {
//             dest -= 1;
//             if dest == 0 {
//                 dest = highest_val;
//             }
//         }
//         cups.extend_after(&dest, next3.into_iter());
//         cups.push_back(start);
//     }
//     cups
// }

// #[derive(Debug)]
// struct LinkedList<T: Eq> {
//     head: Option<Box<Node<T>>>,
//     tail: *mut Node<T>,
// }

// #[derive(Debug)]
// struct Node<T> {
//     value: T,
//     next: Option<Box<Node<T>>>,
// }

// impl<T: Eq> LinkedList<T> {
//     fn new() -> Self {
//         Self {
//             head: None,
//             tail: null_mut(),
//         }
//     }

//     /// Removes and returns an item from the front of the list
//     fn pop_front(&mut self) -> Option<T> {
//         self.head.take().map(|head| {
//             self.head = head.next;
//             if self.head.is_none() {
//                 self.tail = null_mut();
//             }
//             head.value
//         })
//     }

//     /// Appends a value to the front of the list
//     fn push_front(&mut self, value: T) {
//         let mut node = Box::new(Node { value, next: None });
//         if self.head.is_none() {
//             self.tail = &mut *node as *mut _;
//             self.head = Some(node);
//         } else {
//             let old_head = mem::replace(&mut self.head, None);
//             node.next = old_head;
//             self.head = Some(node);
//         }
//     }

//     /// Appends a value to the back of the list
//     fn push_back(&mut self, value: T) {
//         let mut new_tail = Box::new(Node { value, next: None });
//         let new_tail_ptr = &mut *new_tail as *mut _;
//         if self.tail.is_null() {
//             self.head = Some(new_tail);
//         } else {
//             unsafe {
//                 (*self.tail).next = Some(new_tail);
//             }
//         }
//         self.tail = new_tail_ptr;
//     }

//     /// Searches for the initial value and appends the new values after it
//     fn extend_after(&mut self, initial_value: &T, values: impl Iterator<Item = T>) {
//         let mut node = self.head.as_mut().unwrap();
//         while &node.value != initial_value {
//             node = node.next.as_mut().unwrap();
//         }
//         let node_after = mem::replace(&mut node.next, None);
//         for value in values.into_iter() {
//             let new_node = Box::new(Node { value, next: None });
//             node.next = Some(new_node);
//             node = node.next.as_mut().unwrap();
//         }
//         if node_after.is_none() {
//             self.tail = &mut **node as *mut _;
//         } else {
//             node.next = node_after;
//         }
//     }

//     fn iter(&self) -> Iter<'_, T> {
//         Iter {
//             next: self.head.as_ref().map(|node| &**node),
//         }
//     }
// }

// impl<T: Eq> Drop for LinkedList<T> {
//     fn drop(&mut self) {
//         // Drop iteratively instead of letting the compiler do it recursively
//         let mut curr = self.head.take();
//         while let Some(mut node) = curr {
//             curr = node.next.take();
//         }
//     }
// }

// struct Iter<'a, T> {
//     next: Option<&'a Node<T>>,
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_ref().map(|node| &**node);
//             &node.value
//         })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_list() {
//         let mut list = LinkedList::new();
//         list.push_back(1);
//         list.push_back(2);
//         list.push_back(3);
//         assert_eq!(list.pop_front().unwrap(), 1);
//         list.push_front(4);
//         list.push_front(5);
//         assert_eq!(list.pop_front().unwrap(), 5);
//         list.extend_after(&2, vec![8, 9].into_iter());
//         list.extend_after(&3, vec![6, 7].into_iter());
//         assert_eq!(
//             list.iter().cloned().collect::<Vec<_>>(),
//             vec![4, 2, 8, 9, 3, 6, 7]
//         );
//     }
// }
