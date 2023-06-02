pub fn init_server(template: String) {
    match template.to_lowercase().as_str() {
        "blog" => println!("blog template elegido"),
        "todo" => println!("todo template elegido"),
        "profile" => println!("profile template elegido"),
        "default" => println!("usando default template"),
        _ => println!("template no existente")
    }
}
