
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
