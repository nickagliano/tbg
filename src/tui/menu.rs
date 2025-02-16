// TODO: Use this
struct Menu<'a, T: std::fmt::Display> {
    message: &'a str,
    options: Vec<T>,
    selected_index: usize,
    use_simulate_typing: bool,
}
