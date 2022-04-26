use rocket::fairing::AdHoc;

use crate::app::controllers::booking as booking_controller;
use crate::app::controllers::table as table_controller;

#[get("/")]
fn index() -> &'static str {
    "hola desde index"
}

pub fn router() -> AdHoc {
    AdHoc::on_ignite("Mounting routes", |rocket| async {
        rocket
            .mount("/", routes![index])
            .mount(
                "/api/table",
                routes![
                    table_controller::index,
                    table_controller::show,
                    table_controller::store,
                    table_controller::destroy,
                    table_controller::update,
                    table_controller::available_tables,
                    table_controller::log_store_table,
                    table_controller::log_update_table,
                ],
            )
            .mount(
                "/api/booking",
                routes![
                    booking_controller::index,
                    booking_controller::show,
                    booking_controller::store,
                    booking_controller::destroy,
                    booking_controller::update,
                    booking_controller::log_store_booking,
                    booking_controller::log_update_booking,
                ],
            )
    })
}
