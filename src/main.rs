// Розділ 3: Variables (Mutability & Scope)
fn variables_practice() {
    let mut x = 5; // додаємо mut, щоб можна було змінювати
    x += 10;

    let y = 10;
    {
        let y = 5; // Shadowing: ця y живе тільки в цих дужках
        println!("Внутрішня y: {}", y);
    }
    println!("Зовнішня y: {}, x: {}", y, x);
}

// Розділ 4: Basic Types (Numbers & Chars)
fn types_practice() {
    let i: i32 = -10;
    let u: u32 = 10;

    let c = 'z';
    let heart = '❤'; // Rust підтримує Unicode

    println!("Числа: {} та {}, Символи: {} {}", i, u, c, heart);
}

// Розділ 6: Compound Types (Strings & Tuples)
fn compound_types_practice() {
    // String - динамічний рядок, &str - зріз (посилання)
    let s1 = String::from("hello");
    let s2 = "world";

    // Кортеж (Tuple)
    let tuple: (i32, f64, &str) = (500, 6.4, "Rust");
    let (val1, _, _) = tuple; // Деструктуризація

    println!("Рядки: {} {}, Кортеж: {}", s1, s2, val1);
}

fn main() {
    println!("--- Practical 2: Rust by Practice ---");
    variables_practice();
    types_practice();
    compound_types_practice();
    println!("Код успішно скомпілювався та запрацював!");
}