// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
//done

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
    //按字符遍历字符串  // 按字符遍历字符串 // 取出字符；如果字符串为空，会 panic
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data.to_uppercase();

    println!("{}", data);
}
