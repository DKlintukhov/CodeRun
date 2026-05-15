/*
В каждой клетке прямоугольной таблицы N × M N×M записано некоторое число. Изначально игрок находится в левой верхней клетке. За один ход ему разрешается перемещаться в соседнюю клетку либо вправо, либо вниз (влево и вверх перемещаться запрещено). При проходе через клетку с игрока берут столько килограммов еды, какое число записано в этой клетке (еду берут также за первую и последнюю клетки его пути).

Требуется найти минимальный вес еды в килограммах, отдав которую игрок может попасть в правый нижний угол.
Формат ввода

Вводятся два числа N и M — размеры таблицы ( 1 ≤ N ≤ 20 1≤N≤20, 1 ≤ M ≤ 20 1≤M≤20). Затем идет N строк по M чисел в каждой — размеры штрафов в килограммах за прохождение через соответствующие клетки (числа от 0 до 100).
Формат вывода

Выведите минимальный вес еды в килограммах, отдав которую можно попасть в правый нижний угол/.
*/

use std::io;

pub fn parse_matrix(input: &str) -> Result<Vec<Vec<u32>>, &'static str> {
    let mut lines = input.lines();

    // first line → dimensions
    let dims_line = lines.next().ok_or("Missing input")?;
    let dims: Vec<usize> = dims_line
        .split_whitespace()
        .map(|s| s.parse::<usize>().map_err(|_| "Invalid dimension"))
        .collect::<Result<Vec<_>, _>>()?;

    if dims.len() != 2 {
        return Err("Expected exactly two numbers: rows and columns");
    }

    let rows = dims[0];
    let cols = dims[1];
    let mut matrix = Vec::with_capacity(rows);

    for (line_num, line) in lines.enumerate() {
        if line_num >= rows {
            return Err("Too many rows in input");
        }
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().map_err(|_| "Invalid number in row"))
            .collect::<Result<Vec<_>, _>>()?;

        if row.len() != cols {
            return Err("Incorrect number of columns in row");
        }
        matrix.push(row);
    }

    if matrix.len() != rows {
        return Err("Not enough rows provided");
    }

    Ok(matrix)
}

pub fn get_matrix_from_input() -> Result<Vec<Vec<u32>>, &'static str> {
    // Read all stdin into a single string
    use std::io::Read;
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|_| "Failed to read from stdin")?;
    parse_matrix(&buffer)
}
pub fn get_cheapest_path(grid: Vec<Vec<u32>>) -> u32 {
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut dp: Vec<Vec<u32>> = vec![vec![0; cols]; rows];
    dp[0][0] = grid[0][0];
    for i in 1..cols {
        dp[0][i] = dp[0][i - 1] + grid[0][i];
    }

    for j in 1..rows {
        dp[j][0] = dp[j - 1][0] + grid[j][0];
    }

    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
        }
    }

    return dp[rows - 1][cols - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_2x2() {
        let input = "2 2\n1 2\n3 4\n";
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(parse_matrix(input).unwrap(), expected);
    }

    #[test]
    fn parse_single_row() {
        let input = "1 3\n10 20 30\n";
        let expected = vec![vec![10, 20, 30]];
        assert_eq!(parse_matrix(input).unwrap(), expected);
    }

    #[test]
    fn parse_missing_dimension_line() {
        let input = "";
        assert!(parse_matrix(input).is_err());
    }

    #[test]
    fn parse_invalid_dimension_count() {
        let input = "2\n1 2\n"; // only one number on first line
        assert_eq!(
            parse_matrix(input).err().unwrap(),
            "Expected exactly two numbers: rows and columns"
        );
    }

    #[test]
    fn parse_wrong_columns_in_row() {
        let input = "2 2\n1\n3 4\n";
        assert!(parse_matrix(input).is_err());
    }

    #[test]
    fn parse_too_few_rows() {
        let input = "3 2\n1 2\n3 4\n"; // only 2 rows, expected 3
        assert!(parse_matrix(input).is_err());
    }

    #[test]
    fn empty_grid() {
        assert_eq!(get_cheapest_path(vec![]), 0);
    }

    #[test]
    fn single_cell() {
        let grid = vec![vec![5]];
        assert_eq!(get_cheapest_path(grid), 5);
    }

    #[test]
    fn single_row() {
        let grid = vec![vec![2, 3, 1]];
        // Path must go right: 2+3+1 = 6
        assert_eq!(get_cheapest_path(grid), 6);
    }

    #[test]
    fn single_column() {
        let grid = vec![vec![1], vec![2], vec![3]];
        // Only down: 1+2+3 = 6
        assert_eq!(get_cheapest_path(grid), 6);
    }

    #[test]
    fn typical_2x2() {
        let grid = vec![vec![1, 3], vec![5, 2]];
        // Paths: 1->3->2 = 6, or 1->5->2 = 8 → min 6
        assert_eq!(get_cheapest_path(grid), 6);
    }

    #[test]
    fn larger_grid() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        // Dynamic programming result
        assert_eq!(get_cheapest_path(grid), 21);
    }
}
