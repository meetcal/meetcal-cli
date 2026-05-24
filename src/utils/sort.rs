// turn weight class into a tuple marking + as true to put last when sorted
pub fn weight_class_key(class: &str) -> (i32, bool) {
    let weight = class.trim_end_matches("kg");
    let is_plus = weight.ends_with("+");
    let num_str = weight.trim_end_matches("+");
    let num = num_str.parse::<i32>().unwrap();

    (num, is_plus)
}
