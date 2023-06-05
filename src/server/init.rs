use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Result, Write};
use std::path::Path;
use std::process::exit;

use super::templates::setup_blog_template;

pub struct Template<'a> {
    pub file: &'a str,
    pub path: &'a str,
    pub content: &'a str,
}

/// Creates a file with the specified content.
///
/// # Arguments
///
/// * `file_path` - The path of the file to create.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation.
fn create_file_with_content(file_path: String, content: &str) -> Result<()> {
    let dir_path = Path::new(&file_path).parent().unwrap();

    // Create the directory if it doesn't exist
    fs::create_dir_all(dir_path)?;

    // Write the content to the file
    fs::write(&file_path, content)?;

    Ok(())
}

/// Appends a line to a file.
///
/// # Arguments
///
/// * `file_path` - The path of the file to append to.
/// * `line` - The line to append to the file.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation.
fn append_lines_to_file(file: String, line: String) -> Result<()> {
    // Open the file in write mode and append content to the end
    let mut options = OpenOptions::new();
    options.write(true).append(true);
    let file = options.open(file)?;

    let mut writer = BufWriter::new(file);

    // Write each line to the file
    writeln!(writer, "{}", line)?;

    writer.flush()?;
    Ok(())
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
        "blog" => setup_blog_template(),
        "todo" => println!("Using the todo template."),
        "profile" => println!("Using the profile template."),
        "default" => println!("Using the default template."),
        _ => {
            println!("Invalid template specified.");
            exit(1)
        }
    }
}

/// Retrieves the filenames in the specified directory.
///
/// # Arguments
///
/// * `directory` - The path of the directory to get the filenames from.
///
/// # Returns
///
/// A `Result` containing a vector of filenames if successful, or an error if the operation failed.
fn get_filenames(directory: String) -> Result<Vec<String>> {
    let mut file_names = Vec::new();

    for entry in fs::read_dir(directory)? {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                file_names.push(file_name.to_string());
            }
        }
    }

    Ok(file_names)
}

/// Initializes the Ruzky server with the specified directory.
///
/// # Arguments
///
/// * `dir` - The directory to use for initialization.
pub fn init_server_with_dir(dir: String) {
    let cfg = format!(r#"[server]
ip = "localhost"
port = 8080
base_dir = "{}"

"#, dir);

    let filenames = match get_filenames(dir) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("Error to get filenames: {}", error);
            exit(1)
        }
    };

    match create_file_with_content("./ruzky.toml".to_string(), &cfg) {
        Ok(()) => {}
        Err(error) => eprintln!("Error creating file: {}", error),
    }


    for file in &filenames {
        let path = match file.strip_suffix(".json") {
            Some(path) => path.to_string(),
            None => file.to_string(),
        };

        let toml_route = format!(
            r#"[[routes]]
path = "/{}"
file = "{}"
"#,
            path, file
        );

        match append_lines_to_file("./ruzky.toml".to_string(), toml_route) {
            Ok(()) => {}
            Err(error) => eprintln!("Error appending lines: {}", error),
        }

    }
}

/// Prints the details of each template in the given vector.
///
/// # Arguments
///
/// * `templates` - A vector of `Template` struct instances.
///
/// # Examples
///
/// ```
/// let templates = vec![...]; // Vector containing templates
/// make_template(templates);
/// ```
pub fn make_template(templates: Vec<Template>) {
    let cfg = r#"[server]
ip = "localhost"
port = 8080
base_dir = "./ruzky"

"#;

    match create_file_with_content("./ruzky.toml".to_string(), cfg) {
        Ok(()) => {}
        Err(error) => eprintln!("Error creating file: {}", error),
    }

    for template in &templates {
        match create_file_with_content(format!("./ruzky/{}", template.file), template.content) {
            Ok(()) => {}
            Err(error) => eprintln!("Error creating file: {}", error),
        }

        let toml_route = format!(
            r#"[[routes]]
path = "{}"
file = "{}"
"#,
            template.path, template.file
        );

        match append_lines_to_file("./ruzky.toml".to_string(), toml_route) {
            Ok(()) => {}
            Err(error) => eprintln!("Error appending lines: {}", error),
        }
    }
}
