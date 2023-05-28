fn main() {
    println!("Hello, world!");
    println!("你好，世界");
    hello_world();
}
fn hello_world(){
    let chinse = "你好，rust!";
    let english = "hello, rust!";
    let luas = [chinse, english];
    for lua in luas{
        println!("{},{}", lua, "真的是这样的吗");
    }
    
}