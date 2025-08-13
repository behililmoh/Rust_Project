fn main(){
let number =3;
//true condition
if number<5 {
    println!("condition was true");
    }else {
        println!("condition was false");
    }
//false condition
let number2=7;
if number2<5{
    println!("cndition was true");
    }else {
        println!("condition was false");
    }
//err ^ expected `bool`, found integer
//if number {
//    println!("the neumber was three");
//}
if number!=0{
    println!("number was somthing other then zero");
}
//Multiple Conditions with else if
 let number3 =6;
 if number3 %4==0{
     println!("number is divisible by 4");
 }else if number3 %3 ==0{
     println!("number is divisible by 3");
 }else if number3 % 2 ==0{
     println!("number is divdsible by 2");
 }else  {
     println!("number is not divisible by 4,3, or 2");
 }

 //Using if in a let Statement
 let condition = true;
 let number4 =if condition{5}else{10};
 println!("the value of number  is {}",number4);


 //The provided Rust code will not compile.The if and else blocks in a Rust if expression must return values of the same type
//  let condition2 = true;
 //let number5 =if condition2{5}else{"six"};
 //println!("the value of number  is {}",number5);

}
