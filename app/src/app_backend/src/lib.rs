use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Custom types for the eczema awareness system
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct EczemaResource {
    id: u64,
    title: String,
    description: String,
    category: ResourceCategory,
    created_at: u64,
    updated_at: u64,
    verified: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ResourceCategory {
    Treatment,
    Prevention,
    Research,
    DietAdvice,
    Testimonial,
    MedicalAdvice,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CreateResourcePayload {
    title: String,
    description: String,
    category: ResourceCategory,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum EczemaError {
    NotFound,
    AlreadyExists,
    InvalidInput(String),
    Unauthorized,
}

type EczemaResult<T> = Result<T, EczemaError>;

thread_local! {
    static ECZEMA_RESOURCES: RefCell<HashMap<u64, EczemaResource>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);
    static ADMIN: RefCell<Option<Principal>> = RefCell::new(None); // Define the admin
}

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn validate_input(title: &str, description: &str) -> Result<(), EczemaError> {
    if title.is_empty() || title.len() > 100 {
        return Err(EczemaError::InvalidInput("Title must be 1-100 characters long.".to_string()));
    }
    if description.is_empty() || description.len() > 500 {
        return Err(EczemaError::InvalidInput("Description must be 1-500 characters long.".to_string()));
    }
    Ok(())
}

fn is_admin(caller: &Principal) -> bool {
    ADMIN.with(|admin| admin.borrow().as_ref() == Some(caller))
}

#[ic_cdk_macros::update]
fn set_admin(caller: Principal) -> EczemaResult<()> {
    ADMIN.with(|admin| {
        *admin.borrow_mut() = Some(caller);
        Ok(())
    })
}

#[ic_cdk_macros::update]
fn create_resource(payload: CreateResourcePayload) -> EczemaResult<EczemaResource> {
    validate_input(&payload.title, &payload.description)?;
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

#[ic_cdk_macros::query]
fn get_resource(id: u64) -> EczemaResult<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(EczemaError::NotFound)
    })
}

#[ic_cdk_macros::query]
fn list_resources() -> Vec<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .values()
            .cloned()
            .collect()
    })
}

#[ic_cdk_macros::query]
fn list_resources_by_category(category: ResourceCategory) -> Vec<EczemaResource> {
    ECZEMA_RESOURCES.with(|resources| {
        resources
            .borrow()
            .values()
            .filter(|r| r.category == category)
            .cloned()
            .collect()
    })
}

#[ic_cdk_macros::update]
fn update_resource(id: u64, payload: CreateResourcePayload) -> EczemaResult<EczemaResource> {
    validate_input(&payload.title, &payload.description)?;
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

#[ic_cdk_macros::update]
fn delete_resource(id: u64) -> EczemaResult<()> {
    ECZEMA_RESOURCES.with(|resources| {
        if resources.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(EczemaError::NotFound)
        }
    })
}

#[ic_cdk_macros::update]
fn verify_resource(id: u64) -> EczemaResult<EczemaResource> {
    let caller = ic_cdk::caller();
    if !is_admin(&caller) {
        return Err(EczemaError::Unauthorized);
    }

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

#[ic_cdk_macros::query]
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

#[ic_cdk_macros::init]
fn init() {}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ECZEMA_RESOURCES.with(|resources| ic_cdk::storage::stable_save((resources.borrow().clone(),)).unwrap());
    NEXT_ID.with(|next_id| ic_cdk::storage::stable_save((next_id.borrow().clone(),)).unwrap());
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let (stored_resources,): (HashMap<u64, EczemaResource>,) = ic_cdk::storage::stable_restore().unwrap();
    let (stored_next_id,): (u64,) = ic_cdk::storage::stable_restore().unwrap();

    ECZEMA_RESOURCES.with(|resources| *resources.borrow_mut() = stored_resources);
    NEXT_ID.with(|next_id| *next_id.borrow_mut() = stored_next_id);
}

// Export the Candid interface
ic_cdk::export_candid!();
