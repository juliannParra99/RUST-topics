// To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:


use std::io; //impor libraries (crates)

fn main() {
    // The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //hacemos a la variable mutable, puede cambiar de valor;esta variable almacena la entrada del usuario;l valor al que guessestá vinculado, que es el resultado de llamar String::newa , una función que devuelve una nueva instancia de String.La ::sintaxis en la ::newlínea indica que new es una función asociada del String tipo. Una función asociada es una función que se implementa en un tipo, en este caso String. Esta newfunción crea una nueva cadena vacía


    //llamaremos a la stdin función desde el io módulo, lo que nos permitirá manejar la entrada del usuario;La stdin función devuelve una instancia de std::io::Stdin, que es un tipo que representa un identificador de la entrada estándar para su terminal.
    io::stdin()
        .read_line(&mut guess)//The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        .expect("Failed to read line"); //hay que usarlo para evitar errores; bloquea el programa si ocurre un problema

        //At this point, the first part of the game is done: we’re getting input from the keyboard and then printing it.
        println!("You guessed: {guess}");

        //Gneracion de un numero aleatorio: rust no tiene funcionalidad hasta la fecha para generar un numero aleatorio, pero se puede acceder a la funcionalidad  a traves del RAND crate que es una la libreria crate: para acceder a ella primero modificamos el CARGO.toml para indicarle que es  una dependencia que vamos a utilizar para nuestro proyecto; indicamos en las depencias que vamos a utilizar RAND. en CARGO.toml you tell Cargo which external CRATES(MODULOS) your project depends on and which versions of those crates you require
        
}
