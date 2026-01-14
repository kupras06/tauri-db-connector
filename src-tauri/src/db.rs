#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{Column, Row};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::State;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DbConfig {
    pub conn_string: String,
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DbKind {
    Postgres,
    MySql,
    Sqlite,
    Unknown,
}

#[derive(Clone)]
pub enum Connection {
    Postgres(PgPool),
    MySql(MySqlPool),
    Sqlite(SqlitePool),
}

pub struct AppState {
    pub connections: Mutex<HashMap<String, Connection>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            connections: Mutex::new(HashMap::new()),
        }
    }
}

fn detect_db_kind(conn_string: &str) -> DbKind {
    let s = conn_string.to_lowercase();
    if s.starts_with("postgres://")
        || s.starts_with("postgresql://")
        || s.contains("postgresql")
        || s.contains("postgres")
    {
        DbKind::Postgres
    } else if s.starts_with("mysql://") || s.contains("mysql") {
        DbKind::MySql
    } else if s.starts_with("sqlite://")
        || s.starts_with("file:")
        || s.contains(".sqlite")
        || s.ends_with(".db")
    {
        DbKind::Sqlite
    } else {
        DbKind::Unknown
    }
}

#[tauri::command]
pub async fn connect(state: State<'_, AppState>, conn_string: String) -> Result<String, String> {
    let kind = detect_db_kind(&conn_string);

    let id = format!(
        "conn_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    match kind {
        DbKind::Postgres => {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(5))
                .connect(&conn_string)
                .await
                .map_err(|e| e.to_string())?;
            state
                .connections
                .lock()
                .unwrap()
                .insert(id.clone(), Connection::Postgres(pool));
        }
        DbKind::MySql => {
            let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(5))
                .connect(&conn_string)
                .await
                .map_err(|e| e.to_string())?;
            state
                .connections
                .lock()
                .unwrap()
                .insert(id.clone(), Connection::MySql(pool));
        }
        DbKind::Sqlite => {
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&conn_string)
                .await
                .map_err(|e| e.to_string())?;
            state
                .connections
                .lock()
                .unwrap()
                .insert(id.clone(), Connection::Sqlite(pool));
        }
        DbKind::Unknown => return Err("Unsupported database type".to_string()),
    }

    Ok(id)
}

#[tauri::command]
pub async fn disconnect(state: State<'_, AppState>, id: String) -> Result<bool, String> {
    // Remove the connection while the mutex is held, then drop the guard before awaiting.
    let conn_to_close = {
        let mut connections = state.connections.lock().unwrap();
        connections.remove(&id)
    };

    if let Some(conn) = conn_to_close {
        match conn {
            Connection::Postgres(pool) => pool.close().await,
            Connection::MySql(pool) => pool.close().await,
            Connection::Sqlite(pool) => pool.close().await,
        }
        Ok(true)
    } else {
        Err("Connection not found".to_string())
    }
}

#[tauri::command]
pub async fn execute(
    state: State<'_, AppState>,
    id: String,
    sql: String,
) -> Result<Vec<Map<String, Value>>, String> {
    let conn = {
        let guard = state.connections.lock().unwrap();
        guard.get(&id).cloned().ok_or("Connection not found")?
    };

    // Execute the query using the appropriate typed pool and produce a concrete Vec<Map<String, Value>>
    let results = match conn {
        Connection::Postgres(pool) => {
            let rows = sqlx::query(&sql)
                .fetch_all(&pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut results = Vec::new();
            for row in rows {
                let mut map = Map::new();
                for col in row.columns() {
                    let col_name = col.name();

                    let val: Value = if let Ok(v) = row.try_get::<i64, _>(col.ordinal()) {
                        Value::Number(v.into())
                    } else if let Ok(v) = row.try_get::<f64, _>(col.ordinal()) {
                        if let Some(n) = serde_json::Number::from_f64(v) {
                            Value::Number(n)
                        } else {
                            Value::Null
                        }
                    } else if let Ok(v) = row.try_get::<bool, _>(col.ordinal()) {
                        Value::Bool(v)
                    } else if let Ok(v) = row.try_get::<String, _>(col.ordinal()) {
                        Value::String(v)
                    } else {
                        Value::Null
                    };

                    map.insert(col_name.to_string(), val);
                }
                results.push(map);
            }
            results
        }
        Connection::MySql(pool) => {
            let rows = sqlx::query(&sql)
                .fetch_all(&pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut results = Vec::new();
            for row in rows {
                let mut map = Map::new();
                for col in row.columns() {
                    let col_name = col.name();

                    let val: Value = if let Ok(v) = row.try_get::<i64, _>(col.ordinal()) {
                        Value::Number(v.into())
                    } else if let Ok(v) = row.try_get::<f64, _>(col.ordinal()) {
                        if let Some(n) = serde_json::Number::from_f64(v) {
                            Value::Number(n)
                        } else {
                            Value::Null
                        }
                    } else if let Ok(v) = row.try_get::<bool, _>(col.ordinal()) {
                        Value::Bool(v)
                    } else if let Ok(v) = row.try_get::<String, _>(col.ordinal()) {
                        Value::String(v)
                    } else {
                        Value::Null
                    };

                    map.insert(col_name.to_string(), val);
                }
                results.push(map);
            }
            results
        }
        Connection::Sqlite(pool) => {
            let rows = sqlx::query(&sql)
                .fetch_all(&pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut results = Vec::new();
            for row in rows {
                let mut map = Map::new();
                for col in row.columns() {
                    let col_name = col.name();

                    let val: Value = if let Ok(v) = row.try_get::<i64, _>(col.ordinal()) {
                        Value::Number(v.into())
                    } else if let Ok(v) = row.try_get::<f64, _>(col.ordinal()) {
                        if let Some(n) = serde_json::Number::from_f64(v) {
                            Value::Number(n)
                        } else {
                            Value::Null
                        }
                    } else if let Ok(v) = row.try_get::<bool, _>(col.ordinal()) {
                        Value::Bool(v)
                    } else if let Ok(v) = row.try_get::<String, _>(col.ordinal()) {
                        Value::String(v)
                    } else {
                        Value::Null
                    };

                    map.insert(col_name.to_string(), val);
                }
                results.push(map);
            }
            results
        }
    };

    Ok(results)
}

#[tauri::command]
pub async fn get_tables(state: State<'_, AppState>, id: String) -> Result<Vec<String>, String> {
    // Clone the connection (pool + kind) out of the mutex to avoid holding the guard across awaits.
    let conn = {
        let guard = state.connections.lock().unwrap();
        guard.get(&id).cloned().ok_or("Connection not found")?
    };

    let tables = match conn {
        Connection::Postgres(pool) => {
            let rows = sqlx::query(
                "SELECT table_name FROM information_schema.tables WHERE table_schema='public'",
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut tables = Vec::new();
            for row in rows {
                let name: String = row.try_get(0).unwrap_or_default();
                tables.push(name);
            }
            tables
        }
        Connection::MySql(pool) => {
            let rows = sqlx::query("SHOW TABLES")
                .fetch_all(&pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut tables = Vec::new();
            for row in rows {
                let name: String = row.try_get(0).unwrap_or_default();
                tables.push(name);
            }
            tables
        }
        Connection::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut tables = Vec::new();
            for row in rows {
                let name: String = row.try_get(0).unwrap_or_default();
                tables.push(name);
            }
            tables
        }
    };

    Ok(tables)
}
