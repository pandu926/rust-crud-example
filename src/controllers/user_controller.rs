use rocket::serde::json::Json;
use crate::models::user::{User, NewUser};
use crate::services::user_service; // Add this import
use crate::middleware::jwt_authorization::JwtToken;
use bcrypt::hash;


#[post("/user", format = "json", data = "<new_post>")]
pub fn create_post(new_post: Json<NewUser>) -> Json<User> {
   let data_user = new_post.into_inner();
   let user = user_service::send( data_user.name, data_user.email, data_user.password); // Ensure this returns Json<NewUser>
   Json(user) // Ensure user is of type Json<NewUser>
}

#[get("/user/list")]
pub fn get_data(_token: JwtToken) -> Json<Vec<User>> { // Change return type to Json<Vec<User>>
    let data = user_service::get();
    Json(data) // Return data wrapped in Json
}

#[delete("/user/<id>")]
pub fn delete_user(id: i32) -> Result<String, rocket::http::Status> { // Change return type to Result<String>
    user_service::delete(id).map(|_| "sukses hapus".to_string()) // Convert &str to String
        .map_err(|_| rocket::http::Status::NotFound) // Map error to Status
}

#[put("/user/<id>", format = "json", data = "<updated_user>")]
pub fn edit_user(id: i32, updated_user: Json<NewUser>) -> Result<Json<String>, rocket::http::Status> {
    let updated_data = updated_user.into_inner();
    user_service::edit(id, updated_data.name, updated_data.email, updated_data.password)
        .map(|_| Json("User updated successfully".to_string()))
        .map_err(|_| rocket::http::Status::NotFound)
}
