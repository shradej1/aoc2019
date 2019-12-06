fn is_valid_password(pw: u32, allow_larger_groups: bool) -> bool {
    let pw: Vec<char> = pw.to_string().chars().collect();
    if pw.len() != 6 {
        return false;
    }

    let mut repeated_lengths = Vec::new();
    let mut repeated_length = 1;
    let mut last = pw[0];
    for c in &pw[1..] {
        if last == *c {
            repeated_length += 1;
        } else {
            repeated_lengths.push(repeated_length);
            repeated_length = 1;
        }

        last = *c;
    }
    repeated_lengths.push(repeated_length);

    let adjacency_condition = if allow_larger_groups {
        repeated_lengths.iter().find(|&&l| l >= 2).is_some()
    } else {
        repeated_lengths.contains(&2)
    };

    let mut non_decreasing = true;
    let mut last = pw[0];
    for next in &pw[1..] {
        non_decreasing &= last <= *next;
        last = *next;
    }

    adjacency_condition && non_decreasing
}

pub fn password_search_size() -> (u32, u32) {
    let mut count_allowing_larger_groups = 0;
    let mut count_requiring_exact_group_of_2 = 0;
    for pot in 273025..767253 {
        if is_valid_password(pot, true) {
            count_allowing_larger_groups += 1;
        }

        if is_valid_password(pot, false) {
            count_requiring_exact_group_of_2 += 1;
        }
    }
    (
        count_allowing_larger_groups,
        count_requiring_exact_group_of_2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!is_valid_password(111111, false));
    }

    #[test]
    fn test_examples() {
        assert!(is_valid_password(111111, true));
        assert!(!is_valid_password(223450, true));
        assert!(!is_valid_password(123789, true));
        assert!(is_valid_password(123444, true));

        assert!(!is_valid_password(111111, false));
        assert!(is_valid_password(112233, false));
        assert!(!is_valid_password(123444, false));
        assert!(is_valid_password(111122, false));
    }

    #[test]
    fn answer() {
        let (p1, p2) = password_search_size();
        assert_eq!(910, p1);
        assert_eq!(598, p2);
    }
}
