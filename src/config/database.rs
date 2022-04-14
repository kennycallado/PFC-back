use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

/* Establece la conexión con la base de datos */
/* usa diesel y PgConnection */
#[database("diesel")]
pub struct Db(diesel::PgConnection);

/* Función que ejecuta las migraciones*/
#[allow(dead_code)]
pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}
