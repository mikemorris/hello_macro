pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::HelloMacro;
        use hello_macro_derive::HelloMacro;

        #[derive(HelloMacro)]
        struct Pancakes;

        fn main() {
            Pancakes::hello_macro();
        }
    }
}
