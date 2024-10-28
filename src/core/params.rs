#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    pub params: Vec<(String, String)>,
}
impl QueryParams {
    pub fn add<T>(&mut self, name: &str, val: &T, expload: bool)
    where
        T: serde::Serialize,
    {
        let serde_val = serde_json::json!(val);
        match serde_val {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::Object(_) => {
                self.params.push((name.into(), serde_val.to_string()))
            }
            serde_json::Value::String(s_val) => {
                self.params.push((name.into(), s_val.to_string()))
            }
            serde_json::Value::Array(items) => {
                if expload {
                    items.iter().for_each(|i| self.add(name, i, expload))
                } else {
                    let unexploaded = items
                        .iter()
                        .map(|i| {
                            if let serde_json::Value::String(s) = i {
                                s.clone()
                            } else {
                                i.to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(",");
                    self.params.push((name.into(), unexploaded))
                }
            }
        }
    }
    pub fn add_option<T>(&mut self, name: &str, val: &Option<T>, expload: bool)
    where
        T: serde::Serialize,
    {
        if let Some(v) = val {
            self.add(name, v, expload)
        }
    }
    pub fn add_patch<T>(
        &mut self,
        name: &str,
        val: &super::patch::Patch<T>,
        expload: bool,
    )
    where
        T: serde::Serialize,
    {
        if let super::patch::Patch::Value(v) = val {
            self.add(name, v, expload)
        }
    }
}
#[allow(unused)]
pub fn format_string_param<T>(val: &T) -> String
where
    T: serde::Serialize,
{
    let serde_val = serde_json::json!(val);
    if let serde_json::Value::String(str) = serde_val {
        str
    } else {
        serde_val.to_string()
    }
}
