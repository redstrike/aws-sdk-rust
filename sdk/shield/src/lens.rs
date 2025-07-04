// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_attacks_output_output_next_token(
    input: &crate::operation::list_attacks::ListAttacksOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_protection_groups_output_output_next_token(
    input: &crate::operation::list_protection_groups::ListProtectionGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_protections_output_output_next_token(
    input: &crate::operation::list_protections::ListProtectionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_resources_in_protection_group_output_output_next_token(
    input: &crate::operation::list_resources_in_protection_group::ListResourcesInProtectionGroupOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_attacks_output_output_attack_summaries(
    input: crate::operation::list_attacks::ListAttacksOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AttackSummary>> {
    let input = input.attack_summaries?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_protections_output_output_protections(
    input: crate::operation::list_protections::ListProtectionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Protection>> {
    let input = input.protections?;
    ::std::option::Option::Some(input)
}
