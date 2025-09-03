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


          }
