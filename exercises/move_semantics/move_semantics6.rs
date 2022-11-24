// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    let data2 = get_char(data);

    string_uppercase(&data2);
}

// Should not take ownership
fn get_char(data: String) -> String {
    data.chars().last().unwrap();
    data
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    let data3 = &data.to_uppercase();

    println!("{}", data3);
}
