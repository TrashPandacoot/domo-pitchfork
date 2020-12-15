//! Specialized implementation of the Domo Stream API 
//!
//! # [`DomoStreamUploadClient`](`crate::domo::data::DomoStreamUploadClient`) implements endpoints necessary for uploading data to Domo via stream executions
//! 
use futures::Stream;
use futures::StreamExt;
use serde::Serialize;
use crate::{auth::DomoAuthClient, domo::stream::StreamExecution};
use crate::error::DomoErr;
use log::{info, debug};

pub async fn run_domo_stream<T: Serialize, S: Into<String>>(mut data: impl Stream<Item=Result<Vec<T>, Box<dyn std::error::Error + Send + Sync + 'static>>> + Unpin, stream_id: usize, domo_client_id: S, domo_secret: S, buffer_size: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut domo = DomoStreamExecution::new(stream_id, domo_client_id, domo_secret, buffer_size);
    let mut errors = vec![];
    while let Some(outcome) = data.next().await {
        match outcome {
            Ok(upload_data) => {
                if let Err(e) = domo.upload(&upload_data).await {
                    errors.push(e);
                }
            },
            Err(e) => errors.push(e)
        }
    }
    println!("{:#?}", errors);
    domo.commit().await?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct DomoStreamExecution {
    pub stream_id: usize,
    pub execution_id: Option<usize>,
    pub current_data_part: usize,
    pub buffer_size: usize,
    pub buf: Vec<u8>,
    client: surf::Client,
    auth: DomoAuthClient,
}

impl DomoStreamExecution {
    pub fn new<S: Into<String>>(stream_id: usize, domo_client_id: S, domo_secret: S, buffer_size: usize) -> Self {
        let buf_cap = (buffer_size * 3) / 2;
        let b = Vec::with_capacity(buf_cap);
        Self {
            stream_id,
            execution_id: None,
            current_data_part: 0,
            buffer_size: buffer_size,
            buf: b,
            client: surf::Client::new(),
            auth: DomoAuthClient::new(domo_client_id, domo_secret),
        }
    }

    async fn upload_data_part(&mut self, ex_id: usize, bod: Vec<u8>) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
            self.current_data_part += 1;
            let part = self.current_data_part;
            debug!("uploading data part {}", part);
            let uri = format!("https://api.domo.com/v1/streams/{}/executions/{}/part/{}", self.stream_id, ex_id, part);
            let token = &self.auth.get_token().await?;
            let req = surf::put(uri).header("Authorization", format!("Bearer {}", token)).header("Content-Type", "text/csv").body(bod);
            let res: StreamExecution = self.client.send(req).await?.body_json().await?;
            Ok(res)
    }
}

impl DomoStreamExecution {

    async fn create_execution(&self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = format!("https://api.domo.com/v1/streams/{}/executions", self.stream_id);
        let token = &self.auth.get_token().await?;
        let req = surf::post(uri).header("Authorization", format!("Bearer {}", token)).header("Content-Type", "application/json");
        let res: StreamExecution = self.client.send(req).await?.body_json().await?;

        Ok(res)
    }

    pub async fn upload<T: Serialize>(&mut self, data: &[T]) -> Result<Option<StreamExecution>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let csv_data = crate::util::csv::serialize_csv_str(data, false)?;
        self.buf.extend_from_slice(csv_data.as_bytes());

        if self.buf.len() > self.buffer_size {
            let ex_id = if self.execution_id.is_none() {
                let ex = self.create_execution().await?;
                self.execution_id.replace(ex.id);
                ex.id
            } else {
                let e = self.execution_id.clone().unwrap();
                e
            };
            let bod = self.buf.clone();
            let res = self.upload_data_part(ex_id, bod).await?;
            self.buf.clear();
            Ok(Some(res))
        } else {
            debug!("buffering upload...");
            Ok(None)
        }
    }


    pub async fn commit(&mut self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Check if execution has already been created. If it hasn't, i.e. the buf size for a data part
        // was smaller than all data gathered and thus a data part was never uploaded, create an execution.
        let ex_id = if self.execution_id.is_none() {
            let ex = self.create_execution().await?;
            self.execution_id.replace(ex.id);
            ex.id
        } else {
            let e = self.execution_id.clone().unwrap();
            e
        };
        // flush the remaining buf and upload as a data part.
        let bod = self.buf.clone();
        self.upload_data_part(ex_id, bod).await?;

        // now that we've flushed any remaining data and uploaded, commit the execution.
        info!("commiting execution {}", ex_id);
        let uri = format!("https://api.domo.com/v1/streams/{}/executions/{}/commit", self.stream_id, ex_id);
        let token = &self.auth.get_token().await?;
        let req = surf::put(uri).header("Authorization", format!("Bearer {}", token));
        let res: StreamExecution = self.client.send(req).await?.body_json().await?;
        self.execution_id.take();
        Ok(res)
    }

    pub async fn abort(&mut self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let ex_id = self.execution_id.clone().ok_or_else(||DomoErr("No Execution ID".into()))?;
        let uri = format!("https://api.domo.com/v1/streams/{}/executions/{}/abort", self.stream_id, ex_id);
        let token = &self.auth.get_token().await?;
        let req = surf::put(uri).header("Authorization", format!("Bearer {}", token));
        let res: StreamExecution = self.client.send(req).await?.body_json().await?;
        self.execution_id.take();
        self.buf.clear();
        self.current_data_part = 0;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_datapart_for_upload(i: usize) -> Result<Vec<TestRow>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let rows = vec![
            TestRow{
                fake_test_column: "1".into(),
                fake_test_column2: "2".into(),
                test_int: i,
                ..Default::default()
            }
        ];
        Ok(rows)
    } 

    #[test]
    fn test_stream_exploratory() {
        use futures::StreamExt;
        let start = std::time::Instant::now();
        let stream_id = 5706; // Test Stream
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");

        let strm = futures::stream::iter(0usize..40) 
        .map(get_datapart_for_upload)
        .buffer_unordered(500);

        smol::block_on(async {
            run_domo_stream(strm, stream_id, c, s, 750_000).await.expect("to finish upload stream")
        });
        println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
    }

    #[test]
    fn test_stream_threaded_exploratory() {
        use futures::StreamExt;
        let start = std::time::Instant::now();
        let stream_id = 5706; // Test Stream
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");

        let strm = futures::stream::iter(0usize..40) 
        .map(move |i| async move {
            get_datapart_for_upload(i).await
        })
        .buffer_unordered(500);

        smol::block_on(async {
            run_domo_stream(strm, stream_id, c, s, 750_000).await.expect("to finish upload stream")
        });
        println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
    }

    #[derive(Debug, Default, Serialize)]
    struct TestRow {
        pub fake_test_column: String,
        pub fake_test_column2: String,
        pub test_int: usize,
        pub domo_stream_id: usize,
        pub domo_execution_id: usize,
        pub domo_datapart: usize,
        pub rows_in_part: usize,
        pub created_at: String,
    }
}