use std::slice;

fn split_at(slice: &mut [i32], split_idx: usize) -> (&mut [i32], &mut [i32]) {
    let length = slice.len();

    // Returns a raw pointer - Rust does not guarantee that this pointer will remaing valid
    let ptr = slice.as_mut_ptr();

    assert!(split_idx <= length);

    // We are certain that these operations are safe
    // as we have checked that the split index is within the slice length.
    // The compiler can't figure that out though.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, split_idx),
            slice::from_raw_parts_mut(ptr.add(split_idx), length - split_idx),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
