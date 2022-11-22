fn main() {
    println!("Hello, world! My name is {}, and i have {} years old", "julian parra", 23);
     
    
    //"todo!" indica donde me quede en mi proyecto, corta la ejecucion para dar una indicacion de que algo se debe hacer en esa parte del codigo 
    //        todo!("hasta aqui deje mi proyecto mi bro");


    //variables
    // Declare a variable
    let a_number;
    
    // Declare a second variable and bind the value
    let a_word = "Ten";
    
    // Bind a value to the first variable
    a_number = 10;
    
    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    //Cambiar valor de variables

    // The `mut` keyword lets the variable be changed
let mut a_number = 10; 
println!("The number is {}.", a_number);

// Change the value of an immutable variable
a_number = 15;
println!("Now the number is {}.", a_number);

//variable shadowing
// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num" 
let shadow_num = shadow_num + 5; 

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2; 

println!("The number is {}.", shadow_num);

}


