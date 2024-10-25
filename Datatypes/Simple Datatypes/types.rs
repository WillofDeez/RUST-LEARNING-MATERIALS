fn main(){
// INTEGERS
//===========================================    
     //Signed integers(i)
//     i8 i16 i32 i64 i128

    //Unsigned integers(u) (More memory)
//    u8 u16 u32 u64 u128
//===========================================
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unigned Integer: {}", y);

// FLOAT
//===========================================    
//     f32 f64
//===========================================
    let pi: f64 = 3.14;
    
    println!("Value of pi: {}", pi);

// BOOLEAN
//===========================================    
//     bool(true/false)
//===========================================

 let is_snowing: bool = true;
 println!("Is it snowing ?: {}", is_snowing);

// CHARACHTER
//===========================================    
//     char (unicode charachter)
//===========================================
 let questionmark: char = '?';
 println!("This is a questionmark: {}", questionmark);
}