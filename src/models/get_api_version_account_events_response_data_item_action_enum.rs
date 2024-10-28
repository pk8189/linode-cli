#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountEventsResponseDataItemActionEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "account_settings_update"))]
    #[serde(rename = "account_settings_update")]
    AccountSettingsUpdate,
    #[cfg_attr(feature = "cli", value(name = "account_update"))]
    #[serde(rename = "account_update")]
    AccountUpdate,
    #[cfg_attr(feature = "cli", value(name = "backups_cancel"))]
    #[serde(rename = "backups_cancel")]
    BackupsCancel,
    #[cfg_attr(feature = "cli", value(name = "backups_enable"))]
    #[serde(rename = "backups_enable")]
    BackupsEnable,
    #[cfg_attr(feature = "cli", value(name = "backups_restore"))]
    #[serde(rename = "backups_restore")]
    BackupsRestore,
    #[cfg_attr(feature = "cli", value(name = "community_like"))]
    #[serde(rename = "community_like")]
    CommunityLike,
    #[cfg_attr(feature = "cli", value(name = "community_question_reply"))]
    #[serde(rename = "community_question_reply")]
    CommunityQuestionReply,
    #[cfg_attr(feature = "cli", value(name = "credit_card_updated"))]
    #[serde(rename = "credit_card_updated")]
    CreditCardUpdated,
    #[cfg_attr(feature = "cli", value(name = "disk_create"))]
    #[serde(rename = "disk_create")]
    DiskCreate,
    #[cfg_attr(feature = "cli", value(name = "disk_delete"))]
    #[serde(rename = "disk_delete")]
    DiskDelete,
    #[cfg_attr(feature = "cli", value(name = "disk_duplicate"))]
    #[serde(rename = "disk_duplicate")]
    DiskDuplicate,
    #[cfg_attr(feature = "cli", value(name = "disk_imagize"))]
    #[serde(rename = "disk_imagize")]
    DiskImagize,
    #[cfg_attr(feature = "cli", value(name = "disk_resize"))]
    #[serde(rename = "disk_resize")]
    DiskResize,
    #[cfg_attr(feature = "cli", value(name = "disk_update"))]
    #[serde(rename = "disk_update")]
    DiskUpdate,
    #[cfg_attr(feature = "cli", value(name = "dns_record_create"))]
    #[serde(rename = "dns_record_create")]
    DnsRecordCreate,
    #[cfg_attr(feature = "cli", value(name = "dns_record_delete"))]
    #[serde(rename = "dns_record_delete")]
    DnsRecordDelete,
    #[cfg_attr(feature = "cli", value(name = "dns_record_update"))]
    #[serde(rename = "dns_record_update")]
    DnsRecordUpdate,
    #[cfg_attr(feature = "cli", value(name = "dns_zone_create"))]
    #[serde(rename = "dns_zone_create")]
    DnsZoneCreate,
    #[cfg_attr(feature = "cli", value(name = "dns_zone_delete"))]
    #[serde(rename = "dns_zone_delete")]
    DnsZoneDelete,
    #[cfg_attr(feature = "cli", value(name = "dns_zone_import"))]
    #[serde(rename = "dns_zone_import")]
    DnsZoneImport,
    #[cfg_attr(feature = "cli", value(name = "dns_zone_update"))]
    #[serde(rename = "dns_zone_update")]
    DnsZoneUpdate,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer_accept"))]
    #[serde(rename = "entity_transfer_accept")]
    EntityTransferAccept,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer_cancel"))]
    #[serde(rename = "entity_transfer_cancel")]
    EntityTransferCancel,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer_create"))]
    #[serde(rename = "entity_transfer_create")]
    EntityTransferCreate,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer_fail"))]
    #[serde(rename = "entity_transfer_fail")]
    EntityTransferFail,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer_stale"))]
    #[serde(rename = "entity_transfer_stale")]
    EntityTransferStale,
    #[cfg_attr(feature = "cli", value(name = "firewall_apply"))]
    #[serde(rename = "firewall_apply")]
    FirewallApply,
    #[cfg_attr(feature = "cli", value(name = "firewall_create"))]
    #[serde(rename = "firewall_create")]
    FirewallCreate,
    #[cfg_attr(feature = "cli", value(name = "firewall_delete"))]
    #[serde(rename = "firewall_delete")]
    FirewallDelete,
    #[cfg_attr(feature = "cli", value(name = "firewall_device_add"))]
    #[serde(rename = "firewall_device_add")]
    FirewallDeviceAdd,
    #[cfg_attr(feature = "cli", value(name = "firewall_device_remove"))]
    #[serde(rename = "firewall_device_remove")]
    FirewallDeviceRemove,
    #[cfg_attr(feature = "cli", value(name = "firewall_disable"))]
    #[serde(rename = "firewall_disable")]
    FirewallDisable,
    #[cfg_attr(feature = "cli", value(name = "firewall_enable"))]
    #[serde(rename = "firewall_enable")]
    FirewallEnable,
    #[cfg_attr(feature = "cli", value(name = "firewall_update"))]
    #[serde(rename = "firewall_update")]
    FirewallUpdate,
    #[cfg_attr(feature = "cli", value(name = "host_reboot"))]
    #[serde(rename = "host_reboot")]
    HostReboot,
    #[cfg_attr(feature = "cli", value(name = "image_delete"))]
    #[serde(rename = "image_delete")]
    ImageDelete,
    #[cfg_attr(feature = "cli", value(name = "image_update"))]
    #[serde(rename = "image_update")]
    ImageUpdate,
    #[cfg_attr(feature = "cli", value(name = "image_upload"))]
    #[serde(rename = "image_upload")]
    ImageUpload,
    #[cfg_attr(feature = "cli", value(name = "ipaddress_update"))]
    #[serde(rename = "ipaddress_update")]
    IpaddressUpdate,
    #[cfg_attr(feature = "cli", value(name = "lassie_reboot"))]
    #[serde(rename = "lassie_reboot")]
    LassieReboot,
    #[cfg_attr(feature = "cli", value(name = "linode_addip"))]
    #[serde(rename = "linode_addip")]
    LinodeAddip,
    #[cfg_attr(feature = "cli", value(name = "linode_boot"))]
    #[serde(rename = "linode_boot")]
    LinodeBoot,
    #[cfg_attr(feature = "cli", value(name = "linode_clone"))]
    #[serde(rename = "linode_clone")]
    LinodeClone,
    #[cfg_attr(feature = "cli", value(name = "linode_config_create"))]
    #[serde(rename = "linode_config_create")]
    LinodeConfigCreate,
    #[cfg_attr(feature = "cli", value(name = "linode_config_delete"))]
    #[serde(rename = "linode_config_delete")]
    LinodeConfigDelete,
    #[cfg_attr(feature = "cli", value(name = "linode_config_update"))]
    #[serde(rename = "linode_config_update")]
    LinodeConfigUpdate,
    #[cfg_attr(feature = "cli", value(name = "linode_create"))]
    #[serde(rename = "linode_create")]
    LinodeCreate,
    #[cfg_attr(feature = "cli", value(name = "linode_delete"))]
    #[serde(rename = "linode_delete")]
    LinodeDelete,
    #[cfg_attr(feature = "cli", value(name = "linode_deleteip"))]
    #[serde(rename = "linode_deleteip")]
    LinodeDeleteip,
    #[cfg_attr(feature = "cli", value(name = "linode_migrate"))]
    #[serde(rename = "linode_migrate")]
    LinodeMigrate,
    #[cfg_attr(feature = "cli", value(name = "linode_migrate_datacenter"))]
    #[serde(rename = "linode_migrate_datacenter")]
    LinodeMigrateDatacenter,
    #[cfg_attr(feature = "cli", value(name = "linode_migrate_datacenter_create"))]
    #[serde(rename = "linode_migrate_datacenter_create")]
    LinodeMigrateDatacenterCreate,
    #[cfg_attr(feature = "cli", value(name = "linode_mutate"))]
    #[serde(rename = "linode_mutate")]
    LinodeMutate,
    #[cfg_attr(feature = "cli", value(name = "linode_mutate_create"))]
    #[serde(rename = "linode_mutate_create")]
    LinodeMutateCreate,
    #[cfg_attr(feature = "cli", value(name = "linode_reboot"))]
    #[serde(rename = "linode_reboot")]
    LinodeReboot,
    #[cfg_attr(feature = "cli", value(name = "linode_rebuild"))]
    #[serde(rename = "linode_rebuild")]
    LinodeRebuild,
    #[cfg_attr(feature = "cli", value(name = "linode_resize"))]
    #[serde(rename = "linode_resize")]
    LinodeResize,
    #[cfg_attr(feature = "cli", value(name = "linode_resize_create"))]
    #[serde(rename = "linode_resize_create")]
    LinodeResizeCreate,
    #[cfg_attr(feature = "cli", value(name = "linode_shutdown"))]
    #[serde(rename = "linode_shutdown")]
    LinodeShutdown,
    #[cfg_attr(feature = "cli", value(name = "linode_snapshot"))]
    #[serde(rename = "linode_snapshot")]
    LinodeSnapshot,
    #[cfg_attr(feature = "cli", value(name = "linode_update"))]
    #[serde(rename = "linode_update")]
    LinodeUpdate,
    #[cfg_attr(feature = "cli", value(name = "lish_boot"))]
    #[serde(rename = "lish_boot")]
    LishBoot,
    #[cfg_attr(feature = "cli", value(name = "lke_cluster_create"))]
    #[serde(rename = "lke_cluster_create")]
    LkeClusterCreate,
    #[cfg_attr(feature = "cli", value(name = "lke_cluster_delete"))]
    #[serde(rename = "lke_cluster_delete")]
    LkeClusterDelete,
    #[cfg_attr(feature = "cli", value(name = "lke_cluster_recycle"))]
    #[serde(rename = "lke_cluster_recycle")]
    LkeClusterRecycle,
    #[cfg_attr(feature = "cli", value(name = "lke_cluster_regenerate"))]
    #[serde(rename = "lke_cluster_regenerate")]
    LkeClusterRegenerate,
    #[cfg_attr(feature = "cli", value(name = "lke_cluster_update"))]
    #[serde(rename = "lke_cluster_update")]
    LkeClusterUpdate,
    #[cfg_attr(feature = "cli", value(name = "lke_kubeconfig_regenerate"))]
    #[serde(rename = "lke_kubeconfig_regenerate")]
    LkeKubeconfigRegenerate,
    #[cfg_attr(feature = "cli", value(name = "lke_node_create"))]
    #[serde(rename = "lke_node_create")]
    LkeNodeCreate,
    #[cfg_attr(feature = "cli", value(name = "lke_node_delete"))]
    #[serde(rename = "lke_node_delete")]
    LkeNodeDelete,
    #[cfg_attr(feature = "cli", value(name = "lke_node_recycle"))]
    #[serde(rename = "lke_node_recycle")]
    LkeNodeRecycle,
    #[cfg_attr(feature = "cli", value(name = "lke_pool_create"))]
    #[serde(rename = "lke_pool_create")]
    LkePoolCreate,
    #[cfg_attr(feature = "cli", value(name = "lke_pool_delete"))]
    #[serde(rename = "lke_pool_delete")]
    LkePoolDelete,
    #[cfg_attr(feature = "cli", value(name = "lke_pool_recycle"))]
    #[serde(rename = "lke_pool_recycle")]
    LkePoolRecycle,
    #[cfg_attr(feature = "cli", value(name = "lke_token_rotate"))]
    #[serde(rename = "lke_token_rotate")]
    LkeTokenRotate,
    #[cfg_attr(feature = "cli", value(name = "longviewclient_create"))]
    #[serde(rename = "longviewclient_create")]
    LongviewclientCreate,
    #[cfg_attr(feature = "cli", value(name = "longviewclient_delete"))]
    #[serde(rename = "longviewclient_delete")]
    LongviewclientDelete,
    #[cfg_attr(feature = "cli", value(name = "longviewclient_update"))]
    #[serde(rename = "longviewclient_update")]
    LongviewclientUpdate,
    #[cfg_attr(feature = "cli", value(name = "managed_disabled"))]
    #[serde(rename = "managed_disabled")]
    ManagedDisabled,
    #[cfg_attr(feature = "cli", value(name = "managed_enabled"))]
    #[serde(rename = "managed_enabled")]
    ManagedEnabled,
    #[cfg_attr(feature = "cli", value(name = "managed_service_create"))]
    #[serde(rename = "managed_service_create")]
    ManagedServiceCreate,
    #[cfg_attr(feature = "cli", value(name = "managed_service_delete"))]
    #[serde(rename = "managed_service_delete")]
    ManagedServiceDelete,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_config_create"))]
    #[serde(rename = "nodebalancer_config_create")]
    NodebalancerConfigCreate,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_config_delete"))]
    #[serde(rename = "nodebalancer_config_delete")]
    NodebalancerConfigDelete,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_config_update"))]
    #[serde(rename = "nodebalancer_config_update")]
    NodebalancerConfigUpdate,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_create"))]
    #[serde(rename = "nodebalancer_create")]
    NodebalancerCreate,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_delete"))]
    #[serde(rename = "nodebalancer_delete")]
    NodebalancerDelete,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_node_create"))]
    #[serde(rename = "nodebalancer_node_create")]
    NodebalancerNodeCreate,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_node_delete"))]
    #[serde(rename = "nodebalancer_node_delete")]
    NodebalancerNodeDelete,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_node_update"))]
    #[serde(rename = "nodebalancer_node_update")]
    NodebalancerNodeUpdate,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer_update"))]
    #[serde(rename = "nodebalancer_update")]
    NodebalancerUpdate,
    #[cfg_attr(feature = "cli", value(name = "oauth_client_create"))]
    #[serde(rename = "oauth_client_create")]
    OauthClientCreate,
    #[cfg_attr(feature = "cli", value(name = "oauth_client_delete"))]
    #[serde(rename = "oauth_client_delete")]
    OauthClientDelete,
    #[cfg_attr(feature = "cli", value(name = "oauth_client_secret_reset"))]
    #[serde(rename = "oauth_client_secret_reset")]
    OauthClientSecretReset,
    #[cfg_attr(feature = "cli", value(name = "oauth_client_update"))]
    #[serde(rename = "oauth_client_update")]
    OauthClientUpdate,
    #[cfg_attr(feature = "cli", value(name = "obj_access_key_create"))]
    #[serde(rename = "obj_access_key_create")]
    ObjAccessKeyCreate,
    #[cfg_attr(feature = "cli", value(name = "obj_access_key_delete"))]
    #[serde(rename = "obj_access_key_delete")]
    ObjAccessKeyDelete,
    #[cfg_attr(feature = "cli", value(name = "obj_access_key_update"))]
    #[serde(rename = "obj_access_key_update")]
    ObjAccessKeyUpdate,
    #[cfg_attr(feature = "cli", value(name = "password_reset"))]
    #[serde(rename = "password_reset")]
    PasswordReset,
    #[cfg_attr(feature = "cli", value(name = "payment_method_add"))]
    #[serde(rename = "payment_method_add")]
    PaymentMethodAdd,
    #[cfg_attr(feature = "cli", value(name = "payment_submitted"))]
    #[serde(rename = "payment_submitted")]
    PaymentSubmitted,
    #[cfg_attr(feature = "cli", value(name = "placement_group_assign"))]
    #[serde(rename = "placement_group_assign")]
    PlacementGroupAssign,
    #[cfg_attr(feature = "cli", value(name = "placement_group_became_compliant"))]
    #[serde(rename = "placement_group_became_compliant")]
    PlacementGroupBecameCompliant,
    #[cfg_attr(feature = "cli", value(name = "placement_group_became_non_compliant"))]
    #[serde(rename = "placement_group_became_non_compliant")]
    PlacementGroupBecameNonCompliant,
    #[cfg_attr(feature = "cli", value(name = "placement_group_create"))]
    #[serde(rename = "placement_group_create")]
    PlacementGroupCreate,
    #[cfg_attr(feature = "cli", value(name = "placement_group_delete"))]
    #[serde(rename = "placement_group_delete")]
    PlacementGroupDelete,
    #[cfg_attr(feature = "cli", value(name = "placement_group_unassign"))]
    #[serde(rename = "placement_group_unassign")]
    PlacementGroupUnassign,
    #[cfg_attr(feature = "cli", value(name = "placement_group_update"))]
    #[serde(rename = "placement_group_update")]
    PlacementGroupUpdate,
    #[cfg_attr(feature = "cli", value(name = "profile_update"))]
    #[serde(rename = "profile_update")]
    ProfileUpdate,
    #[cfg_attr(feature = "cli", value(name = "stackscript_create"))]
    #[serde(rename = "stackscript_create")]
    StackscriptCreate,
    #[cfg_attr(feature = "cli", value(name = "stackscript_delete"))]
    #[serde(rename = "stackscript_delete")]
    StackscriptDelete,
    #[cfg_attr(feature = "cli", value(name = "stackscript_publicize"))]
    #[serde(rename = "stackscript_publicize")]
    StackscriptPublicize,
    #[cfg_attr(feature = "cli", value(name = "stackscript_revise"))]
    #[serde(rename = "stackscript_revise")]
    StackscriptRevise,
    #[cfg_attr(feature = "cli", value(name = "stackscript_update"))]
    #[serde(rename = "stackscript_update")]
    StackscriptUpdate,
    #[cfg_attr(feature = "cli", value(name = "subnet_create"))]
    #[serde(rename = "subnet_create")]
    SubnetCreate,
    #[cfg_attr(feature = "cli", value(name = "subnet_delete"))]
    #[serde(rename = "subnet_delete")]
    SubnetDelete,
    #[cfg_attr(feature = "cli", value(name = "subnet_update"))]
    #[serde(rename = "subnet_update")]
    SubnetUpdate,
    #[cfg_attr(feature = "cli", value(name = "tag_create"))]
    #[serde(rename = "tag_create")]
    TagCreate,
    #[cfg_attr(feature = "cli", value(name = "tag_delete"))]
    #[serde(rename = "tag_delete")]
    TagDelete,
    #[cfg_attr(feature = "cli", value(name = "tfa_disabled"))]
    #[serde(rename = "tfa_disabled")]
    TfaDisabled,
    #[cfg_attr(feature = "cli", value(name = "tfa_enabled"))]
    #[serde(rename = "tfa_enabled")]
    TfaEnabled,
    #[cfg_attr(feature = "cli", value(name = "ticket_attachment_upload"))]
    #[serde(rename = "ticket_attachment_upload")]
    TicketAttachmentUpload,
    #[cfg_attr(feature = "cli", value(name = "ticket_create"))]
    #[serde(rename = "ticket_create")]
    TicketCreate,
    #[cfg_attr(feature = "cli", value(name = "ticket_update"))]
    #[serde(rename = "ticket_update")]
    TicketUpdate,
    #[cfg_attr(feature = "cli", value(name = "token_create"))]
    #[serde(rename = "token_create")]
    TokenCreate,
    #[cfg_attr(feature = "cli", value(name = "token_delete"))]
    #[serde(rename = "token_delete")]
    TokenDelete,
    #[cfg_attr(feature = "cli", value(name = "token_update"))]
    #[serde(rename = "token_update")]
    TokenUpdate,
    #[cfg_attr(feature = "cli", value(name = "user_create"))]
    #[serde(rename = "user_create")]
    UserCreate,
    #[cfg_attr(feature = "cli", value(name = "user_delete"))]
    #[serde(rename = "user_delete")]
    UserDelete,
    #[cfg_attr(feature = "cli", value(name = "user_ssh_key_add"))]
    #[serde(rename = "user_ssh_key_add")]
    UserSshKeyAdd,
    #[cfg_attr(feature = "cli", value(name = "user_ssh_key_delete"))]
    #[serde(rename = "user_ssh_key_delete")]
    UserSshKeyDelete,
    #[cfg_attr(feature = "cli", value(name = "user_ssh_key_update"))]
    #[serde(rename = "user_ssh_key_update")]
    UserSshKeyUpdate,
    #[cfg_attr(feature = "cli", value(name = "user_update"))]
    #[serde(rename = "user_update")]
    UserUpdate,
    #[cfg_attr(feature = "cli", value(name = "vlan_attach"))]
    #[serde(rename = "vlan_attach")]
    VlanAttach,
    #[cfg_attr(feature = "cli", value(name = "vlan_detach"))]
    #[serde(rename = "vlan_detach")]
    VlanDetach,
    #[cfg_attr(feature = "cli", value(name = "volume_attach"))]
    #[serde(rename = "volume_attach")]
    VolumeAttach,
    #[cfg_attr(feature = "cli", value(name = "volume_clone"))]
    #[serde(rename = "volume_clone")]
    VolumeClone,
    #[cfg_attr(feature = "cli", value(name = "volume_create"))]
    #[serde(rename = "volume_create")]
    VolumeCreate,
    #[cfg_attr(feature = "cli", value(name = "volume_delete"))]
    #[serde(rename = "volume_delete")]
    VolumeDelete,
    #[cfg_attr(feature = "cli", value(name = "volume_detach"))]
    #[serde(rename = "volume_detach")]
    VolumeDetach,
    #[cfg_attr(feature = "cli", value(name = "volume_resize"))]
    #[serde(rename = "volume_resize")]
    VolumeResize,
    #[cfg_attr(feature = "cli", value(name = "volume_update"))]
    #[serde(rename = "volume_update")]
    VolumeUpdate,
    #[cfg_attr(feature = "cli", value(name = "vpc_create"))]
    #[serde(rename = "vpc_create")]
    VpcCreate,
    #[cfg_attr(feature = "cli", value(name = "vpc_delete"))]
    #[serde(rename = "vpc_delete")]
    VpcDelete,
    #[cfg_attr(feature = "cli", value(name = "vpc_update"))]
    #[serde(rename = "vpc_update")]
    VpcUpdate,
}
impl std::fmt::Display for GetApiVersionAccountEventsResponseDataItemActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountEventsResponseDataItemActionEnum::AccountSettingsUpdate => {
                "account_settings_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::AccountUpdate => {
                "account_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::BackupsCancel => {
                "backups_cancel"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::BackupsEnable => {
                "backups_enable"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::BackupsRestore => {
                "backups_restore"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::CommunityLike => {
                "community_like"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::CommunityQuestionReply => {
                "community_question_reply"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::CreditCardUpdated => {
                "credit_card_updated"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DiskCreate => {
                "disk_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DiskDelete => {
                "disk_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DiskDuplicate => {
                "disk_duplicate"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DiskImagize => {
                "disk_imagize"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DiskResize => {
                "disk_resize"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DiskUpdate => {
                "disk_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsRecordCreate => {
                "dns_record_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsRecordDelete => {
                "dns_record_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsRecordUpdate => {
                "dns_record_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsZoneCreate => {
                "dns_zone_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsZoneDelete => {
                "dns_zone_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsZoneImport => {
                "dns_zone_import"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::DnsZoneUpdate => {
                "dns_zone_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::EntityTransferAccept => {
                "entity_transfer_accept"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::EntityTransferCancel => {
                "entity_transfer_cancel"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::EntityTransferCreate => {
                "entity_transfer_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::EntityTransferFail => {
                "entity_transfer_fail"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::EntityTransferStale => {
                "entity_transfer_stale"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallApply => {
                "firewall_apply"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallCreate => {
                "firewall_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallDelete => {
                "firewall_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallDeviceAdd => {
                "firewall_device_add"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallDeviceRemove => {
                "firewall_device_remove"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallDisable => {
                "firewall_disable"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallEnable => {
                "firewall_enable"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::FirewallUpdate => {
                "firewall_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::HostReboot => {
                "host_reboot"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ImageDelete => {
                "image_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ImageUpdate => {
                "image_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ImageUpload => {
                "image_upload"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::IpaddressUpdate => {
                "ipaddress_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LassieReboot => {
                "lassie_reboot"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeAddip => {
                "linode_addip"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeBoot => {
                "linode_boot"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeClone => {
                "linode_clone"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeConfigCreate => {
                "linode_config_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeConfigDelete => {
                "linode_config_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeConfigUpdate => {
                "linode_config_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeCreate => {
                "linode_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeDelete => {
                "linode_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeDeleteip => {
                "linode_deleteip"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeMigrate => {
                "linode_migrate"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeMigrateDatacenter => {
                "linode_migrate_datacenter"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeMigrateDatacenterCreate => {
                "linode_migrate_datacenter_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeMutate => {
                "linode_mutate"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeMutateCreate => {
                "linode_mutate_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeReboot => {
                "linode_reboot"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeRebuild => {
                "linode_rebuild"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeResize => {
                "linode_resize"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeResizeCreate => {
                "linode_resize_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeShutdown => {
                "linode_shutdown"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeSnapshot => {
                "linode_snapshot"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LinodeUpdate => {
                "linode_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LishBoot => "lish_boot",
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeClusterCreate => {
                "lke_cluster_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeClusterDelete => {
                "lke_cluster_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeClusterRecycle => {
                "lke_cluster_recycle"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeClusterRegenerate => {
                "lke_cluster_regenerate"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeClusterUpdate => {
                "lke_cluster_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeKubeconfigRegenerate => {
                "lke_kubeconfig_regenerate"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeNodeCreate => {
                "lke_node_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeNodeDelete => {
                "lke_node_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeNodeRecycle => {
                "lke_node_recycle"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkePoolCreate => {
                "lke_pool_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkePoolDelete => {
                "lke_pool_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkePoolRecycle => {
                "lke_pool_recycle"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LkeTokenRotate => {
                "lke_token_rotate"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LongviewclientCreate => {
                "longviewclient_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LongviewclientDelete => {
                "longviewclient_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::LongviewclientUpdate => {
                "longviewclient_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ManagedDisabled => {
                "managed_disabled"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ManagedEnabled => {
                "managed_enabled"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ManagedServiceCreate => {
                "managed_service_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ManagedServiceDelete => {
                "managed_service_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerConfigCreate => {
                "nodebalancer_config_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerConfigDelete => {
                "nodebalancer_config_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerConfigUpdate => {
                "nodebalancer_config_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerCreate => {
                "nodebalancer_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerDelete => {
                "nodebalancer_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerNodeCreate => {
                "nodebalancer_node_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerNodeDelete => {
                "nodebalancer_node_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerNodeUpdate => {
                "nodebalancer_node_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::NodebalancerUpdate => {
                "nodebalancer_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::OauthClientCreate => {
                "oauth_client_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::OauthClientDelete => {
                "oauth_client_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::OauthClientSecretReset => {
                "oauth_client_secret_reset"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::OauthClientUpdate => {
                "oauth_client_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ObjAccessKeyCreate => {
                "obj_access_key_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ObjAccessKeyDelete => {
                "obj_access_key_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ObjAccessKeyUpdate => {
                "obj_access_key_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PasswordReset => {
                "password_reset"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PaymentMethodAdd => {
                "payment_method_add"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PaymentSubmitted => {
                "payment_submitted"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupAssign => {
                "placement_group_assign"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupBecameCompliant => {
                "placement_group_became_compliant"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupBecameNonCompliant => {
                "placement_group_became_non_compliant"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupCreate => {
                "placement_group_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupDelete => {
                "placement_group_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupUnassign => {
                "placement_group_unassign"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::PlacementGroupUpdate => {
                "placement_group_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::ProfileUpdate => {
                "profile_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::StackscriptCreate => {
                "stackscript_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::StackscriptDelete => {
                "stackscript_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::StackscriptPublicize => {
                "stackscript_publicize"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::StackscriptRevise => {
                "stackscript_revise"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::StackscriptUpdate => {
                "stackscript_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::SubnetCreate => {
                "subnet_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::SubnetDelete => {
                "subnet_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::SubnetUpdate => {
                "subnet_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TagCreate => {
                "tag_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TagDelete => {
                "tag_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TfaDisabled => {
                "tfa_disabled"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TfaEnabled => {
                "tfa_enabled"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TicketAttachmentUpload => {
                "ticket_attachment_upload"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TicketCreate => {
                "ticket_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TicketUpdate => {
                "ticket_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TokenCreate => {
                "token_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TokenDelete => {
                "token_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::TokenUpdate => {
                "token_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::UserCreate => {
                "user_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::UserDelete => {
                "user_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::UserSshKeyAdd => {
                "user_ssh_key_add"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::UserSshKeyDelete => {
                "user_ssh_key_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::UserSshKeyUpdate => {
                "user_ssh_key_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::UserUpdate => {
                "user_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VlanAttach => {
                "vlan_attach"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VlanDetach => {
                "vlan_detach"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeAttach => {
                "volume_attach"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeClone => {
                "volume_clone"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeCreate => {
                "volume_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeDelete => {
                "volume_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeDetach => {
                "volume_detach"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeResize => {
                "volume_resize"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VolumeUpdate => {
                "volume_update"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VpcCreate => {
                "vpc_create"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VpcDelete => {
                "vpc_delete"
            }
            GetApiVersionAccountEventsResponseDataItemActionEnum::VpcUpdate => {
                "vpc_update"
            }
        };
        write!(f, "{}", str_val)
    }
}
