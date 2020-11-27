use diesel_migrations::embed_migrations;
use radmin::diesel::PgConnection;
use radmin::modules::DatabaseModule;
use radmin::ServerError;

embed_migrations!("migrations");

pub struct DbModule;

impl DatabaseModule for DbModule {
    fn run_migrations(&self, conn: &PgConnection) -> Result<(), ServerError> {
        embedded_migrations::run_with_output(conn, &mut ::std::io::stdout())?;
        Ok(())
    }
}
