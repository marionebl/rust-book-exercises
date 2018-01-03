mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // https://doc.rust-lang.org/error-index.html#E0603
    // outermost::middle_secret_function not public
    // outermost::middle_secret_function();

    // https://doc.rust-lang.org/error-index.html#E0603
    // outermost::inside not public
    // outermost::inside::inner_function();  

    // https://doc.rust-lang.org/error-index.html#E0603
    // outermost::inside::secret_function not public
    // outermost::inside::secret_function(); 
}
