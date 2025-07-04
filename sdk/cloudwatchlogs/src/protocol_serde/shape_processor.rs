// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_processor(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Processor,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.add_keys {
        #[allow(unused_mut)]
        let mut object_2 = object.key("addKeys").start_object();
        crate::protocol_serde::shape_add_keys::ser_add_keys(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.copy_value {
        #[allow(unused_mut)]
        let mut object_4 = object.key("copyValue").start_object();
        crate::protocol_serde::shape_copy_value::ser_copy_value(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.csv {
        #[allow(unused_mut)]
        let mut object_6 = object.key("csv").start_object();
        crate::protocol_serde::shape_csv::ser_csv(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.date_time_converter {
        #[allow(unused_mut)]
        let mut object_8 = object.key("dateTimeConverter").start_object();
        crate::protocol_serde::shape_date_time_converter::ser_date_time_converter(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.delete_keys {
        #[allow(unused_mut)]
        let mut object_10 = object.key("deleteKeys").start_object();
        crate::protocol_serde::shape_delete_keys::ser_delete_keys(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.grok {
        #[allow(unused_mut)]
        let mut object_12 = object.key("grok").start_object();
        crate::protocol_serde::shape_grok::ser_grok(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.list_to_map {
        #[allow(unused_mut)]
        let mut object_14 = object.key("listToMap").start_object();
        crate::protocol_serde::shape_list_to_map::ser_list_to_map(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.lower_case_string {
        #[allow(unused_mut)]
        let mut object_16 = object.key("lowerCaseString").start_object();
        crate::protocol_serde::shape_lower_case_string::ser_lower_case_string(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.move_keys {
        #[allow(unused_mut)]
        let mut object_18 = object.key("moveKeys").start_object();
        crate::protocol_serde::shape_move_keys::ser_move_keys(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.parse_cloudfront {
        #[allow(unused_mut)]
        let mut object_20 = object.key("parseCloudfront").start_object();
        crate::protocol_serde::shape_parse_cloudfront::ser_parse_cloudfront(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.parse_json {
        #[allow(unused_mut)]
        let mut object_22 = object.key("parseJSON").start_object();
        crate::protocol_serde::shape_parse_json::ser_parse_json(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.parse_key_value {
        #[allow(unused_mut)]
        let mut object_24 = object.key("parseKeyValue").start_object();
        crate::protocol_serde::shape_parse_key_value::ser_parse_key_value(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.parse_route53 {
        #[allow(unused_mut)]
        let mut object_26 = object.key("parseRoute53").start_object();
        crate::protocol_serde::shape_parse_route53::ser_parse_route53(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.parse_to_ocsf {
        #[allow(unused_mut)]
        let mut object_28 = object.key("parseToOCSF").start_object();
        crate::protocol_serde::shape_parse_to_ocsf::ser_parse_to_ocsf(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.parse_postgres {
        #[allow(unused_mut)]
        let mut object_30 = object.key("parsePostgres").start_object();
        crate::protocol_serde::shape_parse_postgres::ser_parse_postgres(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.parse_vpc {
        #[allow(unused_mut)]
        let mut object_32 = object.key("parseVPC").start_object();
        crate::protocol_serde::shape_parse_vpc::ser_parse_vpc(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.parse_waf {
        #[allow(unused_mut)]
        let mut object_34 = object.key("parseWAF").start_object();
        crate::protocol_serde::shape_parse_waf::ser_parse_waf(&mut object_34, var_33)?;
        object_34.finish();
    }
    if let Some(var_35) = &input.rename_keys {
        #[allow(unused_mut)]
        let mut object_36 = object.key("renameKeys").start_object();
        crate::protocol_serde::shape_rename_keys::ser_rename_keys(&mut object_36, var_35)?;
        object_36.finish();
    }
    if let Some(var_37) = &input.split_string {
        #[allow(unused_mut)]
        let mut object_38 = object.key("splitString").start_object();
        crate::protocol_serde::shape_split_string::ser_split_string(&mut object_38, var_37)?;
        object_38.finish();
    }
    if let Some(var_39) = &input.substitute_string {
        #[allow(unused_mut)]
        let mut object_40 = object.key("substituteString").start_object();
        crate::protocol_serde::shape_substitute_string::ser_substitute_string(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.trim_string {
        #[allow(unused_mut)]
        let mut object_42 = object.key("trimString").start_object();
        crate::protocol_serde::shape_trim_string::ser_trim_string(&mut object_42, var_41)?;
        object_42.finish();
    }
    if let Some(var_43) = &input.type_converter {
        #[allow(unused_mut)]
        let mut object_44 = object.key("typeConverter").start_object();
        crate::protocol_serde::shape_type_converter::ser_type_converter(&mut object_44, var_43)?;
        object_44.finish();
    }
    if let Some(var_45) = &input.upper_case_string {
        #[allow(unused_mut)]
        let mut object_46 = object.key("upperCaseString").start_object();
        crate::protocol_serde::shape_upper_case_string::ser_upper_case_string(&mut object_46, var_45)?;
        object_46.finish();
    }
    Ok(())
}

pub(crate) fn de_processor<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Processor>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ProcessorBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "addKeys" => {
                            builder = builder.set_add_keys(crate::protocol_serde::shape_add_keys::de_add_keys(tokens)?);
                        }
                        "copyValue" => {
                            builder = builder.set_copy_value(crate::protocol_serde::shape_copy_value::de_copy_value(tokens)?);
                        }
                        "csv" => {
                            builder = builder.set_csv(crate::protocol_serde::shape_csv::de_csv(tokens)?);
                        }
                        "dateTimeConverter" => {
                            builder =
                                builder.set_date_time_converter(crate::protocol_serde::shape_date_time_converter::de_date_time_converter(tokens)?);
                        }
                        "deleteKeys" => {
                            builder = builder.set_delete_keys(crate::protocol_serde::shape_delete_keys::de_delete_keys(tokens)?);
                        }
                        "grok" => {
                            builder = builder.set_grok(crate::protocol_serde::shape_grok::de_grok(tokens)?);
                        }
                        "listToMap" => {
                            builder = builder.set_list_to_map(crate::protocol_serde::shape_list_to_map::de_list_to_map(tokens)?);
                        }
                        "lowerCaseString" => {
                            builder = builder.set_lower_case_string(crate::protocol_serde::shape_lower_case_string::de_lower_case_string(tokens)?);
                        }
                        "moveKeys" => {
                            builder = builder.set_move_keys(crate::protocol_serde::shape_move_keys::de_move_keys(tokens)?);
                        }
                        "parseCloudfront" => {
                            builder = builder.set_parse_cloudfront(crate::protocol_serde::shape_parse_cloudfront::de_parse_cloudfront(tokens)?);
                        }
                        "parseJSON" => {
                            builder = builder.set_parse_json(crate::protocol_serde::shape_parse_json::de_parse_json(tokens)?);
                        }
                        "parseKeyValue" => {
                            builder = builder.set_parse_key_value(crate::protocol_serde::shape_parse_key_value::de_parse_key_value(tokens)?);
                        }
                        "parseRoute53" => {
                            builder = builder.set_parse_route53(crate::protocol_serde::shape_parse_route53::de_parse_route53(tokens)?);
                        }
                        "parseToOCSF" => {
                            builder = builder.set_parse_to_ocsf(crate::protocol_serde::shape_parse_to_ocsf::de_parse_to_ocsf(tokens)?);
                        }
                        "parsePostgres" => {
                            builder = builder.set_parse_postgres(crate::protocol_serde::shape_parse_postgres::de_parse_postgres(tokens)?);
                        }
                        "parseVPC" => {
                            builder = builder.set_parse_vpc(crate::protocol_serde::shape_parse_vpc::de_parse_vpc(tokens)?);
                        }
                        "parseWAF" => {
                            builder = builder.set_parse_waf(crate::protocol_serde::shape_parse_waf::de_parse_waf(tokens)?);
                        }
                        "renameKeys" => {
                            builder = builder.set_rename_keys(crate::protocol_serde::shape_rename_keys::de_rename_keys(tokens)?);
                        }
                        "splitString" => {
                            builder = builder.set_split_string(crate::protocol_serde::shape_split_string::de_split_string(tokens)?);
                        }
                        "substituteString" => {
                            builder = builder.set_substitute_string(crate::protocol_serde::shape_substitute_string::de_substitute_string(tokens)?);
                        }
                        "trimString" => {
                            builder = builder.set_trim_string(crate::protocol_serde::shape_trim_string::de_trim_string(tokens)?);
                        }
                        "typeConverter" => {
                            builder = builder.set_type_converter(crate::protocol_serde::shape_type_converter::de_type_converter(tokens)?);
                        }
                        "upperCaseString" => {
                            builder = builder.set_upper_case_string(crate::protocol_serde::shape_upper_case_string::de_upper_case_string(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
