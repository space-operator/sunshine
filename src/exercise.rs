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
