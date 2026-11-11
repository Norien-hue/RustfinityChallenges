pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    let size = slice.len();
    for idx in indices {
        
        // *idx >= 0 not necesary since usize will always be positive.
        if *idx < size {
            slice[*idx] = value;
        }
    }
}
