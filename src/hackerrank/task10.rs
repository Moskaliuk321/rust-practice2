/// Розв'язок задачі HackerRank: Sales by Match (Sock Merchant)
/// Практична робота №10
#[allow(dead_code)]
pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    // Масив для підрахунку кількості шкарпеток кожного кольору (від 1 до 100)
    let mut color_counts = vec![0; 101];

    for &sock in ar {
        if sock >= 1 && sock <= 100 {
            color_counts[sock as usize] += 1;
        }
    }

    let mut pairs = 0;
    for count in color_counts {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_example() {
        let n = 9;
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(n, &ar), 3);
    }
}