//use std::result;

//use std::ops::Index;

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
println!("Multiple Conditions with else if");
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
//let condition2 = true;
 //let number5 =if condition2{5}else{"six"};
 //println!("the value of number  is {}",number5);

 //Repeating Code with loop

let mut counter = 0;
let result=loop{
    counter+=1;
    if counter ==10{
        break counter *2;
    }
};
println!("the result id {}",result);

//loop label to Disambiguate Between Multiple loop
println!("loop label to Disambiguate Between Multiple loop");
let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

//Conditional Loops with while
let mut numb=3;
while numb!=0{
    println!("{}",numb);
    numb -=1;
}
println!("LIFTOFF!!!");

//Looping Through a Collection with for
let b= [10, 20, 30, 40, 50];
let mut index =0;

while index < 5 {
    println!("the valeur is :{}", b[index]);
    index +=1;
}
//Looping through each element of a collection using a for loop
let b2 =[10,20,30,40,50];
for element in b2{
    println!("the value of b2 is :{}", {element})
}
//Contdown
for number in(1..4).rev(){
    println!("{}",number);
}
println!("LIFTOFF!!!");

}
