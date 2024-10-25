fn main() {
    //ARRAYS
    //================================
    //["Datatype";Size(+int)]
    //================================

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apples", "Banana", "Citrus"];
    println!("Fruits: {:?}", fruits);
    println!("Fruit Number 1 is: {}", fruits[0]);
    println!("Fruit Number 2 is: {}", fruits[1]);
    println!("Fruit Number 3 is: {}", fruits[2]);

    //TUPLES
    //================================
    //()
    //================================

    let person: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Person Tuple: {:?}", person);

    //mixed tuple
    let my_mix_tuple = (
        "Kratos",
        25,
        ["Zeus", "Athena", "Thor", "Atlas"],
        [1738, 2000, 1898],
    );
    println!("Mixed Tuple: {:?}", my_mix_tuple);

    //SLICES
    //===================================
    //&[]
    //===================================

    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slice);

    // Reference to string
    let animal_slice: &[&str] = &["Cat", "Dog", "Cattle", "Bober", "Bear"];
    println!("Animal Slice: {:?}", animal_slice);

    // Datatype string and convertion to string
    let book_slice: &[&String] = &[
        &"Attack On Titan".to_string(),
        &"Democracy".to_string(),
        &"IT".to_string(),
        &"The Witcher".to_string(),
    ];
    println!("book Slice: {:?}", book_slice);

    //Strings VS String Slices(&str)
    //Strings are [growable, mutable, owned string type,stored on heap]
    //mutable/dynamic
    //Slower

    let mut john_cena: String = String::from("You cant");
    john_cena.push_str(" see me! ");
    println!("John Cena Says : {}", john_cena);

    //B- &Str (String Slice)
    //Static
    //Faster

    let string: String = String::from("Hello World, Good Afternoon!");

    let slice_1: &str = &string[0..11];

    let slice_2: &str = &string[13..];
    println!("Slice Value: {}", slice_1);
    println!("Slice Value: {}", slice_2);
}
