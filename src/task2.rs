#[test]

/*
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
fn init() -> Option<&'static mut Config> {
    Some(&mut Config {
        a: "A".to_string(),
        b: "B".to_string(),
    })
}


fn main() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}
*/

fn main() {
    unsafe {
        // Ініціалізуємо config зі статичним мутабельним посиланням на Config
        CONFIG = init();

        // Виводимо вміст config
        println!("{:?}", CONFIG);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    a: String,
    b: String,
}

// Статична змінна для зберігання мутабельного посилання на Config

static mut CONFIG: Option<&'static mut Config> = None;

fn init() -> Option<&'static mut Config> {
    // Створюємо Box, що містить екземпляр Config, а потім зливаємо його
    let boxed_config = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    // Зливаємо Box, що повертає мутабельне посилання з тривалістю 'static
    Some(Box::leak(boxed_config))
}


/*
Використання Box::leak: Функція Box::leak береться для того, щоб отримати власність на Box<Config>
і перетворити його на мутабельне посилання з тривалістю 'static. Це запобігає знищенню екземпляра Config,
коли він виходить за межі області видимості.

Типізація змінної: Статична мутабельна змінна config правильно типізується для зберігання
мутабельного посилання на Config з тривалістю 'static.
*/