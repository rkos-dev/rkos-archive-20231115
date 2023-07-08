use crate::parser::{parser_json, PachageTemplete};

mod execute;
mod package_manage;
mod parser;

fn init_rkos_builder(config_store_path: String) {
    //取得默认shell的配置文件，写入当前工作目录
}

fn main() {
    let new_p: PachageTemplete = {
        let temp = parser_json(["config", "package_templete.json"].iter().collect());
        match temp {
            Ok(v) => v,
            Err(e) => {
                println!("erro {}", e);
                panic!("error ");
            }
        }
    };

    println!("new_P {}", new_p.file_name);
    println!("Hello, world!");
}
