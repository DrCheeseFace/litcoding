use std::collections::HashSet;

pub fn is_happy(mut n: i32) -> bool {
    let mut set = HashSet::new();
    while n != 1 {
        let next = get_next(n);
        if set.contains(&next) {
            return false;
        }
        set.insert(next);
        n = next;
    }
    true
}

fn get_next(mut n: i32) -> i32 {
    let mut res = 0;
    while n != 0 {
        res += (n % 10).pow(2);
        n /= 10;
    }
    res
}
