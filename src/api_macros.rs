#[macro_export]
macro_rules! to_json {
    ($obj: expr) => {
        match $obj.to_json() {
            Ok(j) => j,
            Err(err) => return HttpResponse::BadRequest().body(err.msg())
        }
    };
}

pub use to_json;

#[macro_export]
macro_rules! str_to_oid {
    ($str_oid: expr) => {
        match ObjectId::parse_str($str_oid) {
            Ok(oid) => oid,
            Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId")
        }
    };
}

pub use str_to_oid;

#[macro_export]
macro_rules! get_by_id {
    ($ty: ty, $oid: expr, $coll: expr) => {
        match $ty::get_by_id($oid, $coll).await {
            Ok(n) => n,
            Err(err) => return HttpResponse::BadRequest().body(err.msg())
        }
    };
}

pub use get_by_id;

#[macro_export]
macro_rules! resp_with_auth_headers {
    ($token: expr) => {
        HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "x-auth"))
        .insert_header(("x-auth", $token))
    };
}

pub use resp_with_auth_headers;
