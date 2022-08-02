use rocket::{Request, catch};

#[catch(404)]
pub async  fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(500)]
pub async  fn server_error(req: &Request<'_>) -> String {
    format!("Sorry, server is error in {}", req.uri())
}