use serde::Serialize;

#[derive(Serialize)]
pub struct Policy {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
}
impl Policy {
    pub fn new() -> Self {
        Self {
            version: "2012-10-17".to_string(),
            statement: vec![],
        }
    }

    pub fn add_statement(mut self, statement: Statement) -> Self {
        self.statement.push(statement);
        self
    }
}

#[derive(Serialize)]
pub struct Statement {
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Action")]
    pub action: Vec<String>,
    #[serde(rename = "Resource")]
    pub resource: String,
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
}
impl Statement {
    pub fn new(
        effect: String,
        action: Vec<String>,
        resource: String,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            effect,
            action,
            resource,
            condition,
        }
    }
}

#[derive(Serialize)]
pub struct Condition {
    #[serde(rename = "StringLike")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_like: Option<StringLike>,
}
impl Condition {
    pub fn new() -> Self {
        Self {
            string_like: Default::default(),
        }
    }

    pub fn string_like(mut self, string_like: StringLike) -> Self {
        self.string_like = Some(string_like);
        self
    }
}

#[derive(Serialize)]
pub struct StringLike {
    #[serde(rename = "s3:prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
impl StringLike {
    pub fn new() -> Self {
        Self {
            prefix: Default::default(),
        }
    }

    pub fn prefix(mut self, prefix: String) -> Self {
        self.prefix = Some(prefix);
        self
    }
}

pub struct HeadObjectResponse {
    pub s3_path: String,
}

pub struct Session {
    pub bucket: String,
    pub prefix: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
}
