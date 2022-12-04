fn main() {
    println!("Date type: integers");
    //si queremos pasar el valor de un string a un number, tenemos que especificar el tipoo de dato despues de la variable, en este caso u32
    let guess: u32 = "42".parse().expect("Not a number!"); //la 'u' indica que no tiene signo, 'i' si estan signados; los no signados ocupan la mitad de bits que los signados: ver libro de rust 
    println!("{guess}");

    //dos subconjuntos de tipos de datos: escalar y compuesto.
    //escalar:: enteros, números de coma flotante, booleanos y caracteres.

    //enteros:u32: entero sin signo; i32(bits) entero con signo
    //desbordamiento de enteros: cuando el valor de  bits que configuramos, no coincide con el valor numero al que llega esa configuracion.

    let number: u8 = 247; //u8 va de 0 a 255
    //En el caso de a u8, el valor 256 se convierte en 0, el valor 257 se convierte en 1 y así sucesivamente. Esto con un manejador : Rust performs two’s complement wrapping; pero es mejor no confiarse en el
    println!("the value of number is: {}", number);

    //Floating-Point Types
    //f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is  64; TINE SIGNO
    println!("Floating-Point Types");
    let x = 2.4; // f64 
    println!("{x}");
    let j: f32 = 3.7; // f32
    println!("{j}");

    println!("Operaciones numericas.");
    //suma, resta, multiplicación, división y resto.

    // addition
    let sum = 5 + 10;
    println!("sum:{sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference:{difference}");
    
    // multiplication
    let product = 4 * 30;
    println!("product:{product}");
    
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient:{quotient}");
    let floored = 2 / 3; // Results in 0
    println!("floored:{floored}");
    
    // remainder
    let remainder = 43 % 5; //el resto
    println!("remainder:{remainder}");

    println!("booleanos: bool");
    
    let t = true;
    println!("{t}");
    //
    print!("The Character Type");
    //char : solo caracteres unicos y emojis,
    //4 bytes un caracter unicode
    let c = 'z';
    println!("{c}");
    //let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");


    //Compound Types
    //Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    //TUPLA
    // tipo compuesto. Las tuplas tienen una longitud fija: una vez declaradas, no pueden crecer ni reducir su tamaño.

    // Creamos una tupla escribiendo una lista de valores separados por comas entre paréntesis. Cada posición en la tupla tiene un tipo, y los tipos de los diferentes valores en la tupla no tienen que ser iguales. Hemos agregado anotaciones de tipo opcionales en este ejemplo:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let one = tup.1;
    // println!("{one}");
    println!("{}",tup.0);

    // La variable tupse vincula a toda la tupla, porque una tupla se considera un único elemento compuesto. Para obtener los valores individuales de una tupla, podemos usar la coincidencia de patrones para DESESTRUCTURAR un valor de tupla, así:
    
    let tup2 = (500, 6.4, 1);

    let (i, y, z) = tup2; //tambien funciona accediendo a un solo indice y guardarlo en una variable.

    println!("The value of y is {y},i is {i}, z is {z}");

    // //The Array Type; 
    //no confundir con un vector, estos pueden tener mayor flexibilidad
    // Otra forma de tener una colección de valores múltiples es con una matriz . A diferencia de una tupla, todos los elementos de una matriz deben tener el mismo tipo.Tienen longitud fija.
    // las matrices son más útiles cuando sabe que no será necesario cambiar la cantidad de elementos. Por ejemplo, si estuviera usando los nombres de los meses en un programa,
    println!("ARRAY TYPE");
    //
    // Escribe el tipo de una matriz usando corchetes con el tipo de cada elemento, un punto y coma y luego la cantidad de elementos en la matriz, así:
    // despues de declarar la variable entre corchetes va el tipo de dato punto y coma y el N° de datos

    let a = [1, 2, 3, 4, 5];
    println!("el valor del primer indice es: {}", a[0]);
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    println!("{}", months[4]);

    //Accedemos a los valores por indice : en este caso primero se guarda el valor en variables.
    let array_numeros = [1, 2, 3, 4, 5];

    let first = array_numeros[0];
    let second = array_numeros[1];
    println!("first vale: {first}");
    println!("second vale: {second}");

    // También puede inicializar una matriz para que contenga el mismo valor para cada elemento especificando el valor inicial, seguido de un punto y coma, y ​​luego la longitud de la matriz entre corchetes, como se muestra aquí:

    let new_array = [3; 5]; //esto devuelve una matriz con 5 elementos con el valor de 3 cada uno;
    println!("{}",new_array[3] );
    //example practico
    let tupla_example = ([1; 2], [3; 4]);
    //desestructuracion
    let (aa, _bb) = tupla_example;
  println!("el resultado de la operacion es:{}, y el valor de _bb es {} ", aa[0] + tupla_example.1[0], _bb[3]); 
}
