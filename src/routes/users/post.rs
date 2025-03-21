use super::*;

pub async fn login_by_form(
    credentials: Form<LoginForm<'_>>,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Get the user from the database
    let user = match &get_user_from_db(db, credentials.username)
        .await?
        .data
    {
        Some(user) => user.clone(),
        None => {
            return Err(ApiResponse::internal_server_error(
                "No user data".to_string(),
            ));
        }
    };

    // Validate if the given password is correct
    if !Password::verify_password(credentials.password, &user.password).map_err(|e| {
        ApiResponse::internal_server_error(format!("Password verification failed: {}", e))
    })? {
        return Err(ApiResponse::bad_request("Invalid password".to_string()));
    };

    // Add the user to the JWT guard
    JwtGuard::secure(&user, cookies)
        .await
        .map_err(ApiResponse::internal_server_error)?;

    // Return the token
    Ok(ApiResponse::success("Login successful".to_string(), None))
}

pub fn logout(_guard: JwtGuard, cookies: &CookieJar<'_>) -> Success<String> {
    cookies.remove_private(TOKEN_COOKIE);

    ApiResponse::success(
        "Logout successful - token and user info removed".to_string(),
        None,
    )
}

/// This function allows for the creation of a new [`User`] by using a form.
///
/// **Route**: `./form`
///
/// ### Parameters
/// * `db`: Instance of the [`Database`] connection.
/// * `form`: A [`NewUserForm`] for creating a [`User`].
///
/// ### Returns
/// * `Ok(Success<InsertedUser>)`: When `Ok`, it returns [`Success`] with the [`InsertedUser`].
/// * `Err(Error<String>)`: When `Err`, it returns an [`Error`] with [`Null`].
pub async fn create_new_user_by_form(
    form: Form<NewUserForm<'_>>,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Hash the provided password
    let password_hash = form
        .password
        .hash_password()
        .map_err(|e| ApiResponse::internal_server_error(format!("Couldn't hash password: {e}")))?;

    // Create a new User
    let new_user = NewUser {
        username: form.username.to_string(),
        email: form.email.to_string(),
        password: password_hash,
        role: None,
    };

    // Add the new User to the database
    let inserted_user: User = db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)
        })
        .await
        .map_err(|e| match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                ApiResponse::conflict("User already exists".to_string(), e.to_string())
            }
            _ => ApiResponse::internal_server_error(format!("Error creating user: {}", e)),
        })?;

    // Add the user to the JWT guard
    JwtGuard::secure(&inserted_user, cookies)
        .await
        .map_err(ApiResponse::internal_server_error)?;

    // Return success response
    Ok(ApiResponse::success(
        format!("User '{}' created succesfully", inserted_user.username),
        None,
    ))
}

pub async fn inject_user(user: Json<User>, db: Database) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.id = Uuid::new_v4(); // Generate a new UUID

    // Use Diesel to insert the new user
    let result = db
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&new_user) // Clone new_user into the closure
                .execute(c) // Pass the connection
        })
        .await;

    match result {
        Ok(_) => format!("User {username} created"),
        Err(e) => format!("Error creating user: {e}"), // Print error details
    }
}
