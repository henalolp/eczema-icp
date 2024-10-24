use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use ic_cdk_macros::*;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Custom types for the eczema awareness system
#[derive(CandidType, Serialize, Deserialize, Clone)]
struct EczemaResource {
    id: u64,
    title: String,
    description: String,
    category: ResourceCategory,
    created_at: u64,
    updated_at: u64,
    verified: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
enum ResourceCategory {
    Treatment,
    Prevention,
    Research,
    DietAdvice,
    Testimonial,
    MedicalAdvice,
}

#[derive(CandidType, Serialize, Deserialize)]
struct CreateResourcePayload {
    title: String,
    description: String,
    category: ResourceCategory,
}

// Error handling
#[derive(CandidType, Serialize, Deserialize)]
enum EczemaError {
    NotFound,
    AlreadyExists,
    InvalidInput,
    Unauthorized,
}

type EczemaResult<T> = Result<T, EczemaError>;

// State management
thread_local! {
    static ECZEMA_RESOURCES: RefCell<HashMap<u64, EczemaResource>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);
}

// Helper function to get current timestamp
fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// CRUD Operations

#[update]
fn create_resource(payload: CreateResourcePayload) -> EczemaResult<EczemaResource> {
    NEXT_ID.with(|next_id| {
        ECZEMA_RESOURCES.with(|resources| {
            let id = *next_id.borrow();
            let timestamp = get_timestamp();

            let resource = EczemaResource {
                id,
                title: payload.title,
                description: payload.description,
                category: payload.category,
                created_at: timestamp,
                updated_at: timestamp,
                verified: false,
            };

            resources.borrow_mut().insert(id, resource.clone());
            *next_id.borrow_mut() += 1;
            Ok(resource)
        })
    })
}

#[query]
fn get_resource(id: u64) -> EczemaResult<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(EczemaError::NotFound)
    })
}

#[query]
fn list_resources() -> Vec<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .values()
            .cloned()
            .collect()
    })
}

#[query]
fn list_resources_by_category(category: ResourceCategory) -> Vec<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .values()
            .filter(|r| matches!(r.category, category))
            .cloned()
            .collect()
    })
}

#[update]
fn update_resource(id: u64, payload: CreateResourcePayload) -> EczemaResult<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        let mut resources = resources.borrow_mut();
        if let Some(resource) = resources.get_mut(&id) {
            resource.title = payload.title;
            resource.description = payload.description;
            resource.category = payload.category;
            resource.updated_at = get_timestamp();
            Ok(resource.clone())
        } else {
            Err(EczemaError::NotFound)
        }
    })
}

#[update]
fn delete_resource(id: u64) -> EczemaResult<()> {
    ECZEMA_RESOURCES.with(|resources| {
        if resources.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(EczemaError::NotFound)
        }
    })
}

#[update]
fn verify_resource(id: u64) -> EczemaResult<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        let mut resources = resources.borrow_mut();
        if let Some(resource) = resources.get_mut(&id) {
            resource.verified = true;
            resource.updated_at = get_timestamp();
            Ok(resource.clone())
        } else {
            Err(EczemaError::NotFound)
        }
    })
}

// Additional helper functions for specific eczema-related functionality

#[query]
fn search_resources(query: String) -> Vec<EczemaResource> {
    let query = query.to_lowercase();
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .values()
            .filter(|r| {
                r.title.to_lowercase().contains(&query) ||
                r.description.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    })
}

// Initialize the canister
#[init]
fn init() {
    ic_cdk::setup();
}

#[pre_upgrade]
fn pre_upgrade() {
    // Add state preservation logic if needed
}

#[post_upgrade]
fn post_upgrade() {
    // Add state restoration logic if needed
}