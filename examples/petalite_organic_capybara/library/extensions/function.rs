pub fn is_ascending(input: f32, get_value: impl Fn(f32) -> f32) -> bool {
    let current_value = get_value(input);
    let next_input = input + 0.0001;
    let next_value = get_value(next_input);
    next_value > current_value
}
