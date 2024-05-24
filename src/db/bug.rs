pub struct Bug {
    pub _bug_id: i64,
    pub _title: String,
    pub _description: String,
    pub _user_id: i64,
    pub _created_at: String,
}

impl Bug {
    pub fn _new() -> Self {
        // TODO: need this?
        Bug {
            _bug_id: -1,
            _title: String::new(),
            _description: String::new(),
            _user_id: -1,
            _created_at: String::new(),
        }
    }
}


pub struct InsertBug {
    pub title: String,
    pub description: String,
    pub user_id: i64,
}

impl InsertBug {
    pub fn new(title: &str, description: &str, user_id: i64) -> Self {
        InsertBug {
            title: title.to_string(),
            description: description.to_string(),
            user_id,
        }
    }
}

