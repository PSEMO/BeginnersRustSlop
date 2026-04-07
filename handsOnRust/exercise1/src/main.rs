struct Student {
    name: String,
    score: u32,
}

impl Student {
    fn is_passing(&self) -> bool {
        self.score >= 70
    }

    fn give_extra_credit(&mut self, ammount: u32) {
        self.score += ammount;
        if self.score > 100 {
            self.score = 100;
        }
    }
}

fn main() {
    let mut student1 = Student{name: String::from("micheal"), score: 60};
    
    println!("student1: {0}, {1}; is passing: {2}", student1.name, student1.score, student1.is_passing());
    
    student1.give_extra_credit(50);
    println!("student1 if given 50 points: {0}, {1}; is passing: {2}", student1.name, student1.score, student1.is_passing());


    let mut student2 = Student{name: String::from("Elana"), score: 30};
    println!("student2: {0}, {1}; is passing: {2}", student2.name, student2.score, student2.is_passing());
    
    student2.give_extra_credit(50);
    println!("student2 if given 50 points: {0}, {1}; is passing: {2}", student2.name, student2.score, student2.is_passing());

    
    let mut student3 = Student{name: String::from("Jeff"), score: 10};
    println!("student3: {0}, {1}; is passing: {2}", student3.name, student3.score, student3.is_passing());
    
    student3.give_extra_credit(50);
    println!("student3 if given 50 points: {0}, {1}; is passing: {2}", student3.name, student3.score, student3.is_passing());
}