// defined errors

use axum::response::{
    IntoResponse,
    Response,
};
use axum::body::Body;
use core::result::Result;

pub(crate) type Results<T> = Result<T,Exception>;

pub enum Exception {
    // TODO : implement for tracing
    ClientSideError404,
    ApiMissMatch,
    ClientError { code: u16 , msg: String }
}


impl IntoResponse for Exception {
    // asusual implemantaion
    fn into_response(self) -> Response<Body> {
        match self {
            Self::ClientSideError404 => {
                Body::from(fallback_response_message(
                        "not found".to_owned(),
                        "client error".to_owned(),
                        404, "requested page not found".to_owned()
                )).into_response()
            },
            Self::ApiMissMatch => {
                "bad api".into_response()
            },
            Self::ClientError { code, msg } => {
                Body::from(fallback_response_message(
                        "error".to_owned(),
                        "client error".to_owned(),
                        code, msg
                )).into_response()

            }
        }
    }
}

fn fallback_response_message(title:String ,name: String,code: u16,msg: String) -> String {
    format!("<!DOCTYPE html><html><head><title>{title}</title>\
<link rel='stylesheet' href='/__assets/index.Dq5x2fgZ.css'>\
</head> <body>  <div class='w-screen h-screen flex justify-center items-center'>\
<div class='flex flex-col p-4 rounded-lg bg-[#f1f1f1] flex-shrink'>\
<h1 class='mx-0.5 text-lg'><span class='text-red-600'>{code} </span>Not Found</h1>\
<p class='m-0.5 text-gray-500'><span class='text-gray-900'>{name} - \
</span>{msg}</p> </div> </div>
</body></html>")

}
