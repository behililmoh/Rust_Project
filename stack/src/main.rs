fn main(){
    // Creates an empty, mutable vector of integers (i32).
let mut stack = Vec::new();
// 1. PUSH: Adds elements to the end of the vector.
stack.push(1);
stack.push(2);    
stack.push(3);
println!("The stack after adding 1,2,3{:?}",stack);

// 2. POP: Removes and returns the last element of the vecto
   // 2. POP : Supprime et renvoie le dernier élément du vecteur.
    // POP enlève le dernier élément ajouté.
    let top_element = stack.pop();
    println!("Élément retiré: {:?}", top_element);
    println!("La pile après le retrait: {:?}", stack);

    // Un deuxième pop.
    let second_element = stack.pop();
    println!("Deuxième élément retiré: {:?}", second_element);
    println!("La pile après le deuxième retrait: {:?}", stack);

    // Un pop sur une pile vide renvoie None.
    let empty_pop = stack.pop();
    println!("Tentative de retrait sur une pile vide: {:?}", empty_pop)
}