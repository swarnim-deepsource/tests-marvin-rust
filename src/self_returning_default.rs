struct A(i32);

impl A {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for A {
    fn default() -> Self {
        Self::new()
    }
}

struct B(i32);

impl B {
    fn new() -> Self {
        Self::default()
    }

    fn with_zero(mut self) -> Self {
        self.0 = 0;
        self
    }
}

impl Default for B {
    fn default() -> Self {
        Self::new().with_zero()
    }
}

struct C(i32);

impl C {
    const fn new() -> Self {
        Self(0)
    }
}

impl Default for C {
    fn default() -> Self {
        // Should not raise as new is a const fn
        Self::new()
    }
}

struct D(i32);

impl D {
    fn new() -> Self {
        Self::default()
    }

    const fn with_zero(mut self) -> Self {
        self.0 = 0;
        self
    }
}

impl Default for D {
    fn default() -> Self {
        Self::new().with_zero()
    }
}
