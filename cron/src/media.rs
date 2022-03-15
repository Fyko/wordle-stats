use egg_mode::media::{get_status, media_types, upload_media, ProgressInfo};
use egg_mode::Token;
use std::time::Duration;

use tokio::time::sleep;

pub async fn upload_image(bytes: &[u8], token: &Token) -> anyhow::Result<egg_mode::media::MediaId> {
	println!("uploading image");

	let typ = media_types::image_png();
	let handle = upload_media(bytes, &typ, token).await;
	let handle = match handle {
		Ok(handle) => handle,
		Err(e) => {
			println!("{:#?}", e);
			return Err(e.into());
		}
	};

	println!("Media uploaded");
	// Wait 60 seconds for processing
	println!("Waiting for media to finish processing..");

	for ct in 0..=60u32 {
		match get_status(handle.id.clone(), &token).await?.progress {
			None | Some(ProgressInfo::Success) => {
				println!("\nMedia sucessfully processed");
				break;
			}
			Some(ProgressInfo::Pending(_)) | Some(ProgressInfo::InProgress(_)) => {
				sleep(Duration::from_secs(1)).await;
			}
			Some(ProgressInfo::Failed(err)) => return Err(err.into()),
		}

		if ct == 60 {
			return Err(anyhow::anyhow!("Error: timeout"));
		}
	};

	return Ok(handle.id.clone());
}