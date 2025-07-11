// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_git_lab_self_managed_integration_detail(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateGitLabSelfManagedIntegrationDetail,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("instanceUrl").string(input.instance_url.as_str());
    }
    {
        object.key("accessToken").string(input.access_token.as_str());
    }
    Ok(())
}
