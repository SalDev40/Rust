/* Custom trait (like interface) */
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarizeAuthor(&self) -> String;
    fn defSummarize(&self) -> String {
        return format!("Red more from {}", self.summarizeAuthor());
    }
}

pub struct Book {
    author: String,
    length: i32,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        return format!(
            "This is a book by, {}, of length: {}",
            self.author, self.length,
        );
    }
    fn summarizeAuthor(&self) -> String {
        return format!(" books, {}", self.author);
    }
}

pub struct Article {
    pub author: String,
    length: i32,
}

/* impl Trait for Type */
impl Summary for Article {
    fn summarize(&self) -> String {
        return format!(
            "This is a article by, {}, of length: {}",
            self.author, self.length,
        );
    }

    fn summarizeAuthor(&self) -> String {
        return format!(" article, {}", self.author);
    }
}

/* Adding a STL trait to custom type */
impl std::fmt::Display for Article {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        format!("display trait {} ", self);
        Ok(())
    }
}

impl Article {
    fn getAuthor(&mut self) -> &String {
        println!("author: {}", self.author);
        self.author = "monkey".to_string();
        return &self.author;
    }

    fn getLength(&self) -> i32 {
        println!("length: {}", self.length);
        return self.length;
    }
}
pub fn alert(x: &impl Summary) {
    println!("alert: {}", x.summarize());
}

/* Trait Bounds */
pub fn notify<T>(x: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("trait bounds: {}", x.summarize());
}

struct Pair<U> {
    x: U,
    y: U,
}

impl<U> Pair<U> {
    fn new(x: U, y: U) -> Pair<U> {
        return Pair { x, y };
    }
    fn log(&self) {
        println!("log of x");
    }
}

/* Using Trait Bounds to Conditionally Implement Methods */
impl<U: std::fmt::Display + PartialOrd> Pair<U> {
    fn cmp(&self) {
        if self.x > self.y {
            println!("x is bigger, {}", self.x);
        } else {
            println!("y is bigger , {}", self.y);
        }
    }
}

/* conditionally implement a
trait for any type that implements another trait
implement summary for any trait that implementes display + partialOrd */
impl<T: std::fmt::Display + PartialOrd> Summary for T {
    fn summarize(&self) -> String {
        return format!("This is blanket implemetations");
    }
    fn summarizeAuthor(&self) -> String {
        return format!("This is blanket implemetations");
    }
}

pub fn trMain() {
    let book: Book = Book {
        author: "john".to_string(),
        length: 123,
    };
    let mut article: Article = Article {
        author: "apple".to_string(),
        length: 12,
    };

    let len = article.getLength();
    article.getAuthor();
    println!("{}, ", article.author);
    println!("{}, ", len);

    println!("{}, ", book.defSummarize());
    println!("{}, ", article.defSummarize());

    alert(&book);
    notify(&article);

    let one: Pair<Pair<i32>> = Pair::new(Pair::new(1, 2), Pair::new(2, 3));
    let two: Pair<&str> = Pair::new("hello", "ss");

    one.log();
    // one.cmp(); //wont work doesnt fulfill the trait bounds
    two.cmp();

    let s = 3.summarize();
    println!(
        "conditionally implement a traitfor any type \n 
        that implements another trait  {}",
        s
    );
}
