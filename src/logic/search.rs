// NOTE: this is expected to be an non-CASE-SENSITIVE by iterating each string and turn every
//       string to lower case, u can see at .filter bla bla bla *.to_lowercase(), lemme know if you want something diffrent
//       (for example regex), lol.
pub fn filter_items<'a>(all_items: &'a [String], find_string: &str) -> Vec<&'a String> {
    all_items
        .iter()
        .filter(|item| item.to_lowercase().contains(&find_string.to_lowercase()))
        .collect()
}
