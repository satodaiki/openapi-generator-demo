#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/v2";
pub const API_VERSION: &'static str = "1.0.0";

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AddPetResponse {
    /// successful operation
    SuccessfulOperation
    (models::Pet)
    ,
    /// Invalid input
    InvalidInput
}

#[derive(Debug, PartialEq)]
pub enum DeletePetResponse {
    /// Invalid pet value
    InvalidPetValue
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum FindPetsByStatusResponse {
    /// successful operation
    SuccessfulOperation
    (Vec<models::Pet>)
    ,
    /// Invalid status value
    InvalidStatusValue
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum FindPetsByTagsResponse {
    /// successful operation
    SuccessfulOperation
    (Vec<models::Pet>)
    ,
    /// Invalid tag value
    InvalidTagValue
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetPetByIdResponse {
    /// successful operation
    SuccessfulOperation
    (models::Pet)
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Pet not found
    PetNotFound
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdatePetResponse {
    /// successful operation
    SuccessfulOperation
    (models::Pet)
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Pet not found
    PetNotFound
    ,
    /// Validation exception
    ValidationException
}

#[derive(Debug, PartialEq)]
pub enum UpdatePetWithFormResponse {
    /// Invalid input
    InvalidInput
}

#[derive(Debug, PartialEq)]
pub enum UploadFileResponse {
    /// successful operation
    SuccessfulOperation
    (models::ApiResponse)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteOrderResponse {
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Order not found
    OrderNotFound
}

#[derive(Debug, PartialEq)]
pub enum GetInventoryResponse {
    /// successful operation
    SuccessfulOperation
    (std::collections::HashMap<String, i32>)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetOrderByIdResponse {
    /// successful operation
    SuccessfulOperation
    (models::Order)
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Order not found
    OrderNotFound
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum PlaceOrderResponse {
    /// successful operation
    SuccessfulOperation
    (models::Order)
    ,
    /// Invalid Order
    InvalidOrder
}

#[derive(Debug, PartialEq)]
pub enum CreateUserResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq)]
pub enum CreateUsersWithArrayInputResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq)]
pub enum CreateUsersWithListInputResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteUserResponse {
    /// Invalid username supplied
    InvalidUsernameSupplied
    ,
    /// User not found
    UserNotFound
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetUserByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::User)
    ,
    /// Invalid username supplied
    InvalidUsernameSupplied
    ,
    /// User not found
    UserNotFound
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum LoginUserResponse {
    /// successful operation
    SuccessfulOperation
    {
        body: String,
        set_cookie:
        Option<
        String
        >
        ,
        x_rate_limit:
        Option<
        i32
        >
        ,
        x_expires_after:
        Option<
        chrono::DateTime::<chrono::Utc>
        >
    }
    ,
    /// Invalid username/password supplied
    InvalidUsername
}

#[derive(Debug, PartialEq)]
pub enum LogoutUserResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateUserResponse {
    /// Invalid user supplied
    InvalidUserSupplied
    ,
    /// User not found
    UserNotFound
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Add a new pet to the store
    async fn add_pet(
        &self,
        pet: models::Pet,
        context: &C) -> Result<AddPetResponse, ApiError>;

    /// Deletes a pet
    async fn delete_pet(
        &self,
        pet_id: i64,
        api_key: Option<String>,
        context: &C) -> Result<DeletePetResponse, ApiError>;

    /// Finds Pets by status
    async fn find_pets_by_status(
        &self,
        status: &Vec<String>,
        context: &C) -> Result<FindPetsByStatusResponse, ApiError>;

    /// Finds Pets by tags
    async fn find_pets_by_tags(
        &self,
        tags: &Vec<String>,
        context: &C) -> Result<FindPetsByTagsResponse, ApiError>;

    /// Find pet by ID
    async fn get_pet_by_id(
        &self,
        pet_id: i64,
        context: &C) -> Result<GetPetByIdResponse, ApiError>;

    /// Update an existing pet
    async fn update_pet(
        &self,
        pet: models::Pet,
        context: &C) -> Result<UpdatePetResponse, ApiError>;

    /// Updates a pet in the store with form data
    async fn update_pet_with_form(
        &self,
        pet_id: i64,
        name: Option<String>,
        status: Option<String>,
        context: &C) -> Result<UpdatePetWithFormResponse, ApiError>;

    /// uploads an image
    async fn upload_file(
        &self,
        pet_id: i64,
        additional_metadata: Option<String>,
        file: Option<swagger::ByteArray>,
        context: &C) -> Result<UploadFileResponse, ApiError>;

    /// Delete purchase order by ID
    async fn delete_order(
        &self,
        order_id: String,
        context: &C) -> Result<DeleteOrderResponse, ApiError>;

    /// Returns pet inventories by status
    async fn get_inventory(
        &self,
        context: &C) -> Result<GetInventoryResponse, ApiError>;

    /// Find purchase order by ID
    async fn get_order_by_id(
        &self,
        order_id: i64,
        context: &C) -> Result<GetOrderByIdResponse, ApiError>;

    /// Place an order for a pet
    async fn place_order(
        &self,
        order: models::Order,
        context: &C) -> Result<PlaceOrderResponse, ApiError>;

    /// Create user
    async fn create_user(
        &self,
        user: models::User,
        context: &C) -> Result<CreateUserResponse, ApiError>;

    /// Creates list of users with given input array
    async fn create_users_with_array_input(
        &self,
        user: &Vec<models::User>,
        context: &C) -> Result<CreateUsersWithArrayInputResponse, ApiError>;

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: &Vec<models::User>,
        context: &C) -> Result<CreateUsersWithListInputResponse, ApiError>;

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        context: &C) -> Result<DeleteUserResponse, ApiError>;

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        context: &C) -> Result<GetUserByNameResponse, ApiError>;

    /// Logs user into the system
    async fn login_user(
        &self,
        username: String,
        password: String,
        context: &C) -> Result<LoginUserResponse, ApiError>;

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        context: &C) -> Result<LogoutUserResponse, ApiError>;

    /// Updated user
    async fn update_user(
        &self,
        username: String,
        user: models::User,
        context: &C) -> Result<UpdateUserResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Add a new pet to the store
    async fn add_pet(
        &self,
        pet: models::Pet,
        ) -> Result<AddPetResponse, ApiError>;

    /// Deletes a pet
    async fn delete_pet(
        &self,
        pet_id: i64,
        api_key: Option<String>,
        ) -> Result<DeletePetResponse, ApiError>;

    /// Finds Pets by status
    async fn find_pets_by_status(
        &self,
        status: &Vec<String>,
        ) -> Result<FindPetsByStatusResponse, ApiError>;

    /// Finds Pets by tags
    async fn find_pets_by_tags(
        &self,
        tags: &Vec<String>,
        ) -> Result<FindPetsByTagsResponse, ApiError>;

    /// Find pet by ID
    async fn get_pet_by_id(
        &self,
        pet_id: i64,
        ) -> Result<GetPetByIdResponse, ApiError>;

    /// Update an existing pet
    async fn update_pet(
        &self,
        pet: models::Pet,
        ) -> Result<UpdatePetResponse, ApiError>;

    /// Updates a pet in the store with form data
    async fn update_pet_with_form(
        &self,
        pet_id: i64,
        name: Option<String>,
        status: Option<String>,
        ) -> Result<UpdatePetWithFormResponse, ApiError>;

    /// uploads an image
    async fn upload_file(
        &self,
        pet_id: i64,
        additional_metadata: Option<String>,
        file: Option<swagger::ByteArray>,
        ) -> Result<UploadFileResponse, ApiError>;

    /// Delete purchase order by ID
    async fn delete_order(
        &self,
        order_id: String,
        ) -> Result<DeleteOrderResponse, ApiError>;

    /// Returns pet inventories by status
    async fn get_inventory(
        &self,
        ) -> Result<GetInventoryResponse, ApiError>;

    /// Find purchase order by ID
    async fn get_order_by_id(
        &self,
        order_id: i64,
        ) -> Result<GetOrderByIdResponse, ApiError>;

    /// Place an order for a pet
    async fn place_order(
        &self,
        order: models::Order,
        ) -> Result<PlaceOrderResponse, ApiError>;

    /// Create user
    async fn create_user(
        &self,
        user: models::User,
        ) -> Result<CreateUserResponse, ApiError>;

    /// Creates list of users with given input array
    async fn create_users_with_array_input(
        &self,
        user: &Vec<models::User>,
        ) -> Result<CreateUsersWithArrayInputResponse, ApiError>;

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: &Vec<models::User>,
        ) -> Result<CreateUsersWithListInputResponse, ApiError>;

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        ) -> Result<DeleteUserResponse, ApiError>;

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        ) -> Result<GetUserByNameResponse, ApiError>;

    /// Logs user into the system
    async fn login_user(
        &self,
        username: String,
        password: String,
        ) -> Result<LoginUserResponse, ApiError>;

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        ) -> Result<LogoutUserResponse, ApiError>;

    /// Updated user
    async fn update_user(
        &self,
        username: String,
        user: models::User,
        ) -> Result<UpdateUserResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Add a new pet to the store
    async fn add_pet(
        &self,
        pet: models::Pet,
        ) -> Result<AddPetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().add_pet(pet, &context).await
    }

    /// Deletes a pet
    async fn delete_pet(
        &self,
        pet_id: i64,
        api_key: Option<String>,
        ) -> Result<DeletePetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_pet(pet_id, api_key, &context).await
    }

    /// Finds Pets by status
    async fn find_pets_by_status(
        &self,
        status: &Vec<String>,
        ) -> Result<FindPetsByStatusResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().find_pets_by_status(status, &context).await
    }

    /// Finds Pets by tags
    async fn find_pets_by_tags(
        &self,
        tags: &Vec<String>,
        ) -> Result<FindPetsByTagsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().find_pets_by_tags(tags, &context).await
    }

    /// Find pet by ID
    async fn get_pet_by_id(
        &self,
        pet_id: i64,
        ) -> Result<GetPetByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_pet_by_id(pet_id, &context).await
    }

    /// Update an existing pet
    async fn update_pet(
        &self,
        pet: models::Pet,
        ) -> Result<UpdatePetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_pet(pet, &context).await
    }

    /// Updates a pet in the store with form data
    async fn update_pet_with_form(
        &self,
        pet_id: i64,
        name: Option<String>,
        status: Option<String>,
        ) -> Result<UpdatePetWithFormResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_pet_with_form(pet_id, name, status, &context).await
    }

    /// uploads an image
    async fn upload_file(
        &self,
        pet_id: i64,
        additional_metadata: Option<String>,
        file: Option<swagger::ByteArray>,
        ) -> Result<UploadFileResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().upload_file(pet_id, additional_metadata, file, &context).await
    }

    /// Delete purchase order by ID
    async fn delete_order(
        &self,
        order_id: String,
        ) -> Result<DeleteOrderResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_order(order_id, &context).await
    }

    /// Returns pet inventories by status
    async fn get_inventory(
        &self,
        ) -> Result<GetInventoryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_inventory(&context).await
    }

    /// Find purchase order by ID
    async fn get_order_by_id(
        &self,
        order_id: i64,
        ) -> Result<GetOrderByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_order_by_id(order_id, &context).await
    }

    /// Place an order for a pet
    async fn place_order(
        &self,
        order: models::Order,
        ) -> Result<PlaceOrderResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().place_order(order, &context).await
    }

    /// Create user
    async fn create_user(
        &self,
        user: models::User,
        ) -> Result<CreateUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_user(user, &context).await
    }

    /// Creates list of users with given input array
    async fn create_users_with_array_input(
        &self,
        user: &Vec<models::User>,
        ) -> Result<CreateUsersWithArrayInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_users_with_array_input(user, &context).await
    }

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: &Vec<models::User>,
        ) -> Result<CreateUsersWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_users_with_list_input(user, &context).await
    }

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        ) -> Result<DeleteUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_user(username, &context).await
    }

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        ) -> Result<GetUserByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_user_by_name(username, &context).await
    }

    /// Logs user into the system
    async fn login_user(
        &self,
        username: String,
        password: String,
        ) -> Result<LoginUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().login_user(username, password, &context).await
    }

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        ) -> Result<LogoutUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().logout_user(&context).await
    }

    /// Updated user
    async fn update_user(
        &self,
        username: String,
        user: models::User,
        ) -> Result<UpdateUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_user(username, user, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
