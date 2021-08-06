use actix_cors::Cors;

pub fn make_cors_middleware() -> Cors {
    if cfg!(debug_assertions) {
        return Cors::permissive();
    }

    Cors::default()
        .allowed_origin("fluxcap.herokuapp.com")
        .allowed_methods(vec!["GET", "OPTIONS"])
        .allow_any_header()
}
