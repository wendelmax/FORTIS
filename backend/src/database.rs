use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Election {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_election(pool: &PgPool, election: &Election) -> Result<Uuid> {
    let result = sqlx::query(
        r#"
        INSERT INTO elections (id, name, description, start_date, end_date, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#
    )
    .bind(election.id)
    .bind(&election.name)
    .bind(&election.description)
    .bind(election.start_date)
    .bind(election.end_date)
    .bind(&election.status)
    .bind(election.created_at)
    .bind(election.updated_at)
    .fetch_one(pool)
    .await?;
    
    Ok(result.get("id"))
}

pub async fn get_elections(pool: &PgPool) -> Result<Vec<Election>> {
    let elections = sqlx::query_as::<_, Election>(
        "SELECT * FROM elections ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(elections)
}
