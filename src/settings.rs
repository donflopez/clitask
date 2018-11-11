use preferences::{AppInfo, Preferences};
use std::collections::HashMap;
use webtask::Webtask;

const APP_INFO: AppInfo = AppInfo {
    name: "clitask",
    author: "Francisco López",
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Settings {
    pub webtasks: Option<HashMap<String, Webtask>>,
    pub user: Option<String>,
}

impl Settings {
    pub fn is_configured(&self) -> bool {
        match &self.user {
            Some(_user) => true,
            None => false,
        }
    }

    pub fn configure(mut self, user: String) {
        self.user = Some(user);

        self.save(&APP_INFO, ".config");
    }

    pub fn get_webtasks(&self) -> HashMap<String, Webtask> {
        match self.clone().webtasks {
            Some(hash) => hash,
            None => HashMap::new(),
        }
    }

    pub fn insert_webtask(mut self, wt: &Webtask) {
        let mut wts = self.get_webtasks();

        wts.insert(wt.name.clone(), wt.clone());

        self.webtasks = Some(wts);

        self.save(&APP_INFO, ".config");
    }
}
