use rocket::{Responder, response::Redirect, http::Status};
use rocket_dyn_templates::Template;



#[derive(Debug, Responder)]
pub enum HandleResponse {
    Template(Template),
    Redirect(Redirect),
    Status(Status),
}
