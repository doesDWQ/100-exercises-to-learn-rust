// TODO: whenever `title` and `description` are returned via their accessor methods, they
//   should be normalized—i.e. leading and trailing whitespace should be removed.
//   There is a method in Rust's standard library that can help with this, but you won't
//   find it in the documentation for `String`.
//   Can you figure out where it is defined and how to use it?

use std::ops::Deref;

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
       &self.title.trim()
    }

    pub fn description(&self) -> &str {
        &self.description.trim()
    }
}

pub struct A {
    name: String,
}

pub struct B {
    name: String
}

impl B {
    pub fn name(&self) -> &str {
        &self.name
    } 
}

impl Deref for A {
    type Target = B;
    
    fn deref(&self) -> &Self::Target {
        &self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deref(){
        let a = A{name:String::from("dwq")};
        let b = &a;
        assert_eq!(b.name(), "dwq");
    }

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
