#![rustfmt::skip]
enum A {
    B(i32),
    C,
}

fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn foo() {
    //> [RS-W1124]: "Cast from function pointer to `usize`"
    let _ = A::B as usize;
    //> [RS-W1124]: "Cast from function pointer to `i32`"
    let _ = add as i32;
    let _ = add as fn(isize, isize) -> isize;
    let _ = add as *const ();

    let add_closure = |a, b| a + b;
    //> [RS-W1124]: "Cast from closure to `i32`"
    let _ = add_closure as i32;
    let _ = add_closure as fn(isize, isize) -> isize;
    let _ = add_closure as *const ();
}
