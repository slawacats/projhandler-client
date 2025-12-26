use std::env;


fn init() {
    
}

fn load() {
    println!("Загрузка проекта...")


}

fn save() {
    println!("Сохранение проекта...")


}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "load" => load(),
            "save" => save(),
            "init" => init(),
            _ => println!("Неизвестная команда: {}", command),
        }
    } else {
        println!("Использование: projhandler <load|save|init>");
    }
}
