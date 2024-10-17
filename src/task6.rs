#[test]

/*
use std::fmt::Display;

fn main() {
  let mut string = "First".to_owned();

  string.push_str(string.to_uppercase().as_str());
  print_a(&string);
  print_b(&string);
  print_c(&string); // Compilation error
  print_d(&string); // Compilation error
  print_e(&string);
  print_f(&string);
  print_g(&string); // Compilation error
}

fn print_a<T: Display + 'static>(t: &T) {
  println!("{}", t);
}

fn print_b<T>(t: &T)
where
  T: Display + 'static,
{
  println!("{}", t);
}

fn print_c(t: &'static dyn Display) {
  println!("{}", t)
}

fn print_d(t: &'static impl Display) {
  println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
  println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
  println!("{}", t)
}

fn print_g(t: &'static String) {
  println!("{}", t);
}
*/



fn main() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(Box::new(string.clone())); // Передаємо в Box
    print_d(Box::new(string.clone())); // Передаємо в Box
    print_e(&string);
    print_f(&string);
    print_g(&string);
}
use std::fmt::Display;

fn print_a<T: Display>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display,
{
    println!("{}", t);
}

// Використання Box<dyn Display> для динамічного диспетчеризації
fn print_c(t: Box<dyn Display>) {
    println!("{}", t);
}

fn print_d(t: Box<dyn Display>) {
    println!("{}", t);
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t);
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t);
}

fn print_g(t: &String) {
    println!("{}", t);
}



/*
print_c і print_d: Замість прийняття параметра з 'static тривалістю, ви можете приймати динамічний тип через Box<dyn Display>. Це дозволяє передавати об'єкти, що реалізують трейт Display, без обмеження на тривалість 'static.
print_a і print_b: Видалено 'static з обмеження на тип, щоб дозволити передачу локальних змінних.
Виклики print_c та print_d:
Тепер ці функції приймають параметри типу Box<dyn Display>. Щоб передати рядок у ці функції, ми створюємо Box з рядка, використовуючи Box::new(string.clone()).
Я використовую string.clone(), щоб створити копію string, оскільки передача в Box споживає його.
*/