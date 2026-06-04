use actix_web::web::ServiceConfig;
use crate::{
    routes::{
        index_file,
        signup,
        login,
        forgotten_password,
        change_password,
    },
};

// App Configuration
pub fn app_config(cfg: &mut ServiceConfig) {
    cfg
       .service(index_file)
       .service(signup)
       .service(login)
       .service(forgotten_password)
       .service(change_password);
}