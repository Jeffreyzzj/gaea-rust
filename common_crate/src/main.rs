use log::info;
use log4rs;

fn main() {
    //println!("Hello, world!");
    //初始化日志信息
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    
}
