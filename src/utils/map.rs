use maplit::hashmap;

use std::collections::HashMap;

pub fn get_flag_map() -> HashMap<&'static str, char> {
    hashmap! {
        "force" => 'f',
        "interactive" => 'i',
        "recursive" => 'r',
        "dir" => 'd',
        "verbose" => 'v',
        "help" => 'h',
        "version" => 'V'
    }
}
