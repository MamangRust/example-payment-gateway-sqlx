use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "src/gen";

    fs::create_dir_all(out_dir)?;

    tonic_prost_build::configure()
        .build_server(true)
        .out_dir(out_dir)
        .compile_protos(
            &[
                "../../proto/api.proto",
                "../../proto/auth.proto",
                "../../proto/card.proto",
                "../../proto/merchant.proto",
                "../../proto/role.proto",
                "../../proto/saldo.proto",
                "../../proto/topup.proto",
                "../../proto/transaction.proto",
                "../../proto/transfer.proto",
                "../../proto/user.proto",
                "../../proto/withdraw.proto",
            ],
            &["../../proto"],
        )?;

    println!("cargo:rerun-if-changed=../../proto");

    Ok(())
}
