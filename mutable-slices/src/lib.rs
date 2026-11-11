pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
    for num in slice {
        if *num % 2 != 0 {
            *num = *num - 1;
        }else {
            *num = *num * 2;
        }
    }
}
