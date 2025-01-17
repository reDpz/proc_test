use list_comp_macro::comp;
fn main() {
    let list = comp![x in x <- 0..10, x%2 == 0].collect::<Vec<_>>();
    println!("{:?}", list)
}
