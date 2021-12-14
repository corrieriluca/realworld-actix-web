/// This struct represents an User as stored in the database (without
/// the table's unique ID and the password, these can be retrieved with
/// different requests).
pub struct User {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
