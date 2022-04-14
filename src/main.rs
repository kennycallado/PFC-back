/* Declaración de los módulos */
mod app;
mod config;
mod routes;
mod server;

/* Expone los elementos en toda la aplicación */

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
// #[macro_use]
extern crate rocket_sync_db_pools;

fn main() {
    /* aquí iniciaría la base de datos... */
    /* con migraciones si es que las uso */

    /* Las migraciones deben ejecurtarse en server.rs que hace attach */
    /* luego esta parte podría servir para algo previo... */

    /* El macro launch en server hace wrap de main */
    server::main();
}
