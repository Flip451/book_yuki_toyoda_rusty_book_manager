pub mod event;

use crate::tuple_value_object_with_simple_error;

tuple_value_object_with_simple_error!(AccessToken, String, AccessTokenError);
