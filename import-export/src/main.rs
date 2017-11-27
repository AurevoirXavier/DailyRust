extern crate import_export;

// 只导入`X!`这一个宏
#[macro_use(X)]

// X!(); // X!已被定义，但Y!未被定义

macro_rules! Y { () => {} }

X!(); // 均已被定义

fn main() {}
