// What Is Ownership?
//
//Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

// //Ownership Rules
// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
fn main() {
        //Variable scope:

        {                      // s is not valid here, it’s not yet declared
            let s = "hello";   // s is valid from this point forward
    
            // do stuff with s
            
            println!("{}",s)//la variable solo es accedible dentro de su alcance
        }                      // this scope is now over, and s is no longer valid
        

        //The String Type: 

        //hay una diferencia de uso y de practicidad entre un stringe literal y un STRING TYPE; el string type es mas dinamico y permite mas funcionalidades

        // You can create a String from a string literal using the from function, like so:

        // let s = String::from("hello");
        
    // The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from

    //This kind of string can be mutated:

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
    //So, what’s the difference here? Why can String be mutated but literals cannot? The difference is how these two types deal with memory.

}
