fn number_to_word(n: u32) -> String {
    match n {
        0 => "No".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => n.to_string(), // Fallback for numbers not in the range
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| {
            let bottles = start_bottles.saturating_sub(i);
            let next_bottles = bottles.saturating_sub(1);

            let bottles_word = number_to_word(bottles);
            let next_bottles_word = number_to_word(next_bottles).to_lowercase();

            match bottles {
                0 => "No green bottles hanging on the wall,\nno green bottles hanging on the wall,\n\
                       And if one green bottle should accidentally fall,\n\
                       There'll be no green bottles hanging on the wall.\n".to_string(),
                1 => "One green bottle hanging on the wall,\nOne green bottle hanging on the wall,\n\
                       And if one green bottle should accidentally fall,\n\
                       There'll be no green bottles hanging on the wall.\n".to_string(),
                _ => format!(
                    "{0} green bottles hanging on the wall,\n{0} green bottles hanging on the wall,\n\
                     And if one green bottle should accidentally fall,\n\
                     There'll be {1} green {2} hanging on the wall.\n",
                    bottles_word,
                    next_bottles_word,
                    if next_bottles == 1 { "bottle" } else { "bottles" }
                ),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}