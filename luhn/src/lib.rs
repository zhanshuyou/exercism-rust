pub fn is_valid(code: &str) -> bool {
    if code.trim() == "0" {
        return false;
    }

    let nums = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>();

    nums.map_or(false, |nums| {
        nums.iter()
            .rev()
            .enumerate()
            .map(|(i, &n)| {
                if i % 2 == 0 {
                    n
                } else {
                    let mut n = n * 2;
                    if n > 9 {
                        n -= 9;
                    }
                    n
                }
            })
            .sum::<u32>()
            % 10
            == 0
    })
}
