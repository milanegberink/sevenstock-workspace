use aws_sdk_s3::presigning::PresigningConfig;
use lib_core::{model::ModelManager, model::aws::Bucket};

use crate::error::Result;

// pub async fn get_presigned_url(mm: ModelManager) -> Result<String> {
//     let presign_config = PresigningConfig;

//     let url = mm.aws().s3().put_object().bucket(Bucket::Private).key("uploads/test")
//     Ok(url)
// }
