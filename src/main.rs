use {
    crate::cli::{generate_cli, ProgramArgs},
    actix_files::Files,
    actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder},
    lazy_static::lazy_static,
    std::env,
};

mod cli;

lazy_static! {
    static ref ARGS: ProgramArgs = ProgramArgs::init(generate_cli());
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", ARGS.base_dir()).index_file("index.html"))
    })
    .server_hostname(ARGS.server_name())
    .bind(ARGS.bind())?
    .run()
}
