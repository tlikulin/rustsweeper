mod field;
use field::Field;

fn main() {
    let field = Field::<10>::new();
    println!("{field}");
}
