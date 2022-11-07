pub use crate::types::*;
use dioxus::fermi::*;
use dioxus::prelude::*;

pub struct Server {
    pub url_base: String,
    pub min_id: String,
    pub max_id: String,
}

impl Server {
    pub fn new(url_base: String) -> Self {
        Self {
            url_base,
            min_id: String::new(),
            max_id: String::new(),
        }
    }

    pub async fn fetch_newer(&mut self) -> Option<Vec<Status>> {
        let url = format!(
            "{}/api/v1/timelines/public?min_id={}&limit={}&local=true",
            self.url_base, self.min_id, 100
        );

        let response = reqwest_wasm::get(&url).await.unwrap();
        let mut status = response.json::<Vec<Status>>().await.unwrap();

        if !status.is_empty() {
            self.min_id = status[0].id.clone();
            status.iter_mut().for_each(|x| {
                    x.server_base = Some(self.url_base.clone());
                    x.interact_url = Some(format!("{}/interact/{}", self.url_base, x.id));
                }
            
            );
            Some(status)
        } else {
            None
        }
    }

    pub async fn get_icon(&self) -> Option<String> {
        let url = format!("{}/api/v1/instance", self.url_base);
        let response = reqwest_wasm::get(&url).await.unwrap();
        let instance: Instance = response.json().await.unwrap();
        instance.thumbnail
    }
}
