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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_type_serializes_to_snake_case() {
        let ft = FieldType::RowNumber;
        let json = serde_json::to_string(&ft).unwrap();
        assert_eq!(json, "\"row_number\"");

        let ft = FieldType::FirstName;
        assert_eq!(serde_json::to_string(&ft).unwrap(), "\"first_name\"");

        let ft = FieldType::Ipv4;
        assert_eq!(serde_json::to_string(&ft).unwrap(), "\"ipv4\"");

        let ft = FieldType::MacAddress;
        assert_eq!(serde_json::to_string(&ft).unwrap(), "\"mac_address\"");

        let ft = FieldType::HexColor;
        assert_eq!(serde_json::to_string(&ft).unwrap(), "\"hex_color\"");

        let ft = FieldType::DateTime;
        assert_eq!(serde_json::to_string(&ft).unwrap(), "\"date_time\"");
    }

    #[test]
    fn field_type_deserializes_from_snake_case() {
        let ft: FieldType = serde_json::from_str("\"row_number\"").unwrap();
        assert!(matches!(ft, FieldType::RowNumber));

        let ft: FieldType = serde_json::from_str("\"street_address\"").unwrap();
        assert!(matches!(ft, FieldType::StreetAddress));

        let ft: FieldType = serde_json::from_str("\"company_name\"").unwrap();
        assert!(matches!(ft, FieldType::CompanyName));
    }

    #[test]
    fn field_type_rejects_invalid_values() {
        let result = serde_json::from_str::<FieldType>("\"not_a_type\"");
        assert!(result.is_err());
    }

    #[test]
    fn output_format_defaults_to_csv() {
        let json = r#"{"fields": [], "num_rows": 10}"#;
        let schema: Schema = serde_json::from_str(json).unwrap();
        assert_eq!(schema.format, OutputFormat::Csv);
    }

    #[test]
    fn output_format_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&OutputFormat::Csv).unwrap(), "\"csv\"");
        assert_eq!(serde_json::to_string(&OutputFormat::Json).unwrap(), "\"json\"");
    }

    #[test]
    fn output_format_deserializes_lowercase() {
        let f: OutputFormat = serde_json::from_str("\"csv\"").unwrap();
        assert_eq!(f, OutputFormat::Csv);
        let f: OutputFormat = serde_json::from_str("\"json\"").unwrap();
        assert_eq!(f, OutputFormat::Json);
    }

    #[test]
    fn field_options_defaults() {
        let opts = FieldOptions::default();
        assert_eq!(opts.blank_percentage, 0.0);
        assert!(opts.true_percentage.is_none());
        assert!(opts.min.is_none());
        assert!(opts.max.is_none());
        assert!(opts.values.is_empty());
        assert!(opts.date_format.is_none());
        assert!(opts.decimals.is_none());
    }

    #[test]
    fn field_options_deserializes_with_defaults() {
        let json = "{}";
        let opts: FieldOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.blank_percentage, 0.0);
        assert!(opts.values.is_empty());
    }

    #[test]
    fn field_definition_uses_type_rename() {
        let json = r#"{"name": "email", "type": "email"}"#;
        let fd: FieldDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(fd.name, "email");
        assert!(matches!(fd.field_type, FieldType::Email));
    }

    #[test]
    fn field_definition_options_default_when_missing() {
        let json = r#"{"name": "id", "type": "row_number"}"#;
        let fd: FieldDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(fd.options.blank_percentage, 0.0);
    }

    #[test]
    fn enum_value_weight_is_optional() {
        let json = r#"{"value": "Admin"}"#;
        let ev: EnumValue = serde_json::from_str(json).unwrap();
        assert_eq!(ev.value, "Admin");
        assert!(ev.weight.is_none());
    }

    #[test]
    fn enum_value_with_weight() {
        let json = r#"{"value": "Admin", "weight": 25.5}"#;
        let ev: EnumValue = serde_json::from_str(json).unwrap();
        assert_eq!(ev.value, "Admin");
        assert_eq!(ev.weight, Some(25.5));
    }

    #[test]
    fn full_schema_deserialization() {
        let json = r#"{
            "fields": [
                {"name": "id", "type": "row_number", "options": {}},
                {"name": "name", "type": "full_name", "options": {"blank_percentage": 10}},
                {"name": "role", "type": "enum", "options": {
                    "values": [
                        {"value": "Admin", "weight": 5},
                        {"value": "User"}
                    ]
                }},
                {"name": "score", "type": "integer", "options": {"min": -100, "max": 100}}
            ],
            "num_rows": 500,
            "format": "json"
        }"#;
        let schema: Schema = serde_json::from_str(json).unwrap();
        assert_eq!(schema.fields.len(), 4);
        assert_eq!(schema.num_rows, 500);
        assert_eq!(schema.format, OutputFormat::Json);
        assert_eq!(schema.fields[1].options.blank_percentage, 10.0);
        assert_eq!(schema.fields[2].options.values.len(), 2);
        assert_eq!(schema.fields[2].options.values[0].weight, Some(5.0));
        assert!(schema.fields[2].options.values[1].weight.is_none());
        assert_eq!(schema.fields[3].options.min, Some(-100.0));
        assert_eq!(schema.fields[3].options.max, Some(100.0));
    }

    #[test]
    fn schema_serialization_roundtrip() {
        let schema = Schema {
            fields: vec![FieldDefinition {
                name: "test".to_string(),
                field_type: FieldType::Boolean,
                options: FieldOptions {
                    true_percentage: Some(80.0),
                    ..Default::default()
                },
            }],
            num_rows: 10,
            format: OutputFormat::Json,
        };
        let json = serde_json::to_string(&schema).unwrap();
        let deserialized: Schema = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.fields.len(), 1);
        assert_eq!(deserialized.num_rows, 10);
        assert_eq!(deserialized.format, OutputFormat::Json);
        assert_eq!(deserialized.fields[0].options.true_percentage, Some(80.0));
    }

    #[test]
    fn all_field_types_deserialize() {
        let types = [
            "row_number", "first_name", "last_name", "full_name", "email",
            "username", "phone", "city", "state", "country", "zip_code",
            "street_address", "latitude", "longitude", "company_name",
            "job_title", "credit_card", "domain_name", "ipv4", "ipv6",
            "mac_address", "user_agent", "uuid", "color", "hex_color",
            "date", "date_time", "time", "paragraph", "sentence", "word",
            "integer", "decimal", "boolean", "enum", "percentage", "currency",
        ];
        for t in types {
            let json = format!("\"{}\"", t);
            let result = serde_json::from_str::<FieldType>(&json);
            assert!(result.is_ok(), "Failed to deserialize field type: {}", t);
        }
    }
}
