pub trait HelloMacro {
    fn hello_macro();
}


#[cfg(test)]
mod tests {
    use crate::HelloMacro;
    use hello_derive::HelloMacro;
    struct Pancakes;
    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!")
        }
    }



    #[derive(HelloMacro)]
    struct Hotcakes;

    #[test]
    fn pancakes(){
        Pancakes::hello_macro();
    }
    #[test]
    fn hotcakes(){
        Hotcakes::hello_macro();
    }
}