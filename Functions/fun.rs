fn main() {
    person_id("Mike", 20, 210.5);
    let height = 1.82;
    let weight = 82.33;
    calculate_bmi(weight, height);
}

fn person_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old and my height is {} cm.",
        name, age, height,
    );
}

fn calculate_bmi(weight_kg: f32, height_m: f32) {
    let bmi: f32 = weight_kg / (height_m * height_m);
    println!("My bmi is {:.2}", bmi);
}
