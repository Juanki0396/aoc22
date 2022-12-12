pub mod inputs {
    use std::path::Path;

    pub fn read_file(path: &Path) -> String {
        std::fs::read_to_string(path).expect("Couldn't open the file")
    }
}

pub mod elves {
    use std::str::Lines;

    pub struct CaloriesReader<'a> {
        input: Lines<'a>,
    }

    impl<'a> CaloriesReader<'a> {
        pub fn build(text: &'a String) -> Self {
            Self {
                input: text.lines(),
            }
        }
    }

    impl<'a> Iterator for CaloriesReader<'a> {
        type Item = Vec<i32>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut next_item: Self::Item = Vec::new();
            loop {
                match self.input.next() {
                    Some(line) => {
                        if line == "" {
                            return Some(next_item);
                        } else {
                            next_item.push(line.parse::<i32>().expect("Not a integer"))
                        }
                    }
                    None => {
                        if next_item.len() == 0 {
                            return None;
                        } else {
                            return Some(next_item);
                        }
                    }
                }
            }
        }
    }
}
