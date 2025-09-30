use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

/// Database for persisting tasks, orders, and sessions
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

/// Task record for database persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRecord {
    pub id: i64,
    pub task_id: u64,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Order record for database persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRecord {
    pub id: i64,
    pub order_id: String,
    pub product_id: String,
    pub account_id: String,
    pub status: String,
    pub price: f64,
    pub quantity: i32,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Session record for database persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: i64,
    pub session_id: String,
    pub account_id: String,
    pub status: String,
    pub cookies: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Database {
    /// Create a new database instance
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = db_path.as_ref().to_path_buf();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create database directory")?;
        }

        let conn = Connection::open(&db_path).context("Failed to open database connection")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        };

        db.initialize()?;
        info!("Database initialized at {:?}", db.db_path);

        Ok(db)
    }

    /// Create an in-memory database for testing
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to open in-memory database")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path: PathBuf::from(":memory:"),
        };

        db.initialize()?;
        info!("In-memory database initialized");

        Ok(db)
    }

    /// Initialize database schema
    fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Create tasks table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_id INTEGER NOT NULL UNIQUE,
                status TEXT NOT NULL,
                started_at TEXT,
                completed_at TEXT,
                error_message TEXT,
                metadata TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .context("Failed to create tasks table")?;

        // Create index on task_id for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_task_id ON tasks(task_id)",
            [],
        )
        .context("Failed to create index on tasks")?;

        // Create index on status for filtering
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
            [],
        )
        .context("Failed to create index on task status")?;

        // Create orders table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS orders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                order_id TEXT NOT NULL UNIQUE,
                product_id TEXT NOT NULL,
                account_id TEXT NOT NULL,
                status TEXT NOT NULL,
                price REAL NOT NULL,
                quantity INTEGER NOT NULL,
                metadata TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .context("Failed to create orders table")?;

        // Create index on order_id for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_order_id ON orders(order_id)",
            [],
        )
        .context("Failed to create index on orders")?;

        // Create index on account_id for filtering by account
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_account_id ON orders(account_id)",
            [],
        )
        .context("Failed to create index on order account_id")?;

        // Create sessions table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL UNIQUE,
                account_id TEXT NOT NULL,
                status TEXT NOT NULL,
                cookies TEXT,
                last_used_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .context("Failed to create sessions table")?;

        // Create index on session_id for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_session_id ON sessions(session_id)",
            [],
        )
        .context("Failed to create index on sessions")?;

        // Create index on account_id for filtering by account
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_account_id ON sessions(account_id)",
            [],
        )
        .context("Failed to create index on session account_id")?;

        debug!("Database schema initialized successfully");
        Ok(())
    }

    // ============================================
    // Task CRUD Operations
    // ============================================

    /// Insert a new task record
    pub fn insert_task(&self, task_id: u64, status: &str, metadata: Option<&str>) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tasks (task_id, status, metadata, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![task_id, status, metadata, now, now],
        )
        .context("Failed to insert task")?;

        let id = conn.last_insert_rowid();
        debug!("Inserted task with id={}, task_id={}", id, task_id);
        Ok(id)
    }

    /// Update task status and timestamps
    pub fn update_task_status(
        &self,
        task_id: u64,
        status: &str,
        started_at: Option<DateTime<Utc>>,
        completed_at: Option<DateTime<Utc>>,
        error_message: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let started_str = started_at.map(|t| t.to_rfc3339());
        let completed_str = completed_at.map(|t| t.to_rfc3339());

        conn.execute(
            "UPDATE tasks 
             SET status = ?1, started_at = ?2, completed_at = ?3, error_message = ?4, updated_at = ?5
             WHERE task_id = ?6",
            params![status, started_str, completed_str, error_message, now, task_id],
        ).context("Failed to update task status")?;

        debug!("Updated task_id={} to status={}", task_id, status);
        Ok(())
    }

    /// Get task by task_id
    pub fn get_task(&self, task_id: u64) -> Result<Option<TaskRecord>> {
        let conn = self.conn.lock().unwrap();

        let result = conn
            .query_row(
                "SELECT id, task_id, status, started_at, completed_at, error_message, metadata, created_at, updated_at
                 FROM tasks WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok(TaskRecord {
                        id: row.get(0)?,
                        task_id: row.get(1)?,
                        status: row.get(2)?,
                        started_at: row.get::<_, Option<String>>(3)?.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
                        completed_at: row.get::<_, Option<String>>(4)?.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
                        error_message: row.get(5)?,
                        metadata: row.get(6)?,
                        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?).unwrap().with_timezone(&Utc),
                        updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?).unwrap().with_timezone(&Utc),
                    })
                },
            )
            .optional()
            .context("Failed to query task")?;

        Ok(result)
    }

    /// Get all tasks with optional status filter
    pub fn get_tasks(&self, status_filter: Option<&str>) -> Result<Vec<TaskRecord>> {
        let conn = self.conn.lock().unwrap();

        let query = if let Some(status) = status_filter {
            format!("SELECT id, task_id, status, started_at, completed_at, error_message, metadata, created_at, updated_at
                     FROM tasks WHERE status = '{}' ORDER BY created_at DESC", status)
        } else {
            "SELECT id, task_id, status, started_at, completed_at, error_message, metadata, created_at, updated_at
             FROM tasks ORDER BY created_at DESC".to_string()
        };

        let mut stmt = conn.prepare(&query)?;
        let tasks = stmt
            .query_map([], |row| {
                Ok(TaskRecord {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    status: row.get(2)?,
                    started_at: row.get::<_, Option<String>>(3)?.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                    }),
                    completed_at: row.get::<_, Option<String>>(4)?.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                    }),
                    error_message: row.get(5)?,
                    metadata: row.get(6)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .unwrap()
                        .with_timezone(&Utc),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tasks)
    }

    /// Delete a task by task_id
    pub fn delete_task(&self, task_id: u64) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute("DELETE FROM tasks WHERE task_id = ?1", params![task_id])
            .context("Failed to delete task")?;

        debug!("Deleted task_id={}", task_id);
        Ok(())
    }

    // ============================================
    // Order CRUD Operations
    // ============================================

    /// Insert a new order record
    pub fn insert_order(
        &self,
        order_id: &str,
        product_id: &str,
        account_id: &str,
        status: &str,
        price: f64,
        quantity: i32,
        metadata: Option<&str>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO orders (order_id, product_id, account_id, status, price, quantity, metadata, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![order_id, product_id, account_id, status, price, quantity, metadata, now, now],
        ).context("Failed to insert order")?;

        let id = conn.last_insert_rowid();
        debug!("Inserted order with id={}, order_id={}", id, order_id);
        Ok(id)
    }

    /// Update order status
    pub fn update_order_status(&self, order_id: &str, status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE orders SET status = ?1, updated_at = ?2 WHERE order_id = ?3",
            params![status, now, order_id],
        )
        .context("Failed to update order status")?;

        debug!("Updated order_id={} to status={}", order_id, status);
        Ok(())
    }

    /// Get order by order_id
    pub fn get_order(&self, order_id: &str) -> Result<Option<OrderRecord>> {
        let conn = self.conn.lock().unwrap();

        let result = conn
            .query_row(
                "SELECT id, order_id, product_id, account_id, status, price, quantity, metadata, created_at, updated_at
                 FROM orders WHERE order_id = ?1",
                params![order_id],
                |row| {
                    Ok(OrderRecord {
                        id: row.get(0)?,
                        order_id: row.get(1)?,
                        product_id: row.get(2)?,
                        account_id: row.get(3)?,
                        status: row.get(4)?,
                        price: row.get(5)?,
                        quantity: row.get(6)?,
                        metadata: row.get(7)?,
                        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?).unwrap().with_timezone(&Utc),
                        updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?).unwrap().with_timezone(&Utc),
                    })
                },
            )
            .optional()
            .context("Failed to query order")?;

        Ok(result)
    }

    /// Get orders by account_id
    pub fn get_orders_by_account(&self, account_id: &str) -> Result<Vec<OrderRecord>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, order_id, product_id, account_id, status, price, quantity, metadata, created_at, updated_at
             FROM orders WHERE account_id = ?1 ORDER BY created_at DESC"
        )?;

        let orders = stmt
            .query_map(params![account_id], |row| {
                Ok(OrderRecord {
                    id: row.get(0)?,
                    order_id: row.get(1)?,
                    product_id: row.get(2)?,
                    account_id: row.get(3)?,
                    status: row.get(4)?,
                    price: row.get(5)?,
                    quantity: row.get(6)?,
                    metadata: row.get(7)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                        .unwrap()
                        .with_timezone(&Utc),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(orders)
    }

    /// Delete an order by order_id
    pub fn delete_order(&self, order_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute("DELETE FROM orders WHERE order_id = ?1", params![order_id])
            .context("Failed to delete order")?;

        debug!("Deleted order_id={}", order_id);
        Ok(())
    }

    // ============================================
    // Session CRUD Operations
    // ============================================

    /// Insert a new session record
    pub fn insert_session(
        &self,
        session_id: &str,
        account_id: &str,
        status: &str,
        cookies: Option<&str>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO sessions (session_id, account_id, status, cookies, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![session_id, account_id, status, cookies, now, now],
        ).context("Failed to insert session")?;

        let id = conn.last_insert_rowid();
        debug!("Inserted session with id={}, session_id={}", id, session_id);
        Ok(id)
    }

    /// Update session status and last_used_at
    pub fn update_session(
        &self,
        session_id: &str,
        status: &str,
        cookies: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE sessions 
             SET status = ?1, cookies = ?2, last_used_at = ?3, updated_at = ?4
             WHERE session_id = ?5",
            params![status, cookies, now, now, session_id],
        )
        .context("Failed to update session")?;

        debug!("Updated session_id={} to status={}", session_id, status);
        Ok(())
    }

    /// Get session by session_id
    pub fn get_session(&self, session_id: &str) -> Result<Option<SessionRecord>> {
        let conn = self.conn.lock().unwrap();

        let result = conn
            .query_row(
                "SELECT id, session_id, account_id, status, cookies, last_used_at, created_at, updated_at
                 FROM sessions WHERE session_id = ?1",
                params![session_id],
                |row| {
                    Ok(SessionRecord {
                        id: row.get(0)?,
                        session_id: row.get(1)?,
                        account_id: row.get(2)?,
                        status: row.get(3)?,
                        cookies: row.get(4)?,
                        last_used_at: row.get::<_, Option<String>>(5)?.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
                        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?).unwrap().with_timezone(&Utc),
                        updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?).unwrap().with_timezone(&Utc),
                    })
                },
            )
            .optional()
            .context("Failed to query session")?;

        Ok(result)
    }

    /// Get sessions by account_id
    pub fn get_sessions_by_account(&self, account_id: &str) -> Result<Vec<SessionRecord>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, session_id, account_id, status, cookies, last_used_at, created_at, updated_at
             FROM sessions WHERE account_id = ?1 ORDER BY created_at DESC"
        )?;

        let sessions = stmt
            .query_map(params![account_id], |row| {
                Ok(SessionRecord {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    account_id: row.get(2)?,
                    status: row.get(3)?,
                    cookies: row.get(4)?,
                    last_used_at: row.get::<_, Option<String>>(5)?.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                    }),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                        .unwrap()
                        .with_timezone(&Utc),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    /// Delete a session by session_id
    pub fn delete_session(&self, session_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM sessions WHERE session_id = ?1",
            params![session_id],
        )
        .context("Failed to delete session")?;

        debug!("Deleted session_id={}", session_id);
        Ok(())
    }

    /// Get database file path
    pub fn path(&self) -> &Path {
        &self.db_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let db = Database::in_memory().unwrap();
        assert_eq!(db.path(), Path::new(":memory:"));
    }

    #[test]
    fn test_task_crud() {
        let db = Database::in_memory().unwrap();

        // Insert task
        let task_id = 12345u64;
        let id = db
            .insert_task(task_id, "pending", Some("{\"test\":\"data\"}"))
            .unwrap();
        assert!(id > 0);

        // Get task
        let task = db.get_task(task_id).unwrap().unwrap();
        assert_eq!(task.task_id, task_id);
        assert_eq!(task.status, "pending");

        // Update task
        db.update_task_status(
            task_id,
            "completed",
            Some(Utc::now()),
            Some(Utc::now()),
            None,
        )
        .unwrap();
        let task = db.get_task(task_id).unwrap().unwrap();
        assert_eq!(task.status, "completed");

        // Get all tasks
        let tasks = db.get_tasks(None).unwrap();
        assert_eq!(tasks.len(), 1);

        // Get tasks by status
        let tasks = db.get_tasks(Some("completed")).unwrap();
        assert_eq!(tasks.len(), 1);

        // Delete task
        db.delete_task(task_id).unwrap();
        assert!(db.get_task(task_id).unwrap().is_none());
    }

    #[test]
    fn test_order_crud() {
        let db = Database::in_memory().unwrap();

        // Insert order
        let order_id = "ORD-12345";
        let id = db
            .insert_order(
                order_id,
                "PROD-001",
                "ACC-001",
                "pending",
                99.99,
                1,
                Some("{\"notes\":\"test\"}"),
            )
            .unwrap();
        assert!(id > 0);

        // Get order
        let order = db.get_order(order_id).unwrap().unwrap();
        assert_eq!(order.order_id, order_id);
        assert_eq!(order.status, "pending");
        assert_eq!(order.price, 99.99);

        // Update order
        db.update_order_status(order_id, "completed").unwrap();
        let order = db.get_order(order_id).unwrap().unwrap();
        assert_eq!(order.status, "completed");

        // Get orders by account
        let orders = db.get_orders_by_account("ACC-001").unwrap();
        assert_eq!(orders.len(), 1);

        // Delete order
        db.delete_order(order_id).unwrap();
        assert!(db.get_order(order_id).unwrap().is_none());
    }

    #[test]
    fn test_session_crud() {
        let db = Database::in_memory().unwrap();

        // Insert session
        let session_id = "SESS-12345";
        let id = db
            .insert_session(session_id, "ACC-001", "active", Some("cookie_data"))
            .unwrap();
        assert!(id > 0);

        // Get session
        let session = db.get_session(session_id).unwrap().unwrap();
        assert_eq!(session.session_id, session_id);
        assert_eq!(session.status, "active");

        // Update session
        db.update_session(session_id, "inactive", None).unwrap();
        let session = db.get_session(session_id).unwrap().unwrap();
        assert_eq!(session.status, "inactive");

        // Get sessions by account
        let sessions = db.get_sessions_by_account("ACC-001").unwrap();
        assert_eq!(sessions.len(), 1);

        // Delete session
        db.delete_session(session_id).unwrap();
        assert!(db.get_session(session_id).unwrap().is_none());
    }
}
