use crate::handlers::course_handler::get_courses_for_teacher;
use actix_web::web;

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses").route("/{teacher_id}", web::get().to(get_courses_for_teacher)),
    );
}
