#[rustfmt::skip]
use std::rc::Rc;

fn foo() {
    let mut v = Vec::new();
    v.push(Rc::new(0));
    let r = Rc::new(1);
    v.push(r.clone());
    v.insert(0, r.clone());
}
