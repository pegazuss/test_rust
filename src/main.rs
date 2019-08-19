//! test
fn main() {
    println!("Hello, world!");
    println!("let's begin some rust");

    test();

}
fn test()
{
    println!("{} days",29);
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond","James")

    /*#[allow(dead_code)]
    struct Structure(i32);

    println!("this struct'{}' won't print ...", Structure(3))*/


}
