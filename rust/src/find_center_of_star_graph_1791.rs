pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
        return edges[0][0];
    }
    return edges[0][1];
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn eg1() {
        let edges: Vec<Vec<i32>> = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
        let out = find_center(edges);
        assert_eq!(out, 2)
    }

    #[test]
    fn eg2() {
        let edges: Vec<Vec<i32>> = vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]];
        let out = find_center(edges);
        assert_eq!(out, 1)
    }
}
