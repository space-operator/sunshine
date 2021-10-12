use std::ops::{Range, RangeBounds};

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
fn exercise2b() {
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
        fn oscillate_range<T>(&mut self, range: Range<T>) -> &mut Self
        where
            i32: PartialEq<T>;
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
        fn oscillate_range<T>(&mut self, range: Range<T>) -> &mut Self
        where
            i32: PartialEq<T>,
        {
            if self.current_counter == range.start {
                self.current_counter = range.start;
            } else {
                self.current_counter = range.end;
            }
            self
        }
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
        Hint: iter::once(), iter::repeat().
        Hint: Iterator::rev(), Iterator::step_by(), Iterator::chain(), Iterator::flatten().

    2.
    (a) Create a struct that stores u32 with a Vec but not expose it publically.
    (b) Add functions new/is_empty/len/push/pop.
    (c) Make struct generic to allow any values as items.
    (d) Add structure for quick lookup by value.
        Hint: we can use HashSet<(value, index)>.
    (e) Add function contains_value.
    (f) Add functions data, hash_set that return references to inner data.
    (g) Use Interior Mutability, so we can change our struct by reference without it being mutable.
        Hint: use RwLock.
    (h) Manually implement Debug trait.
    (i) Implement AsRef<[T]>, AsMut<[T]>.
    (j) Implement Index.
    (k) Implement IntoIterator.
*/
