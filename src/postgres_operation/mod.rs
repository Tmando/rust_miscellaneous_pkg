//! This is a module which allows you to get data from a postgres database
pub mod postgres_operation {
    use postgres::{ types::ToSql, Client, NoTls };
    // Get results from Postgres Database
    pub fn query_db(
        config: String,
        query: String,
        parameter: Vec<&(dyn ToSql + Sync)>
    ) -> serde_json::Value {
        // Connect to the database.
        let mut client = match Client::connect(&config, NoTls) {
            Err(e) => panic!("{}", e),
            Ok(res) => res,
        };

        // Now we can execute a simple statement that just returns its parameter.
        let rows = match client.query(&query, &parameter) {
            Err(e) => panic!("{}", e),
            Ok(res) => res,
        };
        // if length of rows 0 return null
        if rows.len() == 0 {
            return serde_json::Value::Null;
        }

        let mut res_rows: Vec<std::collections::HashMap<String, serde_json::Value>> = Vec::new();
        // Map each column to a Serde JSON Value
        for row in rows.iter() {
            let mut row_map: std::collections::HashMap<
                String,
                serde_json::Value
            > = std::collections::HashMap::new();
            for column in row.columns().iter() {
                let res = match column.type_().name() {
                    "bpchar" => serde_json::to_value(row.get::<_, String>(column.name())),
                    "_bpchar" => serde_json::to_value(row.get::<_, Vec<String>>(column.name())),
                    "bool" => serde_json::to_value(row.get::<_, bool>(column.name())),
                    "_bool" => serde_json::to_value(row.get::<_, Vec<bool>>(column.name())),
                    "int2" => serde_json::to_value(row.get::<_, i16>(column.name())),
                    "_int2" => serde_json::to_value(row.get::<_, Vec<i16>>(column.name())),
                    "int4" => serde_json::to_value(row.get::<_, i32>(column.name())),
                    "_int4" => serde_json::to_value(row.get::<_, Vec<i32>>(column.name())),
                    "int8" => serde_json::to_value(row.get::<_, i64>(column.name())),
                    "_int8" => serde_json::to_value(row.get::<_, Vec<i64>>(column.name())),
                    "float4" => serde_json::to_value(row.get::<_, f32>(column.name())),
                    "_float4" => serde_json::to_value(row.get::<_, Vec<f32>>(column.name())),
                    "float8" => serde_json::to_value(row.get::<_, f64>(column.name())),
                    "_float8" => serde_json::to_value(row.get::<_, Vec<f64>>(column.name())),
                    "bytea" => {
                        let res_vec = row.get::<_, Vec<u8>>(column.name());
                        serde_json::to_value(format!("{:?}", &res_vec))
                    }
                    "date" =>
                        serde_json::to_value(
                            row.get::<_, chrono::NaiveDate>(column.name()).to_string()
                        ),
                    "_date" => {
                        let result: Vec<String> = row
                            .get::<_, Vec<chrono::NaiveDate>>(column.name())
                            .iter()
                            .map(|x| x.to_string())
                            .collect();
                        serde_json::to_value(result)
                    }
                    "text" => serde_json::to_value(row.get::<_, String>(column.name())),
                    "_text" => serde_json::to_value(row.get::<_, Vec<String>>(column.name())),
                    "timestamp" =>
                        serde_json::to_value(
                            row.get::<_, chrono::NaiveDateTime>(column.name()).to_string()
                        ),
                    "_timestamp" => {
                        let result: Vec<String> = row
                            .get::<_, Vec<chrono::NaiveDateTime>>(column.name())
                            .iter()
                            .map(|x| x.to_string())
                            .collect();
                        serde_json::to_value(result)
                    }
                    "uuid" => {
                        serde_json::to_value(row.get::<_, uuid::Uuid>(column.name()).to_string())
                    }
                    "_uuid" => {
                        let result: Vec<String> = row
                            .get::<_, Vec<uuid::Uuid>>(column.name())
                            .iter()
                            .map(|x| x.to_string())
                            .collect();
                        serde_json::to_value(result)
                    }
                    "inet" =>
                        serde_json::to_value(
                            row.get::<_, std::net::IpAddr>(column.name()).to_string()
                        ),
                    "_inet" => {
                        let result: Vec<String> = row
                            .get::<_, Vec<std::net::IpAddr>>(column.name())
                            .iter()
                            .map(|x| x.to_string())
                            .collect();
                        serde_json::to_value(result)
                    }
                    "bit" =>
                        serde_json::to_value(
                            format!(
                                "{:?}",
                                &row.get::<_, bit_vec::BitVec>(column.name()).to_bytes()
                            )
                        ),
                    "_bit" => {
                        let result: Vec<String> = row
                            .get::<_, Vec<bit_vec::BitVec>>(column.name())
                            .iter()
                            .map(|x| format!("{:?}", x.to_bytes()))
                            .collect();
                        serde_json::to_value(result)
                    }
                    "json" => serde_json::to_value(row.get::<_, serde_json::Value>(column.name())),
                    "_json" => {
                        serde_json::to_value(row.get::<_, Vec<serde_json::Value>>(column.name()))
                    }
                    "jsonb" => serde_json::to_value(row.get::<_, serde_json::Value>(column.name())),
                    "_jsonb" => {
                        serde_json::to_value(row.get::<_, Vec<serde_json::Value>>(column.name()))
                    }
                    "hstore" =>
                        serde_json::to_value(
                            row.get::<_, std::collections::HashMap<String, Option<String>>>(
                                column.name()
                            )
                        ),
                    "_hstore" =>
                        serde_json::to_value(
                            row.get::<_, Vec<std::collections::HashMap<String, Option<String>>>>(
                                column.name()
                            )
                        ),
                    "point" => {
                        let point = row.get::<_, geo_types::Point<f64>>(column.name());
                        let mut point_map: std::collections::HashMap<
                            String,
                            f64
                        > = std::collections::HashMap::new();
                        point_map.insert(String::from("x"), point.x());
                        point_map.insert(String::from("y"), point.y());
                        serde_json::to_value(point_map)
                    }
                    _ =>
                        panic!(
                            "{:?}, {:?}, {:?}",
                            column.name(),
                            column.type_().name(),
                            "Type not to map"
                        ),
                };
                row_map.insert(String::from(column.name()), res.unwrap());
            }
            res_rows.push(row_map);
        }
        return serde_json::to_value(res_rows).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_postgres_connection() {
        super::postgres_operation::query_db(
            String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
            String::from(
                "SELECT 'k'::CHAR(1) as char_val, false::bool as false_val, 10::SMALLINT as small_int, 5::integer as int_num, 3000::bigint as big_int, -1.333::real as decimal_val,30.3333::double precision,'abc \\153\\154\\155 \\052\\251\\124'::bytea, '07-03-2022'::date,'test1'::text,'07-03-2022'::timestamp,'123e4567-e89b-12d3-a456-426614174000'::uuid,'192.168.0.1'::INET,B'1'::bit"
            ),
            vec![]
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_char() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 'k'::CHAR(1) as char_val"),
                    vec![]
                )
            ),
            "[{\"char_val\":\"k\"}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_char_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array['v'::CHAR(1),'i'::CHAR(1),'e'::CHAR(1),'n'::CHAR(1),'n'::CHAR(1),'a'::CHAR(1)] as char_arr"
                    ),
                    vec![]
                )
            ),
            "[{\"char_arr\":[\"v\",\"i\",\"e\",\"n\",\"n\",\"a\"]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_bool() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT false::bool as false_val"),
                    vec![]
                )
            ),
            "[{\"false_val\":false}]"
        )
    }

    #[test]
    #[ignore]
    fn test_postgres_type_bool_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT array[true,false,true,false] as bool_arr"),
                    vec![]
                )
            ),
            "[{\"bool_arr\":[true,false,true,false]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_small_int() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 10::SMALLINT as small_int"),
                    vec![]
                )
            ),
            "[{\"small_int\":10}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_small_int_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array[10::SMALLINT,30::SMALLINT,50::SMALLINT] as small_int_array"
                    ),
                    vec![]
                )
            ),
            "[{\"small_int_array\":[10,30,50]}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_int() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 5::integer as int_num"),
                    vec![]
                )
            ),
            "[{\"int_num\":5}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_int_arr() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array[5::integer,10::integer,100::integer,200::integer] as int_num_array"
                    ),
                    vec![]
                )
            ),
            "[{\"int_num_array\":[5,10,100,200]}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_bigint() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 3000::bigint as big_int"),
                    vec![]
                )
            ),
            "[{\"big_int\":3000}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_bigint_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array[3000::bigint,9000::bigint,12000::bigint] as big_int"
                    ),
                    vec![]
                )
            ),
            "[{\"big_int\":[3000,9000,12000]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_decimal_val() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT -1.333::real as decimal_val"),
                    vec![]
                )
            ),
            "[{\"decimal_val\":-1.3329999446868896}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_decimal_val_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array[-1.333::real,-10.90::real,-20.90::real] as decimal_val"
                    ),
                    vec![]
                )
            ),
            "[{\"decimal_val\":[-1.3329999446868896,-10.899999618530273,-20.899999618530273]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_double_val() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 30.3333::double precision"),
                    vec![]
                )
            ),
            "[{\"float8\":30.3333}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_double_val_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array[30.3333::double precision,20.3333::double precision,5.3333::double precision]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[30.3333,20.3333,5.3333]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_bytea() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 'abc \\153\\154\\155 \\052\\251\\124'::bytea"),
                    vec![]
                )
            ),
            "[{\"bytea\":\"[97, 98, 99, 32, 107, 108, 109, 32, 42, 169, 84]\"}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_date() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT '07-03-2022'::date"),
                    vec![]
                )
            ),
            "[{\"date\":\"2022-07-03\"}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_type_date_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array['07-03-2022'::date,'07-03-2022'::date,'07-03-2022'::date]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[\"2022-07-03\",\"2022-07-03\",\"2022-07-03\"]}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_timestamp() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT '07-03-2022'::timestamp"),
                    vec![]
                )
            ),
            "[{\"timestamp\":\"2022-07-03 00:00:00\"}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_timestamp_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array['07-03-2022'::timestamp,'07-03-2022'::timestamp,'07-03-2022'::timestamp]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[\"2022-07-03 00:00:00\",\"2022-07-03 00:00:00\",\"2022-07-03 00:00:00\"]}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_uuid() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT '123e4567-e89b-12d3-a456-426614174000'::uuid"),
                    vec![]
                )
            ),
            "[{\"uuid\":\"123e4567-e89b-12d3-a456-426614174000\"}]"
        );
    }
    #[test]
    #[ignore]
    fn test_postgres_type_uuid_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array[
                            '123e4567-e89b-12d3-a456-426614174000'::uuid,
                            '123e4567-e89b-12d3-a456-426614174000'::uuid,
                            '123e4567-e89b-12d3-a456-426614174000'::uuid,
                            '123e4567-e89b-12d3-a456-426614174000'::uuid
                        ]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[\"123e4567-e89b-12d3-a456-426614174000\",\"123e4567-e89b-12d3-a456-426614174000\",\"123e4567-e89b-12d3-a456-426614174000\",\"123e4567-e89b-12d3-a456-426614174000\"]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_intet_type() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT '192.168.0.1'::INET"),
                    vec![]
                )
            ),
            "[{\"inet\":\"192.168.0.1\"}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_intet_type_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT 
            array[
                '192.168.0.1'::INET,
                '192.168.0.1'::INET,
                '192.168.0.1'::INET,
                '192.168.0.1'::INET
            ]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[\"192.168.0.1\",\"192.168.0.1\",\"192.168.0.1\",\"192.168.0.1\"]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_bit_type_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT 
            array[
                B'101',
                B'00'
            ]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[\"[160]\",\"[0]\"]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_bit_type() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT B'1'::bit"),
                    vec![]
                )
            ),
            "[{\"bit\":\"[128]\"}]"
        );
    }

    #[test]
    #[ignore]
    fn test_postgres_insert_data() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("INSERT INTO public.test1(test2) VALUES('Hallo Welt')"),
                    vec![]
                )
            ),
            "null"
        );
    }
    #[test]
    #[ignore]
    fn test_json_data() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT '{\"a\":1,\"b\":2}'::json"),
                    vec![]
                )
            ),
            "[{\"json\":{\"a\":1,\"b\":2}}]"
        );
    }
    #[test]
    #[ignore]
    fn test_json_data_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array['{\"a\":1,\"b\":2}'::json,'{\"a\":1,\"b\":2}'::json]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":2}]}]"
        );
    }
    #[test]
    #[ignore]
    fn test_jsonb_data() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT '{\"a\":1,\"b\":2}'::jsonb"),
                    vec![]
                )
            ),
            "[{\"jsonb\":{\"a\":1,\"b\":2}}]"
        );
    }
    #[test]
    #[ignore]
    fn test_jsonb_data_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from(
                        "SELECT array['{\"a\":1,\"b\":2}'::jsonb,'{\"a\":1,\"b\":2}'::jsonb]"
                    ),
                    vec![]
                )
            ),
            "[{\"array\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":2}]}]"
        );
    }
    #[test]
    #[ignore]
    fn test_hstore() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT 'a=>1,a=>2'::hstore"),
                    vec![]
                )
            ),
            "[{\"hstore\":{\"a\":\"1\"}}]"
        );
    }

    #[test]
    #[ignore]
    fn test_hstore_array() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT array['a=>1,a=>2'::hstore,'a=>1,a=>2'::hstore]"),
                    vec![]
                )
            ),
            "[{\"array\":[{\"a\":\"1\"},{\"a\":\"1\"}]}]"
        );
    }

    #[test]
    #[ignore]
    fn test_point() {
        assert_eq!(
            serde_json::Value::to_string(
                &super::postgres_operation::query_db(
                    String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                    String::from("SELECT point(3, 4)"),
                    vec![]
                )
            ),
            "[{\"point\":{\"x\":3.0,\"y\":4.0}}]"
        );
    }
}