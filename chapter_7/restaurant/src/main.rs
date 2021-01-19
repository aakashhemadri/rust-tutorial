#[allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::fmt::Result;
    use std::io::Result as IoResult;
    let mut map = HashMap::new();
    map.insert(1, 2);

    // fn function0() -> io::Result {
    //     // --snip--
    // }

    // fn function1() -> Result {
    //     // --snip--
    // }

    // fn function2() -> IoResult<()> {
    // }
}
