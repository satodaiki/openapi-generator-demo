//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
    AddPetResponse,
    DeletePetResponse,
    FindPetsByStatusResponse,
    FindPetsByTagsResponse,
    GetPetByIdResponse,
    UpdatePetResponse,
    UpdatePetWithFormResponse,
    UploadFileResponse,
    DeleteOrderResponse,
    GetInventoryResponse,
    GetOrderByIdResponse,
    PlaceOrderResponse,
    CreateUserResponse,
    CreateUsersWithArrayInputResponse,
    CreateUsersWithListInputResponse,
    DeleteUserResponse,
    GetUserByNameResponse,
    LoginUserResponse,
    LogoutUserResponse,
    UpdateUserResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Add a new pet to the store
    async fn add_pet(
        &self,
        pet: models::Pet,
        context: &C) -> Result<AddPetResponse, ApiError>
    {
        let context = context.clone();
        info!("add_pet({:?}) - X-Span-ID: {:?}", pet, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Deletes a pet
    async fn delete_pet(
        &self,
        pet_id: i64,
        api_key: Option<String>,
        context: &C) -> Result<DeletePetResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_pet({}, {:?}) - X-Span-ID: {:?}", pet_id, api_key, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Finds Pets by status
    async fn find_pets_by_status(
        &self,
        status: &Vec<String>,
        context: &C) -> Result<FindPetsByStatusResponse, ApiError>
    {
        let context = context.clone();
        info!("find_pets_by_status({:?}) - X-Span-ID: {:?}", status, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Finds Pets by tags
    async fn find_pets_by_tags(
        &self,
        tags: &Vec<String>,
        context: &C) -> Result<FindPetsByTagsResponse, ApiError>
    {
        let context = context.clone();
        info!("find_pets_by_tags({:?}) - X-Span-ID: {:?}", tags, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Find pet by ID
    async fn get_pet_by_id(
        &self,
        pet_id: i64,
        context: &C) -> Result<GetPetByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_pet_by_id({}) - X-Span-ID: {:?}", pet_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Update an existing pet
    async fn update_pet(
        &self,
        pet: models::Pet,
        context: &C) -> Result<UpdatePetResponse, ApiError>
    {
        let context = context.clone();
        info!("update_pet({:?}) - X-Span-ID: {:?}", pet, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Updates a pet in the store with form data
    async fn update_pet_with_form(
        &self,
        pet_id: i64,
        name: Option<String>,
        status: Option<String>,
        context: &C) -> Result<UpdatePetWithFormResponse, ApiError>
    {
        let context = context.clone();
        info!("update_pet_with_form({}, {:?}, {:?}) - X-Span-ID: {:?}", pet_id, name, status, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// uploads an image
    async fn upload_file(
        &self,
        pet_id: i64,
        additional_metadata: Option<String>,
        file: Option<swagger::ByteArray>,
        context: &C) -> Result<UploadFileResponse, ApiError>
    {
        let context = context.clone();
        info!("upload_file({}, {:?}, {:?}) - X-Span-ID: {:?}", pet_id, additional_metadata, file, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Delete purchase order by ID
    async fn delete_order(
        &self,
        order_id: String,
        context: &C) -> Result<DeleteOrderResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_order(\"{}\") - X-Span-ID: {:?}", order_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Returns pet inventories by status
    async fn get_inventory(
        &self,
        context: &C) -> Result<GetInventoryResponse, ApiError>
    {
        let context = context.clone();
        info!("get_inventory() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Find purchase order by ID
    async fn get_order_by_id(
        &self,
        order_id: i64,
        context: &C) -> Result<GetOrderByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_order_by_id({}) - X-Span-ID: {:?}", order_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Place an order for a pet
    async fn place_order(
        &self,
        order: models::Order,
        context: &C) -> Result<PlaceOrderResponse, ApiError>
    {
        let context = context.clone();
        info!("place_order({:?}) - X-Span-ID: {:?}", order, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create user
    async fn create_user(
        &self,
        user: models::User,
        context: &C) -> Result<CreateUserResponse, ApiError>
    {
        let context = context.clone();
        info!("create_user({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Creates list of users with given input array
    async fn create_users_with_array_input(
        &self,
        user: &Vec<models::User>,
        context: &C) -> Result<CreateUsersWithArrayInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_users_with_array_input({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: &Vec<models::User>,
        context: &C) -> Result<CreateUsersWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_users_with_list_input({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        context: &C) -> Result<DeleteUserResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_user(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        context: &C) -> Result<GetUserByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_user_by_name(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Logs user into the system
    async fn login_user(
        &self,
        username: String,
        password: String,
        context: &C) -> Result<LoginUserResponse, ApiError>
    {
        let context = context.clone();
        info!("login_user(\"{}\", \"{}\") - X-Span-ID: {:?}", username, password, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        context: &C) -> Result<LogoutUserResponse, ApiError>
    {
        let context = context.clone();
        info!("logout_user() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Updated user
    async fn update_user(
        &self,
        username: String,
        user: models::User,
        context: &C) -> Result<UpdateUserResponse, ApiError>
    {
        let context = context.clone();
        info!("update_user(\"{}\", {:?}) - X-Span-ID: {:?}", username, user, context.get().0.clone());
        Err("Generic failuare".into())
    }

}
