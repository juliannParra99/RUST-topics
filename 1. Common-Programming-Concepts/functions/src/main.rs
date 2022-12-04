fn main() {
    println!("Hello, world!");
    another_function(3);
    print_labeled_measurement(5, 'h');
    //expresiones y declaraciones: a cortos rasgoz, una declaracion es asignar un valor a una varibale, NO DEVUELVE UN VALOR; en cambio una expression se evaluan como un valor, es decir, representan una operacion matematica, etc; tambien refiere a VALORES DEVUELTOS, lo que quiere decir que una funcion devuelta, una funcion llamada, es una expresion: A su vez, una variable o una funcion son a su ves una declaracion y una expresion. Las expresiones no incluyen puntos y comas finales
    // Si agrega un punto y coma al final de una expresión, la convierte en una declaración y luego no devolverá un valor
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    //Funciones con valores de retorno:el equivalente al return en rust
        //Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

        let j = five();
        println!("The value of j is: {j}");
        //otro ejemplo
        let p = plus_one(5);

        println!("The value of x is: {p}");

        println!("This added of f is {}", f({ //los corchetes son un alcance sintactico una expresion +declaracion
            let y = 1;
            y + 1
          }))
}

fn another_function(x: i32) { 
    println!("The value of p is: {x}");//en las firmas de funcion, hay que aclarar el tipo de cada parametro
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//funciones con variable de retorno:
fn five() -> i32 {
    5  //sin punto y coma porque es una expresión cuyo valor queremos devolver.
}

fn plus_one(p: i32) -> i32 {//retorna un i32  que se epasa como parametro y se le agrega otro valor
    p + 1 //si le colocamos un punto y coma devuelve una declaracion no una expresion.
}

fn f(x: i32) -> i32 { x + 1 }