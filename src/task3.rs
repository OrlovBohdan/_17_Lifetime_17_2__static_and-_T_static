#[test]

/*
fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    println!("static_string reference remains alive: {}", static_string);
}
*/



fn main() {
    {
        // Виводимо статичний рядок
        println!("STATIC_STRING: {}", STATIC_STRING);
    }

    // Доступ до статичного рядка поза блоком
    println!("STATIC_STRING reference remains alive: {}", STATIC_STRING);
}

// Оголошуємо статичний рядок з тривалістю 'static
static STATIC_STRING: &str = "I'm in read-only memory";
/*
Статичний рядок: Змінна STATIC_STRING оголошена як статичний рядок з типом &str.
Це означає, що вона має тривалість 'static і доступна в усій програмі.

Виведення значення: Значення STATIC_STRING виводиться в обох місцях, де ви його використовуєте, без помилок компіляції.
*/