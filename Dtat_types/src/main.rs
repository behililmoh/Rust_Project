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
}