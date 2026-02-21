pub fn iterate_vertical(
    w: usize,
    h: usize,
) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
    (0..w).map(move |x| (0..h).map(move |y| (x, y)))
}

pub fn iterate_horizontal(
    w: usize,
    h: usize,
) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
    (0..h).map(move |y| (0..w).map(move |x| (x, y)))
}

pub fn iterate_diagonal_descending(
    w: usize,
    h: usize,
) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
    let left_col_rev = (0..h).map(move |y| (0, y)).rev();
    let top_row = (1..w).map(move |x| (x, 0));

    left_col_rev.chain(top_row).map(move |(x, y)| {
        (0..)
            .map(move |i| (x + i, y + i))
            .take_while(move |&(x, y)| x < w && y < h)
    })
}

pub fn iterate_diagonal_ascending(
    w: usize,
    h: usize,
) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
    let left_col = (0..h).map(move |y| (0, y));
    let bottom_row = (1..w).map(move |x| (x, h - 1));

    left_col.chain(bottom_row).map(move |(x, y)| {
        (0..)
            .map(move |i| (x + i, (y as i64) - i as i64))
            .take_while(move |&(x, y)| x < w && y >= 0)
            .map(move |(x, y)| (x, y as usize))
    })
}

pub fn iterate_all_cells(w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..w).map(move |x| (0..h).map(move |y| (x, y))).flatten()
}

#[cfg(test)]
mod test {
    use super::{
        iterate_diagonal_ascending, iterate_diagonal_descending, iterate_horizontal,
        iterate_vertical,
    };

    #[test]
    fn test_iterate_vertical_3x3() {
        let expected = vec![
            vec![(0, 0), (0, 1), (0, 2)],
            vec![(1, 0), (1, 1), (1, 2)],
            vec![(2, 0), (2, 1), (2, 2)],
        ];
        let result: Vec<Vec<_>> = iterate_vertical(3, 3).map(|col| col.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_horizontal_3x3() {
        let expected = vec![
            vec![(0, 0), (1, 0), (2, 0)],
            vec![(0, 1), (1, 1), (2, 1)],
            vec![(0, 2), (1, 2), (2, 2)],
        ];
        let result: Vec<Vec<_>> = iterate_horizontal(3, 3).map(|row| row.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_descending_3x3_all_diagonals() {
        let expected = vec![
            vec![(0, 2)],
            vec![(0, 1), (1, 2)],
            vec![(0, 0), (1, 1), (2, 2)],
            vec![(1, 0), (2, 1)],
            vec![(2, 0)],
        ];

        let result: Vec<Vec<_>> = iterate_diagonal_descending(3, 3)
            .map(|diag| diag.collect())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_ascending_3x3_all_diagonals() {
        let expected = vec![
            vec![(0, 0)],
            vec![(0, 1), (1, 0)],
            vec![(0, 2), (1, 1), (2, 0)],
            vec![(1, 2), (2, 1)],
            vec![(2, 2)],
        ];

        let result: Vec<Vec<_>> = iterate_diagonal_ascending(3, 3)
            .map(|diag| diag.collect())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_vertical_1x1() {
        let expected = vec![vec![(0, 0)]];
        let result: Vec<Vec<_>> = iterate_vertical(1, 1).map(|col| col.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_horizontal_1x1() {
        let expected = vec![vec![(0, 0)]];
        let result: Vec<Vec<_>> = iterate_horizontal(1, 1).map(|row| row.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_descending_1x1() {
        let expected = vec![vec![(0, 0)]];
        let result: Vec<Vec<_>> = iterate_diagonal_descending(1, 1)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_ascending_1x1() {
        let expected = vec![vec![(0, 0)]];
        let result: Vec<Vec<_>> = iterate_diagonal_ascending(1, 1)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_vertical_0x0() {
        let expected: Vec<Vec<_>> = vec![];
        let result: Vec<Vec<_>> = iterate_vertical(0, 0).map(|col| col.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_horizontal_0x0() {
        let expected: Vec<Vec<_>> = vec![];
        let result: Vec<Vec<_>> = iterate_horizontal(0, 0).map(|row| row.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_descending_0x0() {
        let expected: Vec<Vec<_>> = vec![];
        let result: Vec<Vec<_>> = iterate_diagonal_descending(0, 0)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_ascending_0x0() {
        let expected: Vec<Vec<_>> = vec![];
        let result: Vec<Vec<_>> = iterate_diagonal_ascending(0, 0)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_vertical_1x5() {
        let expected = vec![vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]];
        let result: Vec<Vec<_>> = iterate_vertical(1, 5).map(|col| col.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_horizontal_5x1() {
        let expected = vec![vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]];
        let result: Vec<Vec<_>> = iterate_horizontal(5, 1).map(|row| row.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_descending_1x5() {
        let expected = vec![
            vec![(0, 4)],
            vec![(0, 3)],
            vec![(0, 2)],
            vec![(0, 1)],
            vec![(0, 0)],
        ];
        let result: Vec<Vec<_>> = iterate_diagonal_descending(1, 5)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_ascending_5x1() {
        let expected = vec![
            vec![(0, 0)],
            vec![(1, 0)],
            vec![(2, 0)],
            vec![(3, 0)],
            vec![(4, 0)],
        ];
        let result: Vec<Vec<_>> = iterate_diagonal_ascending(5, 1)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_vertical_5x4() {
        let expected = vec![
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(1, 0), (1, 1), (1, 2), (1, 3)],
            vec![(2, 0), (2, 1), (2, 2), (2, 3)],
            vec![(3, 0), (3, 1), (3, 2), (3, 3)],
            vec![(4, 0), (4, 1), (4, 2), (4, 3)],
        ];
        let result: Vec<Vec<_>> = iterate_vertical(5, 4).map(|col| col.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_horizontal_5x4() {
        let expected = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)],
            vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)],
            vec![(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)],
        ];
        let result: Vec<Vec<_>> = iterate_horizontal(5, 4).map(|row| row.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_vertical_4x5() {
        let expected = vec![
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            vec![(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)],
            vec![(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)],
            vec![(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)],
        ];
        let result: Vec<Vec<_>> = iterate_vertical(4, 5).map(|col| col.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_horizontal_4x5() {
        let expected = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 1), (1, 1), (2, 1), (3, 1)],
            vec![(0, 2), (1, 2), (2, 2), (3, 2)],
            vec![(0, 3), (1, 3), (2, 3), (3, 3)],
            vec![(0, 4), (1, 4), (2, 4), (3, 4)],
        ];
        let result: Vec<Vec<_>> = iterate_horizontal(4, 5).map(|row| row.collect()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_descending_5x4_all_diagonals() {
        let expected = vec![
            vec![(0, 3)],
            vec![(0, 2), (1, 3)],
            vec![(0, 1), (1, 2), (2, 3)],
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
            vec![(1, 0), (2, 1), (3, 2), (4, 3)],
            vec![(2, 0), (3, 1), (4, 2)],
            vec![(3, 0), (4, 1)],
            vec![(4, 0)],
        ];
        let result: Vec<Vec<_>> = iterate_diagonal_descending(5, 4)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_ascending_5x4_all_diagonals() {
        let expected = vec![
            vec![(0, 0)],
            vec![(0, 1), (1, 0)],
            vec![(0, 2), (1, 1), (2, 0)],
            vec![(0, 3), (1, 2), (2, 1), (3, 0)],
            vec![(1, 3), (2, 2), (3, 1), (4, 0)],
            vec![(2, 3), (3, 2), (4, 1)],
            vec![(3, 3), (4, 2)],
            vec![(4, 3)],
        ];
        let result: Vec<Vec<_>> = iterate_diagonal_ascending(5, 4)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_descending_4x5_all_diagonals() {
        let expected = vec![
            vec![(0, 4)],
            vec![(0, 3), (1, 4)],
            vec![(0, 2), (1, 3), (2, 4)],
            vec![(0, 1), (1, 2), (2, 3), (3, 4)],
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
            vec![(1, 0), (2, 1), (3, 2)],
            vec![(2, 0), (3, 1)],
            vec![(3, 0)],
        ];
        let result: Vec<Vec<_>> = iterate_diagonal_descending(4, 5)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_diagonal_ascending_4x5_all_diagonals() {
        let expected = vec![
            vec![(0, 0)],
            vec![(0, 1), (1, 0)],
            vec![(0, 2), (1, 1), (2, 0)],
            vec![(0, 3), (1, 2), (2, 1), (3, 0)],
            vec![(0, 4), (1, 3), (2, 2), (3, 1)],
            vec![(1, 4), (2, 3), (3, 2)],
            vec![(2, 4), (3, 3)],
            vec![(3, 4)],
        ];
        let result: Vec<Vec<_>> = iterate_diagonal_ascending(4, 5)
            .map(|diag| diag.collect())
            .collect();
        assert_eq!(result, expected);
    }
}
