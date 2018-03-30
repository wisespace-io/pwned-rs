#[derive(Debug)]
pub struct Password {
    pub found: bool,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Breach {
    pub title: String,
    pub name: String,
    pub domain: String,
    pub breach_date: String,
    pub added_date: String,
    pub pwn_count: u64,
    pub description: String,
    pub data_classes: Vec<String>,
    pub is_verified: bool,
    pub is_sensitive: bool,
    pub is_retired: bool,
    pub is_spam_list: bool,
}
