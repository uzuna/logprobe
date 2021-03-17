pub trait HelloMacro {
    fn hello_macro();
    fn hello_method(&self) -> String;
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
        fn hello_method(&self) -> String {
            "Hello, Macro! My name is Pancakes!".to_owned()
        }
    }



    #[derive(HelloMacro)]
    struct Hotcakes {
        a: i32,
    }

    fn check_bound<M: HelloMacro>(x: M) {
        println!("{}", x.hello_method())
    }

    #[test]
    fn pancakes(){
        Pancakes::hello_macro();
        check_bound(Pancakes{});
    }
    #[test]
    fn hotcakes(){
        Hotcakes::hello_macro();
        check_bound(Hotcakes{a:1});
    }
}