use std::time::Instant;

static mut OVERALL_FOR_TIME: u128 = 0;
static mut OVERALL_WHILE_TIME: u128 = 0;
static mut VALUE: u128 = 0;
fn for_loop_instrumentation(arr: [i8; 1_000_000]) {
    let start_time = Instant::now();
    let mut _for_counter = 0;
    let mut dummy_val: i32 = 0;
    for elem in arr {
        _for_counter += 1;
        dummy_val += elem as i32;
    }
    let end_time = start_time.elapsed().as_micros();

    unsafe {
        OVERALL_FOR_TIME += end_time;
        VALUE += dummy_val as u128;
    }
}

fn while_loop_instrumentation(arr: [i8; 1_000_000]) {
    let size = arr.len();
    let mut index = 0;

    let mut _while_counter = 0;
    let mut dummy_val: i32 = 0;

    let start_time = Instant::now();
    while index < size {
        dummy_val += arr[index] as i32;
        index += 1;
        _while_counter += 1;
    }
    let end_time = start_time.elapsed().as_micros();

    unsafe {
        OVERALL_WHILE_TIME += end_time;
        VALUE += dummy_val as u128;
    }
}

fn main() {
    let arr: [i8; 1_000_000] = [120; 1_000_000];

    /*
     * According to docs, while should be slower but it is faster in this test case.
     * // TODO: Need to revisit this
     */
    for _i in 1..=10 {
        while_loop_instrumentation(arr);
        for_loop_instrumentation(arr);
    }

    unsafe {
        println!("Time taken by for loop = {OVERALL_FOR_TIME} µs");
        println!("Time taken by while loop = {OVERALL_WHILE_TIME} µs");

        println!("{VALUE}");
    }
}
