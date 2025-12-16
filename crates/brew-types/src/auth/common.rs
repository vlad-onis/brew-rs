use tracing::debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    // todo: this has to be validated
    pub fn new(email: &str) -> Email {
        debug!("Email has been validated");

        Email(email.to_string())
    }
}

impl From<String> for Email {
    fn from(email: String) -> Self {
        debug!("Email has been validated");
        Email(email)
    }
}

impl From<&str> for Email {
    fn from(email: &str) -> Self {
        debug!("Email has been validated");
        Email(email.to_string())
    }
}

impl From<&Email> for String {
    fn from(email: &Email) -> Self {
        email.0.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Password {
        // todo: strength check the password
        // CAVEAT: if you check it here and later introduce more rules existing passwords will break
        Password(password)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
