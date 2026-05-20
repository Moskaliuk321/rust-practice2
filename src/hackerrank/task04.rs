/// Розв'язок задачі HackerRank: Grading Students
/// Практична робота №4
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple_of_5 = ((grade / 5) + 1) * 5;
                if next_multiple_of_5 - grade < 3 {
                    next_multiple_of_5
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_edge_cases() {
        let input = vec![0, 37, 38, 39, 40, 99, 100];
        let expected = vec![0, 37, 40, 40, 40, 100, 100];
        assert_eq!(grading_students(&input), expected);
    }
}