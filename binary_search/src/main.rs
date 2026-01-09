pub fn do_binary_search(array: Vec<i64>, value:i64) -> i64
{
    let mut low:i64 = 0;
    let mut high:i64 = (array.len() -1) as i64;
    let mut mid:i64 = 0;

    while low <= high
    {   
        mid = ((low + high) /2);

        if mid == value
        {
            return mid
        }
        else if mid < value
        {
            low = mid + 1;
        }
        else if mid > value
        {
            high = mid -1;
        }
        else
        {
            println!("Value not found");
        }
    }
    return -1
}