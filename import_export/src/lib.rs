mod macros {
    #[macro_export] macro_rules! X { () => { Y!(); } }
    #[macro_export] macro_rules! Y { () => {} }
}

// X!和Y!并非在此处定义的，但它们的确被
// 导出了，即便macros并非pub。