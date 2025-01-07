use serde_json::Value;


#[derive(Debug, Clone)]
pub struct HVF {
    // pub values: Vec<Value>
    pub values: Vec<Vec<Vec<f64>>>,
    pub len: usize
}


impl HVF {
    pub fn new(value: &Vec<Value>) -> Self {
        let sub_paths = value.len();

        let mut values: Vec<Vec<Vec<f64>>> = Vec::new();
        
        for sub_path in value {
            let path = Self::convert_to_vec_of_vec(sub_path.as_array().unwrap().to_owned()).unwrap();
            
            values.push(path);
        }

        Self {
            values,
            len: sub_paths
        }
    }

    fn convert_to_vec_of_vec(values: Vec<Value>) -> Result<Vec<Vec<f64>>, String> {
        values
            .into_iter()
            .map(|value| {
                if let Value::Array(array) = value {
                    array
                        .into_iter()
                        .map(|inner_value| {
                            if let Value::Number(num) = inner_value {
                                num.as_f64().ok_or("Failed to convert number to f64".to_string())
                            } else {
                                Err("Expected a number".to_string())
                            }
                        })
                        .collect::<Result<Vec<f64>, String>>()
                } else {
                    Err("Expected an array".to_string())
                }
            })
            .collect()
    }
 
    pub fn values(&self) -> &Vec<Vec<Vec<f64>>> {
        &self.values
    }
}