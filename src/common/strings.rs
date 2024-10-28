use regex::Regex;

pub struct StringPkg;
impl StringPkg {
    pub fn extract_value(input: &str, template: &str, placeholder: &str) -> Option<String> {
        let regex_template = template.replace(placeholder, r"(?P<value>[^ ]+)");
        let re = Regex::new(&regex_template).ok()?;
        if let Some(captures) = re.captures(input) {
            captures.name("value").map(|m| m.as_str().to_string())
        } else {
            None
        }
    }
}
