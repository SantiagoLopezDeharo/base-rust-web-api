use crate::routing::Route;
use crate::domain::user::controller::UserController;

pub fn init_routes() -> Vec<Route> {
    let mut routes = Vec::new();

        routes.extend(UserController::routes());
routes
}
