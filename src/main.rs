// Step 1: Create the function signature for `concatenate_strings`
fn concatenate_strings(s1: &str, s2: &str) -> String {
    // Step 2: Create a new String called `result` and append the contents of `s1` and `s2`
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    
    // Step 3: Return the `result` string from the function
    result
}

fn main() {
    // Step 4: Initialize two String variables, `string1` and `string2`
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rust!");

    // Step 5: Call the `concatenate_strings` function with string slices of the variables
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Step 6: Print the result to the console
    println!("{}", concatenated_string);
}