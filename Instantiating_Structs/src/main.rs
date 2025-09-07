#[derive(Debug)]
//difining and instanting Structure
 struct User {
         active: bool,
         username: String,
         email: String ,
         sign_in_count: u64,
}
fn main(){
    
    let user1 = User {
        active:true,
        username:String::from("usernam"),
        email: String::from("usernam@exmple.org"),
        sign_in_count: 1,
                     };
            
    let mut user2 = User {
        active:true,
        username:String::from("usernam"),
        email: String::from("usernam@exmple.org"),
        sign_in_count: 1,
                     };
                //how to change the value in the email
    user2.email = String::from("anotheremail@examplke.com");

//Printing the structs using debug formatting
println!("{:#?}", user1);
 println!("{:#?}", user2);
  
          }
