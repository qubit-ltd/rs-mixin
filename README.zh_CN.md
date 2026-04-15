# qubit-mixin

[![Crate](https://img.shields.io/crates/v/qubit-mixin.svg)](https://crates.io/crates/qubit-mixin)
[![Documentation](https://docs.rs/qubit-mixin/badge.svg)](https://docs.rs/qubit-mixin)
[![License](https://img.shields.io/crates/l/qubit-mixin.svg)](https://github.com/qubit-ltd/rs-mixin/blob/main/LICENSE)

领域对象的 Trait 混入，提供通用属性和行为。

[English](README.md)

## 概述

`qubit-mixin` 提供了一系列可以组合在一起的 trait 混入，为领域对象添加通用功能。这些 trait 遵循组合优于继承的原则，允许领域对象通过实现不同的 trait 组合来获得所需的功能。

## 功能特性

- **时间戳特质**：管理创建时间、修改时间和删除时间
- **标识符特质**：提供 ID、代码、名称等标识信息
- **状态管理特质**：管理状态、可见性等状态信息
- **用户信息特质**：提供用户名、邮箱、密码等用户相关信息
- **实体关联特质**：管理实体关联关系
- **验证与规范化**：提供数据验证和规范化功能

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-mixin = "0.1"
```

crates.io 上的包名为 **`qubit-mixin`**，在源码中请使用 **`qubit_mixin`** 作为 crate 根路径（连字符对应为下划线）。若曾使用 **`prism3-mixin`** / **`prism3_mixin`**，请将依赖改为 `qubit-mixin`，并把 `use prism3_mixin::...` 改为 `use qubit_mixin::...`。

## 使用示例

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

// 使用 trait 方法
assert_eq!(product.id(), Some(1));
assert_eq!(product.code(), "PROD001");
assert!(!product.is_deleted());
```

## 可用的 Trait

### 基础 Trait

- `Identifiable`: 提供唯一标识符（ID）
- `WithCode`: 提供代码属性
- `WithName`: 提供名称属性
- `WithComment`: 提供备注属性
- `WithEmail`: 提供电子邮件属性
- `WithUsername`: 提供用户名属性
- `WithPassword`: 提供密码属性
- `WithBirthday`: 提供生日属性
- `WithIndex`: 提供索引属性
- `WithKey`: 提供键属性
- `WithSecurityKey`: 提供安全密钥属性
- `WithUdid`: 提供设备唯一标识符
- `WithUuid`: 提供通用唯一标识符
- `WithStatus`: 提供状态属性
- `WithVisibility`: 提供可见性属性
- `WithEntity`: 提供实体关联

### 时间戳 Trait

- `Creatable`: 记录创建时间
- `Modifiable`: 记录最后修改时间
- `Deletable`: 记录删除时间（软删除）
- `Auditable`: 组合创建、修改和删除时间戳

### 功能 Trait

- `Emptyful`: 提供 `is_empty()` 方法
- `Normalizable`: 提供数据规范化功能
- `Predefinable`: 标识对象是否为预定义对象
- `Validatable`: 提供数据验证功能
- `Desensitizable`: 提供敏感数据脱敏
- `DataWithMaxAge`: 定义数据的最大有效期

### 信息 Trait

- `HasSpecificInfo<T>`: 提供特定信息访问
- `HasInfo`: 提供基本信息访问
- `HasInfoWithEntity<E>`: 提供带实体关联的基本信息访问

### 数据结构

- `Info`: 基本信息结构（ID、代码、名称、删除时间）
- `InfoWithEntity<E>`: 带实体关联的基本信息

## 许可证

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 作者

胡海星

