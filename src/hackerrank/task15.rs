/// Розв'язок задачі HackerRank: Drawing Book
/// Практична робота №15
#[allow(dead_code)]
pub fn page_count(n: i32, p: i32) -> i32 {
    // Гортання з початку книги
    let from_start = p / 2;
    
    // Гортання з кінця книги
    let from_end = (n / 2) - (p / 2);

    // Вибираємо мінімальне значення з двох варіантів
    std::cmp::min(from_start, from_end)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_drawing_book_example1() {
        // Книга на 6 сторінок, потрібна 2. З початку: 2/2 = 1. З кінця: 6/2 - 2/2 = 3 - 1 = 2. Мінімум: 1.
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_drawing_book_example2() {
        // Книга на 5 сторінок, потрібна 4. З початку: 4/2 = 2. З кінця: 5/2 - 4/2 = 2 - 2 = 0. Мінімум: 0.
        assert_eq!(page_count(5, 4), 0);
    }
}