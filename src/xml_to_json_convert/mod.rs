//! Module to convert json from xml
pub mod xml_to_json_convert {
    use quickxml_to_serde::*;
    use serde_json::Value;
    /// This function converts an xml string to a json Value
    ///
    pub fn xml_to_json(xml_input: String) -> Value {
        let json_out =
            quickxml_to_serde::xml_string_to_json(xml_input, &Config::new_with_defaults());
        match json_out {
            Ok(res) => return res,
            Err(err) => std::panic::panic_any(err),
        }
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test_convert_xml_to_json_one() {
        let xml_input = r#"<dataset>
        <record>
        <id>1</id>
        <first_name>Nerita</first_name>
        <last_name>Stanney</last_name>
        <email>nstanney0@domainmarket.com</email>
        <gender>Female</gender>
        <ip_address>223.10.217.33</ip_address>
        </record>
        <record>
        <id>2</id>
        <first_name>Domini</first_name>
        <last_name>Bateman</last_name>
        <email>dbateman1@chronoengine.com</email>
        <gender>Female</gender>
        <ip_address>21.158.87.126</ip_address>
        </record>
        <record>
        <id>3</id>
        <first_name>Kariotta</first_name>
        <last_name>Teresa</last_name>
        <email>kteresa2@joomla.org</email>
        <gender>Female</gender>
        <ip_address>26.86.41.10</ip_address>
        </record>
        <record>
        <id>4</id>
        <first_name>Cosetta</first_name>
        <last_name>Fernandes</last_name>
        <email>cfernandes3@samsung.com</email>
        <gender>Female</gender>
        <ip_address>142.79.228.8</ip_address>
        </record>
        <record>
        <id>5</id>
        <first_name>Haze</first_name>
        <last_name>Heindrich</last_name>
        <email>hheindrich4@psu.edu</email>
        <gender>Male</gender>
        <ip_address>249.87.69.82</ip_address>
        </record>
        <record>
        <id>6</id>
        <first_name>Abey</first_name>
        <last_name>Atwell</last_name>
        <email>aatwell5@disqus.com</email>
        <gender>Male</gender>
        <ip_address>194.4.244.83</ip_address>
        </record>
        <record>
        <id>7</id>
        <first_name>Greggory</first_name>
        <last_name>Teresse</last_name>
        <email>gteresse6@sourceforge.net</email>
        <gender>Bigender</gender>
        <ip_address>219.161.232.148</ip_address>
        </record>
        <record>
        <id>8</id>
        <first_name>Brok</first_name>
        <last_name>Plitz</last_name>
        <email>bplitz7@house.gov</email>
        <gender>Male</gender>
        <ip_address>189.72.4.218</ip_address>
        </record>
        <record>
        <id>9</id>
        <first_name>Maggie</first_name>
        <last_name>Lissandri</last_name>
        <email>mlissandri8@ycombinator.com</email>
        <gender>Female</gender>
        <ip_address>204.69.47.52</ip_address>
        </record>
        <record>
        <id>10</id>
        <first_name>Garrott</first_name>
        <last_name>Huett</last_name>
        <email>ghuett9@51.la</email>
        <gender>Male</gender>
        <ip_address>117.118.239.60</ip_address>
        </record>
        </dataset>"#;
        let json_out = super::xml_to_json_convert::xml_to_json(String::from(xml_input)).to_string();
        assert_eq!(json_out,
            "{\"dataset\":{\"record\":[{\"email\":\"nstanney0@domainmarket.com\",\"first_name\":\"Nerita\",\"gender\":\"Female\",\"id\":1,\"ip_address\":\"223.10.217.33\",\"last_name\":\"Stanney\"},{\"email\":\"dbateman1@chronoengine.com\",\"first_name\":\"Domini\",\"gender\":\"Female\",\"id\":2,\"ip_address\":\"21.158.87.126\",\"last_name\":\"Bateman\"},{\"email\":\"kteresa2@joomla.org\",\"first_name\":\"Kariotta\",\"gender\":\"Female\",\"id\":3,\"ip_address\":\"26.86.41.10\",\"last_name\":\"Teresa\"},{\"email\":\"cfernandes3@samsung.com\",\"first_name\":\"Cosetta\",\"gender\":\"Female\",\"id\":4,\"ip_address\":\"142.79.228.8\",\"last_name\":\"Fernandes\"},{\"email\":\"hheindrich4@psu.edu\",\"first_name\":\"Haze\",\"gender\":\"Male\",\"id\":5,\"ip_address\":\"249.87.69.82\",\"last_name\":\"Heindrich\"},{\"email\":\"aatwell5@disqus.com\",\"first_name\":\"Abey\",\"gender\":\"Male\",\"id\":6,\"ip_address\":\"194.4.244.83\",\"last_name\":\"Atwell\"},{\"email\":\"gteresse6@sourceforge.net\",\"first_name\":\"Greggory\",\"gender\":\"Bigender\",\"id\":7,\"ip_address\":\"219.161.232.148\",\"last_name\":\"Teresse\"},{\"email\":\"bplitz7@house.gov\",\"first_name\":\"Brok\",\"gender\":\"Male\",\"id\":8,\"ip_address\":\"189.72.4.218\",\"last_name\":\"Plitz\"},{\"email\":\"mlissandri8@ycombinator.com\",\"first_name\":\"Maggie\",\"gender\":\"Female\",\"id\":9,\"ip_address\":\"204.69.47.52\",\"last_name\":\"Lissandri\"},{\"email\":\"ghuett9@51.la\",\"first_name\":\"Garrott\",\"gender\":\"Male\",\"id\":10,\"ip_address\":\"117.118.239.60\",\"last_name\":\"Huett\"}]}}")
    }
    #[test]
    #[should_panic]
    fn test_convert_xml_to_json_err() {
        super::xml_to_json_convert::xml_to_json(String::from("")).to_string();
    }

}
