pub fn find_consecutive_sequence<I, T>(
    mut line: I,
    length: u32,
    comparator: fn(&T, &T) -> bool,
) -> Option<(T, T)>
where
    I: Iterator<Item = T>,
{
    let mut current = line.next()?;
    let mut count = 1;
    if length < 2 {
        return None;
    }

    for field in line {
        if comparator(&field, &current) {
            count += 1;
            if count >= length {
                return Some((current, field));
            }
        } else {
            current = field;
            count = 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::find_consecutive_sequence;

    #[test]
    fn test_find_consecutive_sequence_exact_match() {
        let v = vec![1, 1, 1, 2, 3];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 3, PartialEq::eq),
            Some((1, 1))
        );
    }

    #[test]
    fn test_find_consecutive_sequence_no_match() {
        let v = vec![1, 2, 2, 1, 2];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 4, PartialEq::eq),
            None
        );
    }

    #[test]
    fn test_find_consecutive_sequence_sequence_at_end() {
        let v = vec!['a', 'b', 'b', 'b'];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 3, PartialEq::eq),
            Some(('b', 'b'))
        );
    }

    #[test]
    fn test_find_consecutive_sequence_interrupted_sequence() {
        let v = vec![0, 0, 1, 0, 0, 0];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 3, PartialEq::eq),
            Some((0, 0))
        );
    }

    #[test]
    fn test_find_consecutive_sequence_single_element() {
        let v = vec![42];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 1, PartialEq::eq),
            None
        );
        assert_eq!(
            find_consecutive_sequence(vec![42].into_iter(), 2, PartialEq::eq),
            None
        );
    }

    #[test]
    fn test_find_consecutive_sequence_empty() {
        let v: Vec<u8> = vec![];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 1, PartialEq::eq),
            None
        );
    }

    #[test]
    fn test_find_consecutive_sequence_multiple_sequences() {
        let v = vec![2, 2, 2, 1, 1, 1, 1];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 4, PartialEq::eq),
            Some((1, 1))
        );
    }

    #[test]
    fn test_find_consecutive_sequence_non_consecutive() {
        let v = vec![3, 3, 4, 3, 3, 3];
        assert_eq!(
            find_consecutive_sequence(v.into_iter(), 3, PartialEq::eq),
            Some((3, 3))
        );
    }
}
