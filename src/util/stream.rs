use crate::{async_domo::streams_service::DomoStreamPitchfork, PitchforkError};
use futures::{Stream, StreamExt};
use log::{error, info};
use std::{pin::Pin, time::Instant};

pub struct StreamPartUpload {
    pub data_part_id: usize,
    pub rows: usize,
    pub upload_successful: bool,
}

/// Helper method to take a Stream of chunks of Serializable rows and upload them in data parts
/// no smaller than the specified `min_rows_per_upload_part`. This is useful if you're pulling from
/// an API that returns Vecs of the type you're creating rows from and you need to query the API multiple
/// times to get all the data to push to Domo.
///
/// # Errors
///
/// Will return `PitchforkError` if it fails to create a Domo Stream Execution or if
/// it fails to commit the stream execution. It will continue/ignore errors from the `data_pull_stream`
/// and from data part uploads to domo.
pub async fn domo_streaming_upload<T: serde::Serialize>(
    stream_id: usize,
    domo_client_id: &str,
    domo_secret: &str,
    mut data_pull_stream: Pin<&mut dyn Stream<Item = Result<Vec<T>, PitchforkError>>>,
    min_rows_per_upload_part: usize,
) -> Result<Vec<StreamPartUpload>, PitchforkError> {
    let domo = DomoStreamPitchfork::with_credentials(domo_client_id, domo_secret);
    let ds_execution = domo.create_execution(stream_id).await?;
    let exec_id = ds_execution.id;

    let mut data_part: Vec<T> = vec![];
    let mut data_part_id = 1;
    let mut upload_data: Vec<T> = vec![];
    let mut upload_part_results: Vec<StreamPartUpload> = vec![];
    let mut time_of_last_domo_part_upload = Instant::now();
    let mut total_rows: usize = 0;

    // Take items as they come into the stream and aggregate them into a data part vec.
    // Once the length of the data part vec is bigger than the `min_rows_per_upload_part`
    // move the data part into the `upload_data` and upload that to Domo.
    while let Some(item) = data_pull_stream.next().await {
        match item {
            Ok(mut t) => {
                data_part.append(&mut t);
            }
            Err(e) => error!("data pull err: {:?}", e),
        }

        // Check if enough rows have accumulated to upload a Domo data part.
        if data_part.len() >= min_rows_per_upload_part {
            let row_count = data_part.len();
            upload_data.append(&mut data_part);
            // Upload Domo data part.
            let upload = domo
                .upload(stream_id, exec_id, data_part_id, &upload_data)
                .await;
            let upload_was_successful = upload.is_ok();
            if let Err(e) = upload {
                error!("{:?}", e);
            }
            info!(
                "part upload: {} | rows: {} | was successful: {}",
                data_part_id, row_count, upload_was_successful
            );
            total_rows += row_count; // increment total row count for end log.

            // Create and push result of Domo data part upload to return at Stream completion.
            upload_part_results.push(StreamPartUpload {
                data_part_id,
                rows: row_count,
                upload_successful: upload_was_successful,
            });

            // Log elapsed time for retrieval of data and upload of data part.
            let end_time_upload_part = Instant::now();
            info!(
                "Time to collect and upload domo data part: {:?}",
                end_time_upload_part.duration_since(time_of_last_domo_part_upload)
            );
            // Reset start point for elapsed time for the next data part.
            time_of_last_domo_part_upload = end_time_upload_part;

            data_part_id += 1; // increment ID for next data part.
            upload_data.clear(); // Clear upload part data/reset for next part upload.
        }
    }

    // Create an upload part for any remaining data below min part size.
    if !data_part.is_empty() || !upload_data.is_empty() {
        let row_count = data_part.len();
        upload_data.append(&mut data_part);
        let upload = domo
            .upload(stream_id, exec_id, data_part_id, &upload_data)
            .await;
        let upload_was_successful = upload.is_ok();
        if let Err(e) = upload {
            error!("{:?}", e);
        }
        info!(
            "final part upload: {} | rows: {} | was successful: {}",
            data_part_id, row_count, upload_was_successful
        );
        total_rows += row_count;
        upload_part_results.push(StreamPartUpload {
            data_part_id,
            rows: row_count,
            upload_successful: upload_was_successful,
        });
        let end_time_upload_part = Instant::now();
        info!(
            "Time to collect and upload domo data part: {:?}",
            end_time_upload_part.duration_since(time_of_last_domo_part_upload)
        );
    }

    // Finished uploading parts, commit Domo stream execution.
    let _commit = domo.commit_execution(stream_id, exec_id).await?;
    info!(
        "Uploaded {} total rows in {} data parts",
        total_rows, data_part_id
    );

    Ok(upload_part_results)
}

/// Helper method to take a Stream of Serializable rows and upload them in data parts
/// no smaller than the specified `min_rows_per_upload_part`. Useful to take a stream
/// of individual rows and upload them in parts of a specified size.
///
/// # Errors
///
/// Will return `PitchforkError` if it fails to create a Domo Stream Execution or if
/// it fails to commit the stream execution. It will continue/ignore errors from the `data_pull_stream`
/// and from data part uploads to domo.
pub async fn domo_streaming_row_upload<T: serde::Serialize>(
    stream_id: usize,
    domo_client_id: &str,
    domo_secret: &str,
    mut data_pull_stream: Pin<&mut dyn Stream<Item = Result<T, PitchforkError>>>,
    min_rows_per_upload_part: usize,
) -> Result<Vec<StreamPartUpload>, PitchforkError> {
    let domo = DomoStreamPitchfork::with_credentials(domo_client_id, domo_secret);
    let ds_execution = domo.create_execution(stream_id).await?;
    let exec_id = ds_execution.id;

    let mut data_part: Vec<T> = vec![];
    let mut data_part_id = 1;
    let mut upload_data: Vec<T> = vec![];
    let mut upload_part_results: Vec<StreamPartUpload> = vec![];
    let mut time_of_last_domo_part_upload = Instant::now();
    let mut total_rows: usize = 0;

    // Take rows as they come into the stream and aggregate them into a data part vec.
    // Once the length of the data part vec is bigger than the `min_rows_per_upload_part`
    // move the data part into the `upload_data` and upload that to Domo.
    while let Some(item) = data_pull_stream.next().await {
        match item {
            Ok(row) => {
                data_part.push(row);
            }
            Err(e) => error!("data pull err: {:?}", e),
        }

        // Check if enough rows have accumulated to upload a Domo data part.
        if data_part.len() >= min_rows_per_upload_part {
            let row_count = data_part.len();
            upload_data.append(&mut data_part);
            // Upload Domo data part.
            let upload = domo
                .upload(stream_id, exec_id, data_part_id, &upload_data)
                .await;
            let upload_was_successful = upload.is_ok();
            if let Err(e) = upload {
                error!("{:?}", e);
            }
            info!(
                "part upload: {} | rows: {} | was successful: {}",
                data_part_id, row_count, upload_was_successful
            );
            total_rows += row_count; // increment total row count for end log.

            // Create and push result of Domo data part upload to return at Stream completion.
            upload_part_results.push(StreamPartUpload {
                data_part_id,
                rows: row_count,
                upload_successful: upload_was_successful,
            });

            // Log elapsed time for retrieval of data and upload of data part.
            let end_time_upload_part = Instant::now();
            info!(
                "Time to collect and upload domo data part: {:?}",
                end_time_upload_part.duration_since(time_of_last_domo_part_upload)
            );
            // Reset start point for elapsed time for the next data part.
            time_of_last_domo_part_upload = end_time_upload_part;

            data_part_id += 1; // increment ID for next data part.
            upload_data.clear(); // Clear upload part data/reset for next part upload.
        }
    }

    // Create an upload part for any remaining data below min part size.
    if !data_part.is_empty() || !upload_data.is_empty() {
        let row_count = data_part.len();
        upload_data.append(&mut data_part);
        let upload = domo
            .upload(stream_id, exec_id, data_part_id, &upload_data)
            .await;
        let upload_was_successful = upload.is_ok();
        if let Err(e) = upload {
            error!("{:?}", e);
        }
        info!(
            "final part upload: {} | rows: {} | was successful: {}",
            data_part_id, row_count, upload_was_successful
        );
        total_rows += row_count;
        upload_part_results.push(StreamPartUpload {
            data_part_id,
            rows: row_count,
            upload_successful: upload_was_successful,
        });
        let end_time_upload_part = Instant::now();
        info!(
            "Time to collect and upload domo data part: {:?}",
            end_time_upload_part.duration_since(time_of_last_domo_part_upload)
        );
    }

    // Finished uploading parts, commit Domo stream execution.
    let _commit = domo.commit_execution(stream_id, exec_id).await?;
    info!(
        "Uploaded {} total rows in {} data parts",
        total_rows, data_part_id
    );

    Ok(upload_part_results)
}
