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

     //Numeric Operations

        //addition
    let _sum =5+10;
    //subtraction 
    let _difference =95.5-4.3;
    //multiplication
    let _produit= 4/30;
    //division
    let _quotent =56.7/32.2;
    let _truncated = -5/3;
    //remainder
    let _remainder = 43%5;
//🦀In Rust, the underscore (_) in a variable name has a special meaning.
//1. Ignore an unused :variable When you write code with a variable that you are not going to use,
// Rust raises a warning. To say 'I know this variable exists, but I don't need it', you can prefix its name with an _.
//📌 2. The _ alone as a variable name You can also simply use _ to indicate that you don't want to name the value at all
//📌 3. In matches, Rust uses _ as a wildcard to match anything that does not correspond to the other cases
//This happens when Rust’s macro system sees a variable like remainder but doesn't recognize 
//it as a proper field or identifier. If you're using something like format_args!, println!,
// or a custom macro and passing a named argument that isn't used in the format string,
// Rust expects you to prefix it with an underscore if it's intentional.


}