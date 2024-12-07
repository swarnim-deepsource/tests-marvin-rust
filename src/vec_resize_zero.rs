#[rustfmt::skip]
fn f() {
    let mut v : Vec<i32> = Vec::new();
        v.resize(0, 1);

    let mut i = RandomType { x: 0u32 };
    i.resize(0, 1);
}

struct RandomType {
    x: u32,
}

impl RandomType {
    fn resize(&mut self, _x: i32, _y: i32) {}
}
