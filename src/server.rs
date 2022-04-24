// use rocket::fairing::AdHoc;

use crate::config::cors;
use crate::config::database;
use crate::routes;

#[launch]
pub fn rocket() -> _ {
    // Ingresa las rutas
    rocket::build()
        .attach(routes::router())
        // Ingresa la conexión con base de datos
        .attach(database::Db::fairing())
        .attach(cors::CORS)
    // Ejecuta las migraciones
    // .attach(AdHoc::on_ignite(
    //     "Diesel Migrations",
    //     database::run_migrations,
    // ))
}
