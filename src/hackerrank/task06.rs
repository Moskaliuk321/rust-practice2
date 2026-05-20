/// Розв'язок задачі HackerRank: Kangaroo
/// Практична робота №6
#[allow(dead_code)]
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо перший кенгуру біжить швидше і відстань ділиться без остачі
    if v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_yes() {
        // Приклад 1: наздожене (0 3 4 2) -> YES
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_no() {
        // Приклад 2: перший повільніший або не потрапить у такт (0 2 5 3) -> NO
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }
}