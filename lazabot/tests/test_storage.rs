//! Integration tests for storage module

use anyhow::Result;
use lazabot::storage::{Database, Cache};
use chrono::Utc;

#[test]
fn test_database_persistence() -> Result<()> {
    let db = Database::in_memory()?;
    
    // Test task persistence
    db.insert_task(100, "pending", Some("{}"))?;
    let task = db.get_task(100)?.unwrap();
    assert_eq!(task.task_id, 100);
    assert_eq!(task.status, "pending");
    
    // Test order persistence
    db.insert_order("TEST-001", "PROD-001", "ACC-001", "pending", 99.99, 1, None)?;
    let order = db.get_order("TEST-001")?.unwrap();
    assert_eq!(order.order_id, "TEST-001");
    assert_eq!(order.price, 99.99);
    
    // Test session persistence
    db.insert_session("SESS-TEST", "ACC-001", "active", Some("cookies"))?;
    let session = db.get_session("SESS-TEST")?.unwrap();
    assert_eq!(session.session_id, "SESS-TEST");
    assert_eq!(session.status, "active");
    
    Ok(())
}

#[test]
fn test_database_updates() -> Result<()> {
    let db = Database::in_memory()?;
    
    // Insert and update task
    db.insert_task(200, "pending", None)?;
    db.update_task_status(200, "completed", Some(Utc::now()), Some(Utc::now()), None)?;
    let task = db.get_task(200)?.unwrap();
    assert_eq!(task.status, "completed");
    assert!(task.started_at.is_some());
    assert!(task.completed_at.is_some());
    
    // Insert and update order
    db.insert_order("TEST-002", "PROD-002", "ACC-002", "pending", 49.99, 2, None)?;
    db.update_order_status("TEST-002", "shipped")?;
    let order = db.get_order("TEST-002")?.unwrap();
    assert_eq!(order.status, "shipped");
    
    // Insert and update session
    db.insert_session("SESS-002", "ACC-002", "active", None)?;
    db.update_session("SESS-002", "expired", Some("new_cookies"))?;
    let session = db.get_session("SESS-002")?.unwrap();
    assert_eq!(session.status, "expired");
    assert!(session.last_used_at.is_some());
    
    Ok(())
}

#[test]
fn test_database_queries() -> Result<()> {
    let db = Database::in_memory()?;
    
    // Insert multiple tasks with different statuses
    db.insert_task(301, "pending", None)?;
    db.insert_task(302, "running", None)?;
    db.insert_task(303, "completed", None)?;
    db.insert_task(304, "pending", None)?;
    
    // Query all tasks
    let all_tasks = db.get_tasks(None)?;
    assert_eq!(all_tasks.len(), 4);
    
    // Query by status
    let pending_tasks = db.get_tasks(Some("pending"))?;
    assert_eq!(pending_tasks.len(), 2);
    
    // Insert multiple orders for same account
    db.insert_order("ORD-301", "PROD-001", "ACC-TEST", "pending", 99.99, 1, None)?;
    db.insert_order("ORD-302", "PROD-002", "ACC-TEST", "completed", 149.99, 2, None)?;
    db.insert_order("ORD-303", "PROD-003", "OTHER-ACC", "pending", 79.99, 1, None)?;
    
    // Query orders by account
    let account_orders = db.get_orders_by_account("ACC-TEST")?;
    assert_eq!(account_orders.len(), 2);
    
    // Insert multiple sessions for same account
    db.insert_session("SESS-301", "ACC-TEST", "active", None)?;
    db.insert_session("SESS-302", "ACC-TEST", "inactive", None)?;
    db.insert_session("SESS-303", "OTHER-ACC", "active", None)?;
    
    // Query sessions by account
    let account_sessions = db.get_sessions_by_account("ACC-TEST")?;
    assert_eq!(account_sessions.len(), 2);
    
    Ok(())
}

#[test]
fn test_database_deletions() -> Result<()> {
    let db = Database::in_memory()?;
    
    // Test task deletion
    db.insert_task(400, "test", None)?;
    assert!(db.get_task(400)?.is_some());
    db.delete_task(400)?;
    assert!(db.get_task(400)?.is_none());
    
    // Test order deletion
    db.insert_order("DEL-001", "PROD", "ACC", "test", 10.0, 1, None)?;
    assert!(db.get_order("DEL-001")?.is_some());
    db.delete_order("DEL-001")?;
    assert!(db.get_order("DEL-001")?.is_none());
    
    // Test session deletion
    db.insert_session("DEL-SESS", "ACC", "test", None)?;
    assert!(db.get_session("DEL-SESS")?.is_some());
    db.delete_session("DEL-SESS")?;
    assert!(db.get_session("DEL-SESS")?.is_none());
    
    Ok(())
}

#[test]
fn test_cache_operations() {
    let cache: Cache<String, i32> = Cache::new("test_cache");
    
    // Test basic operations
    cache.set("key1".to_string(), 42);
    assert_eq!(cache.get(&"key1".to_string()), Some(42));
    assert!(cache.contains(&"key1".to_string()));
    assert_eq!(cache.len(), 1);
    
    // Test update
    cache.set("key1".to_string(), 100);
    assert_eq!(cache.get(&"key1".to_string()), Some(100));
    assert_eq!(cache.len(), 1);
    
    // Test multiple entries
    for i in 0..10 {
        cache.set(format!("key{}", i), i);
    }
    assert_eq!(cache.len(), 10);
    
    // Test removal
    cache.remove(&"key1".to_string());
    assert!(!cache.contains(&"key1".to_string()));
    
    // Test clear
    cache.clear();
    assert!(cache.is_empty());
}

#[test]
fn test_cache_iteration() {
    let cache: Cache<u64, String> = Cache::new("iter_cache");
    
    // Insert test data
    for i in 0..5 {
        cache.set(i, format!("value_{}", i));
    }
    
    // Test keys
    let keys = cache.keys();
    assert_eq!(keys.len(), 5);
    
    // Test values
    let values = cache.values();
    assert_eq!(values.len(), 5);
    
    // Test for_each
    let mut count = 0;
    cache.for_each(|_, _| {
        count += 1;
    });
    assert_eq!(count, 5);
}

#[test]
fn test_cache_clone() {
    let cache1: Cache<String, i32> = Cache::new("original");
    cache1.set("test".to_string(), 42);
    
    let cache2 = cache1.clone();
    assert_eq!(cache2.get(&"test".to_string()), Some(42));
    
    // Modifications in one are visible in the other
    cache2.set("new".to_string(), 100);
    assert_eq!(cache1.get(&"new".to_string()), Some(100));
}
