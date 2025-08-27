pub mod api {
    include!("gen/api.rs");
}

pub mod auth {
    include!("gen/auth.rs");
}

pub mod card {
    include!("gen/card.rs");
}

pub mod merchant {
    include!("gen/merchant.rs");
}

pub mod role {
    include!("gen/role.rs");
}

pub mod saldo {
    include!("gen/saldo.rs");
}

pub mod topup {
    include!("gen/topup.rs");
}

pub mod transaction {
    include!("gen/transaction.rs");
}

pub mod transfer {
    include!("gen/transfer.rs");
}

pub mod user {
    include!("gen/user.rs");
}

pub mod withdraw {
    include!("gen/withdraw.rs");
}
