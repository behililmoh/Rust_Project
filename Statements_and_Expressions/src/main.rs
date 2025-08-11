//Statements and Expressions
fn main(){
    let _y =6;
   // let x =(let t= 8); ---The let t = 8 statement does not return a value, so there isn’t anything for x to bind to. 
    let a ={
        let x=3;
        x + 1 //Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, 
              //you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function
              // return values and expressions next.
     };
    println!("The value of a {}",a);

    //Functions with Return Values
   let x2=five();
   println!("The value of x2 is {}",x2);
   let y2=plus_one(3);
   println!("The value of w2 is {}",y2);
}
fn five()->i32{
    5
}
fn plus_one (x3:i32)->i32{
    x3+1
}