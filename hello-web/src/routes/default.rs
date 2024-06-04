use crate::database::database::get_row;
pub async fn hello_world() -> &'static str {
    let row = get_row().await.unwrap();
    println!("{}", row);
    "Hello, World!"
}