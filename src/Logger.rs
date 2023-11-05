pub fn info(message: &str, prefix: bool) {
    if prefix {
        println!("\x1b[37m [INFO] {}\x1b[0m", message);
    } else {
        println!("\x1b[37m {}\x1b[0m", message);
    }
    
}

pub fn warn(message: &str, prefix: bool) {
    if prefix {
        println!("\x1b[33m [WARNING] {}\x1b[0m", message);
    } else {
        println!("\x1b[33m {}\x1b[0m", message); 
    }
    
}

pub fn error(message: &str, prefix: bool) {
    if prefix {
        println!("\x1b[31m [ERROR] {}\x1b[0m", message);
    } else {
        println!("\x1b[31m {}\x1b[0m", message);
    }
    
}

pub fn success(message: &str, prefix: bool) {
    if prefix {
        println!("\x1b[32m [SUCCESS] {}\x1b[0m", message);
    } else {
        println!("\x1b[32m {}\x1b[0m", message);
    }
}