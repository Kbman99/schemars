use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use ipnet::{IpNet, Ipv4Net, Ipv6Net};

impl JsonSchema for IpNet {
    fn schema_name() -> String {
        "IpNet".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            metadata: Some(Box::new(Metadata {
                description: Some("Represents a generic network address range. This type can have two variants: v4 and v6.".to_string()),
                examples: vec![
                    serde_json::to_value(IpNet::V4("127.0.0.0/24".parse().unwrap())),
                    serde_json::to_value(IpNet::V6("dead::/7".parse().unwrap())),
                ].into_iter().flatten().collect(),
                ..Default::default()
            })),
            subschemas: Some(Box::new(SubschemaValidation {
                any_of: Some(vec![
                    gen.subschema_for::<Ipv4Net>(),
                    gen.subschema_for::<Ipv6Net>(),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl JsonSchema for Ipv4Net {
    fn schema_name() -> String {
        "Ipv4Net".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            metadata: Some(Box::new(Metadata {
                description: Some(
                    "Represents a network address range where the IP addresses are v4 variants".to_string(),
                ),
                examples: vec![serde_json::to_value(
                    "127.0.0.0/24".parse::<Ipv4Net>().unwrap(),
                )]
                .into_iter()
                .flatten()
                .collect(),
                ..Default::default()
            })),
            instance_type: Some(InstanceType::String.into()),
            format: Some("ipv4-cidr".to_string()),
            ..Default::default()
        }
        .into()
    }
}

impl JsonSchema for Ipv6Net {
    fn schema_name() -> String {
        "Ipv6Net".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            metadata: Some(Box::new(Metadata {
                description: Some(
                    "Represents a network address range where the IP addresses are v4 variants".to_string(),
                ),
                examples: vec![serde_json::to_value(
                    "dead::/7".parse::<Ipv6Net>().unwrap(),
                )]
                .into_iter()
                .flatten()
                .collect(),
                ..Default::default()
            })),
            instance_type: Some(InstanceType::String.into()),
            format: Some("ipv6-cidr".to_string()),
            ..Default::default()
        }
        .into()
    }
}
