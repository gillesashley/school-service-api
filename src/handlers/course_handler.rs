use crate::db_access::course_db::get_courses_for_teacher_db;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    let teacher_id = path.into_inner().0;
    match get_courses_for_teacher_db(&app_state.db, teacher_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(err) => {
            println!("Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
