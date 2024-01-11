fn get_nth_word_in_string(input: &String, n: i8) -> &str {
    let mut counter = 0;
    let mut prev_index = 0;
    for (index, &char_elem) in input.as_bytes().iter().enumerate() {
        if char_elem == b' ' {
            counter += 1;
            if counter == n {
                return &input[prev_index..index];
            }
            prev_index = index + 1;
        }
    }

    // When number of spaces is equal to no of words - 1
    // and require last word
    if counter == n - 1 {
        return &input[prev_index..];
    }

    &input[..]
}

fn main() {
    let sentence = String::from("This is a sentence");
    let n = 5;
    println!("In sentence \"{sentence}\"");
    for i in 0..=n {
        let word = get_nth_word_in_string(&sentence, i);
        println!("Word - {i} = \"{word}\"");
    }
}
