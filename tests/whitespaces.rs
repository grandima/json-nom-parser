use json_nom_parser::{LEN_KV_SIZE, parse_sized_json};

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_whitespaces_correctness_all_in_one() {
        let mut key_value_size = LEN_KV_SIZE.clone();
        let input = std::fs::read_to_string("network_data.json").unwrap();
        let (left, res) = parse_sized_json(&mut key_value_size)(&input).unwrap();
        let mut correct_len_kv_size = LEN_KV_SIZE.clone();
        assert_eq!(left.len(), 0);
        for (key, value) in res {
            assert!(value.chars().all(char::is_alphanumeric));
            let entry = correct_len_kv_size.get_mut(&key.len()).unwrap();
            let (_correct_key, correct_value_size) = entry.remove_entry(key).unwrap();
            if entry.is_empty() {
                correct_len_kv_size.remove(&key.len());
            }
            assert_eq!(value.len(), correct_value_size);
        }
        assert_eq!(correct_len_kv_size.len(), 0);
    }
}
