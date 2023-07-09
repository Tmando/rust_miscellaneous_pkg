//! This is a module which allows you to get/insert/delete/update data from mysql/sqlite/postgres database
#[cfg(feature = "sql_operation")]
pub mod sql_operation {
    use futures::TryStreamExt;
    use serde_json::{ Value, to_value };
    use sqlx::{ Row, Column, Sqlite };
    use sqlx::migrate::MigrateDatabase;
    async fn query_mysql(
        config: String,
        query: String,
        parameters: Vec<serde_json::Value>,
        max_connection: i64
    ) -> Value {
        println!("{:?}",config);
        let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(max_connection as u32)
        .connect(&config).await;
        let pool = match pool{
            Ok(res) => res,
            Err(err) => panic!("{:?}",err)
        };
        let mut rows = sqlx::query::<sqlx::MySql>(&query);
        for parm in parameters {
            if parm.is_boolean() {
                rows = rows.bind(parm.as_bool().unwrap());
            } else if parm.is_f64() {
                rows = rows.bind(parm.as_f64().unwrap());
            } else if parm.is_i64() {
                rows = rows.bind(parm.as_i64().unwrap());
            } else if parm.is_string() {
                let cur_parm = parm.as_str().unwrap().to_string();
                rows = rows.bind(cur_parm);
            } else if parm.is_object() || parm.is_array() {
                rows = rows.bind(serde_json::Value::to_string(&parm));
            }
        }
        let mut res = rows.fetch(&pool);
        let mut res_rows: Vec<std::collections::HashMap<String, serde_json::Value>> = Vec::new();
        while let Some(row) = res.try_next().await.unwrap() {
            let mut row_map: std::collections::HashMap<
                String,
                serde_json::Value
            > = std::collections::HashMap::new();
            for column in row.columns().iter() {
                let cur_type = &column.type_info().to_string();
                let res = match cur_type.as_str() {
                    "BOOLEAN" => { to_value::<Option<bool>>(row.try_get(column.name()).unwrap()) }
                    "TINYINT" => { to_value::<Option<i8>>(row.try_get(column.name()).unwrap()) }
                    "SMALLINT" => { to_value::<Option<i16>>(row.try_get(column.name()).unwrap()) }
                    "INT" => { to_value::<Option<i32>>(row.try_get(column.name()).unwrap()) }
                    "BIGINT" => { to_value::<Option<i64>>(row.try_get(column.name()).unwrap()) }
                    "TINYINT UNSIGNED" => {
                        to_value::<Option<u8>>(row.try_get(column.name()).unwrap())
                    }
                    "SMALLINT UNSIGNED" => {
                        to_value::<Option<u16>>(row.try_get(column.name()).unwrap())
                    }
                    "INT UNSIGNED" => {
                        to_value::<Option<u32>>(row.try_get(column.name()).unwrap())
                    }
                    "BIGINT UNSIGNED" => {
                        to_value::<Option<u64>>(row.try_get(column.name()).unwrap())
                    }
                    "FLOAT" => { to_value::<Option<f32>>(row.try_get(column.name()).unwrap()) }
                    "DOUBLE" => { to_value::<Option<f64>>(row.try_get(column.name()).unwrap()) }
                    "VARCHAR" | "CHAR" | "TEXT" => {
                        to_value::<Option<String>>(row.try_get(column.name()).unwrap())
                    }
                    "VARBINARY" | "BINARY" | "BLOB" => {
                        let cur_val: Vec<u8> = row.try_get(column.name()).unwrap();
                        to_value::<Vec<u8>>(cur_val)
                    }
                    "DATETIME" => {
                        let cur_val: Option<sqlx::types::time::PrimitiveDateTime> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "TIMESTAMP" => {
                        let cur_val: Option<sqlx::types::time::OffsetDateTime> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "DATE" => {
                        let cur_val: Option<sqlx::types::time::Date> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "TIME" => {
                        let cur_val: Option<sqlx::types::time::Time> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "DECIMAL" => {
                        let cur_val: Option<rust_decimal::Decimal> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "JSON" => { to_value::<Option<Value>>(row.try_get(column.name()).unwrap()) }
                    _ => panic!("Type {:?} cannot be matched", column.type_info().to_string()),
                };
                row_map.insert(String::from(column.name()), res.unwrap());
            }
            res_rows.push(row_map);
        }
        match res_rows.len() {
            0 => {
                return to_value(serde_json::Value::Null).unwrap();
            }
            _ => {
                return to_value(res_rows).unwrap();
            }
        }
    }
    async fn query_sqlite(
        config: String,
        query: String,
        parameters: Vec<serde_json::Value>,
        max_connection: i64
    ) -> Value {
        if !sqlx::Sqlite::database_exists(&config).await.unwrap_or(false){
            Sqlite::create_database(&config).await.unwrap();
        }
        let pool = sqlx::sqlite::SqlitePoolOptions
            ::new()
            .max_connections(max_connection as u32)
            .connect(&config).await
            .unwrap();
        let mut rows = sqlx::query::<sqlx::Sqlite>(&query);
        for parm in parameters {
            if parm.is_boolean() {
                rows = rows.bind(parm.as_bool().unwrap());
            } else if parm.is_f64() {
                rows = rows.bind(parm.as_f64().unwrap());
            } else if parm.is_i64() {
                rows = rows.bind(parm.as_i64().unwrap());
            } else if parm.is_string() {
                let cur_parm = parm.as_str().unwrap().to_string();
                rows = rows.bind(cur_parm);
            } else if parm.is_object() || parm.is_array() {
                rows = rows.bind(serde_json::Value::to_string(&parm));
            }
        }
        let mut res = rows.fetch(&pool);
        let mut res_rows: Vec<std::collections::HashMap<String, serde_json::Value>> = Vec::new();
        while let Some(row) = res.try_next().await.unwrap() {
            let mut row_map: std::collections::HashMap<
                String,
                serde_json::Value
            > = std::collections::HashMap::new();
            for column in row.columns().iter() {
                let cur_type = &column.type_info().to_string();
                let res = match cur_type.as_str() {
                    "TEXT" => { to_value::<Option<String>>(row.try_get(column.name()).unwrap()) }
                    "INTEGER" => { to_value::<Option<i32>>(row.try_get(column.name()).unwrap()) }
                    "BIGINT" => { to_value::<Option<i64>>(row.try_get(column.name()).unwrap()) }
                    "REAL" => { to_value::<Option<f64>>(row.try_get(column.name()).unwrap()) }
                    "BLOB" => { to_value::<Vec<u8>>(row.try_get(column.name()).unwrap()) },
                    "NULL" => { to_value::<Option<String>>(row.try_get(column.name()).unwrap())
                    },
                    _ => panic!("Type {:?} cannot be matched", column.type_info().to_string()),
                };
                row_map.insert(String::from(column.name()), res.unwrap());
            }
            res_rows.push(row_map);
        }
        match res_rows.len() {
            0 => {
                return to_value(serde_json::Value::Null).unwrap();
            }
            _ => {
                return to_value(res_rows).unwrap();
            }
        }
    }
    async fn query_postgres(
        config: String,
        query: String,
        parameters: Vec<serde_json::Value>,
        max_connection: i64
    ) -> Value {
        let pool = sqlx::postgres::PgPoolOptions
            ::new()
            .max_connections(max_connection as u32)
            .connect(&config).await
            .unwrap();
        let mut rows = sqlx::query::<sqlx::Postgres>(&query);
        for parm in parameters {
            if parm.is_boolean() {
                rows = rows.bind(parm.as_bool().unwrap());
            } else if parm.is_f64() {
                rows = rows.bind(parm.as_f64().unwrap());
            } else if parm.is_i64() {
                rows = rows.bind(parm.as_i64().unwrap());
            } else if parm.is_string() {
                let cur_parm = parm.as_str().unwrap().to_string();
                rows = rows.bind(cur_parm);
            } else if parm.is_object() || parm.is_array() {
                rows = rows.bind(serde_json::Value::to_string(&parm));
            }
        }
        let mut res = rows.fetch(&pool);
        // if length of rows 0 return null
        let mut res_rows: Vec<std::collections::HashMap<String, serde_json::Value>> = Vec::new();
        while let Some(row) = res.try_next().await.unwrap() {
            let mut row_map: std::collections::HashMap<
                String,
                serde_json::Value
            > = std::collections::HashMap::new();
            for column in row.columns().iter() {
                let cur_type = &column.type_info().to_string();
                let res = match cur_type.as_str() {
                    "CHAR" | "VARCHAR" | "TEXT" => {
                        to_value::<Option<String>>(row.try_get(column.name()).unwrap())
                    }
                    "CHAR[]" | "VARCHAR[]" | "TEXT[]" => {
                        to_value::<Vec<Option<String>>>(row.try_get(column.name()).unwrap())
                    }
                    "BOOL" => { to_value::<Option<bool>>(row.try_get(column.name()).unwrap()) }
                    "BOOL[]" => {
                        to_value::<Vec<Option<bool>>>(row.try_get(column.name()).unwrap())
                    }
                    "INT2" => { to_value::<Option<i16>>(row.try_get(column.name()).unwrap()) }
                    "INT2[]" => {
                        to_value::<Vec<Option<i16>>>(row.try_get(column.name()).unwrap())
                    }
                    "INT4" => {
                        to_value::<Option<i32>>(row.try_get_unchecked(column.name()).unwrap())
                    }
                    "INT4[]" => {
                        to_value::<Vec<Option<i32>>>(row.try_get(column.name()).unwrap())
                    }
                    "FLOAT4" => { to_value::<Option<f32>>(row.try_get(column.name()).unwrap()) }
                    "FLOAT4[]" => {
                        to_value::<Vec<Option<f32>>>(row.try_get(column.name()).unwrap())
                    }
                    "FLOAT8" => { to_value::<Option<f64>>(row.try_get(column.name()).unwrap()) }
                    "FLOAT8[]" => {
                        to_value::<Vec<Option<f64>>>(row.try_get(column.name()).unwrap())
                    }
                    "DATE" => {
                        let cur_val: Option<sqlx::types::time::Date> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "DATE[]" => {
                        let cur_val: Vec<Option<sqlx::types::time::Date>> = row
                            .try_get(column.name())
                            .unwrap();
                        let date_vec = cur_val
                            .iter()
                            .map(|x| {
                                match x {
                                    Some(res) => res.to_string(),
                                    None => String::from(""),
                                }
                            })
                            .collect();
                        to_value::<Vec<String>>(date_vec)
                    }
                    "TIMESTAMP" => {
                        let cur_val: Option<sqlx::types::time::PrimitiveDateTime> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "TIMESTAMPTZ" => {
                        let cur_val: Option<sqlx::types::time::OffsetDateTime> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "TIMESTAMPTZ[]" => {
                        let cur_val: Vec<Option<sqlx::types::time::OffsetDateTime>> = row
                            .try_get(column.name())
                            .unwrap();
                        let date_vec = cur_val
                            .iter()
                            .map(|x| {
                                match x {
                                    Some(res) => { res.to_string() }
                                    None => { String::from("") }
                                }
                            })
                            .collect();
                        to_value::<Vec<String>>(date_vec)
                    }
                    "UUID" => {
                        let cur_val: Option<uuid::Uuid> = row.try_get(column.name()).unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "UUID[]" => {
                        let cur_val: Vec<uuid::Uuid> = row.try_get(column.name()).unwrap();
                        let uuid_vec = cur_val
                            .iter()
                            .map(|x| x.to_string())
                            .collect();
                        to_value::<Vec<String>>(uuid_vec)
                    }
                    "BYTEA" => {
                        let cur_val: Vec<u8> = row.try_get(column.name()).unwrap();
                        to_value::<Vec<u8>>(cur_val)
                    }
                    "BYTEA[]" => {
                        let cur_val: Vec<Vec<u8>> = row.try_get(column.name()).unwrap();
                        to_value::<Vec<Vec<u8>>>(cur_val)
                    }
                    "NUMERIC" => {
                        let cur_val: Option<rust_decimal::Decimal> = row
                            .try_get(column.name())
                            .unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "INET" => {
                        let cur_val: Option<std::net::IpAddr> = row.try_get(column.name()).unwrap();
                        match cur_val {
                            Some(res) => { to_value(res.to_string()) }
                            None => { to_value(serde_json::Value::Null) }
                        }
                    }
                    "INET[]" => {
                        let cur_val: Vec<Option<std::net::IpAddr>> = row
                            .try_get(column.name())
                            .unwrap();
                        let inet_vec = cur_val
                            .iter()
                            .map(|x| {
                                match x {
                                    Some(res) => { res.to_string() }
                                    None => { String::from("") }
                                }
                            })
                            .collect();
                        to_value::<Vec<String>>(inet_vec)
                    }
                    "BIT" => {
                        let cur_val: bit_vec::BitVec = row.try_get(column.name()).unwrap();
                        to_value::<Vec<bool>>(
                            cur_val
                                .iter()
                                .map(|x| x)
                                .collect()
                        )
                    }
                    "xml" => {
                        let cur_val: &str = row
                            .try_get_raw(column.name())
                            .unwrap()
                            .as_str()
                            .unwrap();
                        to_value::<String>(String::from(cur_val))
                    }
                    "JSON" | "JSONB" => {
                        let cur_val: Option<Value> = row.try_get(column.name()).unwrap();
                        to_value::<Option<Value>>(cur_val)
                    }
                    "JSON[]" | "JSONB[]" => {
                        let cur_val: Vec<Option<Value>> = row.try_get(column.name()).unwrap();
                        to_value::<Vec<Option<Value>>>(cur_val)
                    }
                    _ => panic!("Type {:?} cannot be matched", column.type_info().to_string()),
                };
                row_map.insert(String::from(column.name()), res.unwrap());
            }
            res_rows.push(row_map);
        }
        match res_rows.len() {
            0 => {
                return to_value(serde_json::Value::Null).unwrap();
            }
            _ => {
                return to_value(res_rows).unwrap();
            }
        }
    }
    pub async fn query_db(
        config: String,
        query: String,
        parameters: Vec<serde_json::Value>,
        max_connection: i64
    ) -> Value {
        let db_name = &config[..config.find(":").unwrap()];

        return match db_name {
            "postgresql" => query_postgres(config, query, parameters, max_connection).await,
            "sqlite" => query_sqlite(config, query, parameters, max_connection).await,
            "mysql" => query_mysql(config, query, parameters, max_connection).await,
            _ => panic!("Database not found"),
        };
    }
}

#[cfg(feature = "sql_operation")]
#[cfg(test)]
mod tests {
    #[test]
    fn test_mysql_sqlx_date() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("mysql://root:geheim@localhost:12345/"),
                String::from("SELECT DATE('2017-06-15') as datetest"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"datetest\":\"2017-06-15\"}]");
    }

    #[test]
    fn test_mysql_sqlx_string() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("mysql://root:geheim@localhost:12345/"),
                String::from("SELECT 'k' as char_val"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"char_val\":\"k\"}]");
    }
    #[test]
    fn test_mysql_sqlx_int() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("mysql://root:geheim@localhost:12345/"),
                String::from("SELECT 20 as int_val"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"int_val\":20}]");
    }
    #[test]
    fn test_mysql_sqlx_boolean() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("mysql://root:geheim@localhost:12345/"),
                String::from("SELECT TRUE as boolval"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"boolval\":1}]");
    }
    #[test]
    fn test_mysql_sqlx_boolean_false() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("mysql://root:geheim@localhost:12345/"),
                String::from("SELECT FALSE as boolval"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"boolval\":0}]");
    }

    #[test]
    fn test_postgres_sqlx() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 'k'::CHAR(1) as char_val"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"char_val\":\"k\"}]");
    }
    #[test]
    fn test_postgres_sqlx_char_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['v'::CHAR(1),'i'::CHAR(1),'e'::CHAR(1),'n'::CHAR(1),'n'::CHAR(1),'a'::CHAR(1)] as char_arr"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"char_arr\":[\"v\",\"i\",\"e\",\"n\",\"n\",\"a\"]}]");
        println!("{:?}", response)
    }
    #[test]
    fn test_postgres_sqlx_bool() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT false::bool as false_val"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"false_val\":false}]");
    }
    #[test]
    fn test_postgres_sqlx_bool_arr() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array[true,false,true,false] as bool_arr"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"bool_arr\":[true,false,true,false]}]");
    }

    #[test]
    fn test_postgres_sqlx_small_int() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 10::SMALLINT as small_int"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"small_int\":10}]");
    }

    #[test]
    fn test_postgres_sqlx_small_int_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array[10::SMALLINT,10::SMALLINT,10::SMALLINT] as small_int"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"small_int\":[10,10,10]}]");
    }
    #[test]
    fn test_postgres_sqlx_int() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 10::integer as int"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"int\":10}]");
    }

    #[test]
    fn test_postgres_sqlx_int_array() {
        let test: bool = true;
        let val = serde_json::Value::Bool(test);
        println!("{:?}", val);
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array[10::integer,10::integer,10::integer] as int"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"int\":[10,10,10]}]");
    }

    #[test]
    fn test_postgres_sqlx_float() {
        let test: bool = true;
        let val = serde_json::Value::Bool(test);
        println!("{:?}", val);
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT -1.333::real as float_val"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"float_val\":-1.3329999446868896}]");
    }
    #[test]
    fn test_postgres_sqlx_float_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array[-1.333::real,-1.333::real,-1.333::real] as float_val"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"float_val\":[-1.3329999446868896,-1.3329999446868896,-1.3329999446868896]}]");
    }
    #[test]
    fn test_postgres_double_value() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 30.3333::double precision"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"float8\":30.3333}]");
    }
    #[test]
    fn test_postgres_double_value_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array[30.3333::double precision,30.3333::double precision]"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[30.3333,30.3333]}]");
    }

    #[test]
    fn test_postgres_date() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '10-01-2022'::date"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"date\":\"2022-10-01\"}]");
    }
    #[test]
    fn test_postgres_date_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['10-01-2022'::date,'10-01-2022'::date,'10-01-2022'::date]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[\"2022-10-01\",\"2022-10-01\",\"2022-10-01\"]}]");
    }
    #[test]
    fn test_timestamp() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '10-01-2022'::timestamp"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"timestamp\":\"2022-10-01 0:00:00.0\"}]");
    }
    #[test]
    fn test_timestampz() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '10-01-2022'::timestamptz"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"timestamptz\":\"2022-10-01 0:00:00.0 +00:00:00\"}]");
    }
    #[test]
    fn test_timestampz_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['10-01-2022'::timestamptz,'10-01-2022'::timestamptz,'10-01-2022'::timestamptz]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[\"2022-10-01 0:00:00.0 +00:00:00\",\"2022-10-01 0:00:00.0 +00:00:00\",\"2022-10-01 0:00:00.0 +00:00:00\"]}]");
    }
    #[test]
    fn test_postgres_type_uuid() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '123e4567-e89b-12d3-a456-426614174000'::uuid"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"uuid\":\"123e4567-e89b-12d3-a456-426614174000\"}]");
    }
    #[test]
    fn test_postgres_type_uuid_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['123e4567-e89b-12d3-a456-426614174000'::uuid,'123e4567-e89b-12d3-a456-426614174000'::uuid]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[\"123e4567-e89b-12d3-a456-426614174000\",\"123e4567-e89b-12d3-a456-426614174000\"]}]");
    }
    #[test]
    fn test_postgres_type_bytea() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 'abc \\153\\154\\155 \\052\\251\\124'::bytea"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"bytea\":[97,98,99,32,107,108,109,32,42,169,84]}]");
    }
    #[test]
    fn test_postgres_type_bytea_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['abc \\153\\154\\155 \\052\\251\\124'::bytea,'abc \\153\\154\\155 \\052\\251\\124'::bytea,'abc \\153\\154\\155 \\052\\251\\124'::bytea]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[[97,98,99,32,107,108,109,32,42,169,84],[97,98,99,32,107,108,109,32,42,169,84],[97,98,99,32,107,108,109,32,42,169,84]]}]");
    }
    #[test]
    fn test_postgres_type_decimal() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 10.333998::decimal"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"numeric\":\"10.33399800\"}]");
    }
    #[test]
    fn test_postgres_intet() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '192.168.0.1'::INET"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"inet\":\"192.168.0.1\"}]");
    }
    #[test]
    fn test_postgres_intet_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['192.168.0.1'::INET,'192.168.0.1'::INET,'192.168.0.1'::INET]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[\"192.168.0.1\",\"192.168.0.1\",\"192.168.0.1\"]}]");
    }

    #[test]
    fn test_postgres_string() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 'test'::varchar(50)"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"varchar\":\"test\"}]");
    }
    #[test]
    fn test_postgres_bit_vec() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT B'11'::bit"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"bit\":[true]}]");
    }
    #[test]
    fn test_postgres_xml() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '<foo>bar</foo><foo>bar</foo>'::xml"),
                vec![],
                5
            )
        );
    }
    #[test]
    fn test_postgres_json() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '{\"a\":1,\"b\":2}'::json"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"json\":{\"a\":1,\"b\":2}}]");
    }
    #[test]
    fn test_postgres_json_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array['{\"a\":1,\"b\":2}'::json,'{\"a\":1,\"b\":2}'::json]"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":2}]}]");
    }
    #[test]
    fn test_postgres_jsonb_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array['{\"a\":1,\"b\":2}'::jsonb,'{\"a\":1,\"b\":2}'::jsonb]"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":2}]}]");
    }
    #[test]
    fn test_postgres_jsonb() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT '{\"a\":1,\"b\":2}'::jsonb"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"jsonb\":{\"a\":1,\"b\":2}}]");
    }
    #[test]
    fn test_postgres_text() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 'This is a test text'::text"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"text\":\"This is a test text\"}]");
    }
    #[test]
    fn test_postgres_text_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['This is a test text'::text,'This is a test text'::text,'This is a test text'::text]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[\"This is a test text\",\"This is a test text\",\"This is a test text\"]}]");
    }
    #[test]
    fn test_postgres_char_multi() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT 'This is a test text'::char(200)"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"bpchar\":\"This is a test text                                                                                                                                                                                     \"}]");
    }
    #[test]
    fn test_postgres_char_multi_array() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from(
                    "SELECT array['This is a test text'::char(200),'This is a test text'::char(200)]"
                ),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[\"This is a test text                                                                                                                                                                                     \",\"This is a test text                                                                                                                                                                                     \"]}]");
    }
    #[test]
    fn test_postgres_numeric_null() {
        let response = tokio_test::block_on(
            super::sql_operation::query_db(
                String::from("postgresql://postgres:postgres@localhost:5500/test1db"),
                String::from("SELECT array[NULL::int,10::int,20::int]"),
                vec![],
                5
            )
        );
        assert_eq!(serde_json::Value::to_string(&response),"[{\"array\":[null,10,20]}]");
    }
}