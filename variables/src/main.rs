//Variables and Mutability
fn main(){
    let mut x =5;//valure mutable
    println!("the value off x is ={}",x);
    x=6;
    println!("the valus off x is {}",x);
    //Constants
    const THREE_HOURS_IN_SCONDS:u32 =60*60*3;
    println!("60 * 60 *3 ={}",THREE_HOURS_IN_SCONDS);
    //Shadowing
    let y =5;
    let y=y+1;
    {
     let y=y*2;
     println!("The value of x in inner scope is :{}",y);
    }
    println!("The values of x is :{}",y);
    
    let spaces ="    ";
    println!("the valus  spaces is {}",spaces);

    let spaces=spaces.len(); 
 println!("the new value of spaces is :{}",spaces);   
}