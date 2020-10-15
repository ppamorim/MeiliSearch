use std::fs::File;
use std::path::Path;

use actix_web::{get, post};
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::dump::{DumpInfo, DumpStatus, list_dumps_folder, compressed_dumps_folder, init_dump_process};
use crate::Data;
use crate::error::{Error, ResponseError};
use crate::helpers::Authentication;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(trigger_dump)
        .service(list_dumps)
        .service(get_dump_status);
}

#[post("/dumps/create", wrap = "Authentication::Private")]
async fn trigger_dump(
    data: web::Data<Data>,
) -> Result<HttpResponse, ResponseError> {
    let dumps_folder = Path::new(&data.dumps_folder);
    match init_dump_process(&data, &dumps_folder) {
        Ok(resume) => Ok(HttpResponse::Accepted().json(resume)),
        Err(e) => Err(e.into())
    }
}

#[get("/dumps", wrap = "Authentication::Private")]
async fn list_dumps(
    data: web::Data<Data>,
) -> Result<HttpResponse, ResponseError> {
    let dumps_folder = Path::new(&data.dumps_folder);
    match list_dumps_folder(dumps_folder) {
        Ok(dumps_info) => Ok(HttpResponse::Ok().json(dumps_info)),
        Err(e) => Err(Error::not_found("dump does not exist").into())
    }
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpStatusResponse {
    status: String,
}

#[derive(Deserialize)]
struct DumpParam {
    dump_uid: String,
}

#[get("/dumps/{dump_uid}/status", wrap = "Authentication::Private")]
async fn get_dump_status(
    data: web::Data<Data>,
    path: web::Path<DumpParam>,
) -> Result<HttpResponse, ResponseError> {
    let dumps_folder = Path::new(&data.dumps_folder);
    let dump_uid = &path.dump_uid;

    if let Some(resume) = DumpInfo::get_current() {
        if &resume.uid == dump_uid {
            return Ok(HttpResponse::Ok().json(resume));
        }
    }

    if File::open(compressed_dumps_folder(Path::new(dumps_folder), dump_uid)).is_ok() {
        let resume = DumpInfo::new(
            dump_uid.into(),
            DumpStatus::Done
        );

        Ok(HttpResponse::Ok().json(resume))
    } else {
        Err(Error::not_found("dump does not exist").into())
    }
}
