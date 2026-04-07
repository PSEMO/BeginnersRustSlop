struct Book {
    title: String,
    is_available: bool,
}

impl Book {
    fn borrow_book(&mut self) {
        self.is_available = false;
    }
    fn matches_title(&self, search: &str) -> bool {
        self.title == search
    }
    fn return_book(&mut self) {
        self.is_available = true;
    }
}

fn main() {
    let mut book = Book { title: String::from("Rust"), is_available: true};

    println!("True {}", book.matches_title(&String::from("Rust")));
    println!("False {}", book.matches_title(&String::from("asd")));
    println!("True {}", book.is_available);
    book.return_book();
    println!("True {}", book.is_available);
    book.borrow_book();
    println!("False {}", book.is_available);
    book.return_book();
    println!("True {}", book.is_available);
    book.borrow_book();
    println!("False {}", book.is_available);
}