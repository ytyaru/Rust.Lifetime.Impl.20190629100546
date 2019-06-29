/*
 * Rustのライフタイム省略規則。
 * CreatedAt: 2019-06-29
 */
fn main() {
    let s = S { field: "A" };
    println!("{}", s.method1("B"));
}
#[derive(Debug)]
struct S<'a> {
    field: &'a str,
}
impl<'a> S<'a> {
    fn level(&self) -> i32 { 3 }
    fn get_field(&self) -> &str { self.field }
    fn method1(&self, p1: &str) -> &str {
        println!("{}", p1);
        self.field
    }
}

