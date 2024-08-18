use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;

use axum::body::Body;
use axum::response::IntoResponse;

use crate::err::Results;
use crate::err::Exception;



pub struct LoginStateModel {
    db:RefCell<HashMap<u8,String>>
}

impl LoginStateModel {
    fn new() -> Self {
        Self { db : HashMap::new().into() }
    }
}

impl LoginStateModel {
    fn create_user(&self ,name : String) -> Results<impl IntoResponse> {
        if self.db.borrow_mut().insert(1, name.clone()).is_none() {
            Err(Exception::ApiMissMatch)
        } else {
            Ok( ().into_response() )
        }
    }
}

pub fn get_new_model() -> LoginStateModel {
    LoginStateModel::new()
}
