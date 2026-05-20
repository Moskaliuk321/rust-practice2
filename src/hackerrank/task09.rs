/// Розв'язок задачі HackerRank: Migratory Birds
/// Практична робота №9
#[allow(dead_code)]
pub fn migratory_birds(arr: &[i32]) -> i32 {
    // Індекси 0..6 (індекс відповідає ID птаха від 1 до 5, 0 не використовуємо)
    let mut counts = vec![0; 6];

    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut most_frequent_bird = 1;

    // Проходимо від 1 до 5. Оскільки йдемо по порядку зростання, 
    // знак `>` гарантує, що при однаковій частоті ми залишимо менший ID.
    for bird_id in 1..=5 {
        if counts[bird_id] > max_count {
            max_count = counts[bird_id];
            most_frequent_bird = bird_id as i32;
        }
    }

    most_frequent_bird
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_example() {
        // Птах 4 зустрічається 3 рази, птах 3 — 3 рази. Має повернути 3.
        let arr = vec![1, 4, 4, 4, 5, 3, 3, 3];
        assert_eq!(migratory_birds(&arr), 3);
    }
}