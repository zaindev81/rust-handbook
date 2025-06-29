use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub active: bool,
    pub created_at: Option<String>, // Optional field for created_at timestamp
}

impl User {
    pub fn new(id: u32, name: &str, email: &str, role: &str) -> Self {
        User {
            id,
            name,
            email,
            role,
            active,
            created_at: chrono::Utc::now().to_rfc3339().into(), // Set created_at to current time
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerMetrics {
    pub total_requests: u64,
    pub total_users: usize,
    pub memory_usage_mb: f64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthChecks {
    pub database: String,
    pub memory: String,
    pub disk: String,
}