#[derive(Clone)]
pub enum UserJoin {
    UserRole,
}

#[derive(Clone)]
pub enum UserFilter {
    NameLike(String),
    Email(String),
    IsActive(bool),
    IsDeleted(bool),
}

#[derive(Clone)]
pub enum UserOrder {
    Id,
    Name,
    Email,
    CreatedAt,
}
