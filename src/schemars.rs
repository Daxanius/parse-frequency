use super::Frequency;
use schemars::{
    JsonSchema, SchemaGenerator,
    schema::{InstanceType, Metadata, Schema, SchemaObject},
};

impl JsonSchema for Frequency {
    fn schema_name() -> String {
        "Frequency".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::Integer.into()),
            format: Some("frequency".to_string()),
            metadata: Some(Box::new(Metadata {
                description: Some(
                    "A frequency value like \"2.4 GHz\", \"100 kHz\", or \"440Hz\"".to_string(),
                ),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
