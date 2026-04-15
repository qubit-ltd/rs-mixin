# qubit-mixin

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-mixin.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-mixin)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-mixin/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-mixin?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-mixin.svg?color=blue)](https://crates.io/crates/qubit-mixin)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Trait mixins for domain objects, providing common properties and behaviors.

## Overview

`qubit-mixin` provides a collection of trait mixins that can be composed together to add common functionality to domain objects. These traits follow the principle of composition over inheritance and allow domain objects to gain required functionality by implementing different trait combinations.

## Features

- **Timestamp Traits**: Manage creation time, modification time, and deletion time
- **Identifier Traits**: Provide ID, code, name and other identification information
- **State Management Traits**: Manage status, visibility and other states
- **User Information Traits**: Provide username, email, password and other user-related information
- **Entity Association Traits**: Manage entity relationships
- **Validation & Normalization**: Provide data validation and normalization functionality

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-mixin = "0.1"
```

On crates.io the package is **`qubit-mixin`**; in Rust code, import it as **`qubit_mixin`** (hyphens map to underscores in the crate root). If you used the former **`prism3-mixin`** / **`prism3_mixin`**, switch the dependency key to `qubit-mixin` and update `use` paths to `qubit_mixin`.

## Usage

```rust
use qubit_mixin::{Identifiable, WithCode, WithName, Auditable, Creatable, Modifiable, Deletable};
use chrono::{DateTime, Utc};

struct Product {
    id: Option<i64>,
    code: String,
    name: String,
    create_time: DateTime<Utc>,
    modify_time: Option<DateTime<Utc>>,
    delete_time: Option<DateTime<Utc>>,
}

impl Identifiable for Product {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl WithCode for Product {
    fn code(&self) -> &str {
        &self.code
    }

    fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }
}

impl WithName for Product {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Creatable for Product {
    fn create_time(&self) -> DateTime<Utc> {
        self.create_time
    }

    fn set_create_time(&mut self, time: DateTime<Utc>) {
        self.create_time = time;
    }
}

impl Modifiable for Product {
    fn modify_time(&self) -> Option<DateTime<Utc>> {
        self.modify_time
    }

    fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
        self.modify_time = time;
    }
}

impl Deletable for Product {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.delete_time
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.delete_time = time;
    }
}

impl Auditable for Product {}

let mut product = Product {
    id: Some(1),
    code: "PROD001".to_string(),
    name: "Widget".to_string(),
    create_time: Utc::now(),
    modify_time: None,
    delete_time: None,
};

// Use the trait methods
assert_eq!(product.id(), Some(1));
assert_eq!(product.code(), "PROD001");
assert!(!product.is_deleted());
```

## Available Traits

### Basic Traits

- `Identifiable`: Provides unique identifier (ID)
- `WithCode`: Provides code property
- `WithName`: Provides name property
- `WithComment`: Provides comment property
- `WithEmail`: Provides email property
- `WithUsername`: Provides username property
- `WithPassword`: Provides password property
- `WithBirthday`: Provides birthday property
- `WithIndex`: Provides index property
- `WithKey`: Provides key property
- `WithSecurityKey`: Provides security key property
- `WithUdid`: Provides device unique identifier
- `WithUuid`: Provides universally unique identifier
- `WithStatus`: Provides status property
- `WithVisibility`: Provides visibility property
- `WithEntity`: Provides entity association

### Timestamp Traits

- `Creatable`: Records creation time
- `Modifiable`: Records last modification time
- `Deletable`: Records deletion time (soft delete)
- `Auditable`: Combines creation, modification, and deletion timestamps

### Functional Traits

- `Emptyful`: Provides `is_empty()` method
- `Normalizable`: Provides data normalization functionality
- `Predefinable`: Indicates whether object is predefined
- `Validatable`: Provides data validation functionality
- `Desensitizable`: Provides sensitive data desensitization
- `DataWithMaxAge`: Defines maximum age for data

### Information Traits

- `HasSpecificInfo<T>`: Provides specific information access
- `HasInfo`: Provides basic information access
- `HasInfoWithEntity<E>`: Provides basic information with entity association

### Data Structures

- `Info`: Basic information structure (ID, code, name, delete_time)
- `InfoWithEntity<E>`: Basic information with entity association

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author

Haixing Hu

