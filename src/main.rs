mod field;
use field::Field;

fn main() {
    let mut field = Field::<5, 20>::new();
    println!("{field}");
    println!();
    field.reveal();
    println!("{field}");
}
