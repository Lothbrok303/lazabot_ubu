//! Test database functionality with migration and CRUD operations

use anyhow::Result;
use lazabot::storage::{Cache, Database};
use tracing_subscriber;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Testing Database Module ===\n");

    // Test 1: Create in-memory database
    println!("1. Creating in-memory database...");
    let db = Database::in_memory()?;
    println!("   ✓ Database initialized at: {:?}\n", db.path());

    // Test 2: Task CRUD operations
    println!("2. Testing Task CRUD operations...");

    // Insert tasks
    let task_id1 = 1001u64;
    let task_id2 = 1002u64;
    db.insert_task(task_id1, "pending", Some("{\"type\":\"monitor\"}"))?;
    db.insert_task(task_id2, "pending", Some("{\"type\":\"checkout\"}"))?;
    println!("   ✓ Inserted 2 tasks");

    // Get task
    let task = db.get_task(task_id1)?.expect("Task should exist");
    println!(
        "   ✓ Retrieved task: task_id={}, status={}",
        task.task_id, task.status
    );

    // Update task
    use chrono::Utc;
    db.update_task_status(task_id1, "running", Some(Utc::now()), None, None)?;
    let task = db.get_task(task_id1)?.expect("Task should exist");
    println!("   ✓ Updated task status to: {}", task.status);

    // List all tasks
    let tasks = db.get_tasks(None)?;
    println!("   ✓ Total tasks: {}", tasks.len());

    // Filter by status
    let pending_tasks = db.get_tasks(Some("pending"))?;
    println!("   ✓ Pending tasks: {}\n", pending_tasks.len());

    // Test 3: Order CRUD operations
    println!("3. Testing Order CRUD operations...");

    // Insert orders
    db.insert_order("ORD-001", "PROD-001", "ACC-001", "pending", 99.99, 1, None)?;
    db.insert_order("ORD-002", "PROD-002", "ACC-001", "pending", 149.99, 2, None)?;
    db.insert_order(
        "ORD-003",
        "PROD-003",
        "ACC-002",
        "completed",
        79.99,
        1,
        None,
    )?;
    println!("   ✓ Inserted 3 orders");

    // Get order
    let order = db.get_order("ORD-001")?.expect("Order should exist");
    println!(
        "   ✓ Retrieved order: order_id={}, price=${}",
        order.order_id, order.price
    );

    // Update order status
    db.update_order_status("ORD-001", "completed")?;
    let order = db.get_order("ORD-001")?.expect("Order should exist");
    println!("   ✓ Updated order status to: {}", order.status);

    // Get orders by account
    let acc_orders = db.get_orders_by_account("ACC-001")?;
    println!("   ✓ Orders for ACC-001: {}\n", acc_orders.len());

    // Test 4: Session CRUD operations
    println!("4. Testing Session CRUD operations...");

    // Insert sessions
    db.insert_session("SESS-001", "ACC-001", "active", Some("encrypted_cookies_1"))?;
    db.insert_session("SESS-002", "ACC-002", "active", Some("encrypted_cookies_2"))?;
    println!("   ✓ Inserted 2 sessions");

    // Get session
    let session = db.get_session("SESS-001")?.expect("Session should exist");
    println!(
        "   ✓ Retrieved session: session_id={}, status={}",
        session.session_id, session.status
    );

    // Update session
    db.update_session("SESS-001", "inactive", None)?;
    let session = db.get_session("SESS-001")?.expect("Session should exist");
    println!("   ✓ Updated session status to: {}", session.status);

    // Get sessions by account
    let acc_sessions = db.get_sessions_by_account("ACC-001")?;
    println!("   ✓ Sessions for ACC-001: {}\n", acc_sessions.len());

    // Test 5: Cache operations
    println!("5. Testing Cache operations...");

    let task_cache: Cache<u64, String> = Cache::new("task_cache");

    // Insert into cache
    task_cache.set(task_id1, "cached_task_1".to_string());
    task_cache.set(task_id2, "cached_task_2".to_string());
    println!("   ✓ Inserted 2 items into cache");

    // Get from cache
    let cached = task_cache.get(&task_id1).expect("Should be in cache");
    println!("   ✓ Retrieved from cache: {}", cached);

    // Check cache stats
    println!("   ✓ Cache size: {}", task_cache.len());
    println!("   ✓ Cache name: {}\n", task_cache.name());

    // Test 6: Cleanup operations
    println!("6. Testing cleanup operations...");

    db.delete_task(task_id1)?;
    println!("   ✓ Deleted task {}", task_id1);

    db.delete_order("ORD-001")?;
    println!("   ✓ Deleted order ORD-001");

    db.delete_session("SESS-001")?;
    println!("   ✓ Deleted session SESS-001");

    task_cache.clear();
    println!("   ✓ Cleared cache\n");

    // Test 7: Create file-based database
    println!("7. Testing file-based database...");
    let db_path = std::env::temp_dir().join("lazabot_test_new.db");

    // Remove old file if it exists
    if db_path.exists() {
        std::fs::remove_file(&db_path).ok();
    }

    {
        let file_db = Database::new(&db_path)?;
        println!("   ✓ Created file-based database at: {:?}", file_db.path());

        // Insert some data
        file_db.insert_task(2001, "test", None)?;
        let task = file_db.get_task(2001)?.expect("Task should exist");
        println!("   ✓ Verified data persistence: task_id={}", task.task_id);

        // Database is dropped here, releasing the file lock
    }

    // Cleanup
    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
        println!("   ✓ Cleaned up test database file\n");
    }

    println!("=== All Database Tests Passed! ===");

    Ok(())
}
