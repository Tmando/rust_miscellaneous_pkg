//! Collection to create different fake data
#[cfg(feature = "faker")]
pub mod faker {
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