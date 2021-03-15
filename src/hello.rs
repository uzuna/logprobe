

// trait
pub trait HelloMacro {
    fn hello_macro();
}


#[cfg(test)]
mod tests {
    use crate::hello::HelloMacro;
    struct Pancakes;
    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!")
        }
    }

    #[test]
    fn pancakes(){
        Pancakes::hello_macro();
    }
}