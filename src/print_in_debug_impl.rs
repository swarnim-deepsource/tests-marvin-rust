use std::fmt::{self, Debug};

struct A;

impl Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("hello");
        Ok(())
    }
}

struct B;

impl Debug for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        eprintln!("hello");
        Ok(())
    }
}

struct C;

impl Debug for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::println!("hello");
        Ok(())
    }
}

struct D;

impl Debug for D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::eprint!("hello");
        Ok(())
    }
}
