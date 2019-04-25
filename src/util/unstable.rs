use crate::auth::DomoClientAppCredentials;
use crate::domo::stream::StreamExecution;
use crate::error::DomoError;
use crate::pitchfork::DomoPitchfork;
use crossbeam;
use csv;
use futures::{Future, Stream};
use rayon::prelude::*;
use reqwest::r#async::{Client, Decoder};
use serde;
use serde::Serialize;
use std::env;
use std::io::{self, Cursor};
use std::mem;
use time::PreciseTime;
use tokio_core::reactor::Core;

#[doc(hidden)]
fn fetch() -> impl Future<Item = (), Error = ()> {
    let client = Client::new();
    // let json = |mut res : Response | {
    //     res.json()::<Vec<Dataset>>()
    // };

    client
        .get("https://domo.com")
        .send()
        .and_then(|mut res| {
            println!("{}", res.status());

            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| println!("request error: {}", err))
        .map(|body| {
            let mut body = Cursor::new(body);
            let _ = io::copy(&mut body, &mut io::stdout()).map_err(|err| {
                println!("stdout error: {}", err);
            });
        })
}
#[doc(hidden)]
fn fetch2() -> impl Future<Item = String, Error = ()> {
    let client = Client::new();
    // let json = |mut res : Response | {
    //     res.json()::<Vec<Dataset>>()
    // };

    client
        .get("https://domo.com")
        .send()
        .and_then(|mut res| {
            println!("{}", res.status());

            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| println!("request error: {}", err))
        .map(|body| {
            let mut b = vec![];
            let mut body = Cursor::new(body);
            let _ = io::copy(&mut body, &mut b).map_err(|err| {
                println!("stdout error: {}", err);
            });
            let s = String::from_utf8(b).map_err(|err| {
                println!("Error Converting from vec to utf8 str: {}", err);
            });
            s.unwrap_or_default()
        })
}

#[doc(hidden)]
pub fn fetchy() -> Result<(), DomoError> {
    let mut core = Core::new()?;
    core.run(fetch())?;
    Ok(())
}
#[doc(hidden)]
pub fn fetchy2() -> Result<String, DomoError> {
    let mut core = Core::new()?;
    let s = core.run(fetch2())?;
    Ok(s)
}

/// time runs a function given to it and measures function execution time
/// and returns the function result and time as a tuple.
#[doc(hidden)]
pub fn time<F, T>(f: F) -> (T, f64)
where
    F: FnOnce() -> T,
{
    let start = PreciseTime::now();
    let func_res = f();
    let end = PreciseTime::now();

    let walltime_nano = start
        .to(end)
        .num_nanoseconds()
        .expect("Took greater than 2^63 nanoseconds");
    let walltime_secs = walltime_nano as f64 / 1_000_000_000.0;
    (func_res, walltime_secs)
}

/// uses rayon to run a function taking a parameter of type A, returning vecs of records, for every val in Vec<A>
/// Then serializes to csv and uploads the parts concurrently via Domo Streams.
/// You'd use this method if you can retrieve the data in chunks/concurrently. If you only need to the serialization to csv
/// and upload concurrently use one of the other `upload_serializable_data...` methods instead.
/// i.e. give it a function, a vec of the input param for that function, and a stream id and it will run
/// the function multiple times, serialize the resulting Vec of data, and upload it to Domo.
#[doc(hidden)]
pub fn retrieve_and_upload_rayon<F, A, T>(f: F, a: Vec<A>, stream_id: u64) -> Result<(), DomoError>
where
    T: Serialize + Send,
    F: Send + Copy + Sync + FnOnce(A) -> Result<Vec<T>, DomoError>,
    A: Send + Clone,
{
    // Create Stream Execution:
    let token = get_domo_token();
    let domo_client = DomoPitchfork::with_token(&token);
    let execution = domo_client.streams()
        .create_stream_execution(stream_id)
        // .context("Failed to create stream execution")
        ?;
    let ex_id = execution.id;
    println!("Created Stream Execution ID {}", ex_id);
    if let Err(err) = a
        .into_par_iter()
        .enumerate()
        .map(|(csv_part, param)| {
            let data = f(param)
            // .context("failed getting data")
            ?;
            process_data(stream_id, ex_id, csv_part as u32, &data)
        })
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
        .unwrap()
    {
        println!("DomoError during data part upload: {:?}", err);
    }
    // .unwrap_or(Ok(()))

    // Commit Stream Execution.
    let _commit_result = domo_client.streams()
        .commit_execution(stream_id, execution.id)
        // .context(format!(
        //     "Failed committing execution {} on stream {}",
        //     execution.id, stream_id
        // ))
        ?;
    println!("Stream Execution Result: {:?}", _commit_result);

    Ok(())
}

/// This is the same thing as `retrieve_and_upload_rayon` but with a different implementation. Instead of using rayon,
/// this is just using crossbeam and using a fork & join threading method to parallelize it.
/// Why is there two methods that do the samething? Mostly for the sake of learning, It'll eventually get consolidated
/// once I've decided which implementation to keep.
#[doc(hidden)]
pub fn retreive_and_upload_data_par_fork<F, T, A>(
    f: F,
    a: Vec<A>,
    stream_id: u64,
) -> Result<(), DomoError>
where
    T: Serialize + Send,
    F: Send + Copy + Sync + FnOnce(A) -> Result<Vec<T>, DomoError>,
    A: Send + Clone,
{
    // Create Stream Execution:
    let token = get_domo_token();
    let domo_client = DomoPitchfork::with_token(&token);
    let execution = domo_client.streams().create_stream_execution(stream_id)?;
    let ex_id = execution.id;
    println!("Created Stream Execution ID {}", ex_id);
    let chunks: usize = 8;
    let p: Vec<(usize, A)> = a.into_iter().enumerate().collect();
    let worklists = split_vec_into_chunks(&p, chunks);
    let _ = crossbeam::scope(|scope| {
        for worklist in worklists {
            scope.spawn(move |_| {
                for (csv_part, param) in worklist {
                    match f(param) {
                        Ok(data) => {
                            if let Err(err) = process_data(stream_id, ex_id, csv_part as u32, &data)
                            {
                                println!("{:?}", err);
                            }
                        }
                        Err(err) => println!("{:?}", err),
                    };
                }
            });
        }
    });

    // Commit Stream Execution.
    let commit_result = domo_client
        .streams()
        .commit_execution(stream_id, execution.id)?;
    println!("Stream Execution Result: {:?}", commit_result);

    Ok(())
}

/// A fork & join to chunk, serialize, and upload data in parallel. I.e. it divides the data param into
/// several chunks, then fork & joins to serialize the chunks to csv and upload to Domo on different threads.
#[doc(hidden)]
pub fn upload_serializable_data_par_fork<T>(data: &[T], stream_id: u64) -> Result<(), DomoError>
where
    T: Serialize + Send + Clone,
{
    // Divide the work into several chunks.
    const NTHREADS: usize = 8;
    let worklists = split_vec_into_chunks(&data, NTHREADS);

    // Create Stream Execution:
    let token = get_domo_token();
    let domo_client = DomoPitchfork::with_token(&token);
    let execution = domo_client
        .streams()
        .create_stream_execution(stream_id)
        // .context("Failed to create stream execution")
        ?;
    let ex_id = execution.id;
    println!("Created Stream Execution ID {}", ex_id);

    // Fork & Join: Spawn a thread to handle each chunk.
    let _ = crossbeam::scope(|scope| {
        for (csv_part, worklist) in worklists.into_iter().enumerate() {
            scope.spawn(move |_| {
                process_data(stream_id, ex_id, csv_part as u32, &worklist)
                // .context("Processing Data Failed")
            });
        }
    });

    // Commit Stream Execution.
    let commit_result = domo_client
        .streams()
        .commit_execution(stream_id, execution.id)
        // .context(format!(
        //     "Failed committing execution {} on stream {}",
        //     execution.id, stream_id
        // ))
        ?;
    println!("Stream Execution Result: {:?}", commit_result);

    Ok(())
}

/// using rayon instead of crossbeam to parallelize CSV serialization + Upload.
/// You'd use this method if you already have your entire list of records to serialize into CSV/upload
/// and want to run the serialization/upload steps concurrently.
/// Similar to the other rayon vs crossbeam methods here, it's for learning's sake and will be consolidated
/// down at a later point.
#[doc(hidden)]
pub fn upload_serializable_data_rayon<T: Serialize + Send + Clone>(
    data: &[T],
    stream_id: u64,
) -> Result<(), DomoError> {
    // Divide the work into several chunks.
    const NTHREADS: usize = 8;
    let worklists = split_vec_into_chunks(&data, NTHREADS);
    // Create Stream Execution:
    let token = get_domo_token();
    let domo_client = DomoPitchfork::with_token(&token);
    let execution = domo_client
        .streams()
        .create_stream_execution(stream_id)
        // .context("Failed to create stream execution")
        ?;
    let ex_id = execution.id;
    println!("Created Stream Execution ID {}", ex_id);
    if let Err(err) = worklists
        .into_par_iter()
        .enumerate()
        .map(|(csv_part, worklist)| process_data(stream_id, ex_id, csv_part as u32, &worklist))
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
        .unwrap()
    {
        println!("DomoError during data part upload: {:?}", err);
    }
    // .unwrap_or(Ok(()))

    // Commit Stream Execution.
    let commit_result = domo_client
        .streams()
        .commit_execution(stream_id, execution.id)
        // .context(format!(
        //     "Failed committing execution {} on stream {}",
        //     execution.id, stream_id
        // ))
        ?;
    println!("Stream Execution Result: {:?}", commit_result);

    Ok(())
}

/// A simple serialize to CSV and upload it to Domo.
#[doc(hidden)]
fn process_data<T: Serialize>(
    stream_id: u64,
    ex_id: u32,
    csv_part: u32,
    data_chunk: &[T],
) -> Result<(), DomoError> {
    let csv = get_upload_csv_ser(&data_chunk)
    // .context("failed to create csv from data chunk.")
    ?;
    println!("Uploading data part {} ...", csv_part);
    upload_data_part(stream_id, ex_id, csv_part, &csv)
        // .context(format!("Failed to upload data part {}", csv_part))
        ?;
    println!("Finished Uploading Data Part {}", csv_part);
    Ok(())
}
/// Return CSV string from a Vec of Records to upload to Domo.
#[doc(hidden)]
fn get_upload_csv_ser<T: Serialize>(data: &[T]) -> Result<String, DomoError> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for record in data {
        wtr.serialize(record)
            // .context("Failed serializing record to CSV row")
            ?;
    }
    let csv_str = String::from_utf8(wtr.into_inner()?)
        // .context("Failed converting CSV wtr buffer into String")
        ?;

    Ok(csv_str)
}

#[doc(hidden)]
fn get_domo_token() -> String {
    let domo_client_id = env::var("DOMO_CLIENT_ID")
        // .context("No DOMO_CLIENT_ID Env Var found")
        .unwrap();
    let domo_secret = env::var("DOMO_SECRET")
        // .context("No DOMO_SECRET Env Var found")
        .unwrap();
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .build();
    client_creds.get_access_token()
}

/// Upload a data part to a Stream Execution.
#[doc(hidden)]
fn upload_data_part(
    stream_id: u64,
    execution_id: u32,
    csv_part: u32,
    csv: &str,
) -> Result<StreamExecution, DomoError> {
    let token = get_domo_token();
    let domo_client = DomoPitchfork::with_token(&token);
    domo_client
        .streams()
        .upload_part(stream_id, execution_id, csv_part, csv)
}

/// Utility method to break the list of Symbols into chunks for Forking.
#[doc(hidden)]
fn split_vec_into_chunks<T: Clone>(vec_to_split: &[T], chunks: usize) -> Vec<Vec<T>> {
    let chunk_size = vec_to_split.len() / chunks;
    let mut vec_chunks = Vec::new();
    for chunk in vec_to_split.chunks(chunk_size) {
        vec_chunks.push(chunk.to_vec());
    }
    vec_chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fetchy() {
        let result = fetchy();
        assert!(result.is_ok());
    }
    #[test]
    fn test_fetchy2() {
        let result = fetchy2();
        if let Ok(res) = result {
            // println!("{}", res);
            assert!(!res.is_empty());
        }
    }
}
