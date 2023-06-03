use std::process::exit;

struct Template<'a> {
    file: &'a str,
    path: &'a str,
    content: &'a str,
}

/// Initializes the Ruzky server with the specified template.
///
/// # Arguments
///
/// * `template` - A string representing the template to use for initialization.
///
/// # Examples
///
/// ```
/// init_server("blog");
/// init_server("default");
/// ```
pub fn init_server(template: &str) {
    match template.to_lowercase().as_str() {
        "blog" => println!("Using the blog template."),
        "todo" => println!("Using the todo template."),
        "profile" => println!("Using the profile template."),
        "default" => println!("Using the default template."),
        _ => {
            println!("Invalid template specified.");
            exit(0)
        },
    }
}

fn setup_blog_template() {
    let template: Vec<Template> = vec![
        Template {
            path: "/",
            file: "index.json",
            content: "nothing..."
        }
    ];
}
