/// Module containing all route handlers for the application
/// 
/// This module organizes the route handlers into logical groups:
/// - auth: Authentication-related routes (login, token validation)
/// - health: Health check endpoints
/// - redirect: URL redirection handling
/// - register: User registration endpoints
/// - shorten: URL shortening endpoints
pub mod auth;
pub mod health;
pub mod redirect;
pub mod register;
pub mod shorten;
