#[test]

/*

/* Fill in the blank in two ways */
fn main() {
    __;
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}
*/

//1
fn main() {
    let v: &'static str = "hello"; // Присвоюємо статичний рядок
    need_static(v);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}


//2

/*fn main() {
    need_static("hello"); // Використання літерального рядка

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}*/


/*
//1
Можна використовувати літеральний рядок, який має статичне життя
//2
можна безпосередньо передати рядок у функцію need_static
*/