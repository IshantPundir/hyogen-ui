use std::{collections::HashMap, fs, path::Path};

use serde_json::Value;

use super::hvf::HVF;

#[derive(Debug)]
pub enum HvfError {
    InvalidPath,
    InvalidFormat,
    ParseError,
}

pub struct HVFLoader {
    // values: HashMap<String, HVF>,
    values: HashMap<String, HashMap<String, HVF>>
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
                let mut hvf_values = HashMap::new();

                // Populate the `values` HashMap from the JSON object
                if let Some(obj) = class_data.as_object() {
                    for (id, data) in obj.iter() {
                        hvf_values.insert(id.clone(), HVF::new(data.as_array().unwrap()));
                    }
                }

                // let hvf = HVF::new(class_data, class_id.clone());
                values.insert(class_id.clone(), hvf_values);
            }
        }

        Ok(Self { values })
    }

    pub fn get(&self, class: &str, id: &str) -> Option<&HVF> {
        // Retrieve the HVF by class, then retrieve the value by ID
        self.values.get(class).and_then(|hvf| hvf.get(id))
    }
}
