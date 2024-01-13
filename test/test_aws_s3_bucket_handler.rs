

#[cfg(test)]
mod tests {
    use std::env;

    use aws_sdk_s3::Error;
    use colored::Colorize;
    use log::info;

    use common_modules::are_env_vars_set;
    use common_modules::aws_connectors::aws_s3_bucket_handler::AwsS3BucketHandler;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_aws_s3_storage_methods() -> Result<(), Error> {
        env_logger::init();
        let mut colored_string: colored::ColoredString;

        let env_file_path = "secrets.cfg";
        dotenv::from_path(env_file_path).ok();

        let env_vars_to_check = [
            "AWS_ACCESS_KEY_ID",
            "AWS_SECRET_ACCESS_KEY",
            "AWS_DEFAULT_REGION",
            "AWS_ENDPOINT_URL",
            "AWS_BUCKET_NAME",
        ];

        if are_env_vars_set(&env_vars_to_check) {
            colored_string = "All environment variables are set.".blue();
            info!("{}", colored_string);
        } else {
            colored_string = "Error: Some or all environment variables are not set.".red();
            panic!("{}", colored_string);
        }

        let config = aws_config::load_from_env().await;

        let mut region = String::from("eu-central-1");
        colored_string = "Error: AWS_DEFAULT_REGION environment variable expected".red();
        region = std::env::var("AWS_DEFAULT_REGION").expect(&colored_string.to_string());

        colored_string = "Error: AWS_BUCKET_NAME environment variable expected".red();
        let bucket_name = std::env::var("AWS_BUCKET_NAME").expect(&colored_string.to_string());

        let blob_name = "sample.txt"; // AWS S3 Bucket terminology would be "key" for blob_name
        let upload_file_path = "assets/sample.txt";
        let download_file_path = "output/sample-copy.txt";

        let aws_s3_bucket_handler = AwsS3BucketHandler::new(&bucket_name, &region).await?;
        aws_s3_bucket_handler.create_bucket().await?;
        aws_s3_bucket_handler.show_buckets().await?;
        aws_s3_bucket_handler.upload_blob(&blob_name, &upload_file_path).await?;

        aws_s3_bucket_handler.download_blob(&blob_name, &download_file_path).await?;
        aws_s3_bucket_handler.delete_blob(&blob_name).await?;
        aws_s3_bucket_handler.delete_bucket().await?;

        Ok(())
    }
}