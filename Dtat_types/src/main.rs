fn main(){
  let celsius :f64 = 25.0 ;//type annotation required for floating-point calculations
  let fahrenheit = (celsius *9.0 / 5.0)+32.0;
  println!("{}°C est égal à {}°F", celsius, fahrenheit );

  //- f64 is a scalar type for floating-point numbers .
  //- Here, Rust automatically infers the type of fahrenheit.
  
  let _guess: u32 = "42".parse().expect("Not a number!");
 
 //Integer  
let x= -42;
let y: u8 = 255;
println!("the value of x: {} and the value of  y is:{}",x ,y);
//Types disponibles : | Taille | Signé | Non signé |
//                    |--------|-------|-----------| 
//                    | 8-bit  |   i8  | u8        |
//                    | 16-bit |   i16 | u16       |
//                    | 32-bit |   i32 | u32       |
//                    | 64-bit |   i64 | u64       | 
//                    | 128-bit|   i128| u128      |
//                    | Arch   |  isize| usize     |

//Floating-point
let _a: f32=2.15;//simple precision
let _b =-2.718;// double precision
println!("{} et {}",_a,_b);

//Booleans
    let is_active = true;

    let is_disable: bool = false; // with explicit type annotation

println!("Active:{}, Disable:{}", is_active,  is_disable);

//Characters
 
    let letter = 'A';
    let emoji = '😻';
    let symbol: char = 'ℤ';// with explicit type annotation

     println!("Lettre: {}, Emoji: {}, Symbole: {}", letter, emoji, symbol);

}