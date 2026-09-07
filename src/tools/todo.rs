// =============================================================================
//  src/tools/todo.rs — Interactive 3-tier task manager (Phase 5)
// =============================================================================

use inquire::{Confirm, Select, Text};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn todo_file_path() -> PathBuf {
    dirs_next_or_home().join(".todo_list.txt")
}

fn dirs_next_or_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Runs the interactive todo task manager.
pub fn run(action_opt: Option<&str>, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = todo_file_path();

    if !todo_file.exists() {
        let _ = fs::File::create(&todo_file);
    }

    let action = match action_opt {
        Some(a) => a,
        None => {
            let choice = Select::new(
                "📋 FANCYBASH TO-DO MANAGER",
                vec![
                    "1. ➕ Add Task",
                    "2. 📋 View / List Tasks",
                    "3. 🎉 Mark Task as Done",
                    "4. 🗑️ Clear All Tasks",
                    "❌ Exit",
                ],
            )
            .prompt()?;

            if choice.contains("Exit") {
                return Ok(());
            }

            if choice.contains("Add") {
                "add"
            } else if choice.contains("View") {
                "list"
            } else if choice.contains("Mark") {
                "done"
            } else if choice.contains("Clear") {
                "clear"
            } else {
                "list"
            }
        }
    };

    match action {
        "add" => {
            let task = if !args.is_empty() {
                args.join(" ")
            } else {
                Text::new("Enter task description:").prompt()?
            };

            let clean_task = task.trim();
            if clean_task.is_empty() {
                println!("❌ Task cannot be empty!");
                return Ok(());
            }

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&todo_file)?;
            writeln!(file, "{}", clean_task)?;
            println!("✔ Added task: \"{}\"", clean_task);
        }

        "list" | "ls" => {
            show_list(&todo_file)?;
        }

        "done" | "rm" => {
            let tasks = read_tasks(&todo_file)?;
            if tasks.is_empty() {
                println!("📋 No tasks to complete!");
                return Ok(());
            }

            let line_num: Option<usize> = args
                .first()
                .and_then(|s| s.parse::<usize>().ok());

            if let Some(num) = line_num {
                if num < 1 || num > tasks.len() {
                    println!("❌ Invalid task number! Valid range: 1–{}", tasks.len());
                    return Ok(());
                }
                let completed = tasks[num - 1].clone();
                remove_task(&todo_file, num - 1)?;
                println!("🎉 Completed: \"{}\"", completed);
            } else {
                let options: Vec<String> = tasks
                    .iter()
                    .enumerate()
                    .map(|(i, t)| format!("{:2}. {}", i + 1, t))
                    .collect();

                let selected = Select::new("Select task to mark as Done:", options).prompt()?;
                let idx = selected
                    .split('.')
                    .next()
                    .and_then(|s| s.trim().parse::<usize>().ok())
                    .ok_or("Invalid selection")?;

                if idx >= 1 && idx <= tasks.len() {
                    let completed = tasks[idx - 1].clone();
                    remove_task(&todo_file, idx - 1)?;
                    println!("🎉 Completed: \"{}\"", completed);
                }
            }
        }

        "clear" => {
            if Confirm::new("Are you sure you want to clear ALL tasks?")
                .with_default(false)
                .prompt()?
            {
                fs::write(&todo_file, "")?;
                println!("🗑️ All tasks cleared!");
            }
        }

        _ => {
            println!("Usage: fancybash todo [add <task> | list | done <num> | clear]");
        }
    }

    Ok(())
}

fn read_tasks(file_path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !file_path.exists() {
        return Ok(Vec::new());
    }
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut tasks = Vec::new();
    for line in reader.lines() {
        let l = line?;
        if !l.trim().is_empty() {
            tasks.push(l);
        }
    }
    Ok(tasks)
}

fn remove_task(file_path: &PathBuf, index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = read_tasks(file_path)?;
    let new_content: String = tasks
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, t)| format!("{}\n", t))
        .collect();

    fs::write(file_path, new_content)?;
    Ok(())
}

fn show_list(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = read_tasks(file_path)?;
    if tasks.is_empty() {
        println!("📋 No pending tasks! Add one with: todo add <task>");
        return Ok(());
    }

    println!("\n📋 PENDING TASKS ({} total):", tasks.len());
    println!("──────────────────────────────────────");
    for (i, task) in tasks.iter().enumerate() {
        println!("  {:2}. [ ] {}", i + 1, task);
    }
    println!("──────────────────────────────────────\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_empty_tasks() {
        let temp = std::env::temp_dir().join(format!("test_todo_{}.txt", std::process::id()));
        let _ = fs::remove_file(&temp);
        let tasks = read_tasks(&temp).unwrap();
        assert!(tasks.is_empty());
    }
}
