fn foo() {
    let a: i32 = 8;
    let b = a.clone();
    let c = (&a).clone();
    let d = String::from("Hi there");
    let dr = &d;
    let e = d.clone();
    let f = dr.clone(); // don't raise if it's a reference to non Copy type
}
