/// The `read_cfg` module provides functionality to read and parse the configuration file for the Ruzky application.
///
/// This module contains functions and structs that handle the reading and parsing of the configuration file used by
/// the Ruzky application. It provides an interface to access and retrieve configuration values, allowing the
/// application to be configured based on the specified settings.
pub mod read_cfg;

/// The `init` module contains functions related to initializing the Ruzky application, such as setting up a new project from a template.
///
/// This module provides functions and utilities to initialize the Ruzky application, including creating a new project
/// structure based on a predefined template. It handles tasks such as copying template files, configuring project settings,
/// and preparing the initial setup for the application.
pub mod init;

/// The `start` module includes functions for starting and running the Ruzky application, such as starting the server.
///
/// This module provides functions and utilities to start and run the Ruzky application. It handles tasks such as starting
/// the server, establishing database connections, and initializing any required services or components. It encapsulates
/// the logic necessary to begin the execution of the application.
pub mod start;

/// The `templates` module handles the creation of JSON files containing static data, specifically templates for applications such as blogs, todos, and profiles.
///
/// This module provides functions and structures to generate JSON files that serve as templates for various types of applications. These templates can be used to provide initial data or structure for creating instances of blogs, todos, or profiles.
///
/// The `templates` module encapsulates the logic for creating and managing these JSON templates. It includes functions for generating the necessary JSON content with static data, defining the structure and fields of each template, and organizing the templates for easy access and usage.
pub mod templates;
