fn get_min_from_vector<T>(nums: &[T]) -> T 
where
    T: Ord + Copy
{
    let mut min_num = nums[0];
    for i in nums {
        if *i < min_num {
            min_num = *i;
        }
    }

    min_num
}

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    for i in (0..=10).rev() {
        nums.push(i);
    }
    let min_num = get_min_from_vector(&nums);
    println!("Minimum elemenet in {:?} is {min_num}", nums);

    // TODO: revist this. It shouldn't accept strings.
    // let temps = vec!["hello","jp"];
    // let t = get_min_from_vector(&temps);
    // println!("{}",t);
}
