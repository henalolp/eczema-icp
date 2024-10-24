# Eczema Awareness Backend - ICP Blockchain

## Overview
This project provides a **Rust backend** for managing eczema awareness resources on the **Internet Computer (ICP)** blockchain. The backend manages operations such as creating, updating, verifying, and searching for eczema-related content (like treatments, prevention tips, diet advice, etc.). This project leverages **Candid** for data serialization and **IC-CDK** macros to interact with the Internet Computer.

---

## Features
- **Create** and manage eczema resources (e.g., treatment plans, research, testimonials).
- **Query** resources by ID, category, or search keywords.
- **Update** and verify resources.
- **List** all resources or filter them by category.
- **Delete** unwanted or outdated resources.
- **Verify** resources for accuracy and trustworthiness.
- **Persistent storage** using `RefCell` for resource management.
  
---

## Data Structures

- **EczemaResource**:  
  Represents a single resource with attributes like title, description, category, creation date, etc.
  
  ```rust
  pub struct EczemaResource {
      id: u64,
      title: String,
      description: String,
      category: ResourceCategory,
      created_at: u64,
      updated_at: u64,
      verified: bool,
  }
  ```

- **ResourceCategory**:  
  Enum representing the categories a resource can belong to.

  ```rust
  pub enum ResourceCategory {
      Treatment,
      Prevention,
      Research,
      DietAdvice,
      Testimonial,
      MedicalAdvice,
  }
  ```

- **EczemaError**:  
  Custom error types for resource operations.

  ```rust
  pub enum EczemaError {
      NotFound,
      AlreadyExists,
      InvalidInput,
      Unauthorized,
  }
  ```

---

## API Endpoints

| Endpoint                      | Type   | Description                          |
|-------------------------------|--------|--------------------------------------|
| `create_resource`             | Update | Add a new eczema resource.          |
| `get_resource(id: u64)`       | Query  | Retrieve a resource by its ID.      |
| `list_resources`              | Query  | List all available resources.       |
| `list_resources_by_category`  | Query  | List resources by their category.   |
| `update_resource(id, payload)`| Update | Modify an existing resource.        |
| `delete_resource(id: u64)`    | Update | Remove a resource by ID.            |
| `verify_resource(id: u64)`    | Update | Mark a resource as verified.        |
| `search_resources(query)`     | Query  | Search resources by title/description. |

---

## Installation

1. Ensure you have the following installed:
   - Rust: [Install Rust](https://www.rust-lang.org/tools/install)
   - DFX SDK: [Install DFX](https://internetcomputer.org/docs/current/developer-docs/quickstart/dfx-install/)
  
2. Clone the repository:
   ```bash
   git clone https://github.com/PreciousMuemi/eczema-icp.git
   cd eczema-icp
   cd app
   ```

3. Install dependencies:
   ```bash
   cargo build
   ```

4. Deploy to ICP:
   ```bash
   dfx start --background
   dfx deploy
   ```

---

## Usage

1. **Create a resource**:
   ```rust
   let payload = CreateResourcePayload {
       title: String::from("Eczema Treatment"),
       description: String::from("Details on treatment options."),
       category: ResourceCategory::Treatment,
   };
   let result = create_resource(payload);
   ```

2. **Get a resource by ID**:
   ```rust
   let resource = get_resource(1);
   ```

3. **Search resources**:
   ```rust
   let results = search_resources(String::from("treatment"));
   ```

---

## Pre-upgrade and Post-upgrade Hooks

- **`pre_upgrade`**: Handles any cleanup or state preparation before a canister upgrade.
- **`post_upgrade`**: Restores data and state after a canister upgrade.

---

## Exporting the Candid Interface
```rust
ic_cdk::export_candid!();
```

---

## License
This project is open-source under the [MIT License](https://opensource.org/licenses/MIT).

---

Feel free to contribute or raise issues for enhancements!
