//! Specialized implementation of the Domo Stream API 
//!
//! # [`DomoStreamUploadClient`](`crate::domo::data::DomoStreamUploadClient`) implements endpoints necessary for uploading data to Domo via stream executions
//! 
use std::sync::{Mutex, Arc, atomic::{Ordering, AtomicUsize}};
use serde::Serialize;
use crate::{auth::DomoAuthClient, domo::stream::StreamExecution};
use crate::error::DomoErr;
use log::{info, debug};

/// A specialized wrapper around the Domo Stream API for executions/uploads.
/// Makes it easy to upload Data to a Domo in a multi-threaded way while making use
/// of things like buffered data parts etc., so you can focus on writing code to get
/// the data to send to Domo. Especially useful for situations where the the amount of
/// data retrieved is variable. i.e. if you were pulling data from an API requiring many
/// API calls to get all the data for the Domo dataset, but each API call might return 0 to
/// N number of "rows" for your dataset.
#[derive(Clone)]
pub struct DomoStreamUploadClient {
    inner: Arc<DomoExecution>,
}

impl DomoStreamUploadClient {
    /// buffer_size is the target size in bytes for data part upload sizes. The buffers initial capacity will be set to 1.5 * buffer_size.
    pub fn new<S: Into<String>>(stream_id: usize, client_id: S, secret: S, buffer_size: usize) -> DomoStreamUploadClient {
        DomoStreamUploadClient{
            inner: Arc::new(DomoExecution::new(stream_id, client_id, secret, buffer_size))
        }
    }


    /// Uploads data to a Domo stream execution. If a stream execution hasn't been started by the DomoStreamUploadClient instance, one will be created.
    /// Data is serialized to CSV and stored in a buffer until the buffer size (bytes) specified in the constructor is reached. Once the buffer size has been reached it will be uploaded to Domo
    /// as a data part and cleared. When the commit method is called and remaining data in the buffer will be flushed and uploaded as a final data part before committing the
    /// stream execution.
    pub async fn upload<T: Serialize>(&self, data: &[T]) -> Result<Option<StreamExecution>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.inner.upload(data).await
    }

    /// Commits an active stream execution if one has been started by the DomoStreamUploadClient instance.
    /// If there is data remaining in the buffer it will be flushed and uploaded as a final data part before committing 
    /// the stream execution.
    pub async fn commit(&self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.inner.commit().await
    }
    /// Aborts any active stream execution started by the DomoStreamUploadClient instance and clears the data buffer and resets
    /// all stream execution metrics (i.e. current data part number).
    pub async fn abort(&self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.inner.abort().await
    }
}
#[derive(Debug, Default)]
pub(crate) struct InnerDomoExecution {
    pub(crate) execution_id: Option<usize>,
}
#[derive(Debug)]
pub(crate) struct DomoExecution {
    pub stream_id: usize,
    inner: Arc<Mutex<InnerDomoExecution>>,
    pub current_data_part: AtomicUsize,
    pub buffer_size: usize,
    // pub buf: Vec<T>,
    pub buf: Arc<Mutex<Vec<u8>>>,
    client: surf::Client,
    auth: DomoAuthClient,
}

impl DomoExecution {
    pub fn new<S: Into<String>>(stream_id: usize, domo_client_id: S, domo_secret: S, buffer_size: usize) -> Self {
        let buf_cap = (buffer_size * 3) / 2;
        let b = Vec::with_capacity(buf_cap);
        Self {
            stream_id,
            inner: Arc::new(Mutex::new(Default::default())),
            current_data_part: AtomicUsize::new(0),
            buffer_size: buffer_size,
            buf: Arc::new(Mutex::new(b)),
            client: surf::Client::new(),
            auth: DomoAuthClient::new(domo_client_id, domo_secret),
        }
    }

    async fn upload_data_part(&self, ex_id: usize, bod: Vec<u8>) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
            let part = self.current_data_part.fetch_add(1, Ordering::Relaxed);
            debug!("uploading data part {}", part);
            let uri = format!("https://api.domo.com/v1/streams/{}/executions/{}/part/{}", self.stream_id, ex_id, part);
            let token = &self.auth.get_token().await?;
            let req = surf::put(uri).header("Authorization", format!("Bearer {}", token)).header("Content-Type", "text/csv").body(bod);
            let res: StreamExecution = self.client.send(req).await?.body_json().await?;
            Ok(res)
    }
}

impl DomoExecution {

    async fn create_execution(&self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = format!("https://api.domo.com/v1/streams/{}/executions", self.stream_id);
        let token = &self.auth.get_token().await?;
        let req = surf::post(uri).header("Authorization", format!("Bearer {}", token)).header("Content-Type", "application/json");
        let res: StreamExecution = self.client.send(req).await?.body_json().await?;

        Ok(res)
    }

    async fn upload<T: Serialize>(&self, data: &[T]) -> Result<Option<StreamExecution>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let csv_data = crate::util::csv::serialize_csv_str(data, false)?;
        let mut b = self.buf.lock().unwrap();
        b.extend_from_slice(csv_data.as_bytes());

        if b.len() > self.buffer_size {
            let mut lock = self.inner.lock().unwrap();
            let ex_id = if lock.execution_id.is_none() {
                let ex = self.create_execution().await?;
                lock.execution_id.replace(ex.id);
                ex.id
            } else {
                let e = lock.execution_id.clone().unwrap();
                e
            };
            std::mem::drop(lock);
            let bod = b.clone();
            let res = self.upload_data_part(ex_id, bod).await?;
            b.clear();
            Ok(Some(res))
        } else {
            debug!("buffering upload...");
            Ok(None)
        }
    }


    async fn commit(&self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut lock = self.inner.lock().unwrap();
        // Check if execution has already been created. If it hasn't, i.e. the buf size for a data part
        // was smaller than all data gathered and thus a data part was never uploaded, create an execution.
        let ex_id = if lock.execution_id.is_none() {
            let ex = self.create_execution().await?;
            lock.execution_id.replace(ex.id);
            ex.id
        } else {
            let e = lock.execution_id.clone().unwrap();
            e
        };
        // flush the remaining buf and upload as a data part.
        let b = self.buf.lock().unwrap();
        let bod = b.clone();
        self.upload_data_part(ex_id, bod).await?;

        // now that we've flushed any remaining data and uploaded, commit the execution.
        info!("commiting execution {}", ex_id);
        let uri = format!("https://api.domo.com/v1/streams/{}/executions/{}/commit", self.stream_id, ex_id);
        let token = &self.auth.get_token().await?;
        let req = surf::put(uri).header("Authorization", format!("Bearer {}", token));
        let res: StreamExecution = self.client.send(req).await?.body_json().await?;
        lock.execution_id.take();
        Ok(res)
    }

    async fn abort(&self) -> Result<StreamExecution, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut lock = self.inner.lock().unwrap();
        let ex_id = lock.execution_id.clone().ok_or_else(||DomoErr("No Execution ID".into()))?;
        let uri = format!("https://api.domo.com/v1/streams/{}/executions/{}/abort", self.stream_id, ex_id);
        let token = &self.auth.get_token().await?;
        let req = surf::put(uri).header("Authorization", format!("Bearer {}", token));
        let res: StreamExecution = self.client.send(req).await?.body_json().await?;
        lock.execution_id.take();
        self.buf.lock().unwrap().clear();
        self.current_data_part.store(0, Ordering::Relaxed);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_upload() {
        smol::block_on(async {

        let start = std::time::Instant::now();
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");

        let stream_id = 5706; // Test Stream
        let domo = DomoExecution::new(stream_id, c, s, 5000);
        for  i in 0..350 {
            let rows = vec![
                TestRow{
                    fake_test_column: "1".into(),
                    fake_test_column2: "2".into(),
                    test_int: i,
                    ..Default::default()
                }
            ];
            domo.upload(&rows).await.expect("upload to succeed");
        }
        domo.commit().await.expect("commit to succeed");
        assert_ne!(350, domo.current_data_part.fetch_add(0, Ordering::Relaxed));
        println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
        })
    }

    #[test]
    fn test_stream_threaded() {

        let start = std::time::Instant::now();
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");

        let stream_id = 5706; // Test Stream
        let d = Arc::new(DomoExecution::new(stream_id, c, s, 5000));
        let mut handles = vec![];

        for thread_num in 1..11 {
            let domo = d.clone();
            let h = std::thread::spawn(move || {
                for i in 1..36 {
                    let rows = vec![
                        TestRow{
                            fake_test_column: "1".into(),
                            fake_test_column2: "2".into(),
                            test_int: i * thread_num,
                            ..Default::default()
                        }
                    ];
                    smol::block_on(async {
                        domo.upload(&rows).await.expect("upload to succeed");
                    })
                }
                ()
            });
            handles.push(h);
        }

        let mut hc = 0;
        for h in handles {
            h.join().unwrap();
            hc = hc + 1;
        }
        assert_eq!(hc, 10);
        smol::block_on(async {
            d.commit().await.expect("commit shouldn't have failed");
        });
        println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
    }

    #[test]
    fn test_domox_stream_threaded() {
        let start = std::time::Instant::now();
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");

        let stream_id = 5706; // Test Stream
        let d = DomoStreamUploadClient::new(stream_id, c, s, 5000);
        let mut handles = vec![];

        for thread_num in 1..11 {
            let domo = d.clone();
            let h = std::thread::spawn(move || {
                for i in 1..36 {
                    let rows = vec![
                        TestRow{
                            fake_test_column: "1".into(),
                            fake_test_column2: "2".into(),
                            test_int: i * thread_num,
                            ..Default::default()
                        }
                    ];
                    smol::block_on( async {
                        domo.upload(&rows).await.expect("upload to succeed");
                    })
                }
                ()
            });
            handles.push(h);
        }

        let mut hc = 0;
        for h in handles {
            h.join().unwrap();
            hc = hc + 1;
        }
        assert_eq!(hc, 10);
        smol::block_on(async {
            d.commit().await.expect("commit shouldn't have failed");
        });
        println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
    }


    #[test]
    fn test_stream_upload_works_when_the_buffer_size_is_not_reached_before_commit() {

        let start = std::time::Instant::now();
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");

        let stream_id = 5706; // Test Stream
        let d = Arc::new(DomoExecution::new(stream_id, c, s, 75000));
        let mut handles = vec![];

        for thread_num in 1..11 {
            let domo = d.clone();
            let h = std::thread::spawn(move || {
                for i in 1..36 {
                    let rows = vec![
                        TestRow{
                            fake_test_column: "1".into(),
                            fake_test_column2: "2".into(),
                            test_int: i * thread_num,
                            ..Default::default()
                        }
                    ];
                    smol::block_on(async {
                        domo.upload(&rows).await.expect("upload to succeed");
                    })
                }
                ()
            });
            handles.push(h);
        }

        let mut hc = 0;
        for h in handles {
            h.join().unwrap();
            hc = hc + 1;
        }
        assert_eq!(hc, 10);
        smol::block_on(async {
            d.commit().await.expect("commit shouldn't have failed");
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