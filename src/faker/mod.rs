//! Collection to create different fake data
#[cfg(feature = "faker")]
pub mod faker {
    use std::{ collections::HashMap, convert::TryInto };

    use serde_json::Value;

    pub fn generate_fake_json(input_json: String)->Value {
        let json: serde_json::Value = serde_json::from_str(&input_json).unwrap();
        let base_payload = json.as_object().unwrap();
        let mut count = 5;
        if
        json.get("count").is_some() &&
        json.get("count").unwrap().is_number()
        {
            count = json.get("count").unwrap().as_i64().unwrap();
        }

        let mut result_map: Vec<HashMap<String, serde_json::Value>> = Vec::new();
        for i in 0..count as usize {
            let data = base_payload.get("data").unwrap().as_array().unwrap();
            result_map.insert(i, HashMap::new());
            for n in 0..data.len() {
                let curDataEntry = data[n].as_object().unwrap();
                let field_name = curDataEntry
                    .get("field_name")
                    .unwrap()
                    .as_str()
                    .unwrap();
                let field_type = curDataEntry
                    .get("field_type")
                    .unwrap()
                    .as_str()
                    .unwrap();
                let field_value = match field_type {
                    "city" => Value::String(fakeit::address::city()),
                    "country" => Value::String(fakeit::address::country()),
                    "abr" => Value::String(fakeit::address::country_abr()),
                    "latitude" =>
                        Value::Number(
                            serde_json::Number
                                ::from_f64(fakeit::address::latitude() as f64)
                                .unwrap()
                        ),
                    "longitude" =>
                        Value::Number(
                            serde_json::Number
                                ::from_f64(fakeit::address::longitude() as f64)
                                .unwrap()
                        ),
                    "state" => Value::String(fakeit::address::state()),
                    "street" => Value::String(fakeit::address::street()),
                    "street_name" => Value::String(fakeit::address::street_name()),
                    "street_number" => Value::String(fakeit::address::street_number()),
                    "street_prefix" => Value::String(fakeit::address::street_prefix()),
                    "street_suffix" => Value::String(fakeit::address::street_suffix()),
                    "zip" => Value::String(fakeit::address::zip()),
                    "animal" => Value::String(fakeit::animal::animal()),
                    "cat" => Value::String(fakeit::animal::cat()),
                    "dog" => Value::String(fakeit::animal::dog()),
                    "farm" => Value::String(fakeit::animal::farm()),
                    "alcohol" => Value::String(fakeit::beer::alcohol()),
                    "blg" => Value::String(fakeit::beer::blg()),
                    "hop" => Value::String(fakeit::beer::hop()),
                    "malt" => Value::String(fakeit::beer::malt()),
                    "beer_name" => Value::String(fakeit::beer::name()),
                    "style" => Value::String(fakeit::beer::style()),
                    "yeast" => Value::String(fakeit::beer::yeast()),
                    "bool" => Value::Bool(fakeit::bool_rand::bool()),
                    "full_color" => Value::String(fakeit::color::full()),
                    "hex" => Value::String(fakeit::color::hex()),
                    "safe_color" => Value::String(fakeit::color::safe()),
                    "company" => Value::String(fakeit::company::company()),
                    "buzzword" => Value::String(fakeit::company::buzzword()),
                    "bs" => Value::String(fakeit::company::bs()),
                    "company_suffix" => Value::String(fakeit::company::company_suffix()),
                    "email" => Value::String(fakeit::contact::email()),
                    "phone" => Value::String(fakeit::contact::phone()),
                    "phone_formatted" => Value::String(fakeit::contact::phone_formatted()),
                    "currency_long" => Value::String(fakeit::currency::long()),
                    "currency_short" => Value::String(fakeit::currency::short()),
                    "day" => Value::String(fakeit::datetime::day()),
                    "hour" => Value::String(fakeit::datetime::hour()),
                    "minute" => Value::String(fakeit::datetime::minute()),
                    "month" => Value::String(fakeit::datetime::month()),
                    "nanosecond" => Value::String(fakeit::datetime::nanosecond()),
                    "second" => Value::String(fakeit::datetime::second()),
                    "timezone" => Value::String(fakeit::datetime::timezone()),
                    "timezone_abv" => Value::String(fakeit::datetime::timezone_abv()),
                    "timezone_full" => Value::String(fakeit::datetime::timezone_full()),
                    "timezone_offset" => Value::String(fakeit::datetime::timezone_offset()),
                    "week_day" => Value::String(fakeit::datetime::week_day()),
                    "year" => Value::String(fakeit::datetime::year()),
                    "date" => Value::String(fakeit::datetime::date().to_string()),
                    "extension" => Value::String(fakeit::file::extension()),
                    "mime_type" => Value::String(fakeit::file::mime_type()),
                    "abbreviation" => Value::String(fakeit::hacker::abbreviation()),
                    "adjective" => Value::String(fakeit::hacker::adjective()),
                    "ingverb" => Value::String(fakeit::hacker::ingverb()),
                    "pharse" => Value::String(fakeit::hacker::phrase()),
                    "verb" => Value::String(fakeit::hacker::verb()),
                    "domain_name" => Value::String(fakeit::internet::domain_name()),
                    "domain_suffix" => Value::String(fakeit::internet::domain_suffix()),
                    "http_method" => Value::String(fakeit::internet::http_method()),
                    "ipv4_address" => Value::String(fakeit::internet::ipv4_address()),
                    "ipv6_address" => Value::String(fakeit::internet::ipv6_address()),
                    "mac_address" => Value::String(fakeit::internet::mac_address()),
                    "username" => Value::String(fakeit::internet::username()),
                    "job_descriptor" => Value::String(fakeit::job::descriptor()),
                    "job_level" => Value::String(fakeit::job::level()),
                    "job_title" => Value::String(fakeit::job::title()),
                    "language_abbreviation" => Value::String(fakeit::language::abbreviation()),
                    "programming_language" => Value::String(fakeit::language::programming()),
                    "random_language" => Value::String(fakeit::language::random()),
                    "log_level_appache" => Value::String(fakeit::log_level::apache()),
                    "log_level_general" => Value::String(fakeit::log_level::general()),
                    "log_level_sys_log" => Value::String(fakeit::log_level::syslog()),
                    "full_name" => Value::String(fakeit::name::full()),
                    "first_name" => Value::String(fakeit::name::first()),
                    "last_name" => Value::String(fakeit::name::last()),
                    "prefix_name" => Value::String(fakeit::name::prefix()),
                    "suffix_name" => Value::String(fakeit::name::suffix()),
                    "password" => {
                        let mut upper = true;
                        let mut numeric = true;
                        let mut special = true;
                        let mut num = 10;
                        if
                            curDataEntry.get("upper").is_some() &&
                            curDataEntry.get("upper").unwrap().is_boolean()
                        {
                            upper = curDataEntry.get("upper").unwrap().as_bool().unwrap();
                        }
                        if
                            curDataEntry.get("numeric").is_some() &&
                            curDataEntry.get("numeric").unwrap().is_boolean()
                        {
                            numeric = curDataEntry.get("numeric").unwrap().as_bool().unwrap();
                        }
                        if
                            curDataEntry.get("special").is_some() &&
                            curDataEntry.get("special").unwrap().is_boolean()
                        {
                            special = curDataEntry.get("special").unwrap().as_bool().unwrap();
                        }
                        if
                            curDataEntry.get("num").is_some() &&
                            curDataEntry.get("num").unwrap().is_number()
                        {
                            num = curDataEntry.get("num").unwrap().as_i64().unwrap();
                        }

                        Value::String(
                            fakeit::password::generate(
                                upper,
                                numeric,
                                special,
                                num.try_into().unwrap()
                            )
                        )
                    }
                    "credit_card_type" => Value::String(fakeit::payment::credit_card_type()),
                    "credit_card_number" => Value::String(fakeit::payment::credit_card_number()),
                    "credit_card_luhn_number" =>
                        Value::String(fakeit::payment::credit_card_luhn_number()),
                    "credit_card_cvv" => Value::String(fakeit::payment::credit_card_cvv()),
                    "status_codes_general" =>
                        Value::Number(
                            serde_json::Number
                                ::from_f64(fakeit::status_code::general() as f64)
                                .unwrap()
                        ),
                    "status_codes_simple" =>
                        Value::Number(
                            serde_json::Number
                                ::from_f64(fakeit::status_code::simple() as f64)
                                .unwrap()
                        ),
                    "uuid_v1" => Value::String(fakeit::unique::uuid_v1()),
                    "uuid_v4" => Value::String(fakeit::unique::uuid_v4()),
                    "user_agent_chrome" => Value::String(fakeit::user_agent::chrome()),
                    "user_agent_firefox" => Value::String(fakeit::user_agent::firefox()),
                    "user_agent_linux_platform_token" =>
                        Value::String(fakeit::user_agent::linux_platform_token()),
                    "user_agent_opera" => Value::String(fakeit::user_agent::opera()),
                    "user_agent_random_platform" =>
                        Value::String(fakeit::user_agent::random_platform()),
                    "user_agent_safari" => Value::String(fakeit::user_agent::safari()),
                    "windows_platform_token" =>
                        Value::String(fakeit::user_agent::windows_platform_token()),
                    "car_maker" => Value::String(fakeit::vehicle::car_maker()),
                    "car_model" => Value::String(fakeit::vehicle::car_model()),
                    "fuel" => Value::String(fakeit::vehicle::fuel()),
                    "transmission_gear" => Value::String(fakeit::vehicle::transmission_gear()),
                    "vehicle_type" => Value::String(fakeit::vehicle::vehicle_type()),
                    "question" => Value::String(fakeit::words::question()),
                    "quote" => Value::String(fakeit::words::quote()),
                    "word" => Value::String(fakeit::words::word()),
                    "paragraph" => {
                        let mut count = 1;
                        let mut sentence_count = 1;
                        let mut word_count = 1;
                        let mut separator = "\n";
                        if
                            curDataEntry.get("count").is_some() &&
                            curDataEntry.get("count").unwrap().is_number()
                        {
                            count = curDataEntry.get("count").unwrap().as_i64().unwrap();
                        }
                        if
                            curDataEntry.get("sentence_count").is_some() &&
                            curDataEntry.get("sentence_count").unwrap().is_number()
                        {
                            sentence_count = curDataEntry
                                .get("sentence_count")
                                .unwrap()
                                .as_i64()
                                .unwrap();
                        }
                        if
                            curDataEntry.get("word_count").is_some() &&
                            curDataEntry.get("word_count").unwrap().is_number()
                        {
                            word_count = curDataEntry.get("word_count").unwrap().as_i64().unwrap();
                        }
                        if
                            curDataEntry.get("separator").is_some() &&
                            curDataEntry.get("separator").unwrap().is_string()
                        {
                            separator = curDataEntry.get("separator").unwrap().as_str().unwrap();
                        }

                        Value::String(
                            fakeit::words::paragraph(
                                count,
                                sentence_count,
                                word_count,
                                String::from(separator)
                            )
                        )
                    }
                    "sentence" => {
                        let mut word_count = 1;
                        if
                            curDataEntry.get("word_count").is_some() &&
                            curDataEntry.get("word_count").unwrap().is_number()
                        {
                            word_count = curDataEntry.get("word_count").unwrap().as_i64().unwrap();
                        }
                        Value::String(fakeit::words::sentence(word_count))
                    }
                    _ => Value::String(String::from("")),
                };
                result_map[i].insert(String::from(field_name), field_value);
            }
        }
        return serde_json::json!(result_map);
    }

    pub fn get_city() -> String {
        return fakeit::address::city();
    }

    pub fn get_country() -> String {
        return fakeit::address::country();
    }

    pub fn get_country_abr() -> String {
        return fakeit::address::country_abr();
    }

    pub fn get_latitude() -> f32 {
        return fakeit::address::latitude();
    }

    pub fn get_longitude() -> f32 {
        return fakeit::address::longitude();
    }

    pub fn get_state() -> String {
        return fakeit::address::state();
    }

    pub fn get_street() -> String {
        return fakeit::address::street();
    }

    pub fn get_street_name() -> String {
        return fakeit::address::street_name();
    }

    pub fn get_street_number() -> String {
        return fakeit::address::street_number();
    }

    pub fn get_street_prefix() -> String {
        return fakeit::address::street_prefix();
    }

    pub fn get_street_suffix() -> String {
        return fakeit::address::street_suffix();
    }

    pub fn get_zip() -> String {
        return fakeit::address::zip();
    }

    pub fn get_anmimal() -> String {
        return fakeit::animal::animal();
    }

    pub fn get_cat() -> String {
        return fakeit::animal::cat();
    }

    pub fn get_dog() -> String {
        return fakeit::animal::dog();
    }

    pub fn get_farm() -> String {
        return fakeit::animal::farm();
    }

    pub fn get_pet_name() -> String {
        return fakeit::animal::pet_name();
    }

    pub fn get_alcohol() -> String {
        return fakeit::beer::alcohol();
    }

    pub fn get_blg() -> String {
        return fakeit::beer::blg();
    }

    pub fn get_hop() -> String {
        return fakeit::beer::hop();
    }

    pub fn get_ibu() -> String {
        return fakeit::beer::ibu();
    }

    pub fn get_malt() -> String {
        return fakeit::beer::malt();
    }

    pub fn get_name() -> String {
        return fakeit::beer::name();
    }

    pub fn get_style() -> String {
        return fakeit::beer::style();
    }

    pub fn get_yeast() -> String {
        return fakeit::beer::yeast();
    }

    pub fn get_bool() -> bool {
        return fakeit::bool_rand::bool();
    }

    pub fn get_full_color() -> String {
        return fakeit::color::full();
    }

    pub fn get_hex_color() -> String {
        return fakeit::color::hex();
    }

    pub fn get_rgb_color() -> [i16; 3] {
        return fakeit::color::rgb();
    }

    pub fn get_safe_color() -> String {
        return fakeit::color::safe();
    }

    pub fn get_company() -> String {
        return fakeit::company::company();
    }

    pub fn get_buzzword() -> String {
        return fakeit::company::buzzword();
    }

    pub fn get_bs() -> String {
        return fakeit::company::bs();
    }

    pub fn get_company_suffix() -> String {
        return fakeit::company::company_suffix();
    }

    pub fn get_email() -> String {
        return fakeit::contact::email();
    }

    pub fn get_phone() -> String {
        return fakeit::contact::phone();
    }

    pub fn get_phone_formatted() -> String {
        return fakeit::contact::phone_formatted();
    }

    pub fn get_currency_long() -> String {
        return fakeit::currency::long();
    }

    pub fn get_currency_short() -> String {
        return fakeit::currency::short();
    }

    pub fn get_day() -> String {
        return fakeit::datetime::day();
    }

    pub fn get_hour() -> String {
        return fakeit::datetime::hour();
    }

    pub fn get_minute() -> String {
        return fakeit::datetime::minute();
    }

    pub fn get_secound() -> String {
        return fakeit::datetime::second();
    }

    pub fn get_timezone() -> String {
        return fakeit::datetime::timezone();
    }

    pub fn get_week_day() -> String {
        return fakeit::datetime::week_day();
    }

    pub fn get_year() -> String {
        return fakeit::datetime::year();
    }

    pub fn get_extension() -> String {
        return fakeit::file::extension();
    }

    pub fn get_mime_type() -> String {
        return fakeit::file::mime_type();
    }

    pub fn get_abbreviation() -> String {
        return fakeit::hacker::abbreviation();
    }

    pub fn get_adjective() -> String {
        return fakeit::hacker::adjective();
    }

    pub fn get_ingverb() -> String {
        return fakeit::hacker::ingverb();
    }

    pub fn get_noun() -> String {
        return fakeit::hacker::noun();
    }

    pub fn get_pharse() -> String {
        return fakeit::hacker::phrase();
    }

    pub fn get_verb() -> String {
        return fakeit::hacker::verb();
    }

    pub fn get_hippster_word() -> String {
        return fakeit::hipster::word();
    }

    pub fn get_domain_name() -> String {
        return fakeit::internet::domain_name();
    }

    pub fn get_domain_suffix() -> String {
        return fakeit::internet::domain_suffix();
    }

    pub fn get_http_method() -> String {
        return fakeit::internet::http_method();
    }

    pub fn get_ipv4_address() -> String {
        return fakeit::internet::ipv4_address();
    }

    pub fn get_ipv6_address() -> String {
        return fakeit::internet::ipv6_address();
    }

    pub fn get_mac_address() -> String {
        return fakeit::internet::mac_address();
    }

    pub fn get_username() -> String {
        return fakeit::internet::username();
    }

    pub fn get_job_descriptor() -> String {
        return fakeit::job::descriptor();
    }

    pub fn get_job_level() -> String {
        return fakeit::job::level();
    }

    pub fn get_job_title() -> String {
        return fakeit::job::title();
    }

    pub fn get_language_abbreviation() -> String {
        return fakeit::language::abbreviation();
    }

    pub fn get_language_programming() -> String {
        return fakeit::language::programming();
    }

    pub fn get_language_random() -> String {
        return fakeit::language::random();
    }

    pub fn get_log_level_appache() -> String {
        return fakeit::log_level::apache();
    }

    pub fn get_log_level_general() -> String {
        return fakeit::log_level::general();
    }

    pub fn get_log_level_sys_log() -> String {
        return fakeit::log_level::syslog();
    }

    pub fn get_full_name() -> String {
        return fakeit::name::full();
    }

    pub fn get_first_name() -> String {
        return fakeit::name::first();
    }

    pub fn get_last_name() -> String {
        return fakeit::name::last();
    }

    pub fn get_prefix_name() -> String {
        return fakeit::name::prefix();
    }

    pub fn get_suffix_name() -> String {
        return fakeit::name::suffix();
    }

    pub fn get_generate_password(upper: bool, numeric: bool, special: bool, num: i8) -> String {
        return fakeit::password::generate(upper, numeric, special, num);
    }

    pub fn get_credit_card_type() -> String {
        return fakeit::payment::credit_card_type();
    }

    pub fn get_credit_card_number() -> String {
        return fakeit::payment::credit_card_number();
    }

    pub fn get_credit_card_luhn_number() -> String {
        return fakeit::payment::credit_card_luhn_number();
    }

    pub fn get_credit_card_exp() -> String {
        return fakeit::payment::credit_card_exp();
    }

    pub fn get_credit_card_ccv() -> String {
        return fakeit::payment::credit_card_cvv();
    }

    pub fn get_status_codes_general() -> i16 {
        return fakeit::status_code::general();
    }

    pub fn get_status_codes_simple() -> i16 {
        return fakeit::status_code::simple();
    }

    pub fn get_uuid_v1() -> String {
        return fakeit::unique::uuid_v1();
    }

    pub fn get_uuid_v4() -> String {
        return fakeit::unique::uuid_v4();
    }

    pub fn get_user_agent_chrome() -> String {
        return fakeit::user_agent::chrome();
    }

    pub fn get_user_agent_firefox() -> String {
        return fakeit::user_agent::firefox();
    }

    pub fn get_user_agent_linux_platform_token() -> String {
        return fakeit::user_agent::linux_platform_token();
    }

    pub fn get_user_agent_mac_platform_token() -> String {
        return fakeit::user_agent::mac_platform_token();
    }

    pub fn get_user_agent_opera() -> String {
        return fakeit::user_agent::opera();
    }

    pub fn get_user_agent_random_platform() -> String {
        return fakeit::user_agent::random_platform();
    }

    pub fn get_user_agent_safari() -> String {
        return fakeit::user_agent::safari();
    }

    pub fn get_user_agent_windows_platform_token() -> String {
        return fakeit::user_agent::windows_platform_token();
    }

    pub fn get_car_maker() -> String {
        return fakeit::vehicle::car_maker();
    }

    pub fn get_car_model() -> String {
        return fakeit::vehicle::car_model();
    }

    pub fn get_fuel() -> String {
        return fakeit::vehicle::fuel();
    }

    pub fn get_transmission_gear() -> String {
        return fakeit::vehicle::transmission_gear();
    }

    pub fn get_vehicle_type() -> String {
        return fakeit::vehicle::vehicle_type();
    }

    pub fn get_paragraph(
        count: i64,
        sentence_count: i64,
        word_count: i64,
        separator: String
    ) -> String {
        return fakeit::words::paragraph(count, sentence_count, word_count, separator);
    }

    pub fn get_question() -> String {
        return fakeit::words::question();
    }

    pub fn get_quote() -> String {
        return fakeit::words::quote();
    }

    pub fn get_sentence(word_count: i64) -> String {
        return fakeit::words::sentence(word_count);
    }

    pub fn get_word() -> String {
        return fakeit::words::word();
    }
}

#[test]
fn test_fake_json() {
    use std::io::prelude::*;
    let first_test_json = r#"{
        "count":100000,
        "data":[{"field_name":"city","field_type":"city"},
        {"field_name":"country","field_type":"country"},
        {"field_name":"abr","field_type":"abr"},
        {"field_name":"latitude","field_type":"latitude"},
        {"field_name":"longitude","field_type":"longitude"},
        {"field_name":"state","field_type":"state"},
        {"field_name":"street","field_type":"street"},
        {"field_name":"street_name","field_type":"street_name"},
        {"field_name":"street_number","field_type":"street_number"},
        {"field_name":"street_prefix","field_type":"street_prefix"},
        {"field_name":"street_suffix","field_type":"street_suffix"},
        {"field_name":"zip","field_type":"zip"},
        {"field_name":"animal","field_type":"animal"},
        {"field_name":"cat","field_type":"cat"},
        {"field_name":"dog","field_type":"dog"},
        {"field_name":"farm","field_type":"farm"},
        {"field_name":"alcohol","field_type":"alcohol"},
        {"field_name":"blg","field_type":"blg"},
        {"field_name":"hop","field_type":"hop"},
        {"field_name":"malt","field_type":"malt"},
        {"field_name":"beer_name","field_type":"beer_name"},
        {"field_name":"style","field_type":"style"},
        {"field_name":"yeast","field_type":"yeast"},
        {"field_name":"bool","field_type":"bool"},
        {"field_name":"full_color","field_type":"full_color"},
        {"field_name":"hex","field_type":"hex"},
        {"field_name":"safe_color","field_type":"safe_color"},
        {"field_name":"company","field_type":"company"},
        {"field_name":"buzzword","field_type":"buzzword"},
        {"field_name":"bs","field_type":"bs"},
        {"field_name":"company_suffix","field_type":"company_suffix"},
        {"field_name":"email","field_type":"email"},
        {"field_name":"phone","field_type":"phone"},
        {"field_name":"phone_formatted","field_type":"phone_formatted"},
        {"field_name":"currency_long","field_type":"currency_long"},
        {"field_name":"currency_short","field_type":"currency_short"},
        {"field_name":"day","field_type":"day"},
        {"field_name":"hour","field_type":"hour"},
        {"field_name":"minute","field_type":"minute"},
        {"field_name":"month","field_type":"month"},
        {"field_name":"nanosecond","field_type":"nanosecond"},
        {"field_name":"second","field_type":"second"},
        {"field_name":"timezone","field_type":"timezone"},
        {"field_name":"timezone_abv","field_type":"timezone_abv"},
        {"field_name":"timezone_full","field_type":"timezone_full"},
        {"field_name":"timezone_offset","field_type":"timezone_offset"},
        {"field_name":"week_day","field_type":"week_day"},
        {"field_name":"year","field_type":"year"},
        {"field_name":"date","field_type":"date"},
        {"field_name":"extension","field_type":"extension"},
        {"field_name":"mime_type","field_type":"mime_type"},
        {"field_name":"abbreviation","field_type":"abbreviation"},
        {"field_name":"adjective","field_type":"adjective"},
        {"field_name":"ingverb","field_type":"ingverb"},
        {"field_name":"pharse","field_type":"pharse"},
        {"field_name":"verb","field_type":"verb"},
        {"field_name":"domain_name","field_type":"domain_name"},
        {"field_name":"domain_suffix","field_type":"domain_suffix"},
        {"field_name":"http_method","field_type":"http_method"},
        {"field_name":"ipv4_address","field_type":"ipv4_address"},
        {"field_name":"ipv6_address","field_type":"ipv6_address"},
        {"field_name":"mac_address","field_type":"mac_address"},
        {"field_name":"username","field_type":"username"},
        {"field_name":"job_descriptor","field_type":"job_descriptor"},
        {"field_name":"job_level","field_type":"job_level"},
        {"field_name":"language_abbreviation","field_type":"language_abbreviation"},
        {"field_name":"programming_language","field_type":"programming_language"},
        {"field_name":"random_language","field_type":"random_language"},
        {"field_name":"log_level_appache","field_type":"log_level_appache"},
        {"field_name":"log_level_general","field_type":"log_level_general"},
        {"field_name":"log_level_sys_log","field_type":"log_level_sys_log"},
        {"field_name":"full_name","field_type":"full_name"},
        {"field_name":"first_name","field_type":"first_name"},
        {"field_name":"last_name","field_type":"last_name"},
        {"field_name":"prefix_name","field_type":"prefix_name"},
        {"field_name":"suffix_name","field_type":"suffix_name"},
        {"field_name":"password","field_type":"password"},
        {"field_name":"credit_card_type","field_type":"credit_card_type"},
        {"field_name":"credit_card_number","field_type":"credit_card_number"},
        {"field_name":"credit_card_luhn_number","field_type":"credit_card_luhn_number"},
        {"field_name":"credit_card_cvv","field_type":"credit_card_cvv"},
        {"field_name":"status_codes_general","field_type":"status_codes_general"},
        {"field_name":"status_codes_simple","field_type":"status_codes_simple"},
        {"field_name":"uuid_v1","field_type":"uuid_v1"},
        {"field_name":"uuid_v4","field_type":"uuid_v4"},
        {"field_name":"user_agent_chrome","field_type":"user_agent_chrome"},
        {"field_name":"user_agent_firefox","field_type":"user_agent_firefox"},
        {"field_name":"user_agent_linux_platform_token","field_type":"user_agent_linux_platform_token"},
        {"field_name":"user_agent_opera","field_type":"user_agent_opera"},
        {"field_name":"user_agent_random_platform","field_type":"user_agent_random_platform"},
        {"field_name":"user_agent_safari","field_type":"user_agent_safari"},
        {"field_name":"windows_platform_token","field_type":"windows_platform_token"},
        {"field_name":"car_maker","field_type":"car_maker"},
        {"field_name":"car_model","field_type":"car_model"},
        {"field_name":"fuel","field_type":"fuel"},
        {"field_name":"transmission_gear","field_type":"transmission_gear"},
        {"field_name":"vehicle_type","field_type":"vehicle_type"},
        {"field_name":"question","field_type":"question"},
        {"field_name":"quote","field_type":"quote"},
        {"field_name":"word","field_type":"word"},
        {"field_name":"paragraph","field_type":"paragraph"},
        {"field_name":"sentence","field_type":"sentence"}]
    }"#;
    let result = super::faker::faker::generate_fake_json(String::from(first_test_json));
    let mut file = std::fs::File::create("result.json").unwrap();
    file.write_all(result.to_string().as_bytes()).unwrap();

}
