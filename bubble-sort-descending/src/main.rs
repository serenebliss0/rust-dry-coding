fn main() 
{
    let mut arr = vec![5, 2, 8, 1, 4];

    let n = arr.len();

    // Step 1: outer loop – number of passes
    for i in 0..n 
    {
        // Step 2: inner loop – go through unsorted portion
        for j in 0..(n - i - 1) 
        {
            // just change the sign `>` to `<`
            if arr[j] < arr[j + 1]
            {
                // Step 4: swap if out of order
                arr.swap(j, j + 1);
            }
        }
        // After each pass, largest remaining element is at the end
        println!("After pass {}: {:?}", i + 1, arr);
    }

    println!("Sorted array: {:?}", arr);
}
