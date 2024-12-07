#[rustfmt::skip]
fn f() {
       for x in Option::from(10) {}

    let res: Result<i32, &str> = Ok(-23);
       for x in res {}


    for x in [1] {
        /* do nothing */
    }
}
