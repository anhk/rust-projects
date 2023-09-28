pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl User {
    pub fn string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.active, self.username, self.email, self.sign_in_count
        )
    }
}
