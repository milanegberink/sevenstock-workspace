use lib_core::{aws::Bucket, model::ModelManager};

use crate::error::Result;

pub async fn get_presigned_url(mm: ModelManager) -> Result<String> {
    let url = mm.aws().s3().get_object().bucket(Bucket::Private)
    Ok(url)
}
