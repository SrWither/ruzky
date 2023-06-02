/// Initializes the Ruzky server with the specified template.
///
/// # Arguments
///
/// * `template` - A string representing the template to use for initialization.
///
/// # Examples
///
/// ```
/// init_server("blog".to_string());
/// init_server("default".to_string());
/// ```
pub fn init_server(template: String) {
    match template.to_lowercase().as_str() {
        "blog" => println!("Using the blog template."),
        "todo" => println!("Using the todo template."),
        "profile" => println!("Using the profile template."),
        "default" => println!("Using the default template."),
        _ => println!("Invalid template specified."),
    }
}
