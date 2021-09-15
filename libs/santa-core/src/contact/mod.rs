pub trait ContactMethod {
    /// Get an immutable str ref to the value of the ContactMethod
    fn value(&self) -> &str;

    /// Deliver the message
    fn send_message(&self, msg: &str);
}

pub mod email;
pub mod phone;
