mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

   pub mod inside {
       use outermost::middle_secret_function;

        pub fn inner_function() {
            middle_secret_function()
        }

        fn secret_function() {}
    }
}

#[test]
fn test() {
    outermost::middle_function();

    // https://doc.rust-lang.org/error-index.html#E0603
    // outermost::middle_secret_function not public
    // outermost::middle_secret_function();

    outermost::inside::inner_function();  

    // https://doc.rust-lang.org/error-index.html#E0603
    // outermost::inside::secret_function not public
    // outermost::inside::secret_function(); 
}
