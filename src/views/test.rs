use dioxus::prelude::*;
use crate::server::habits::{get_habits, get_first_habit};
use uuid::Uuid;
use std::path::PathBuf;
use tracing::{info, error};

fn check_db_file() -> String {
    use rusqlite::Connection;
    let potential_paths = vec![
        "/data/user/0/com.example.Myquest/databases/habits.db",
        "/data/user/0/com.example.Myquest/files/habits.db",
        "/data/data/com.example.Myquest/databases/habits.db",
        "/data/data/com.example.Myquest/files/habits.db",
    ];

    for path in &potential_paths {
        let pb = PathBuf::from(path);
        if pb.exists() {
            // Try to open the database
            match Connection::open(&pb) {
                Ok(_) => return format!("DB found and openable at: {} - Size: {} bytes", 
                    path, 
                    pb.metadata().map(|m| m.len()).unwrap_or(0)
                ),
                Err(e) => return format!("DB found but not openable at: {} - Error: {}", path, e),
            }
        }
    }
    
    "DB file not found in any expected location".to_string()
}
#[component]
pub fn TestPage() -> Element {
    let mut debug_messages = use_signal(|| vec!["Starting database checks...".to_string()]);
    let mut habits_count = use_signal(|| 0);
    let mut first_habit_id = use_signal(|| None::<Uuid>);
    let mut db_status = use_signal(|| "Checking...".to_string());

    // Check DB file existence
    use_effect(move || {
        let mut db_status = db_status.clone();
        let status = check_db_file();
        db_status.set(status);
    });

    // Load habits
    use_effect(move || {
        let mut debug_messages = debug_messages.clone();
        let mut habits_count = habits_count.clone();
        let mut first_habit_id = first_habit_id.clone();

        spawn(async move {
            let mut messages = debug_messages.read().clone();
            
            // Try to get all habits
            messages.push("Attempting to load all habits...".to_string());
            debug_messages.set(messages.clone());

            match get_habits().await {
                Ok(habits) => {
                    messages.push(format!("Successfully loaded {} habits:", habits.len()));
                    for habit in &habits {
                        messages.push(format!("- Habit: {} (ID: {})", habit.title, habit.id));
                    }
                    habits_count.set(habits.len());
                }
                Err(e) => {
                    messages.push(format!("Error loading habits: {:?}", e));
                }
            }

            // Try to get first habit
            messages.push("Attempting to load first habit...".to_string());
            match get_first_habit().await {
                Ok(Some(habit)) => {
                    messages.push(format!("First habit found: {} (ID: {})", habit.title, habit.id));
                    first_habit_id.set(Some(habit.id));
                }
                Ok(None) => {
                    messages.push("No first habit found - database might be empty".to_string());
                }
                Err(e) => {
                    messages.push(format!("Error loading first habit: {:?}", e));
                }
            }

            debug_messages.set(messages);
        });
    });

    rsx! {
        div {
            style: "padding: 20px;",
            
            // Database Status
            div {
                style: "background-color: #e0f0e0; padding: 10px; margin-bottom: 20px; border-radius: 5px;",
                h3 { "Database Status" }
                p { "{db_status.read()}" }
            }

            // Habits Summary
            div {
                style: "background-color: #f0e0e0; padding: 10px; margin-bottom: 20px; border-radius: 5px;",
                h3 { "Habits Summary" }
                p { "Total habits found: {habits_count.read()}" }
                p { 
                    "First habit ID: ",
                    {match *first_habit_id.read() {
                        Some(id) => id.to_string(),
                        None => "None".to_string()
                    }}
                }
            }

            // Debug Log
            div {
                style: "background-color: #ffe; padding: 10px; border-radius: 5px;",
                h3 { "Debug Log" }
                
                div {
                    style: "max-height: 300px; overflow-y: auto;",
                    for (i, msg) in debug_messages.read().iter().enumerate() {
                        div {
                            key: "{i}",
                            style: "padding: 5px; border-bottom: 1px solid #ddd;",
                            "#{i}: {msg}"
                        }
                    }
                }
            }
        }
    }
}