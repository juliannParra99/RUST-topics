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




    let mut  x = 5;
    println!("The value of x is: {x}");
    //espera el mismo tipo de dato cuando se cambia el valor con MUT
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    println!("Shadowing:");
    // se puede utilizar una misma variable y sobreescribirla, el nuevo valor opacara  la  primer variable y solo tendra en cuenta al nuevo valor asignado.

    // La otra diferencia entre muty shadowing es que debido a que estamos creando efectivamente una nueva variable cuando usamos la letpalabra clave nuevamente, podemos cambiar el tipo de valor pero reutilizar el mismo nombre.

    let x = 5;
    
    let x = x + 1;
    println!("the value of x is : {x}");
        
        //el espacio separado por llaves indica un ambito interno
        //em el ambito interno la variable tiene el valor que se le declara alli dentro.
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
    
    println!("The value of x is: {x}");

    let spaces = "          ";
    //dice cuantos valores tiene el sring: se remplaza la variable anterior por la cantidad de numeros del string
    let spaces = spaces.len();
    
    print!("La cantidad de espacios es : {spaces}")



}


