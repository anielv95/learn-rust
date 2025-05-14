fn main() {
    let integer_var: u32 = 3;
    let float_var: f64 = 3.141592;
    let boolean_var: bool = true;
    let character_var: char= 'a';
    let second_character_var: &str= "ab";
    let string_var: &str = "this is a string";
    println!("Hello, world!");
    println!("integer variable {integer_var}");
    println!("float variable {float_var}");
    println!("boolean variable {boolean_var}");
    println!("character variable {character_var}");
    println!("string variable {string_var} {second_character_var}");


    println!("some operations between variables:\n");
    let operation_var = integer_var+integer_var;
    println!("sum:\n{operation_var}");
    let operation_var = float_var-3.0;
    println!("substraction:\n{operation_var}");
    let operation_var = float_var*5.7;
    println!("multiplication:\n{operation_var}");
    let operation_var = float_var*float_var/2.03;
    println!("division:\n{operation_var}");
    let operation_var = 4/integer_var;
    println!("truncated division:\n{operation_var}");
    let operation_var = 9.0%2.0;
    println!("remainder:\n{operation_var}");
}
