#[test]

/*
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
*/

fn main() {
    {
        // Створюємо ціле число для використання в `coerce_static`:
        let lifetime_num = 9;

        // Підлаштовуємо `NUM` до тривалості `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

// Статична константа з тривалістю 'static.
static NUM: i32 = 18;

// Повертає посилання на `NUM`, яке має тривалість 'a.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}


/*
Статична константа: NUM визначено як статична константа з типом i32. Це означає, що вона існує протягом усієї тривалості програми.

Функція coerce_static: Ця функція приймає посилання на ціле число з тривалістю 'a і повертає посилання на NUM.
Оскільки NUM має тривалість 'static, воно може бути безпечно повернене.

Локальна змінна lifetime_num: У функції main ви визначаєте lifetime_num, яке має локальну тривалість.
Викликаючи coerce_static(&lifetime_num), ви фактично отримуєте посилання на NUM, а не на lifetime_num, оскільки NUM завжди доступний
*/