// impl<'t> ActivitiesRequestBuilder<'t, ActivityLogEntry> {
//     /// Returns a list of Domo activity log entries that meet the search criteria.
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// # use domo_pitchfork::domo::activity_log::ActivityLogSearchQuery;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// // Search for the first 1000 log entries for a user
//     /// // starting from 16 Apr 2019 8:35 PDT
//     /// let query = ActivityLogSearchQuery {
//     ///     user_id: Some(1_704_739_518),
//     ///     start: 1_555_428_851_882, // 16 Apr 2019 8:35 PDT
//     ///     end: None,
//     ///     limit: Some(1000), // Max per query is 1000.
//     ///     offset: None, // defaults to 0
//     /// };
//     /// let list = domo.audit().search(query)?;
//     /// list.iter().map(|s| println!("event text: {}", s.event_text));
//     /// # Ok::<(),PitchforkError>(())
//     /// ```
//     pub fn search(
//         mut self,
//         mut query: ActivityLogSearchQuery,
//     ) -> Result<Vec<ActivityLogEntry>, PitchforkError> {
//         let q = query.create_query_string();
//         self.url.push_str(&format!("/audit?{}", q));
//         debug!("[Activity Log API] {}", self.url);
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         let res = req.send_json()?;
//         let ds_list = serde_json::from_reader(res)?;
//         Ok(ds_list)
//     }
// }