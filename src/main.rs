mod field;
use field::Field;

fn main() {
    let field = Field::<4>::new();
    println!("{field}");
}
