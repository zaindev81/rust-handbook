use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::User;

#[drive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<u32, User>>>,
    pub request_count: Arc<RwLock<u64>>,
    pub start_time: std::time::Instant,
}

impl AppState {
    pub fn new() -> Self {
        let mut users = HashMap::new();

        users.insert(1, User {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            role: "admin".to_string(),
            active: true,
            created_at: "2023-01-15T10:30:00Z".to_string(),
        });

        users.insert(2, User {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            role: "user".to_string(),
            active: true,
            created_at: "2023-02-20T14:15:00Z".to_string(),
        });

        users.insert(3, User {
            id: 3,
            name: "Charlie Brown".to_string(),
            email: "charlie@example.com".to_string(),
            role: "user".to_string(),
            active: false,
            created_at: "2023-03-10T09:45:00Z".to_string(),
        });

        Self {
            users: Arc::new(RwLock::new(users)),
            request_count: Arc::new(RwLock::new(0)),
            start_time: std::time::Instant::now(),
        }
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }
}