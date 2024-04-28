pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    let mut minefield: Vec<Vec<char>> = minefield.iter().map(|row| row.chars().collect()).collect();
    let rows = minefield.len();
    let cols = minefield[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if minefield[i][j] == '*' {
                continue;
            }
            let mut count = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = i as i32 + dx;
                    let ny = j as i32 + dy;
                    if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 && minefield[nx as usize][ny as usize] == '*' {
                        count += 1;
                    }
                }
            }
            if count > 0 {
                minefield[i][j] = std::char::from_digit(count, 10).unwrap();
            }
        }
    }
    minefield.iter().map(|r| r.iter().collect()).collect()
}
