// Підключаємо папку hackerrank як модуль
pub mod hackerrank;

// --- Код з Практичної №2 (Variables & Types) ---
fn variables_practice() {
    let mut x = 5;
    x += 10;
    let y = 10;
    {
        let y = 5;
        println!("Внутрішня y: {}", y);
    }
    println!("Зовнішня y: {}, x: {}", y, x);
}

fn types_practice() {
    let i: i32 = -10;
    let u: u32 = 10;
    let c = 'z';
    let heart = '❤';
    println!("Числа: {} та {}, Символи: {} {}", i, u, c, heart);
}

fn compound_types_practice() {
    let s1 = String::from("hello");
    let s2 = "world";
    let tuple: (i32, f64, &str) = (500, 6.4, "Rust");
    let (val1, _, _) = tuple;
    println!("Рядки: {} {}, Кортеж: {}", s1, s2, val1);
}

// --- Головна функція ---
fn main() {
    println!("--- Practical 2: Rust by Practice ---");
    variables_practice();
    types_practice();
    compound_types_practice();

    println!("\n--- Practical 3: HackerRank Staircase ---");
    // Викликаємо функцію з нашого нового модуля для перевірки
    let result = hackerrank::task03::staircase(6);
    println!("{}", result);
}