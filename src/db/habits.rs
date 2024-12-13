use crate::models::habit::Habit;
use chrono::NaiveDate;
use rusqlite::{Connection, OptionalExtension, Result};
use uuid::Uuid;

pub fn save_habit(conn: &Connection, habit: &Habit) -> Result<()> {
    conn.execute(
        "INSERT INTO habits (id, title, start_date, color, week_start)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            habit.id.to_string(),
            &habit.title,
            habit.start_date.to_string(),
            &habit.color,
            &habit.week_start,
        ),
    )?;
    Ok(())
}

pub fn update_habit_title(conn: &Connection, id: Uuid, title: &str) -> Result<()> {
    conn.execute(
        "UPDATE habits SET title = ?1 WHERE id = ?2",
        [title, &id.to_string()],
    )?;
    Ok(())
}

pub fn save_completed_day(conn: &Connection, habit_id: Uuid, date: NaiveDate) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO completed_days (habit_id, date) VALUES (?1, ?2)",
        [&habit_id.to_string(), &date.to_string()],
    )?;
    Ok(())
}

pub fn delete_completed_day(conn: &Connection, habit_id: Uuid, date: NaiveDate) -> Result<()> {
    conn.execute(
        "DELETE FROM completed_days WHERE habit_id = ?1 AND date = ?2",
        [&habit_id.to_string(), &date.to_string()],
    )?;
    Ok(())
}

pub fn load_habit(conn: &Connection, id: Uuid) -> Result<Option<Habit>> {
    let mut stmt =
        conn.prepare("SELECT id, title, start_date, color, week_start FROM habits WHERE id = ?1")?;

    let habit = stmt
        .query_row([&id.to_string()], |row| {
            Ok(Habit {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                start_date: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
                    .unwrap(),
                color: row.get(3)?,
                week_start: row.get(4)?,
            })
        })
        .optional()?;

    Ok(habit)
}

pub fn load_first_habit(conn: &Connection) -> Result<Option<Habit>> {
    conn.query_row(
        "SELECT id, title, start_date, color, week_start FROM habits LIMIT 1",
        [],
        |row| {
            Ok(Habit {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                start_date: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
                    .unwrap(),
                color: row.get(3)?,
                week_start: row.get(4)?,
            })
        },
    )
    .optional()
}

pub fn load_completed_days(conn: &Connection, habit_id: Uuid) -> Result<Vec<NaiveDate>> {
    let mut stmt = conn.prepare("SELECT date FROM completed_days WHERE habit_id = ?1")?;

    let dates = stmt
        .query_map([&habit_id.to_string()], |row| {
            let date_str: String = row.get(0)?;
            Ok(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap())
        })?
        .filter_map(Result::ok)
        .collect();

    Ok(dates)
}

pub fn update_habit_color(conn: &Connection, id: Uuid, color: &str) -> Result<()> {
    conn.execute(
        "UPDATE habits SET color = ?1 WHERE id = ?2",
        [color, &id.to_string()],
    )?;
    Ok(())
}

pub fn update_habit_start_date(conn: &Connection, id: Uuid, start_date: NaiveDate) -> Result<()> {
    conn.execute(
        "UPDATE habits SET start_date = ?1 WHERE id = ?2",
        [&start_date.to_string(), &id.to_string()],
    )?;
    Ok(())
}

pub fn update_habit_week_start(conn: &Connection, id: Uuid, week_start: &str) -> Result<()> {
    conn.execute(
        "UPDATE habits SET week_start = ?1 WHERE id = ?2",
        [week_start, &id.to_string()],
    )?;
    Ok(())
}
