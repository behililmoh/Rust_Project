fn main(){
    let tup =(500, 6.4, 1);
     let (x,y ,z)=tup;
     println!("the valus of y is :{}",y);
     println!("the valus of x is :{}",x);
     println!("the valus of z is :{}",z);
     //We can also access a tuple element directly by using a period (.)
     // followed by the index of the value we want to access. 
     let a =tup.0;
     let b=tup.1;
     let c =tup.2;
     println!("{} - {} - {}",a ,b , c);
     
     //The Array Type
     //Another way to have a collection of multiple values is with an array.
     let a=[1,2,0,4,5];
     for i in a.iter(){
        println!("{}",i);
     }

     
}    