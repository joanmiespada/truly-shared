
pub fn remove_stage_prefix(input:String, api_stage: String) -> String {

    let pattern1 = format!("/{}",api_stage);
    let pattern = pattern1.as_str();
    let last_v1_index = input.rfind(pattern);
    let result = match last_v1_index {
        Some(index) => input[(index + pattern.len())..].to_string(),
        None => input.to_string(),
    };

    result

}

