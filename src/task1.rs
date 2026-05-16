/*
Рассмотрим три числа a a, b b и c c. Упорядочим их по возрастанию.

Какое число будет стоять между двумя другими?

Формат ввода

В единственной строке записаны три целых числа a a, b b, c c ( − 1000 ≤ a , b , c ≤ 1000 −1000≤a,b,c≤1000), числа разделены одиночными пробелами.
Формат вывода

Выведите число, которое будет стоять между двумя другими после упорядочивания.
*/

pub fn median_from_string(input: &str) -> i32 {
    let mut v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    v.sort();
    v[v.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_odd() {
        assert_eq!(median_from_string("3 1 2"), 2);
    }

    #[test]
    fn simple_even() {
        assert_eq!(median_from_string("4 1 3 2"), 3);
    }

    #[test]
    fn single_element() {
        assert_eq!(median_from_string("42"), 42);
    }

    #[test]
    #[should_panic]
    fn empty_input_panics() {
        median_from_string("");
    }

    #[test]
    fn extra_whitespace() {
        assert_eq!(median_from_string("   7   5   9   "), 7);
    }
}
