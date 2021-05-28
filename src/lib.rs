use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::convert::TryInto;

#[cfg(test)]
mod tests {
    use crate::Generator;
    #[test]
    fn test() {
        let mut g = Generator::new();
        g.train("some stupid words to test some stupid code");
        println!("{}", g.generate(20))
    }
}
type Node = Option<Box<str>>;
type Key = [Node; 2];

#[derive(Default)]
pub struct Generator {
    text: String,
    chain: HashMap<Key, Vec<Node>>,
}
impl Generator {
    pub fn new() -> Self {
        Generator {
            text: String::new(),
            chain: HashMap::new(),
        }
    }
    pub fn train(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        if text.contains('\n') {
            for s in text.split('\n') {
                self.train(s)
            }
            return;
        }
        self.text.push_str(text);
        self.text.push('\n');
        let mut text = text
            .split_whitespace()
            .map(String::from)
            .map(String::into_boxed_str)
            .map(Option::from)
            .collect::<Vec<_>>();
        text.insert(0, None);
        text.insert(0, None);
        text.push(None);
        for window in text.windows(3) {
            let (key, value) = window.split_at(2);
            match self.chain.get_mut(key) {
                Some(vector) => vector.extend_from_slice(value),
                None => {
                    let key = key.to_owned().try_into().unwrap();
                    self.chain.insert(key, value.to_vec());
                }
            };
        }
    }
    pub fn generate(&self, tries: usize) -> Option<String> {
        if tries == 0 {
            return None;
        }
        let mut rng = rand::thread_rng();
        let mut string: Vec<Node> = vec![None, None];
        loop {
            let index = &string[string.len() - 2..];
            let variants = &self.chain.get(index)?;
            let choice = variants.choose(&mut rng)?.clone();
            if choice.is_none() {
                break;
            }
            string.push(choice);
        }
        let result = string
            .into_iter()
            .skip(2)
            .map(Option::unwrap)
            .collect::<Vec<_>>()
            .join(" ");
        if !self.text.contains(&result) {
            Some(result)
        } else {
            self.generate(tries - 1)
        }
    }
}
