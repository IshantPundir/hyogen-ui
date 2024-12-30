use std::{collections::HashMap, fs, path::Path};

use serde_json::Value;

#[derive(Debug)]
pub enum HvfError {
    InvalidPath,
    InvalidFormat,
    ParseError,
}

pub struct HVF {
    class_id: String,
    values: HashMap<String, Value>,
}

impl HVF {
    fn new(class: &Value, class_id: String) -> Self {
        let mut values = HashMap::new();

        // Populate the `values` HashMap from the JSON object
        if let Some(obj) = class.as_object() {
            for (id, data) in obj.iter() {
                values.insert(id.clone(), data.clone());
            }
        }

        Self { class_id, values }
    }

    pub fn class(&self) -> &str {
        &self.class_id
    }

    pub fn get(&self, id: &str) -> Option<&Value> {
        self.values.get(id)
    }
}

pub struct HVFLoader {
    values: HashMap<String, HVF>,
}

impl HVFLoader {
    pub fn new(file: &str) -> Result<Self, HvfError> {
        let path = Path::new(file);

        // Check if path exists
        if !path.exists() {
            tracing::error!("File {} does not exist", file);
            return Err(HvfError::InvalidPath);
        }

        // Check if it's a valid HVF file.
        if path.extension().and_then(|ext| ext.to_str()) != Some("hvf") {
            tracing::error!("Invalid hyogen file provided!");
            return Err(HvfError::InvalidFormat);
        }

        // Load the HVF file
        let content = fs::read_to_string(path).map_err(|_| HvfError::ParseError)?;

        // Parse the HVF file as JSON
        let data: Value = serde_json::from_str(&content).map_err(|_| HvfError::ParseError)?;

        let mut values = HashMap::new();

        // Parse through each class in the HVF structure
        if let Some(obj) = data.as_object() {
            for (class_id, class_data) in obj.iter() {
                let hvf = HVF::new(class_data, class_id.clone());
                values.insert(class_id.clone(), hvf);
            }
        }

        Ok(Self { values })
    }

    pub fn get(&self, class: &str, id: &str) -> Option<&Value> {
        // Retrieve the HVF by class, then retrieve the value by ID
        self.values.get(class).and_then(|hvf| hvf.get(id))
    }
}
