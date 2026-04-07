struct Document {
    content: String,
}

impl Document {
    fn first_word(&self) -> &str {
        let bytes = self.content.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &self.content[0..i];
            }
        }

        &self.content[..]
    }
    fn word_count(&self) -> usize {
        let bytes = self.content.as_bytes();

        let mut word_count = 1;
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                word_count += 1;
            }
        }

        word_count
    }
}

fn main() {
    let mut document = Document { content: String::from("Lorem ipsum.") };

    let word = document.first_word();
    println!("{}", word);

    let count = document.word_count();
    println!("{}", count);
}