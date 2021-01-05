pub struct Person<T, U> {
    name: T,
    age: U,
}

//for all generic types
impl<T, U> Person<T, U> {
    pub fn new(name: T, age: U) -> Person<T, U> {
        return Person { name, age };
    }

    pub fn getGenTypes(&self) -> (&T, &U) {
        println!("Generic T and U");
        return (&self.name, &self.age);
    }
}

//only for string,i32 types
impl Person<String, i32> {
    pub fn getNoTypes(&self) -> (&String, &i32) {
        println!("String and i32");
        return (&self.name, &self.age);
    }
}

//only for string,i32 types
impl<T> Person<T, i32> {
    pub fn getOneTypes(&self) -> (&T, &i32) {
        println!("T and i32");
        return (&self.name, &self.age);
    }
}

pub fn genMain() {
    println!(
        "{:?} ",
        crate::generics::Person::new("Sosa", 3).getGenTypes()
    );
    // println!("{:?} ", Person::new("Sosa", 3).getNoTypes());
    println!("{:?} ", Person::new("Sosa", 3).getOneTypes());
    println!("{:?} ", Person::new(12, 3).getOneTypes());
    println!("{:?} ", Person::new(12, 3).getGenTypes());
    // println!("{:?} ",Person::new(12, "hello").getOneTypes());
    println!("{:?} ", Person::new(12, "hello").getGenTypes());
}
