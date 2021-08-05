use actix_cors::Cors;

pub fn make_cors_middleware() -> Cors {
    if cfg!(debug_assertions) {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        return cors;
    }

    Cors::default()
        .allowed_origin("https://fluxcap.herokuapp.com/")
        .allowed_methods(vec!["GET"])
        .allow_any_header()
        .max_age(3600)
}
