const MAX_POINT : u32 = 100000;

fn main() {
    //定义变量用let  后置类型 而且要有初始值

    
    //let a;
    let a = 1;
    let mut b : u32 = 1;//变量默认（就是）不可变变量，加mut才可变
    println!("a = {}", a);
    println!("b = {}", b);
    b = 2;
    println!("b = {}", b);
    // 隐藏性
    // 定义的重名变量  后面的变量覆盖前面的变量 前面的就隐藏了
    let b: f32 = 1.1;
    println!("b = {}", b);
    //常量
    println!("MAX_POINT = {}", MAX_POINT);
}
