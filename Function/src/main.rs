fn main(){
 println!("hllo world");
  
  another_function();
  another_function2(5);
 anther_function3(5, 'h');
}

 fn another_function(){
    println!("ANOTHER FUNCYION");
 }
 fn anther_function3(value:i32,unit_label:char){
  println!("The measurement is :{}{}",value,unit_label);

 }
 fn another_function2(x:i32){
   println!("the value of x is :{}",x);
 }