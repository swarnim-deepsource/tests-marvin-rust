use std::fmt::{self, Display};

struct A;

impl Display for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("hello");
        Ok(())
    }
}

struct B;

impl Display for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        eprintln!("hello");
        Ok(())
    }
}

struct C;

impl Display for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::println!("hello");
        Ok(())
    }
}
struct D;

impl Display for D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::eprint!("hello");
        Ok(())
    }
}
