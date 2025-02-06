use kimainai::error::AppError;

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    println!("KimaiNai");
    Ok(())
}
