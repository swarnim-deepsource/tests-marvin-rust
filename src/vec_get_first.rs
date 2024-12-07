use std::collections::{HashMap, VecDeque};

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let one = v.get(0);
    let one_mut = v.get_mut(0);
    let mut dq = VecDeque::from([1, 2, 3, 4]);
    let one = dq.get(0);
    let one_mut = dq.get_mut(0);
    let h: HashMap<usize, usize> = HashMap::new();
    let one = h.get(&0);
}
