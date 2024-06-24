pub fn climb_stairs(n: i32) -> i32 {

    if n == 0 || n == 1 {
        return 1;
    }
    climb_stairs(n - 1) + climb_stairs(n - 2)
}




