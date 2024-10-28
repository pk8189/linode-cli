#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionSupportTicketsTicketIdAttachmentsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "ticket-id", long = "ticket-id"))]
    pub ticket_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionSupportTicketsTicketIdAttachmentsBody,
}
