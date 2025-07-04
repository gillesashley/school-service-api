use crate::errors::MyError;
use crate::models::course::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query_as!(
        Course,
        "SELECT * FROM courses WHERE teacher_id = $1",
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
