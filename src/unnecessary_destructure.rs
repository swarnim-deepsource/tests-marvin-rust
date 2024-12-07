#[derive(Clone, Copy)]
struct S {
    s: &'static str,
}

fn foo() {
    let a = S { s: "Hello, world!" };
    let b = S { ..a };
}
