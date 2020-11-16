use std::sync::Arc;
use crate::{DomoApi, domo::stream::DomoStream};

use serde::Serialize;



pub struct StreamBuilder {
    pub(crate) client: Arc<DomoApi>
}

impl StreamBuilder {
    pub fn list(self) -> StreamListBuilder {
        StreamListBuilder::new(self.client)
    }
    // pub fn get(self)
    // pub fn delete(self, stream_id: usize)
    // pub fn search(self)
    // pub fn executions()

}

#[derive(Serialize)]
pub struct StreamListBuilder {
    #[serde(skip_serializing)]
    api: Arc<DomoApi>,
    limit: Option<usize>,
    offset: Option<usize>,
    sort: Option<String>,
}

impl StreamListBuilder {
    pub fn new(client: Arc<DomoApi>) -> Self {
        Self {
            api: client,
            limit: Some(50),
            offset: None,
            sort: Some("name".to_string()),
        }
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn sort<S: Into<String>>(&mut self, sort: S) -> &mut Self {
        self.sort = Some(sort.into());
        self
    }
    pub async fn execute(&self) -> Result<Vec<DomoStream>,Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.api.auth.get_token().await?;
        let req = surf::get("https://api.domo.com/v1/streams").query(self)?.header("Authorization", format!("Bearer {}", token));
        let s = self.api.client.send(req).await?.body_json().await?;
        Ok(s)
    }
}



#[cfg(test)]
mod tests {
    use crate::DomoClient;

    use super::*;

    #[test]
    fn test_stream_list_builder() {
        smol::block_on(async {
            let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
            let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");
            
            let domo = DomoClient::new(c, s);
            let streams = domo.streams().list().execute().await.unwrap();
            // dbg!(&streams);
            assert_eq!(streams.len(), 50);
            let five_streams = domo.streams().list().limit(5).execute().await.unwrap();
            dbg!(&five_streams);
            assert_eq!(five_streams.len(), 5);

        })
    }

    #[test]
    fn test_stream_list_builder_threaded() {
        smol::block_on(async {
            let start = std::time::Instant::now();
            let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
            let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");
            let mut ds = vec![];
            let mut handles = vec![];
            
            let domo = DomoClient::new(c, s);
            for thread_num in 0..41 {
                let d = domo.clone();
                let h = std::thread::spawn(move || smol::block_on(async {
                    d.streams().list().limit(5).offset(thread_num * 5).execute().await
                }));
                handles.push(h);
            }
            for h in handles {
                let mut res = h.join().unwrap().unwrap();
                ds.append(&mut res);
            }
            dbg!(&ds);
            println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
            assert_eq!(ds.len(), 205);

        })
    }
}