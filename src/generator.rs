use crate::schema::{FieldDefinition, FieldOptions, FieldType, Schema};
use fake::faker::address::en::*;
use fake::faker::chrono::en::*;
use fake::faker::company::en::*;
use fake::faker::creditcard::en::*;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::Fake;
use rand::Rng;
use std::collections::HashMap;

pub struct Generator {
    rng: rand::rngs::ThreadRng,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn generate(&mut self, schema: &Schema) -> GeneratedData {
        // Build the output field list, expanding lookups into their columns
        let mut fields: Vec<String> = Vec::new();
        for field in &schema.fields {
            if matches!(field.field_type, FieldType::Lookup) {
                let prefix = field.options.prefix.as_deref().unwrap_or("");
                for col in &field.options.columns {
                    fields.push(format!("{}{}", prefix, col));
                }
            } else {
                fields.push(field.name.clone());
            }
        }

        let mut rows: Vec<HashMap<String, serde_json::Value>> = Vec::with_capacity(schema.num_rows);

        for row_index in 0..schema.num_rows {
            let mut row = HashMap::new();
            for field in &schema.fields {
                if matches!(field.field_type, FieldType::Lookup) {
                    let lookup_values = self.generate_lookup(field);
                    for (k, v) in lookup_values {
                        row.insert(k, v);
                    }
                } else {
                    let value = self.generate_field(field, row_index + 1);
                    row.insert(field.name.clone(), value);
                }
            }
            rows.push(row);
        }

        GeneratedData { fields, rows }
    }

    fn generate_field(&mut self, field: &FieldDefinition, row_number: usize) -> serde_json::Value {
        if field.options.blank_percentage > 0.0 {
            let roll: f64 = self.rng.gen_range(0.0..100.0);
            if roll < field.options.blank_percentage {
                return serde_json::Value::Null;
            }
        }

        match &field.field_type {
            FieldType::RowNumber => serde_json::Value::Number(row_number.into()),
            FieldType::FirstName => json_string(FirstName().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::LastName => json_string(LastName().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::FullName => json_string(Name().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Email => json_string(SafeEmail().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Username => json_string(Username().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Phone => json_string(PhoneNumber().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::City => json_string(CityName().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::State => json_string(StateName().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Country => json_string(CountryName().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::ZipCode => json_string(ZipCode().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::StreetAddress => {
                let building: String = BuildingNumber().fake_with_rng(&mut self.rng);
                let street: String = StreetName().fake_with_rng(&mut self.rng);
                json_string(format!("{} {}", building, street))
            }
            FieldType::Latitude => {
                let lat: f64 = Latitude().fake_with_rng(&mut self.rng);
                serde_json::json!(lat)
            }
            FieldType::Longitude => {
                let lon: f64 = Longitude().fake_with_rng(&mut self.rng);
                serde_json::json!(lon)
            }
            FieldType::CompanyName => json_string(CompanyName().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::JobTitle => json_string(Profession().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::CreditCard => json_string(CreditCardNumber().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::DomainName => json_string(DomainSuffix().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Ipv4 => json_string(IPv4().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Ipv6 => json_string(IPv6().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::MacAddress => json_string(MACAddress().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::UserAgent => json_string(UserAgent().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Uuid => json_string(uuid::Uuid::new_v4().to_string()),
            FieldType::Color => json_string(self.generate_color_name()),
            FieldType::HexColor => json_string(self.generate_hex_color()),
            FieldType::Date => {
                let d: chrono::NaiveDate = Date().fake_with_rng(&mut self.rng);
                json_string(d.format("%Y-%m-%d").to_string())
            }
            FieldType::DateTime => {
                let dt: chrono::DateTime<chrono::Utc> = DateTime().fake_with_rng(&mut self.rng);
                json_string(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            }
            FieldType::Time => {
                let t: chrono::NaiveTime = Time().fake_with_rng(&mut self.rng);
                json_string(t.format("%H:%M:%S").to_string())
            }
            FieldType::Paragraph => json_string(Paragraph(3..6).fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Sentence => json_string(Sentence(5..12).fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Word => json_string(Word().fake_with_rng::<String, _>(&mut self.rng)),
            FieldType::Integer => self.generate_integer(&field.options),
            FieldType::Decimal => self.generate_decimal_field(&field.options),
            FieldType::Boolean => self.generate_boolean(&field.options),
            FieldType::Enum => self.generate_enum(&field.options),
            FieldType::Percentage => self.generate_percentage(&field.options),
            FieldType::Currency => self.generate_currency(&field.options),
            FieldType::Lookup => serde_json::Value::Null, // handled separately in generate()
        }
    }

    fn generate_lookup(&mut self, field: &FieldDefinition) -> Vec<(String, serde_json::Value)> {
        let prefix = field.options.prefix.as_deref().unwrap_or("");
        let columns = &field.options.columns;

        if field.options.data.is_empty() || columns.is_empty() {
            return columns
                .iter()
                .map(|c| (format!("{}{}", prefix, c), serde_json::Value::Null))
                .collect();
        }

        // Check blank percentage
        if field.options.blank_percentage > 0.0 {
            let roll: f64 = self.rng.gen_range(0.0..100.0);
            if roll < field.options.blank_percentage {
                return columns
                    .iter()
                    .map(|c| (format!("{}{}", prefix, c), serde_json::Value::Null))
                    .collect();
            }
        }

        // Pick a random row
        let row_idx = self.rng.gen_range(0..field.options.data.len());
        let data_row = &field.options.data[row_idx];

        columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                let value = data_row
                    .get(i)
                    .map(|s| json_string(s.clone()))
                    .unwrap_or(serde_json::Value::Null);
                (format!("{}{}", prefix, col), value)
            })
            .collect()
    }

    fn generate_integer(&mut self, options: &FieldOptions) -> serde_json::Value {
        let min = options.min.unwrap_or(0.0) as i64;
        let max = options.max.unwrap_or(10000.0) as i64;
        let value = self.rng.gen_range(min..=max);
        serde_json::Value::Number(value.into())
    }

    fn generate_decimal(&mut self, min: f64, max: f64, decimals: u32) -> serde_json::Value {
        let value: f64 = self.rng.gen_range(min..=max);
        let factor = 10_f64.powi(decimals as i32);
        let rounded = (value * factor).round() / factor;
        serde_json::json!(rounded)
    }

    fn generate_decimal_field(&mut self, options: &FieldOptions) -> serde_json::Value {
        let min = options.min.unwrap_or(0.0);
        let max = options.max.unwrap_or(10000.0);
        let decimals = options.decimals.unwrap_or(2);
        self.generate_decimal(min, max, decimals)
    }

    fn generate_boolean(&mut self, options: &FieldOptions) -> serde_json::Value {
        let true_pct = options.true_percentage.unwrap_or(50.0);
        let roll: f64 = self.rng.gen_range(0.0..100.0);
        serde_json::Value::Bool(roll < true_pct)
    }

    fn generate_enum(&mut self, options: &FieldOptions) -> serde_json::Value {
        if options.values.is_empty() {
            return serde_json::Value::Null;
        }

        let mut total_specified_weight = 0.0;
        let mut unweighted_count = 0;

        for v in &options.values {
            if let Some(w) = v.weight {
                total_specified_weight += w;
            } else {
                unweighted_count += 1;
            }
        }

        let remaining_weight = (100.0 - total_specified_weight).max(0.0);
        let default_weight = if unweighted_count > 0 {
            remaining_weight / unweighted_count as f64
        } else {
            0.0
        };

        let roll: f64 = self.rng.gen_range(0.0..100.0);
        let mut cumulative = 0.0;
        for v in &options.values {
            let weight = v.weight.unwrap_or(default_weight);
            cumulative += weight;
            if roll < cumulative {
                return json_string(v.value.clone());
            }
        }

        json_string(options.values.last().unwrap().value.clone())
    }

    fn generate_percentage(&mut self, options: &FieldOptions) -> serde_json::Value {
        let min = options.min.unwrap_or(0.0);
        let max = options.max.unwrap_or(100.0);
        let decimals = options.decimals.unwrap_or(2);
        self.generate_decimal(min, max, decimals)
    }

    fn generate_currency(&mut self, options: &FieldOptions) -> serde_json::Value {
        let min = options.min.unwrap_or(0.0);
        let max = options.max.unwrap_or(10000.0);
        let decimals = options.decimals.unwrap_or(2);
        self.generate_decimal(min, max, decimals)
    }

    fn generate_color_name(&mut self) -> String {
        let colors = [
            "Red", "Blue", "Green", "Yellow", "Orange", "Purple", "Pink",
            "Brown", "Black", "White", "Gray", "Cyan", "Magenta", "Teal",
            "Navy", "Maroon", "Olive", "Coral", "Salmon", "Indigo",
        ];
        colors[self.rng.gen_range(0..colors.len())].to_string()
    }

    fn generate_hex_color(&mut self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            self.rng.gen_range(0..=255u8),
            self.rng.gen_range(0..=255u8),
            self.rng.gen_range(0..=255u8)
        )
    }
}

fn json_string(s: String) -> serde_json::Value {
    serde_json::Value::String(s)
}

pub struct GeneratedData {
    pub fields: Vec<String>,
    pub rows: Vec<HashMap<String, serde_json::Value>>,
}

impl GeneratedData {
    pub fn to_csv(&self) -> Result<String, csv::Error> {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record(&self.fields)?;

        for row in &self.rows {
            let record: Vec<String> = self
                .fields
                .iter()
                .map(|f| match row.get(f) {
                    Some(serde_json::Value::Null) => String::new(),
                    Some(serde_json::Value::String(s)) => s.clone(),
                    Some(serde_json::Value::Number(n)) => n.to_string(),
                    Some(serde_json::Value::Bool(b)) => b.to_string(),
                    Some(v) => v.to_string(),
                    None => String::new(),
                })
                .collect();
            wtr.write_record(&record)?;
        }

        let bytes = wtr.into_inner().map_err(|e| e.into_error())?;
        Ok(String::from_utf8(bytes).unwrap_or_default())
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let rows: Vec<serde_json::Value> = self
            .rows
            .iter()
            .map(|row| {
                let mut map = serde_json::Map::new();
                for field in &self.fields {
                    if let Some(value) = row.get(field) {
                        map.insert(field.clone(), value.clone());
                    }
                }
                serde_json::Value::Object(map)
            })
            .collect();

        serde_json::to_string_pretty(&rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;

    fn make_field(name: &str, field_type: FieldType) -> FieldDefinition {
        FieldDefinition {
            name: name.to_string(),
            field_type,
            options: FieldOptions::default(),
        }
    }

    fn make_field_with_opts(name: &str, field_type: FieldType, options: FieldOptions) -> FieldDefinition {
        FieldDefinition {
            name: name.to_string(),
            field_type,
            options,
        }
    }

    fn make_schema(fields: Vec<FieldDefinition>, num_rows: usize) -> Schema {
        Schema {
            fields,
            num_rows,
            format: OutputFormat::Json,
        }
    }

    fn generate_values(field: &FieldDefinition, count: usize) -> Vec<serde_json::Value> {
        let schema = make_schema(vec![field.clone()], count);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        data.rows.into_iter().map(|r| r[&field.name].clone()).collect()
    }

    // ── Row Number ──

    #[test]
    fn row_number_is_sequential() {
        let field = make_field("id", FieldType::RowNumber);
        let values = generate_values(&field, 5);
        for (i, v) in values.iter().enumerate() {
            assert_eq!(v.as_u64().unwrap(), (i + 1) as u64);
        }
    }

    // ── String Field Types ──

    #[test]
    fn string_types_produce_non_empty_strings() {
        let types = vec![
            FieldType::FirstName, FieldType::LastName, FieldType::FullName,
            FieldType::Email, FieldType::Username, FieldType::Phone,
            FieldType::City, FieldType::State, FieldType::Country,
            FieldType::ZipCode, FieldType::StreetAddress,
            FieldType::CompanyName, FieldType::JobTitle, FieldType::CreditCard,
            FieldType::DomainName, FieldType::Ipv4, FieldType::Ipv6,
            FieldType::MacAddress, FieldType::UserAgent, FieldType::Uuid,
            FieldType::Color, FieldType::HexColor,
            FieldType::Date, FieldType::DateTime, FieldType::Time,
            FieldType::Paragraph, FieldType::Sentence, FieldType::Word,
        ];
        for ft in types {
            let field = make_field("test", ft.clone());
            let values = generate_values(&field, 3);
            for v in &values {
                assert!(v.is_string(), "Expected string for {:?}, got {:?}", ft, v);
                assert!(!v.as_str().unwrap().is_empty(), "Empty string for {:?}", ft);
            }
        }
    }

    // ── Email format ──

    #[test]
    fn email_contains_at_sign() {
        let field = make_field("email", FieldType::Email);
        let values = generate_values(&field, 20);
        for v in &values {
            assert!(v.as_str().unwrap().contains('@'), "Email missing @: {}", v);
        }
    }

    // ── UUID format ──

    #[test]
    fn uuid_is_valid_format() {
        let field = make_field("id", FieldType::Uuid);
        let values = generate_values(&field, 10);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(uuid::Uuid::parse_str(s).is_ok(), "Invalid UUID: {}", s);
        }
    }

    // ── Hex Color format ──

    #[test]
    fn hex_color_format() {
        let field = make_field("c", FieldType::HexColor);
        let values = generate_values(&field, 20);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(s.starts_with('#'), "Hex color should start with #: {}", s);
            assert_eq!(s.len(), 7, "Hex color should be 7 chars: {}", s);
            assert!(u32::from_str_radix(&s[1..], 16).is_ok(), "Invalid hex: {}", s);
        }
    }

    // ── Date/Time formats ──

    #[test]
    fn date_format_yyyy_mm_dd() {
        let field = make_field("d", FieldType::Date);
        let values = generate_values(&field, 10);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok(),
                "Invalid date format: {}", s);
        }
    }

    #[test]
    fn datetime_format_iso8601() {
        let field = make_field("dt", FieldType::DateTime);
        let values = generate_values(&field, 10);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(s.ends_with('Z'), "DateTime should end with Z: {}", s);
            assert!(s.contains('T'), "DateTime should contain T: {}", s);
        }
    }

    #[test]
    fn time_format_hh_mm_ss() {
        let field = make_field("t", FieldType::Time);
        let values = generate_values(&field, 10);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(chrono::NaiveTime::parse_from_str(s, "%H:%M:%S").is_ok(),
                "Invalid time format: {}", s);
        }
    }

    // ── Latitude / Longitude ──

    #[test]
    fn latitude_in_range() {
        let field = make_field("lat", FieldType::Latitude);
        let values = generate_values(&field, 50);
        for v in &values {
            let n = v.as_f64().unwrap();
            assert!((-90.0..=90.0).contains(&n), "Latitude out of range: {}", n);
        }
    }

    #[test]
    fn longitude_is_number() {
        let field = make_field("lng", FieldType::Longitude);
        let values = generate_values(&field, 50);
        for v in &values {
            assert!(v.is_f64() || v.is_i64(), "Longitude should be a number: {:?}", v);
        }
    }

    // ── Integer ──

    #[test]
    fn integer_defaults() {
        let field = make_field("n", FieldType::Integer);
        let values = generate_values(&field, 100);
        for v in &values {
            let n = v.as_i64().unwrap();
            assert!((0..=10000).contains(&n), "Integer out of default range: {}", n);
        }
    }

    #[test]
    fn integer_with_range() {
        let field = make_field_with_opts("n", FieldType::Integer, FieldOptions {
            min: Some(-50.0),
            max: Some(50.0),
            ..Default::default()
        });
        let values = generate_values(&field, 200);
        for v in &values {
            let n = v.as_i64().unwrap();
            assert!((-50..=50).contains(&n), "Integer out of specified range: {}", n);
        }
    }

    #[test]
    fn integer_negative_range() {
        let field = make_field_with_opts("n", FieldType::Integer, FieldOptions {
            min: Some(-1000.0),
            max: Some(-1.0),
            ..Default::default()
        });
        let values = generate_values(&field, 100);
        for v in &values {
            let n = v.as_i64().unwrap();
            assert!((-1000..=-1).contains(&n), "Not in negative range: {}", n);
        }
    }

    // ── Decimal ──

    #[test]
    fn decimal_respects_range() {
        let field = make_field_with_opts("d", FieldType::Decimal, FieldOptions {
            min: Some(1.0),
            max: Some(10.0),
            decimals: Some(3),
            ..Default::default()
        });
        let values = generate_values(&field, 100);
        for v in &values {
            let n = v.as_f64().unwrap();
            assert!((1.0..=10.0).contains(&n), "Decimal out of range: {}", n);
        }
    }

    #[test]
    fn decimal_precision() {
        let field = make_field_with_opts("d", FieldType::Decimal, FieldOptions {
            min: Some(0.0),
            max: Some(1.0),
            decimals: Some(2),
            ..Default::default()
        });
        let values = generate_values(&field, 100);
        for v in &values {
            let s = v.to_string();
            if let Some(dot_pos) = s.find('.') {
                let decimal_digits = s.len() - dot_pos - 1;
                assert!(decimal_digits <= 2, "Too many decimals in {}", s);
            }
        }
    }

    // ── Percentage ──

    #[test]
    fn percentage_defaults_0_to_100() {
        let field = make_field("pct", FieldType::Percentage);
        let values = generate_values(&field, 100);
        for v in &values {
            let n = v.as_f64().unwrap();
            assert!((0.0..=100.0).contains(&n), "Percentage out of default range: {}", n);
        }
    }

    // ── Currency ──

    #[test]
    fn currency_defaults() {
        let field = make_field("price", FieldType::Currency);
        let values = generate_values(&field, 100);
        for v in &values {
            let n = v.as_f64().unwrap();
            assert!((0.0..=10000.0).contains(&n), "Currency out of default range: {}", n);
        }
    }

    // ── Boolean ──

    #[test]
    fn boolean_default_distribution() {
        let field = make_field("active", FieldType::Boolean);
        let values = generate_values(&field, 1000);
        let true_count = values.iter().filter(|v| v.as_bool() == Some(true)).count();
        // With 50% default, expect roughly 400-600 out of 1000
        assert!(true_count > 350 && true_count < 650,
            "Boolean default 50% produced {} trues out of 1000", true_count);
    }

    #[test]
    fn boolean_skewed_distribution() {
        let field = make_field_with_opts("active", FieldType::Boolean, FieldOptions {
            true_percentage: Some(90.0),
            ..Default::default()
        });
        let values = generate_values(&field, 1000);
        let true_count = values.iter().filter(|v| v.as_bool() == Some(true)).count();
        assert!(true_count > 800, "90% true_percentage produced only {} trues", true_count);
    }

    #[test]
    fn boolean_always_false() {
        let field = make_field_with_opts("b", FieldType::Boolean, FieldOptions {
            true_percentage: Some(0.0),
            ..Default::default()
        });
        let values = generate_values(&field, 100);
        for v in &values {
            assert_eq!(v.as_bool(), Some(false));
        }
    }

    // ── Enum ──

    #[test]
    fn enum_empty_values_returns_null() {
        let field = make_field_with_opts("e", FieldType::Enum, FieldOptions {
            values: vec![],
            ..Default::default()
        });
        let values = generate_values(&field, 10);
        for v in &values {
            assert!(v.is_null(), "Empty enum should produce null, got {:?}", v);
        }
    }

    #[test]
    fn enum_single_value_always_selected() {
        let field = make_field_with_opts("e", FieldType::Enum, FieldOptions {
            values: vec![EnumValue { value: "Only".into(), weight: None }],
            ..Default::default()
        });
        let values = generate_values(&field, 50);
        for v in &values {
            assert_eq!(v.as_str().unwrap(), "Only");
        }
    }

    #[test]
    fn enum_weighted_distribution() {
        let field = make_field_with_opts("role", FieldType::Enum, FieldOptions {
            values: vec![
                EnumValue { value: "Admin".into(), weight: Some(10.0) },
                EnumValue { value: "User".into(), weight: Some(90.0) },
            ],
            ..Default::default()
        });
        let values = generate_values(&field, 1000);
        let admin_count = values.iter().filter(|v| v.as_str() == Some("Admin")).count();
        let user_count = values.iter().filter(|v| v.as_str() == Some("User")).count();
        assert!(admin_count > 30 && admin_count < 200,
            "10% Admin produced {} out of 1000", admin_count);
        assert!(user_count > 750,
            "90% User produced only {} out of 1000", user_count);
    }

    #[test]
    fn enum_unweighted_values_get_remaining() {
        let field = make_field_with_opts("role", FieldType::Enum, FieldOptions {
            values: vec![
                EnumValue { value: "Admin".into(), weight: Some(20.0) },
                EnumValue { value: "UserA".into(), weight: None },
                EnumValue { value: "UserB".into(), weight: None },
            ],
            ..Default::default()
        });
        let values = generate_values(&field, 1000);
        let admin_count = values.iter().filter(|v| v.as_str() == Some("Admin")).count();
        // Admin=20%, UserA=40%, UserB=40%
        assert!(admin_count > 100 && admin_count < 350,
            "Admin count {} not near 20%", admin_count);
    }

    #[test]
    fn enum_all_values_returned() {
        let field = make_field_with_opts("color", FieldType::Enum, FieldOptions {
            values: vec![
                EnumValue { value: "Red".into(), weight: None },
                EnumValue { value: "Green".into(), weight: None },
                EnumValue { value: "Blue".into(), weight: None },
            ],
            ..Default::default()
        });
        let values = generate_values(&field, 1000);
        let has_red = values.iter().any(|v| v.as_str() == Some("Red"));
        let has_green = values.iter().any(|v| v.as_str() == Some("Green"));
        let has_blue = values.iter().any(|v| v.as_str() == Some("Blue"));
        assert!(has_red && has_green && has_blue, "Not all enum values appeared");
    }

    // ── Blank Percentage ──

    #[test]
    fn blank_percentage_zero_produces_no_nulls() {
        let field = make_field("name", FieldType::FirstName);
        let values = generate_values(&field, 200);
        let null_count = values.iter().filter(|v| v.is_null()).count();
        assert_eq!(null_count, 0, "0% blank should produce no nulls");
    }

    #[test]
    fn blank_percentage_100_produces_all_nulls() {
        let field = make_field_with_opts("name", FieldType::FirstName, FieldOptions {
            blank_percentage: 100.0,
            ..Default::default()
        });
        let values = generate_values(&field, 100);
        for v in &values {
            assert!(v.is_null(), "100% blank should be null, got {:?}", v);
        }
    }

    #[test]
    fn blank_percentage_produces_some_nulls() {
        let field = make_field_with_opts("name", FieldType::FirstName, FieldOptions {
            blank_percentage: 50.0,
            ..Default::default()
        });
        let values = generate_values(&field, 1000);
        let null_count = values.iter().filter(|v| v.is_null()).count();
        assert!(null_count > 300 && null_count < 700,
            "50% blank produced {} nulls out of 1000", null_count);
    }

    // ── Color ──

    #[test]
    fn color_is_known_name() {
        let known = [
            "Red", "Blue", "Green", "Yellow", "Orange", "Purple", "Pink",
            "Brown", "Black", "White", "Gray", "Cyan", "Magenta", "Teal",
            "Navy", "Maroon", "Olive", "Coral", "Salmon", "Indigo",
        ];
        let field = make_field("c", FieldType::Color);
        let values = generate_values(&field, 50);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(known.contains(&s), "Unknown color: {}", s);
        }
    }

    // ── Street Address ──

    #[test]
    fn street_address_has_building_and_street() {
        let field = make_field("addr", FieldType::StreetAddress);
        let values = generate_values(&field, 20);
        for v in &values {
            let s = v.as_str().unwrap();
            assert!(s.contains(' '), "Street address should have spaces: {}", s);
        }
    }

    // ── Generate multiple fields ──

    #[test]
    fn generate_multiple_fields() {
        let schema = make_schema(vec![
            make_field("id", FieldType::RowNumber),
            make_field("name", FieldType::FullName),
            make_field("email", FieldType::Email),
        ], 5);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        assert_eq!(data.fields, vec!["id", "name", "email"]);
        assert_eq!(data.rows.len(), 5);
        for row in &data.rows {
            assert!(row.contains_key("id"));
            assert!(row.contains_key("name"));
            assert!(row.contains_key("email"));
        }
    }

    #[test]
    fn generate_zero_rows() {
        let schema = make_schema(vec![make_field("id", FieldType::RowNumber)], 0);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        assert_eq!(data.rows.len(), 0);
    }

    // ── CSV Output ──

    #[test]
    fn to_csv_has_header_and_rows() {
        let schema = make_schema(vec![
            make_field("id", FieldType::RowNumber),
            make_field("name", FieldType::FirstName),
        ], 3);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let csv = data.to_csv().unwrap();
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        assert_eq!(lines.len(), 4); // header + 3 rows
        assert_eq!(lines[0], "id,name");
    }

    #[test]
    fn to_csv_null_is_empty_string() {
        let schema = make_schema(vec![
            make_field_with_opts("name", FieldType::FirstName, FieldOptions {
                blank_percentage: 100.0,
                ..Default::default()
            }),
        ], 1);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let csv = data.to_csv().unwrap();
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        // CSV may quote the empty value or leave it bare
        let data_line = lines[1].trim_matches('"');
        assert!(data_line.is_empty(), "Null should be empty in CSV, got: {}", lines[1]);
    }

    #[test]
    fn to_csv_boolean_serialization() {
        let schema = make_schema(vec![
            make_field_with_opts("b", FieldType::Boolean, FieldOptions {
                true_percentage: Some(100.0),
                ..Default::default()
            }),
        ], 1);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let csv = data.to_csv().unwrap();
        assert!(csv.contains("true"));
    }

    #[test]
    fn to_csv_integer_serialization() {
        let schema = make_schema(vec![
            make_field_with_opts("n", FieldType::Integer, FieldOptions {
                min: Some(42.0),
                max: Some(42.0),
                ..Default::default()
            }),
        ], 1);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let csv = data.to_csv().unwrap();
        assert!(csv.contains("42"));
    }

    // ── JSON Output ──

    #[test]
    fn to_json_is_valid_array() {
        let schema = make_schema(vec![
            make_field("id", FieldType::RowNumber),
            make_field("name", FieldType::FirstName),
        ], 3);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let json = data.to_json().unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 3);
        for row in &parsed {
            assert!(row.is_object());
            assert!(row.get("id").is_some());
            assert!(row.get("name").is_some());
        }
    }

    #[test]
    fn to_json_contains_all_fields() {
        let schema = make_schema(vec![
            make_field("z_field", FieldType::RowNumber),
            make_field("a_field", FieldType::RowNumber),
        ], 1);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        assert_eq!(data.fields, vec!["z_field", "a_field"]);
        let json = data.to_json().unwrap();
        assert!(json.contains("z_field"), "JSON should contain z_field");
        assert!(json.contains("a_field"), "JSON should contain a_field");
    }

    #[test]
    fn to_json_null_values() {
        let schema = make_schema(vec![
            make_field_with_opts("name", FieldType::FirstName, FieldOptions {
                blank_percentage: 100.0,
                ..Default::default()
            }),
        ], 1);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let json = data.to_json().unwrap();
        assert!(json.contains("null"));
    }

    #[test]
    fn to_json_empty_rows() {
        let schema = make_schema(vec![make_field("id", FieldType::RowNumber)], 0);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let json = data.to_json().unwrap();
        assert_eq!(json.trim(), "[]");
    }

    // ── IPv4 format ──

    #[test]
    fn ipv4_has_four_octets() {
        let field = make_field("ip", FieldType::Ipv4);
        let values = generate_values(&field, 20);
        for v in &values {
            let s = v.as_str().unwrap();
            let parts: Vec<&str> = s.split('.').collect();
            assert_eq!(parts.len(), 4, "IPv4 should have 4 octets: {}", s);
        }
    }

    // ── Lookup ──

    fn make_lookup_field(prefix: Option<&str>, columns: Vec<&str>, data: Vec<Vec<&str>>) -> FieldDefinition {
        FieldDefinition {
            name: "".to_string(),
            field_type: FieldType::Lookup,
            options: FieldOptions {
                prefix: prefix.map(|s| s.to_string()),
                columns: columns.into_iter().map(|s| s.to_string()).collect(),
                data: data.into_iter().map(|r| r.into_iter().map(|s| s.to_string()).collect()).collect(),
                ..Default::default()
            },
        }
    }

    #[test]
    fn lookup_produces_correlated_columns() {
        let field = make_lookup_field(None, vec!["city", "state"], vec![
            vec!["Miami", "Florida"],
            vec!["Toronto", "Ontario"],
        ]);
        let schema = make_schema(vec![field], 50);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);

        assert_eq!(data.fields, vec!["city", "state"]);
        for row in &data.rows {
            let city = row["city"].as_str().unwrap();
            let state = row["state"].as_str().unwrap();
            match city {
                "Miami" => assert_eq!(state, "Florida"),
                "Toronto" => assert_eq!(state, "Ontario"),
                _ => panic!("Unexpected city: {}", city),
            }
        }
    }

    #[test]
    fn lookup_with_prefix() {
        let field = make_lookup_field(Some("office_"), vec!["city", "state"], vec![
            vec!["NYC", "NY"],
        ]);
        let schema = make_schema(vec![field], 3);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);

        assert_eq!(data.fields, vec!["office_city", "office_state"]);
        for row in &data.rows {
            assert_eq!(row["office_city"].as_str().unwrap(), "NYC");
            assert_eq!(row["office_state"].as_str().unwrap(), "NY");
        }
    }

    #[test]
    fn lookup_without_prefix() {
        let field = make_lookup_field(None, vec!["a", "b"], vec![vec!["x", "y"]]);
        let schema = make_schema(vec![field], 1);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        assert_eq!(data.fields, vec!["a", "b"]);
    }

    #[test]
    fn lookup_empty_data_produces_nulls() {
        let field = make_lookup_field(None, vec!["city", "state"], vec![]);
        let schema = make_schema(vec![field], 3);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        for row in &data.rows {
            assert!(row["city"].is_null());
            assert!(row["state"].is_null());
        }
    }

    #[test]
    fn lookup_blank_percentage() {
        let field = FieldDefinition {
            name: "".to_string(),
            field_type: FieldType::Lookup,
            options: FieldOptions {
                blank_percentage: 100.0,
                columns: vec!["a".to_string()],
                data: vec![vec!["val".to_string()]],
                ..Default::default()
            },
        };
        let schema = make_schema(vec![field], 20);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        for row in &data.rows {
            assert!(row["a"].is_null(), "100% blank lookup should produce nulls");
        }
    }

    #[test]
    fn lookup_mixed_with_regular_fields() {
        let schema = make_schema(vec![
            make_field("id", FieldType::RowNumber),
            make_lookup_field(None, vec!["city", "country"], vec![
                vec!["Paris", "FR"],
            ]),
            make_field("email", FieldType::Email),
        ], 3);
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        assert_eq!(data.fields, vec!["id", "city", "country", "email"]);
        for row in &data.rows {
            assert!(row["id"].is_number());
            assert_eq!(row["city"].as_str().unwrap(), "Paris");
            assert_eq!(row["country"].as_str().unwrap(), "FR");
            assert!(row["email"].as_str().unwrap().contains('@'));
        }
    }

    #[test]
    fn lookup_csv_output() {
        let schema = Schema {
            fields: vec![
                make_field("id", FieldType::RowNumber),
                make_lookup_field(None, vec!["city", "state"], vec![
                    vec!["Miami", "FL"],
                ]),
            ],
            num_rows: 2,
            format: OutputFormat::Csv,
        };
        let mut generator = Generator::new();
        let data = generator.generate(&schema);
        let csv = data.to_csv().unwrap();
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        assert_eq!(lines[0], "id,city,state");
        assert!(lines[1].starts_with("1,Miami,FL"));
        assert!(lines[2].starts_with("2,Miami,FL"));
    }
}
