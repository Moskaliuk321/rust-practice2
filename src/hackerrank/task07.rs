/// Розв'язок задачі HackerRank: Between Two Sets
/// Практична робота №7

// Функція для пошуку Найбільшого Спільного Дільника (НСД / GCD)
fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

// Функція для пошуку Найменшого Спільного Кратного (НСК / LCM)
fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[allow(dead_code)]
pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }

    // Знаходимо НСК для всього масиву A
    let mut lcm_a = a[0];
    for &val in a.iter().skip(1) {
        lcm_a = lcm(lcm_a, val);
    }

    // Знаходимо НСД для всього масиву B
    let mut gcd_b = b[0];
    for &val in b.iter().skip(1) {
        gcd_b = gcd(gcd_b, val);
    }

    // Рахуємо скільки кратних чисел для lcm_a ділять gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;
    
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets_example() {
        // Приклад з умови: A = [2, 4], B = [16, 32, 96] -> числа 4, 8, 16 (всього 3)
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}