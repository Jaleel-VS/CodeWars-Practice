fn are_you_playing_banjo(name: &str) -> String {
    let first_letter = name.chars().next().unwrap();

    if first_letter == 'R' || first_letter == 'r' {
        format!("{} plays banjo", name)
    } else {
        format!("{} does not play banjo", name)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Ricky"), "Ricky plays banjo");
        assert_eq!(are_you_playing_banjo("ricky"), "ricky plays banjo");
    }

    #[test]
    fn test_not_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Paul"), "Paul does not play banjo");
        assert_eq!(are_you_playing_banjo("paul"), "paul does not play banjo");
    }
}