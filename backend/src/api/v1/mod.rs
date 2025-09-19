//! API v1 do FORTIS Backend

use actix_web::web;

pub mod auth;
pub mod elections;
pub mod votes;
pub mod nodes;
// pub mod audit;
pub mod zkp;
pub mod tse;
pub mod urnas;

/// Configurar rotas da API v1
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/auth")
                .configure(auth::configure)
        )
        .service(
            web::scope("/elections")
                .configure(elections::configure)
        )
        .service(
            web::scope("/votes")
                .configure(votes::configure)
        )
        .service(
            web::scope("/nodes")
                .configure(nodes::configure)
        )
        // .service(
        //     web::scope("/audit")
        //         .configure(audit::config_audit_routes)
        // )
        .service(
            web::scope("/zkp")
                .configure(zkp::config_zkp_routes)
        )
        .service(
            web::scope("/tse")
                .configure(tse::config_tse_routes)
        )
        .service(
            web::scope("/urnas")
                .configure(urnas::configure)
        );
}
