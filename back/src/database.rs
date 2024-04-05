use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            host: String::from("127.0.0.1"),
            port: String::from("5432"),
            database: String::from("beevolution"),
            user: String::from("bee_user"),
            password: String::from("bee_password"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_database() {
        let db = Database::default();
        assert_eq!(db.host, String::from("127.0.0.1"));
        assert_eq!(db.port, String::from("5432"));
        assert_eq!(db.database, String::from("beevolution"));
        assert_eq!(db.user, String::from("bee_user"));
        assert_eq!(db.password, String::from("bee_password"));
    }
}
