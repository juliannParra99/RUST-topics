fn main() {
    let number = 3;

    if number < 5 { //la condicion siempre tiene que dar como resultado un bool sino da error
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 3;

    if number2 != 0 {
        println!("number was something other than zero");
    }

    let number3 = 6;
    
    //si tenemos mas de un else if es usual refactorizar el codigo; lo vemos mas adelante con una herramienta llamada MATCH , en cap 6
    if number3 % 4 == 0 {
        println!("number3 is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number3 is divisible by 3");//ejecuta el 1er cuerpo en que da verdadera la condicion
    } else if number3 % 2 == 0 {
        println!("number3 is divisible by 2");
    } else {
        println!("number3 is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number4 = if condition { 5 } else { 6 }; // los valores que tienen el potencial de ser resultados de cada brazo del ifdeben ser del mismo tipo; sino da error
    
    println!("The value of number4 is: {number4}");
    
    //EJERCICIO RARO
    let x = true;
    let y = if x {};
    println!("{y:?}"); //el ':?' y el :'#?' se usa para que pueda mostrarse un valor vacio sin un error; el valor por defecto si no asignamos nada a la variable como en este caso es 2 parentesis

    //Repetition with Loops : Bucles of Rust: loop, whiley for
    println!("LOOPS DE RUST");
    //Repeating Code with loop:La looppalabra clave le dice a Rust que ejecute un bloque de código una y otra vez para siempre o hasta que le digas explícitamente que se detenga.

    // loop {
    //     println!("again!"); Esto genera un bucle continuo
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //break para cortar el ciclo y devolver el valor despues de la palabra
        }
    };//Después del bucle, usamos un punto y coma para finalizar la instrucción que asigna el valor a result.

    println!("The result is {result}");

    //Etiquetas de bucle para desambiguar entre múltiples bucles
    //Si tiene bucles dentro de bucles, break y continue aplíquelo al bucle más interno en ese punto. Opcionalmente, puede especificar una etiqueta de bucle en un bucle que luego podemos usar con break o continue para especificar que esas palabras clave se aplican al bucle etiquetado en lugar del bucle más interno. Las etiquetas de bucle deben comenzar con una comilla simple. Aquí hay un ejemplo con dos bucles anidados:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10; //cada vez que el ciclo interno da una vuelta, se vuelve a asignar 10 a remaining, osea, volvio al valor original

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //ya no se ejecuta el ciclo interno pq se cierra el externo
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    //El ciclo externo tiene la etiqueta 'counting_upy contará de 0 a 2. El ciclo interno sin etiqueta cuenta hacia atrás de 10 a 9. El primero break que no especifica una etiqueta solo saldrá del ciclo interno. La break 'counting_up;instrucción saldrá del ciclo externo. Este código imprime:

    //Bucles condicionales con while
    println!("Bucles con while");
    // Mientras la condición es verdadera, el bucle se ejecuta. Cuando la condición deja de ser verdadera, el programa llama breaky detiene el ciclo.
    let mut number_while = 3; //ESTE MISMO EJEMPLO SE REPITE CON UN FOR Y UN RANGE MAS ABAJO; SE LEERA MAS FACIL

    while number_while != 0 {
        println!("{number_while}!");

        number_while -= 1;
    }

    println!("LIFTOFF!!!");
    //You can choose to use the while construct to loop over the elements of a collection, such as an array:while para array 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0; 

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    //esta manera es propensa a errores, por ejemplo si cambiamos el tamaño de la matriz y olvidamos ajustar el index; ademas consume tiempo de ejecucion verificar si el indice se ajusta a la logitud del array; por eso para realizar la misma accion es mejor utilizar un FOR

    //Bucle a través de una colección con for
    println!("Bucles con for"); //es el que mas vamos a utilizar, inclusive para realizar acciones que podriamos con while;aumentamos la seguridad del código y eliminamos la posibilidad de errores que podrían resultar de ir más allá del final de la matriz o no ir lo suficientemente lejos y perder algunos elementos.

    let d = [100, 200, 300, 400, 500];

    for element in d {
        println!("the value is: {element}");
    }

    //ejemplo con un RANGE, para realziar una cuenta regresiva.
    // Range, provisto por la biblioteca estándar, que genera todos los números en secuencia comenzando desde un número y terminando antes de otro número
    for number in (1..4).rev() {
        println!("Numero del rango: {number}!");
    }
    println!("LIFTOFF!!!");

    //ejercicio
    println!("ejercicio");
    let t = [5; 10];
    let mut sum = 0;
    for x in t {
      sum += x;
    }
    println!("El resultado es :{sum}");

    println!("ejemplo raro que puede servir para pensar");
    let mut xx = 0;
    'a: loop {
      xx += 1;
      'b: loop {
        if xx > 10 {
          continue 'a;
        } else {
          break 'b;
        }      
      }
      break println!("{xx}");       
    }

}


