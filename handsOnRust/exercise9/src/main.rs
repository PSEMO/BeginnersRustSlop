//turned into a mod to make private things actually private
//cuz otherwise its really dumb
mod grade_module {
    pub struct Grade {
        value: i32,
    }

    impl Grade {
        pub fn new(input: i32) -> Result<Grade, String> {
            if input < 0 || input > 100 {
                return Result::Err(String::from("Value has to be between 0 and 100"));
            }
            
            return Result::Ok(Grade { value: input });
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    pub fn parse_record(input: &str) -> Result<Grade, String> {
        match input.trim().parse::<i32>() {
            Ok(value) => /*match*/return Result::Ok(Grade::new(value)?)/* {
                Ok(grade) => return Result::Ok(grade),
                Err(msg) => return Result::Err(msg),
            }*/,
            Err(_) => return Result::Err(String::from("invalid text")),
        };
    }
}

fn main() {
    let error_msg = "if you see this, something must have gone really wrong";

    println!("{:?}", grade_module::Grade::new(87).expect(error_msg).value());
    println!("{:?}", grade_module::Grade::new(13).expect(error_msg).value());

    match grade_module::parse_record("101") {
        Ok(grade) => println!("{:?}", grade.value()),
        Err(msg) => println!("{:?}", msg),
    }
    match grade_module::parse_record("asd") {
        Ok(grade) => println!("{:?}", grade.value()),
        Err(msg) => println!("{:?}", msg),
    }
    match grade_module::parse_record("23") {
        Ok(grade) => println!("{:?}", grade.value()),
        Err(msg) => println!("{:?}", msg),
    }
}