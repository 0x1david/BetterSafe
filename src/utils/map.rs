use std::collections::HashMap;

pub fn get_flag_map() -> HashMap<String, char> {
    let mut flag_map: HashMap<String, char> = HashMap::new();
    flag_map.insert("force".to_string(), 'f');
    flag_map.insert("interactive".to_string(), 'i');
    flag_map.insert("recursive".to_string(), 'r');
    flag_map.insert("dir".to_string(), 'd');
    flag_map.insert("verbose".to_string(), 'v');
    flag_map.insert("help".to_string(), 'h');
    flag_map.insert("version".to_string(), 'V');
    flag_map
}
