use std::collections::HashMap;
use std::hash::Hash;

pub fn impl_main() {
    let mut user = User::new(1, "john_doe".to_string(), "john@gmail.com".to_string());
    user.deactivate();
    user.display();

    let is_email_valid = user.is_valid_email();
    println!("Is email valid? {}", is_email_valid);

    // 1. Create a new user repository
    let mut user_repo = UserRepository::new();

    match user_repo.create_user("alice".to_string(), "alice@example.com".to_string()) {
        Ok(id) => println!("Created user Alice with ID: {}", id),
        Err(e) => println!("Error creating Alice: {}", e),
    }

    let bot_id = user_repo.create_user("bot".to_string(), "bob@comopany.com".to_string()).unwrap();
    println!("Created user Bot with ID: {}", bot_id);

    if let Some(user) = user_repo.get_user(1) {
        println!("Found user: {:?}", user.display());
        println!("User details: {:?}", user);
    } else {
        println!("User with ID 1 not found");
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: u64,
}

impl User {
     pub fn new(id: u32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
            is_active: true,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn is_valid_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.')
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn display(&self) {
        println!(
            "User ID: {}, Username: {}, Email: {}, Active: {}, Created At: {}",
            self.id, self.username, self.email, self.is_active, self.created_at
        );
    }

    pub fn update_email(&mut self, new_email: &str) -> Result<(), String> {
        if new_email.contains('@') && new_email.contains('.') {
            self.email = new_email.to_string();
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
}

pub struct UserRepository {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserRepository {
 pub fn new() -> Self {
        UserRepository {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_user(&mut self, username: String, email: String) -> Result<u32, String> {
        let user = User::new(self.next_id, username, email);

        if !user.is_valid_email() {
            return Err("Invalid email format".to_string());
        }

        let id = self.next_id;
        self.users.insert(id, user);
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn get_user_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.get_mut(&id)
    }

    pub fn delete_user(&mut self, id: u32) -> bool {
      self.users.remove(&id).is_some()
    }

    pub fn get_active_users(&self) -> Vec<&User> {
        self.users.values().filter(|user| user.is_active).collect()
    }
}


