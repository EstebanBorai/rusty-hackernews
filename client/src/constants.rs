pub mod api {
    #[cfg(debug_assertions)]
    pub mod v1 {
        pub const PREVIEWS: &str = "http://0.0.0.0:3000/api/v1/previews";
        pub const STORIES: &str = "http://0.0.0.0:3000/api/v1/stories";
    }

    #[cfg(not(debug_assertions))]
    pub mod v1 {
        pub const PREVIEWS: &str = "https://fluxcap.herokuapp.com/api/v1/previews";
        pub const STORIES: &str = "https://fluxcap.herokuapp.com/api/v1/stories";
    }
}
