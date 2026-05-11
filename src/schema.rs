use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<FieldDefinition>,
    pub num_rows: usize,
    #[serde(default = "default_format")]
    pub format: OutputFormat,
}

fn default_format() -> OutputFormat {
    OutputFormat::Csv
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub options: FieldOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    RowNumber,
    FirstName,
    LastName,
    FullName,
    Email,
    Username,
    Phone,
    City,
    State,
    Country,
    ZipCode,
    StreetAddress,
    Latitude,
    Longitude,
    CompanyName,
    JobTitle,
    CreditCard,
    DomainName,
    Ipv4,
    Ipv6,
    MacAddress,
    UserAgent,
    Uuid,
    Color,
    HexColor,
    Date,
    DateTime,
    Time,
    Paragraph,
    Sentence,
    Word,
    Integer,
    Decimal,
    Boolean,
    Enum,
    Percentage,
    Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldOptions {
    /// Percentage of values that should be null/blank (0-100)
    #[serde(default)]
    pub blank_percentage: f64,

    /// For Boolean: percentage that should be true (0-100), default 50
    #[serde(default)]
    pub true_percentage: Option<f64>,

    /// For Integer/Decimal/Percentage/Currency: minimum value
    #[serde(default)]
    pub min: Option<f64>,

    /// For Integer/Decimal/Percentage/Currency: maximum value
    #[serde(default)]
    pub max: Option<f64>,

    /// For Enum: list of possible values with optional weights
    #[serde(default)]
    pub values: Vec<EnumValue>,

    /// For Date/DateTime: format string
    #[serde(default)]
    pub date_format: Option<String>,

    /// For Decimal/Currency: number of decimal places
    #[serde(default)]
    pub decimals: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub value: String,
    /// Weight as percentage (0-100). If weights don't sum to 100,
    /// remaining weight is distributed equally among unweighted values.
    #[serde(default)]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Csv,
    Json,
}
