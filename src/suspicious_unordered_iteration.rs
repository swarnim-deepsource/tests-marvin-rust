mod test {
    use std::collections::{HashMap, HashSet};

    fn hashmap() {
        let mut s: HashMap<u32, u32> = [(1, 1), (2, 2), (3, 3)].into();
        let _ = s.keys().take_while(|&&k| k >= 1);
        let _ = s.clone().into_keys().take_while(|&k| k >= 1);
        let _ = s.values().skip_while(|&&v| v >= 1);
        let _ = s.clone().into_values().skip_while(|&v| v >= 1);
        let _ = s.iter().map_while(|(k, v)| k.checked_sub(*v));
        let _ = s.iter_mut().map_while(|(k, v)| k.checked_sub(*v));
        let _ = s.drain().take_while(|(k, _)| *k == 1);
    }

    fn hashset() {
        let s1: HashSet<usize> = [1, 1, 2, 3, 6].into();
        let mut s2: HashSet<usize> = [3, 2, 5, 4, 9].into();

        let _ = s1.difference(&s2).take_while(|&&k| k != 0);
        let _ = s1.symmetric_difference(&s2).skip_while(|&&k| k != 0);
        let _ = s1.intersection(&s2).skip_while(|&&k| k != 0);
        let _ = s1.union(&s2).take_while(|&&k| k != 0);
        let _ = s2.drain().map_while(|k| k.checked_sub(1));
    }

    fn no_match() {
        let mut s: HashMap<usize, usize> = [(1, 1), (2, 2), (3, 3)].into();
        let _ = s.keys().map(|k| k.rotate_left(2));
        let _ = s.values_mut().map(|k| {
            *k *= 2;
        });

        let v = vec![1, 2, 3];
        let _ = v.into_iter().map_while(|i: u32| i.checked_sub(100));
    }
}
