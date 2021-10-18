use std::{
    collections::{HashMap, HashSet},
    ops::{Range, RangeBounds},
};

#[test]
fn parallel() {
    use core::sync::atomic::{AtomicU32, Ordering};
    use core::time::Duration;
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    let barrier = Arc::new(Barrier::new(2));
    let mutex = Arc::new(Mutex::new(()));
    let func = move || {
        barrier.wait();
        for j1 in 0..16 {
            let lock = mutex.lock();
            for j2 in 0..32 {
                print!("[{} {}] ", j1, j2);
            }
            println!();
            drop(lock);
            thread::sleep(Duration::from_nanos(1));
        }
    };

    let t1 = thread::spawn(func.clone());
    let t2 = thread::spawn(func);
    t1.join().unwrap();
    t2.join().unwrap();
}

#[test]
fn iterator() {
    //let a = vec![1, 1, 2, 1, 2, 3, 1, 2, 3, 4, 1, 2, 3, 4, 5].into_iter();

    struct Data {
        current_counter: i32,
        current_max: i32,
    }

    impl Data {
        fn new() -> Self {
            Self::default()
        }
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                current_counter: 1,
                current_max: 1,
            }
        }
    }

    impl Iterator for Data {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let ret = self.current_counter;

            if ret == self.current_max {
                self.current_counter = 1;
                self.current_max += 1;
            } else {
                self.current_counter += 1;
            }
            Some(ret)
        }
    }

    //  conter  1   1   2   1   2   3   1   2   3   4   1   2   3   4   5
    //  cmax    1   2   2   3   3   3   4   4   4   4   5   5   5   5   5
    //  ret     1   1   2   1   2   3   1   2   3   4   1   2   3   4   5

    let mut data = Data::default();
    assert_eq!(data.next(), Some(1)); // 1
    assert_eq!(data.next(), Some(1)); // 1
    assert_eq!(data.next(), Some(2)); //    2
    assert_eq!(data.next(), Some(1)); // 1
    assert_eq!(data.next(), Some(2)); //    2
    assert_eq!(data.next(), Some(3)); //       3
    assert_eq!(data.next(), Some(1)); // 1
    assert_eq!(data.next(), Some(2)); //    2
    assert_eq!(data.next(), Some(3)); //       3
    assert_eq!(data.next(), Some(4)); //          4
    assert_eq!(data.next(), Some(1)); // 1
    assert_eq!(data.next(), Some(2)); //    2
    assert_eq!(data.next(), Some(3)); //       3
    assert_eq!(data.next(), Some(4)); //          4
    assert_eq!(data.next(), Some(5)); //             5

    assert_eq!(
        Data::default().take(15).collect::<Vec<i32>>(),
        vec![1, 1, 2, 1, 2, 3, 1, 2, 3, 4, 1, 2, 3, 4, 5]
    );

    /*
        Exercises:
        2.
        (a) Make an struct and implement Iterator trait for it that gives sequence:
                1,  1, 2,  1, 2, 3,  1, 2, 3, 4,  1, 2, 3, 4, 5,  ...

        (b) Create iterators using only core/std functions/methods/traits:
                0, 1, 2, 3, ...;
                0, 2, 4, 6, ...;
                5, 6, 5, 6, ...;
                10, 9, 8, 7, ..., 2, 1, 0;
                1, 2, 3, 4, 3, 2, 1;
                1, 100, 101, 102, 103, 1000, 3, 2, 1, 1, 1, 1, 1, 1, ...;
            Hint: (a..b), (a..=b).
            Hint: core::iter::once(), core::iter::repeat(),
            Hint: Iterator::rev(), Iterator::step_by(), Iterator::chain(), Iterator::flatten().
    */
}

#[test]
fn exercise1b() {
    #[derive(PartialEq, Debug, Copy, Clone)]
    struct Data {
        current_counter: i32,
    }

    impl Data {
        fn new() -> Self {
            Self { current_counter: 0 }
        }

        fn start_with(start_counter: i32) -> Self {
            Self {
                current_counter: start_counter,
            }
        }
    }

    trait DataFunctions {
        fn increment_by_one(&mut self) -> &mut Self;
        fn increment_by_two(&mut self) -> &mut Self;
        fn oscillate(&mut self, upper_bound: i32, lower_bound: i32) -> &mut Self;
        // fn oscillate_range<T>(&mut self, range: Range<T>) -> &mut Self
        // where
        //     i32: PartialEq<T>;
    }

    impl DataFunctions for Data {
        fn increment_by_one(&mut self) -> &mut Self {
            self.current_counter += 1;
            self
        }

        fn increment_by_two(&mut self) -> &mut Self {
            self.current_counter += 2;
            self
        }
        fn oscillate(&mut self, upper_bound: i32, lower_bound: i32) -> &mut Self {
            if self.current_counter == lower_bound {
                self.current_counter = upper_bound;
            } else {
                self.current_counter = lower_bound;
            }
            self
        }
        // fn oscillate_range<T>(&mut self, range: Range<T>) -> &mut Self
        // where
        //     i32: PartialEq<T>,
        // {
        //     if self.current_counter == range.start {
        //         self.current_counter = range.start;
        //     } else {
        //         self.current_counter = range.end;
        //     }
        //     self
        // }
    }

    // Test
    let mut data = Data::new();

    data.increment_by_one();
    assert_eq!(data.current_counter, 1);
    data.increment_by_one();
    data.increment_by_two();
    assert_eq!(data.current_counter, 4);

    dbg! {data};

    let mut data2 = Data::start_with(5);
    assert_eq!(data2.current_counter, 5);
    dbg! {data2};
    data2.oscillate(5, 6).oscillate(5, 6);

    assert_eq!(data2.current_counter, 5);
}

#[test]
fn exercise1b2() {
    use core::iter::{once, repeat};

    let iter = 0..;
    check_first_n(iter, &[0, 1, 2, 3, 4, 5, 6, 7]);

    let iter = (0..).step_by(2);

    let iter = check_first_n(iter, &[0, 2, 4, 6, 8, 10, 12]);

    let iter = (5..=6).cycle();
    check_first_n(iter, &[5, 6, 5, 6, 5, 6, 5, 6]);

    let iter = (0..=10).rev();
    check_all(iter, &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    let iter = (1..=4).chain((1..=3).rev());
    check_all(iter, &[1, 2, 3, 4, 3, 2, 1]);

    let iter = once(1)
        .chain(100..=103)
        .chain(once(1000))
        .chain((1..=3).rev())
        .chain(repeat(500).take(4))
        .chain(repeat(1));

    check_first_n(
        iter,
        &[
            1, 100, 101, 102, 103, 1000, 3, 2, 1, 500, 500, 500, 500, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
        ],
    );

    let data: Vec<_> = (0..100).collect();

    let mut iter = (100..120).zip('a'..'z');
    assert_eq!(iter.next(), Some((100, 'a')));
    assert_eq!(iter.next(), Some((101, 'b')));
    assert_eq!(iter.next(), Some((102, 'c')));

    /*
    for i in 0..=10 {
        println!("{}", i);
    }

    let iter = (0..=10).into_iter(); // 0..=10;
    while Some(j) = iter.next() {
        println!("{}", i);
    }
    */
}

fn check_first_n<T: Iterator<Item = i32>>(iter: T, expected: &[i32]) {
    assert_eq!(iter.take(expected.len()).collect::<Vec<i32>>(), expected);
}

fn check_all<T: Iterator<Item = i32>>(iter: T, expected: &[i32]) {
    assert_eq!(iter.collect::<Vec<i32>>(), expected);
}

/*
    for item in vec[1, 2, 3].iter() {

    }

    //

    let mut iter = vec[1, 2, 3].iter();
    while let Some(item) in iter.next() {

    }

    //

    1.
    (a) Make an struct and implement Iterator trait for it that gives sequence:
            1,  1, 2,  1, 2, 3,  1, 2, 3, 4,  1, 2, 3, 4, 5,  ...
    (b) Create iterators using only core/std functions/methods/traits:
            0, 1, 2, 3, ...;
            0, 2, 4, 6, ...;
            5, 6, 5, 6, ...;
            10, 9, 8, 7, ..., 2, 1, 0;
            1, 2, 3, 4, 3, 2, 1;
            1, 100, 101, 102, 103, 1000, 3, 2, 1, 1, 1, 1, 1, 1, ...;
        Hint: (a..b), (a..=b).
        Hint: core::iter::once(), core::iter::repeat().
        Hint: Iterator::rev(), Iterator::step_by(), Iterator::chain(), Iterator::flatten().

    2.
    (a) Create a struct that stores u32 with a Vec but not expose it publically.
    (b) Add function new.
    (b) Add methods new/is_empty/len/push/pop.
    (c) Make struct generic to allow any values as items.
    (d) Add member for quick lookup by value.
        Hint: we can use HashMap<value, index>.
    (e) Add method contains_value.
    (f) Add methods items, ids that return references to inner data.

    (g) Manually implement Debug trait.
    (h) Implement AsRef<[T]>, AsMut<[T]>, Into<Vec<T>> traits.
    (i) Implement Index trait.
    (j) Implement IntoIterator trait.

    3.
    (k) Copy struct from exercise 2 and rename it.
    (l) Use Interior Mutability, so we can change our struct by reference without it being mutable.
        Hint: use RwLock.
*/

#[test]
fn exercise2() {
    mod module {
        use std::collections::BTreeSet;
        use std::fmt::Debug;
        use std::hash::Hash;
        use std::ops::Add;

        #[derive(Clone, Debug)]
        pub struct DataStore<T> {
            ids: BTreeSet<(T, usize)>,
            items: Vec<T>,
        }

        impl<T: Eq + Hash + Clone + Ord + Debug> DataStore<T> {
            pub fn new() -> Self {
                Self {
                    ids: BTreeSet::new(),
                    items: vec![],
                }
            }

            pub fn is_empty(&self) -> bool {
                self.items.is_empty()
            }

            pub fn len(&self) -> usize {
                self.items.len()
            }

            pub fn push(&mut self, value: T) {
                let index = self.items.len();
                self.items.push(value.clone());
                let is_added = self.ids.insert((value, index));
                assert!(is_added);
            }

            pub fn pop(&mut self) -> Option<T> {
                let pop = self.items.pop();
                pop.map(|value| {
                    let index = self.items.len();
                    let pair = (value, index);
                    let is_removed = self.ids.remove(&pair);
                    assert!(is_removed);
                    pair.0
                })
            }

            pub fn lookup(&self, value: T) -> Vec<usize> {
                self.ids
                    .range((value.clone(), 0)..=(value, usize::MAX))
                    .map(|(_, index)| *index)
                    .collect()
            }

            pub fn contains_value(&self, value: T) -> bool {
                self.ids
                    .range((value.clone(), 0)..=(value, usize::MAX))
                    .next()
                    .is_some()
            }

            pub fn ids(&self) -> &BTreeSet<(T, usize)> {
                &self.ids
            }

            pub fn items(&self) -> &Vec<T> {
                &self.items
            }

            pub fn into_raw(self) -> (BTreeSet<(T, usize)>, Vec<T>) {
                (self.ids, self.items)
            }

            pub fn into_ids(self) -> BTreeSet<(T, usize)> {
                self.ids
            }

            pub fn into_items(self) -> Vec<T> {
                self.items
            }
        }
    }

    use module::DataStore;
    let mut datastore = DataStore::new();
    datastore.push(1);
    datastore.push(3);
    datastore.push(2);
    datastore.push(4);
    datastore.push(2);
    datastore.push(5);
    datastore.push(4);

    // .items = [1, 3, 2, 4, 2, 5, 4]
    // .ids = [(1, 0), (2, 2), (2, 4), (3, 1), (4, 3), (4, 6), (5, 5)]

    assert_eq!(datastore.lookup(1), vec![0]);
    assert_eq!(datastore.lookup(2), vec![2, 4]);
    assert_eq!(datastore.lookup(3), vec![1]);
    assert_eq!(datastore.lookup(4), vec![3, 6]);
    assert_eq!(datastore.lookup(5), vec![5]);
}
