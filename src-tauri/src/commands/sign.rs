use crate::TELEGRAM;
use base64_url;

#[tauri::command]
pub async fn request_code(phone: &str) -> Result<(), ()> {
    let mut telegram = TELEGRAM.lock().await;

    telegram.as_mut().unwrap().request_code(phone).await;

    Ok(())
}

#[tauri::command]
pub async fn request_qrcode() -> Result<String, ()> {
    let mut telegram = TELEGRAM.lock().await;

    let token = telegram.as_mut().unwrap().export_qrtoken().await;
    let bytes: Vec<u8> = serde_json
        ::from_str(
            &format!(
                "[{}]",
                format!("{:?}", token).split("token: [").nth(1).unwrap().split("]").next().unwrap()
            )
        )
        .unwrap();

    let url = format!("tg://login?token={}", base64_url::encode(&bytes));

    Ok(url)
}

#[tauri::command]
pub async fn get_update() -> Result<usize, ()> {
    let telegram = TELEGRAM.lock().await;
    let dialogs_amount = telegram.as_ref().unwrap().get_update().await;

    Ok(dialogs_amount)
}

#[tauri::command]
pub async fn sign_in(code: &str) -> Result<usize, ()> {
    let telegram = TELEGRAM.lock().await;
    let dialogs_amount = telegram.as_ref().unwrap().sign_in(code).await;

    Ok(dialogs_amount)
}
