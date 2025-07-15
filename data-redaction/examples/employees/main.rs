//! Shows how redaction is intended to be used to protect sensitive data in telemetry.
//!
//! A given application uses a specific data taxonomy appropriate for its context.
//! Each company or government can have its own taxonomy that defines the types of
//! data the organization recognizes.
//!
//! The redaction framework exists to help prevent sensitive information from being
//! leaked into an application's telemetry. For example, it's generally not a good idea to
//! emit the user's identity in a cloud service's log.
//!
//! Redaction is different from deletion. The redaction framework replaces sensitive data
//! with something else. Often in production, sensitive data is replaced with a hash value.
//! The hash value is not reversible, so the sensitive data cannot be recovered from it.
//! However, having a consistent hash value for a given piece of sensitive data enables correlation
//! across multiple independent log entries in a telemetry system. So for example, although you might
//! now know which user is experiencing problems, you can tell that a specific user is experiencing problems
//! and can track what that user has been doing to get into trouble.
//!
//! In this example, we do the following:
//!
//! * Create a custom taxonomy. Normally, an application would typically use a taoxnomy provided by their company to be
//!   used across multiple applications, but here we're doing it stand-alone for the sake of the example.
//!
//! * Initialize a RedactionEngine. The engine controls which redaction logic to apply to individual classes of data.
//!   Although this is being hardcoded in this example, the specific redaction algorithm to use for a given data class
//!   should typically be control through external configuration state that the application consumes.
//!
//! * Once the redaction engine is initialized, it is handed over to this application's logging system. This is a made-up
//!   piece of code standing in for whatever logging framework the application uses for logging.
//!
//! * The application does its business and emits logs along the way. The logging system then redacts this data so that the
//!   log output by the application doesn't contain any sensitive information.

mod employee;
mod example_taxonomy;
mod logging;

use data_redaction::{xxH3Redactor, RedactionEngineBuilder};
use std::fs::{OpenOptions, File};
use std::io::{self, Write, BufReader};
use example_taxonomy::*;
use employee::*;
use logging::{set_redaction_engine_for_logging, log};

fn main() {
    // First step, we create a redaction engine that prescribes how to redact individual data classes.
    // Normally, the specific algorithm to adopt for a given data class would be controlled by external configuration,
    // but for the sake of this example, we hardcode it.
    //
    // If at runtime, an unconfigured data class is encountered, then the data just
    // gets erased, so it is not logged at all, avoiding a potential privacy leak.
    let engine = RedactionEngineBuilder::new()
        .add_class_redactor(
            &ExampleTaxonomy::PersonallyIdentifiableInformation.data_class(),
            Box::new(xxH3Redactor::with_secret(vec![0; 192])),
        )
        .add_class_redactor(
            &ExampleTaxonomy::OrganizationallyIdentifiableInformation.data_class(),
            Box::new(data_redaction::SimpleRedactor::with_mode(
                data_redaction::SimpleRedactorMode::PassthroughAndTag),
            )
        )
        .build();

    // now configure the logging system to use the redaction engine
    set_redaction_engine_for_logging(engine);

    // now go run the app's business logic
    app_loop();
}

fn app_loop() {
    let json_path = "employees.json";
    let mut employees: Vec<Employee> = if let Ok(file) = File::open(json_path) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        Vec::new()
    };

    loop {
        println!("Enter employee info (or type 'quit' to exit):");
        let mut input = String::new();
        print!("Name: ");
        io::stdout().flush().unwrap();
        _ = io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        if name.eq_ignore_ascii_case("quit") { break; }
        input.clear();

        print!("Address: ");
        io::stdout().flush().unwrap();
        _ = io::stdin().read_line(&mut input).unwrap();
        let address = input.trim().to_string();
        input.clear();

        print!("Employee ID: ");
        io::stdout().flush().unwrap();
        _ = io::stdin().read_line(&mut input).unwrap();
        let employee_id = input.trim().to_string();
        input.clear();

        print!("Age: ");
        io::stdout().flush().unwrap();
        _ = io::stdin().read_line(&mut input).unwrap();
        let age: u32 = match input.trim().parse() {
            Ok(a) => a,
            Err(_) => {
                println!("Invalid age, try again.");
                continue;
            }
        };

        let employee = Employee {
            name: PersonallyIdentifiableInformation::new(name),
            address: PersonallyIdentifiableInformation::new(address),
            employee_id: OrganizationallyIdentifiableInformation::new(employee_id),
            age,
        };
        employees.push(employee.clone());

        let file = OpenOptions::new().write(true).create(true).truncate(true).open(json_path).unwrap();
        serde_json::to_writer_pretty(file, &employees).unwrap();
        println!("Employee added.\n");

        // Here we log the employee creation event. OUr little logging framework takes as input a set of name/value pairs that provide
        // a structured log record.
        //
        // By default, the provided values are serialized to string using the `Display` trait. We can use the `:?` syntax to serialize a value
        // using the `Debug` trait and use `:@` to use the `Extract` trait which will redact the data before emitting the log record.
        log!(event = "Employee created",
             name:@ = employee.name,
             address:@ = employee.address,
             employee_id:@ = employee.employee_id,
             age = employee.age);
    }
}
