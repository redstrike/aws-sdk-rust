// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_approval_teams_output_output_next_token(
    input: &crate::operation::list_approval_teams::ListApprovalTeamsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_identity_sources_output_output_next_token(
    input: &crate::operation::list_identity_sources::ListIdentitySourcesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_policies_output_output_next_token(
    input: &crate::operation::list_policies::ListPoliciesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_policy_versions_output_output_next_token(
    input: &crate::operation::list_policy_versions::ListPolicyVersionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_resource_policies_output_output_next_token(
    input: &crate::operation::list_resource_policies::ListResourcePoliciesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_sessions_output_output_next_token(
    input: &crate::operation::list_sessions::ListSessionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_approval_teams_output_output_approval_teams(
    input: crate::operation::list_approval_teams::ListApprovalTeamsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ListApprovalTeamsResponseApprovalTeam>> {
    let input = input.approval_teams?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_identity_sources_output_output_identity_sources(
    input: crate::operation::list_identity_sources::ListIdentitySourcesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::IdentitySourceForList>> {
    let input = input.identity_sources?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_policies_output_output_policies(
    input: crate::operation::list_policies::ListPoliciesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Policy>> {
    let input = input.policies?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_policy_versions_output_output_policy_versions(
    input: crate::operation::list_policy_versions::ListPolicyVersionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PolicyVersionSummary>> {
    let input = input.policy_versions?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_resource_policies_output_output_resource_policies(
    input: crate::operation::list_resource_policies::ListResourcePoliciesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ListResourcePoliciesResponseResourcePolicy>> {
    let input = input.resource_policies?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_sessions_output_output_sessions(
    input: crate::operation::list_sessions::ListSessionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ListSessionsResponseSession>> {
    let input = input.sessions?;
    ::std::option::Option::Some(input)
}
