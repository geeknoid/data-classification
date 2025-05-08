use crate::Redactor;
use data_classification::ClassId;

/// Mode of operation for the `SimpleRedactor`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimpleRedactorMode {
    /// Erases the original string.
    Erase,

    /// Erases the original string and tags it with the class id.
    EraseAndTag,

    /// Passes the original string through without modification.
    Passthrough,

    /// Passes the original string through and tags it with the class id.
    PassthroughAndTag,

    /// Replaces the original string with a repeated character.
    Replace(char),

    /// Replaces the original string with a repeated character and tags it with the class id.
    ReplaceAndTag(char),

    /// Inserts a custom string in place of the original string.
    Insert(String),

    /// Inserts a custom string in place of the original string and tags it with the class id.
    InsertAndTag(String),
}

/// A redactor that performs a variety of simple transformations on the input text.
#[derive(Clone)]
pub struct SimpleRedactor {
    mode: SimpleRedactorMode,
}

impl SimpleRedactor {
    /// Creates a new instance with the default mode of `SimpleRedactorMode::Replace('*')`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: SimpleRedactorMode::Replace('*'),
        }
    }

    /// Creates a new instance with an explicit mode of operation.
    #[must_use]
    pub const fn with_mode(mode: SimpleRedactorMode) -> Self {
        Self { mode }
    }
}

impl Redactor for SimpleRedactor {
    fn redact<'a>(&self, class_id: &ClassId, value: &str, output: &'a mut dyn FnMut(&str)) {
        static ASTERISKS: &str = "********************************";

        match &self.mode {
            SimpleRedactorMode::Erase => {
                // nothing
            }
            SimpleRedactorMode::EraseAndTag => {
                output(format!("<{class_id}:>").as_str());
            }
            SimpleRedactorMode::Passthrough => {
                output(value);
            }
            SimpleRedactorMode::PassthroughAndTag => {
                output(format!("<{class_id}:{value}>").as_str());
            }
            SimpleRedactorMode::Replace(c) => {
                let len = value.len();
                if *c == '*' && len < ASTERISKS.len() {
                    output(&ASTERISKS[0..len]);
                } else {
                    output(c.to_string().repeat(len).as_str());
                }
            }
            SimpleRedactorMode::ReplaceAndTag(c) => {
                let len = value.len();
                if *c == '*' && len < ASTERISKS.len() {
                    output(format!("<{class_id}:{}>", &ASTERISKS[0..len]).as_str());
                } else {
                    output(
                        format!("<{class_id}:{}>", (*c.to_string()).repeat(len).as_str()).as_str(),
                    );
                }
            }
            SimpleRedactorMode::Insert(s) => {
                output(s.as_str());
            }
            SimpleRedactorMode::InsertAndTag(s) => {
                output(format!("<{class_id}:{s}>").as_str());
            }
        }
    }
}

impl Default for SimpleRedactor {
    fn default() -> Self {
        Self::new()
    }
}
