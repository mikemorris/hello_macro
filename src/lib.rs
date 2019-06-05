pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_derive() {
        use crate::HelloMacro;
        use hello_macro_derive::HelloMacro;

        #[derive(HelloMacro)]
        struct Pancakes;

        Pancakes::hello_macro();
    }

    #[test]
    #[ignore]
    fn prints_name() {
        // assert that printed name matches expected
    }
}
