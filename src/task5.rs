#[test]

/*
/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}
*/
fn main() {
    // i is owned and contains no references:
    let i = 5;
    print_it(i);

    // &i only has the lifetime defined by the scope of main()
    print_it(&i); // Тепер працює

    print_it1(&i); // Тепер працює

    // but this one WORKS!
    print_it2(&i); // Це працює
}
use std::fmt::Debug;

fn print_it<T: Debug>(input: T) {
    println!("value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug) {
    println!("value passed in is: {:?}", input);
}

fn print_it2<T: Debug>(input: &T) {
    println!("value passed in is: {:?}", input);
}




/*
Статична тривалість: У Rust, якщо змінна не має 'static тривалості, ви не можете передати її посиланням до функцій,
які вимагають таку тривалість. Коли функція використовує 'static, це означає, що значення, передане в цю функцію,
не може містити посилань на локальні змінні.

Оновлений код: Вилучивши 'static з функцій print_it і print_it1, ви дозволяєте їм працювати з будь-якими типами,
які реалізують трейти Debug, без прив'язки до тривалості 'static.
*/