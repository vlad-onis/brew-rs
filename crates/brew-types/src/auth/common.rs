#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    // todo: this has to be validated
    pub fn new(email: &str) -> Email {
        Email(email.to_string())
    }
}

impl From<String> for Email {
    fn from(email: String) -> Self {
        Email(email)
    }
}

impl From<&str> for Email {
    fn from(email: &str) -> Self {
        Email(email.to_string())
    }
}

impl From<&Email> for String {
    fn from(email: &Email) -> Self {
        email.0.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Password {
        Password(password)
    }
}
