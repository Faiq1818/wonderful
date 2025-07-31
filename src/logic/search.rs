pub fn filter_items<'a>(all_items: &'a [String], find_string: &str) -> Vec<&'a String> {
    all_items
        .iter()
        .filter(|item| item.to_lowercase().contains(&find_string.to_lowercase()))
        .collect()
}
