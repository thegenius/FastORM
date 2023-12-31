use fastorm_macro::Entity;
use fastorm_trait::Entity;
use sqlx::any::AnyArguments;
use sqlx::any::AnyRow;
use sqlx::Arguments;
use sqlx::Error;
use sqlx::Row;

#[derive(Entity, Clone)]
pub struct TestDescribe {
    #[PrimaryKey]
    id: i32,
    #[PrimaryKey]
    id2: i32,
    content: String,
}

#[test]
fn it_works() {
    let describe = TestDescribe {
        id: 23,
        id2: 24,
        content: "content".to_string(),
    };
    let desc = describe.get_fields_name();
    assert_eq!(
        desc,
        vec!["id".to_string(), "id2".to_string(), "content".to_string()]
    );

    let primary_fields = describe.get_primary_fields_name();
    assert_eq!(primary_fields, vec!["id".to_string(), "id2".to_string()]);

    let body_fields = describe.get_body_fields_name();
    assert_eq!(body_fields, vec!["content".to_string()]);

    let arguments: AnyArguments = describe.clone().into_update_any_arguments();
    let len = arguments.values.0.len();
    assert_eq!(len, 3);

    let arguments: AnyArguments = describe.clone().into_insert_any_arguments();
    let len = arguments.values.0.len();
    assert_eq!(len, 3);

    let arguments: AnyArguments = describe.clone().into_upsert_any_arguments();
    let len = arguments.values.0.len();
    assert_eq!(len, 4);
}
