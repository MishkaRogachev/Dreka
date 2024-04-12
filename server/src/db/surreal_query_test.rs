use super::surreal_query::{Builder, SetMode, ReturnType};

#[test]
fn test_create_thing_and_return_fields() {
    assert_eq!(
        Builder::new()
            .create()
            .thing("table", "id")
            .content(serde_json::json!({"field1": 765, "field2": "some_value"}))
            .returns(ReturnType::Fields { fields: vec!["id".into(), "field2".into()] })
            .to_final_string(),
        "CREATE type::thing(\"table\", \"id\") CONTENT {\"field1\":765,\"field2\":\"some_value\"} RETURN id, field2;"
    );
}

#[test]
fn test_create_table_return_mone() {
    assert_eq!(
        Builder::new()
            .create()
            .table("table")
            .content(serde_json::json!({"field": "value"}))
            .returns(ReturnType::None)
            .to_final_string(),
        "CREATE type::table(\"table\") CONTENT {\"field\":\"value\"} RETURN NONE;"
    );
}

#[test]
fn test_update_thing() {
    assert_eq!(
        Builder::new()
            .update()
            .thing("table", "id")
            .merge(serde_json::json!({"field": "value"}))
            .to_final_string(),
        "UPDATE type::thing(\"table\", \"id\") MERGE {\"field\":\"value\"};"
    );
}

#[test]
fn test_delete_thing() {
    assert_eq!(
        Builder::new()
            .delete()
            .thing("table", "id")
            .to_final_string(),
        "DELETE type::thing(\"table\", \"id\");"
    );
}

#[test]
fn test_select_one_thing() {
    assert_eq!(
        Builder::new()
            .select()
            .all()
            .from()
            .thing("table", "id")
            .to_final_string(),
        "SELECT * FROM type::thing(\"table\", \"id\");"
    );
}

#[test]
fn test_select_all_table() {
    assert_eq!(
        Builder::new()
            .select()
            .all()
            .from()
            .table("table")
            .to_final_string(),
        "SELECT * FROM type::table(\"table\");"
    );
}

#[test]
fn test_select_ids_table() {
    assert_eq!(
        Builder::new()
            .select()
            .some("id".into())
            .from()
            .table("table")
            .to_final_string(),
        "SELECT id FROM type::table(\"table\");"
    );
}

#[test]
fn test_select_where_table() {
    assert_eq!(
        Builder::new()
            .select()
            .all()
            .from()
            .table("table")
            .equals("field1", serde_json::json!("value1"))
            .equals("field2", serde_json::json!("value2"))
            .to_final_string(),
        "SELECT * FROM type::table(\"table\") WHERE field1 = \"value1\" AND field2 = \"value2\";"
    );
}

#[test]
fn test_select_id_where_state_is_pending() {
    assert_eq!(
        Builder::new()
            .select()
            .some("id".into())
            .from()
            .table("table")
            .equals("state", serde_json::json!("pending"))
            .to_final_string(),
        "SELECT id FROM type::table(\"table\") WHERE state = \"pending\";"
    );
}

#[test]
fn test_transaction_with_two_statements() {
    assert_eq!(
        Builder::new()
            .begin_tx()
            .create()
            .table("table")
            .set("field", serde_json::json!({"internal_field": "value"}), SetMode::Equal)
            .set("value", serde_json::json!(1), SetMode::Add)
            .update()
            .table("table_2")
            .set("balance", serde_json::json!(100), SetMode::Subtract)
            .end_tx()
            .to_final_string(),
        "BEGIN TRANSACTION;\r\n\
        CREATE type::table(\"table\") SET field = {\"internal_field\":\"value\"}, value += 1;\r\n\
        UPDATE type::table(\"table_2\") SET balance -= 100;\r\n\
        COMMIT TRANSACTION;"
    );
}
