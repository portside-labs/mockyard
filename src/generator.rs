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
        let mut rows: Vec<HashMap<String, serde_json::Value>> = Vec::with_capacity(schema.num_rows);

        for row_index in 0..schema.num_rows {
            let mut row = HashMap::new();
            for field in &schema.fields {
                let value = self.generate_field(field, row_index + 1);
                row.insert(field.name.clone(), value);
            }
            rows.push(row);
        }

        GeneratedData {
            fields: schema.fields.iter().map(|f| f.name.clone()).collect(),
            rows,
        }
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
        }
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
