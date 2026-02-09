// Faker library integration for dynamic data generation

use fake::Fake;
use fake::faker::name::en::*;
use fake::faker::internet::en::*;
use fake::faker::phone_number::en::*;
use fake::faker::address::en::*;
use fake::faker::company::en::*;
use fake::faker::lorem::en::*;
use fake::faker::chrono::en::*;
use rand::Rng;

/// Generate fake data based on the variable name
/// Supports syntax: {{f:firstname}}, {{f:lastname}}, {{f:email}}, etc.
pub fn generate_fake_value(variable_name: &str) -> Option<String> {
    let name = variable_name.to_lowercase();
    
    match name.as_str() {
        // Names
        "firstname" | "first_name" => Some(FirstName().fake()),
        "lastname" | "last_name" => Some(LastName().fake()),
        "fullname" | "full_name" | "name" => Some(Name().fake()),
        "namewithtitle" | "name_with_title" => Some(NameWithTitle().fake()),
        "title" => Some(Title().fake()),
        "suffix" => Some(Suffix().fake()),
        
        // Internet
        "email" => Some(SafeEmail().fake()),
        "username" => Some(Username().fake()),
        "password" => Some(Password(8..16).fake()),
        "domain" => Some(DomainSuffix().fake()),
        "ipv4" => Some(IPv4().fake::<std::net::Ipv4Addr>().to_string()),
        "ipv6" => Some(IPv6().fake::<std::net::Ipv6Addr>().to_string()),
        "useragent" | "user_agent" => Some(UserAgent().fake()),
        "url" => Some(format!("https://{}", FreeEmail().fake::<String>())),
        
        // Phone
        "phone" | "phonenumber" | "phone_number" => Some(PhoneNumber().fake()),
        "cellnumber" | "cell_number" => Some(CellNumber().fake()),
        
        // Address
        "street" | "streetname" | "street_name" => Some(StreetName().fake()),
        "city" | "cityname" | "city_name" => Some(CityName().fake()),
        "state" | "statename" | "state_name" => Some(StateName().fake()),
        "stateabbr" | "state_abbr" => Some(StateAbbr().fake()),
        "zipcode" | "zip_code" | "zip" => Some(ZipCode().fake()),
        "country" | "countryname" | "country_name" => Some(CountryName().fake()),
        "countrycode" | "country_code" => Some(CountryCode().fake()),
        "latitude" | "lat" => Some(Latitude().fake::<f64>().to_string()),
        "longitude" | "lon" | "lng" => Some(Longitude().fake::<f64>().to_string()),
        
        // Company
        "company" | "companyname" | "company_name" => Some(CompanyName().fake()),
        "companysuffix" | "company_suffix" => Some(CompanySuffix().fake()),
        "industry" => Some(Industry().fake()),
        "profession" => Some(Profession().fake()),
        
        // Lorem
        "word" => Some(Word().fake()),
        "words" => Some(Words(3..5).fake::<Vec<String>>().join(" ")),
        "sentence" => Some(Sentence(3..10).fake()),
        "sentences" => Some(Sentences(2..4).fake::<Vec<String>>().join(" ")),
        "paragraph" => Some(Paragraph(3..7).fake()),
        "paragraphs" => Some(Paragraphs(2..4).fake::<Vec<String>>().join("\n\n")),
        
        // Numbers
        "number" | "int" | "integer" => Some(rand::thread_rng().gen_range(1..1000).to_string()),
        "float" | "decimal" => Some(format!("{:.2}", rand::thread_rng().gen_range(1.0..1000.0))),
        "digit" => Some(rand::thread_rng().gen_range(0..10).to_string()),
        "boolean" | "bool" => Some(rand::thread_rng().gen_bool(0.5).to_string()),
        
        // Date/Time
        "date" => {
            let date: chrono::DateTime<chrono::Utc> = DateTime().fake();
            Some(date.format("%Y-%m-%d").to_string())
        },
        "datetime" | "timestamp" => {
            let datetime: chrono::DateTime<chrono::Utc> = DateTime().fake();
            Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
        },
        "time" => Some(format!("{:02}:{:02}:{:02}", 
            rand::thread_rng().gen_range(0..24),
            rand::thread_rng().gen_range(0..60),
            rand::thread_rng().gen_range(0..60)
        )),
        
        // UUID
        "uuid" | "guid" => Some(uuid::Uuid::new_v4().to_string()),
        
        // Color
        "color" => {
            let colors = ["red", "blue", "green", "yellow", "purple", "orange", "pink", "brown", "black", "white"];
            Some(colors[rand::thread_rng().gen_range(0..colors.len())].to_string())
        },
        "hexcolor" | "hex_color" => Some(format!("#{:06x}", rand::thread_rng().gen_range(0..0xFFFFFF))),
        
        _ => None,
    }
}

/// Check if a variable uses faker syntax (f:variablename)
pub fn is_faker_variable(variable: &str) -> bool {
    variable.starts_with("f:")
}

/// Extract the faker variable name from the full variable (f:firstname -> firstname)
pub fn extract_faker_name(variable: &str) -> &str {
    if let Some(stripped) = variable.strip_prefix("f:") {
        stripped
    } else {
        variable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_faker_variable() {
        assert!(is_faker_variable("f:firstname"));
        assert!(is_faker_variable("f:email"));
        assert!(!is_faker_variable("firstname"));
        assert!(!is_faker_variable("USER_NAME"));
    }

    #[test]
    fn test_extract_faker_name() {
        assert_eq!(extract_faker_name("f:firstname"), "firstname");
        assert_eq!(extract_faker_name("f:email"), "email");
        assert_eq!(extract_faker_name("firstname"), "firstname");
    }

    #[test]
    fn test_generate_fake_value() {
        // Test that we can generate values for known types
        assert!(generate_fake_value("firstname").is_some());
        assert!(generate_fake_value("email").is_some());
        assert!(generate_fake_value("phone").is_some());
        assert!(generate_fake_value("uuid").is_some());
        
        // Test unknown type returns None
        assert!(generate_fake_value("unknown_type").is_none());
    }

    #[test]
    fn test_generate_fake_value_case_insensitive() {
        assert!(generate_fake_value("FirstName").is_some());
        assert!(generate_fake_value("FIRSTNAME").is_some());
        assert!(generate_fake_value("firstName").is_some());
    }
}
