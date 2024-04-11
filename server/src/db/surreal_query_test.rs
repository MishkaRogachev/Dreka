use super::surreal_query::Builder;

#[test]
fn test_create_thing() {
    assert_eq!(
        Builder::new()
            .create()
            .thing("table", "id")
            .content(serde_json::json!({"field": "value"}))
            .to_string(),
        "CREATE type::thing($tb, $uid) CONTENT $data"
    );
}

#[test]
fn test_create_table() {
    assert_eq!(
        Builder::new()
            .create()
            .table("table")
            .content(serde_json::json!({"field": "value"}))
            .to_string(),
        "CREATE type::table($tb) CONTENT $data"
    );
}

#[test]
fn test_update_thing() {
    assert_eq!(
        Builder::new()
            .update()
            .thing("table", "id")
            .content(serde_json::json!({"field": "value"}))
            .to_string(),
        "UPDATE type::thing($tb, $uid) CONTENT $data"
    );
}

#[test]
fn test_delete_thing() {
    assert_eq!(
        Builder::new()
            .delete()
            .thing("table", "id")
            .to_string(),
        "DELETE type::thing($tb, $uid)"
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
            .to_string(),
        "SELECT * FROM type::thing($tb, $uid)"
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
            .to_string(),
        "SELECT * FROM type::table($tb)"
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
            .to_string(),
        "SELECT id FROM type::table($tb)"
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
            .to_string(),
        "SELECT * FROM type::table($tb) WHERE field1 = $value AND field2 = $value2"
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
            .to_string(),
        "SELECT id FROM type::table($tb) WHERE state = $value"
    );
}
