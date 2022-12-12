// To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:


use std::io; //impor libraries (crates); std es la biblioteca standar, viene con Rust
use rand::Rng;//El Rng rasgo define los métodos que implementan los generadores de números aleatorios
use std::cmp::Ordering;

fn main() {
    // The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.
    println!("Guess the number!");
    //Gneracion de un numero aleatorio que rust tiene que adivinar en el juego: rust no tiene funcionalidad hasta la fecha para generar un numero aleatorio, pero se puede acceder a la funcionalidad  a traves del RAND crate que es una la libreria crate: para acceder a ella primero modificamos el CARGO.toml para indicarle que es  una dependencia que vamos a utilizar para nuestro proyecto; indicamos en las depencias que vamos a utilizar RAND. en CARGO.toml you tell Cargo which external CRATES(MODULOS) your project depends on and which versions of those crates you require
    //Generando un Número Aleatorio
    //Comencemos a usar randpara generar un número para adivinar.
    let secret_number = rand::thread_rng().gen_range(1..=100);//gen_range método toma una expresión de rango como argumento y genera un número aleatorio en el rango.

    // println!("The secret number is: {secret_number}"); // esto es util en desarrollo

    
    loop { //un loop para que el jugador pueda jugar varias veces seguidas y no se corte
        
            println!("Please input your guess.");
        
            let mut guess = String::new(); //hacemos a la variable mutable, puede cambiar de valor;esta variable almacena la entrada del usuario;l valor al que guessestá vinculado, que es el resultado de llamar String::new() , una función que devuelve una nueva instancia de String.La ::sintaxis en la ::newlínea indica que new es una función asociada del String tipo. Una función asociada es una función que se implementa en un tipo, en este caso String. Esta newfunción crea una nueva cadena vacía
        
            //llamaremos a la stdin función desde el io módulo, lo que nos permitirá manejar la entrada del usuario;La stdin función devuelve una instancia de std::io::Stdin, que es un tipo que representa un identificador de la entrada estándar para su terminal. Maneja lo que el usuario ingresa
            //TRAE LO QUE ESCRIBE EL USUARIO
            io::stdin()
                .read_line(&mut guess)//The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of datawithout needing to copy that data into memory multiple times.
                .expect("Failed to read line"); //hay que usarlo para evitar errores; bloquea el programa si ocurre un problema

            //para que el valor ingresado sea un numero y pueda ser comparado con match; lo sombreamos
            //esto hace que si en el expected sea un numero se devuelva un numero y  se compare con el valor escondido, si el formato no es un numero ; MANEJAMOS EL ERROR 
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, //le dice al programa que vaya a la siguiente iteración del loopy solicite otra suposición
        };

            println!("You guessed: {guess}");
            //COMPARA EL NUMERO INGRESADO CON EL ALEATORIO
            match guess.cmp(&secret_number) { //cmp compara dos valos;aquí está comparando el guess con el secret_number; el numero va a ser un less, grater o equal, y dependiendo que va a ejecutar el brazp cprrespondiente
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                println!("You win!");
                break;//Sale del ciclo cuando ganas
            }
        }
        
    }
        
}
