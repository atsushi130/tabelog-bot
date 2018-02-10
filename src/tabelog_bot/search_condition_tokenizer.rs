
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

pub struct SearchConditionTokenizer;

impl SearchConditionTokenizer {

    pub fn analyze(&self, value: &str) -> (String, String) {

        let mut splitted: Vec<&str> = value.split(' ').collect();
        splitted.remove(0);

        if let Some(area) = splitted.first() {
            if let Some(word) = splitted.last() {
                return (area.to_string(), word.to_string())
            }
        }

        ("".to_string(), "".to_string())
    }

}
