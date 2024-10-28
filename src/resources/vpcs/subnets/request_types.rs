#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    pub vpc_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "vpc-subnet-id", long = "vpc-subnet-id"))]
    pub vpc_subnet_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionVpcsVpcIdSubnetsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    pub vpc_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "page", long = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "page-size", long = "page-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    pub vpc_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "vpc-subnet-id", long = "vpc-subnet-id"))]
    pub vpc_subnet_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionVpcsVpcIdSubnetsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    pub vpc_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionVpcsVpcIdSubnetsBody,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    pub vpc_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "vpc-subnet-id", long = "vpc-subnet-id"))]
    pub vpc_subnet_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdBody,
}
