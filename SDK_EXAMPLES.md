
## SDK Usage Examples


### Cancel an entity transfer
__Deprecated__ Please run [Cancel a service transfer](https://techdocs.akamai.com/linode-api/reference/delete-service-transfer).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/account/entity-transfers/{token}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .entity_transfers()
    .delete(linode_api::resources::account::entity_transfers::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionAccountEntityTransfersTokenApiVersionEnum::V4,
        token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete an OAuth client
Deletes an OAuth Client registered with Linode. The Client ID and Client secret will no longer be accepted by [login.linode.com](https://login.linode.com), and all tokens issued to this client will be invalidated (meaning that if your application was using a token, it will no longer work).


<<LB>>

---


- __CLI__.

    ```
    linode-cli account client-delete \
  edc6790ea9db4d224c5c
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/account/oauth-clients/{clientId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .delete(linode_api::resources::account::oauth_clients::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionAccountOauthClientsClientIdApiVersionEnum::V4,
        client_id: "string".to_string(),
    })
    .await;
```

    
### Delete a payment method
Deactivate the specified Payment Method.

The default Payment Method can not be deleted. To add a new default Payment Method, run the [Add a payment method](https://techdocs.akamai.com/linode-api/reference/post-payment-method) operation. To designate an existing Payment Method as the default method, run the [Set a default payment method](https://techdocs.akamai.com/linode-api/reference/post-make-payment-method-default) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli payment-methods delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/account/payment-methods/{paymentMethodId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payment_methods()
    .delete(linode_api::resources::account::payment_methods::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionAccountPaymentMethodsPaymentMethodIdApiVersionEnum::V4,
        payment_method_id: 123,
    })
    .await;
```

    
### Cancel a service transfer
Cancels the Service Transfer for the provided token. Once canceled, a transfer cannot be accepted or otherwise acted on in any way. If canceled in error, the transfer must be [created](https://techdocs.akamai.com/linode-api/reference/post-service-transfer) again.

When canceled, an email notification for the cancellation is sent to the account that created this transfer. Transfers can not be canceled if they are expired or have been accepted.

This operation can only be accessed by the unrestricted users of the account that created this transfer.


<<LB>>

---


- __CLI__.

    ```
    linode-cli service-transfers \
  cancel 123E4567-E89B-12D3-A456-426614174000
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/account/service-transfers/{token}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .service_transfers()
    .delete(linode_api::resources::account::service_transfers::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionAccountServiceTransfersTokenApiVersionEnum::V4,
        token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete a user
Deletes a user. The API immediately logs the user out and removes all of its `grants`.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- You can't delete a child account parent user (proxy user). The API returns a 403 error if you target a proxy user with this operation.

- A parent account using an unrestricted proxy user can use this operation to delete a child account user.


<<LB>>

---


- __CLI__.

    ```
    linode-cli users delete example_user
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/account/users/{username}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .users()
    .delete(linode_api::resources::account::users::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionAccountUsersUsernameApiVersionEnum::V4,
        username: "string".to_string(),
    })
    .await;
```

    
### Delete a managed MySQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Remove a Managed MySQL Database from your Account.

Requires `read_write` access to the Database.

The Database must have an `active`, `failed`, or `degraded` status to perform this operation.

Only unrestricted Users can access this operation, and have access regardless of the acting token's OAuth scopes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/databases/mysql/instances/{instanceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .delete(linode_api::resources::databases::mysql::instances::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionDatabasesMysqlInstancesInstanceIdApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Delete a managed MySQL database backup
__This operation is currently only available for customers who already have an active Managed Database.__

Delete a single backup for an accessible Managed MySQL Database.

Requires `read_write` access to the Database.

The Database must not be provisioning to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-backup-delete 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/databases/mysql/instances/{instanceId}/backups/{backupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .backups()
    .delete(linode_api::resources::databases::mysql::instances::backups::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
        instance_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### Delete a managed PostgreSQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Remove a Managed PostgreSQL Database from your Account.

Requires `read_write` access to the Database.

The Database must have an `active`, `failed`, or `degraded` status to perform this operation.

Only unrestricted Users can access this operation, and have access regardless of the acting token's OAuth scopes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/databases/postgresql/instances/{instanceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .delete(linode_api::resources::databases::postgresql::instances::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionDatabasesPostgresqlInstancesInstanceIdApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Delete a managed PostgreSQL database backup
__This operation is currently only available for customers who already have an active Managed Database.__

Delete a single backup for an accessible Managed PostgreSQL Database.

Requires `read_write` access to the Database.

The Database must not be provisioning to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-backup-delete 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/databases/postgresql/instances/{instanceId}/backups/{backupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .backups()
    .delete(linode_api::resources::databases::postgresql::instances::backups::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
        instance_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### Delete a domain
Deletes a Domain from Linode's DNS Manager. The Domain will be removed from Linode's nameservers shortly after this operation completes. This also deletes all associated Domain Records.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains delete 1234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/domains/{domainId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .delete(linode_api::resources::domains::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionDomainsDomainIdApiVersionEnum::V4,
        domain_id: 123,
    })
    .await;
```

    
### Delete a domain record
Deletes a Record on this Domain.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains records-delete 123 234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/domains/{domainId}/records/{recordId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .records()
    .delete(linode_api::resources::domains::records::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum::V4,
        domain_id: 123,
        record_id: 123,
    })
    .await;
```

    
### Delete an image
Deletes a private image you have permission to `read_write`.

> 🚧
>
> - You can't undo this delete action.
>
> - When you delete an image, all [replicated instances](https://techdocs.akamai.com/linode-api/reference/post-replicate-image) of that image are also deleted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli images delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/images/{imageId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .delete(linode_api::resources::images::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionImagesImageIdApiVersionEnum::V4,
        image_id: "linode/debian11".to_string(),
    })
    .await;
```

    
### Delete a Linode
Deletes a Linode you have permission to `read_write`.

__Deleting a Linode is a destructive action and cannot be undone.__

Additionally, deleting a Linode:

  - Gives up any IP addresses the Linode was assigned.
  - Deletes all Disks, Backups, Configs, etc.
  - Detaches any Volumes associated with the Linode.
  - Stops billing for the Linode and its associated services. You will be billed for time used within the billing period the Linode was active.

Linodes that are in the process of [cloning](https://techdocs.akamai.com/linode-api/reference/post-clone-linode-instance) or [backup restoration](https://techdocs.akamai.com/linode-api/reference/post-restore-backup) cannot be deleted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/linode/instances/{linodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .delete(linode_api::resources::linode::instances::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Delete a config profile
Deletes the specified Configuration profile from the specified Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-delete 123 23456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/linode/instances/{linodeId}/configs/{configId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .delete(linode_api::resources::linode::instances::configs::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdConfigsConfigIdApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
    })
    .await;
```

    
### Delete a configuration profile interface
Deletes an Interface from the Configuration Profile.

- The User accessing this operation must have `read_write` grants to the Linode.
- A successful request triggers a `linode_config_update` event.
- Active Interfaces cannot be deleted. The associated Linode must first be shut down (or restarted using another Configuration Profile) before such Interfaces can be deleted from a Configuration Profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-interface-delete $linodeId $configId $interfaceId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/{interfaceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .interfaces()
    .delete(linode_api::resources::linode::instances::configs::interfaces::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
        interface_id: 123,
    })
    .await;
```

    
### Delete a disk
Deletes a Disk you have permission to `read_write`.

__Deleting a Disk is a destructive action and cannot be undone.__


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-delete 123 24674
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .delete(linode_api::resources::linode::instances::disks::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdDisksDiskIdApiVersionEnum::V4,
        linode_id: 123,
        disk_id: 123,
    })
    .await;
```

    
### Delete an IPv4 address
Deletes a public or private IPv4 address associated with this Linode. This will fail if it is the Linode's last remaining public IPv4 address, or if the address has a 1:1 NAT with an active VPC Subnet address.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes ip-delete 97.107.143.141
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/linode/instances/{linodeId}/ips/{address}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .ips()
    .delete(linode_api::resources::linode::instances::ips::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum::V4,
        linode_id: 123,
        address: "string".to_string(),
    })
    .await;
```

    
### Delete a StackScript
Deletes a private StackScript you have permission to `read_write`. You cannot delete a public StackScript.


<<LB>>

---


- __CLI__.

    ```
    linode-cli stackscripts delete 10079
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    stackscripts:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/linode/stackscripts/{stackscriptId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .stackscripts()
    .delete(linode_api::resources::linode::stackscripts::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLinodeStackscriptsStackscriptIdApiVersionEnum::V4,
        stackscript_id: "string".to_string(),
    })
    .await;
```

    
### Delete a Kubernetes cluster
Deletes a Cluster you have permission to `read_write`.

__Deleting a Cluster is a destructive action and cannot be undone.__

Deleting a Cluster:

- Deletes all Linodes in all pools within this Kubernetes cluster
- Deletes all supporting Kubernetes services for this Kubernetes cluster (API server, etcd, etc)
- Deletes all NodeBalancers created by this Kubernetes cluster
- Does not delete any of the volumes created by this Kubernetes cluster


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/lke/clusters/{clusterId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .delete(linode_api::resources::lke::clusters::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Delete the control plane access control list
Disable control plane access controls and deletes all rules. This has the same effect as calling `PUT` with an acl json map value of `{“enabled” : false}`. __Note__: control plane ACLs may not currently be available to all users.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-acl-delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/lke/clusters/{clusterId}/control_plane_acl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .control_plane_acl()
    .delete(linode_api::resources::lke::clusters::control_plane_acl::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdControlPlaneAclApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Delete a Kubeconfig
Delete and regenerate the Kubeconfig file for a Cluster.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke kubeconfig-delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/lke/clusters/{clusterId}/kubeconfig`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .kubeconfig()
    .delete(linode_api::resources::lke::clusters::kubeconfig::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdKubeconfigApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Delete a node
Deletes a specific Node from a Node Pool.

__Deleting a Node is a destructive action and cannot be undone.__

Deleting a Node will reduce the size of the Node Pool it belongs to.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke node-delete 12345 12345-6aa78910bc
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/lke/clusters/{clusterId}/nodes/{nodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .nodes()
    .delete(linode_api::resources::lke::clusters::nodes::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdNodesNodeIdApiVersionEnum::V4,
        cluster_id: 123,
        node_id: "string".to_string(),
    })
    .await;
```

    
### Delete a node pool
Delete a specific Node Pool from a Kubernetes cluster.

__Deleting a Node Pool is a destructive action and cannot be undone.__

Deleting a Node Pool will delete all Linodes within that Pool.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke pool-delete 12345 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .pools()
    .delete(linode_api::resources::lke::clusters::pools::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdPoolsPoolIdApiVersionEnum::V4,
        cluster_id: 123,
        pool_id: 123,
    })
    .await;
```

    
### Delete a service token
Delete and regenerate the service account token for a Cluster.

__Note__. When regenerating a service account token, the Cluster's control plane components and Linode CSI drivers are also restarted and configured with the new token. High Availability Clusters should not experience any disruption, while standard Clusters may experience brief control plane downtime while components are restarted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke service-token-delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/lke/clusters/{clusterId}/servicetoken`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .servicetoken()
    .delete(linode_api::resources::lke::clusters::servicetoken::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdServicetokenApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Delete a Longview client
Deletes a Longview Client from your Account.

__All information stored for this client will be lost.__

This _does not_ uninstall the Longview Client application for your Linode - you must do that manually.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview delete 789
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/longview/clients/{clientId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .clients()
    .delete(linode_api::resources::longview::clients::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionLongviewClientsClientIdApiVersionEnum::V4,
        client_id: 123,
    })
    .await;
```

    
### Delete a managed contact
Deletes a Managed Contact.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed contact-delete 567
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/managed/contacts/{contactId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .contacts()
    .delete(linode_api::resources::managed::contacts::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionManagedContactsContactIdApiVersionEnum::V4,
        contact_id: 123,
    })
    .await;
```

    
### Delete a managed service
Deletes a Managed Service.  This service will no longer be monitored by Linode Managed.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed service-delete 9994
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/managed/services/{serviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .delete(linode_api::resources::managed::services::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionManagedServicesServiceIdApiVersionEnum::V4,
        service_id: 123,
    })
    .await;
```

    
### Delete a firewall
Delete a Firewall resource by its ID. This removes all of the Firewall's Rules from any services that the Firewall was assigned to.

- Assigned Linodes must not have any ongoing live migrations.

- A `firewall_delete` Event is generated when this operation returns successfully.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/networking/firewalls/{firewallId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .delete(linode_api::resources::networking::firewalls::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionNetworkingFirewallsFirewallIdApiVersionEnum::V4,
        firewall_id: 123,
    })
    .await;
```

    
### Delete a firewall device
Removes a Firewall Device, which removes a Firewall from the service it was assigned to by the Device. This removes all of the Firewall's Rules from the service. If any other Firewalls have been assigned to the service, then those Rules remain in effect.

- Assigned Linodes must not have any ongoing live migrations.

- A `firewall_device_remove` Event is generated when the Firewall Device is removed successfully.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls device-delete 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/networking/firewalls/{firewallId}/devices/{deviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .devices()
    .delete(linode_api::resources::networking::firewalls::devices::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdApiVersionEnum::V4,
        firewall_id: 123,
        device_id: 123,
    })
    .await;
```

    
### Delete an IPv6 range
Removes this IPv6 range from your account and disconnects the range from any assigned Linodes.

__Note__. Shared IPv6 ranges cannot be deleted at this time. Please contact Customer Support for assistance.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking v6-range-delete 2001:0db8::
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/networking/ipv6/ranges/{range}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv6()
    .ranges()
    .delete(linode_api::resources::networking::ipv6::ranges::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionNetworkingIpv6RangesRangeApiVersionEnum::V4,
        range: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
    })
    .await;
```

    
### Delete a NodeBalancer
Deletes a NodeBalancer.

__This is a destructive action and cannot be undone.__

Deleting a NodeBalancer will also delete all associated Configs and Nodes, although the backend servers represented by the Nodes will not be changed or removed. Deleting a NodeBalancer will cause you to lose access to the IP Addresses assigned to this NodeBalancer.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/nodebalancers/{nodeBalancerId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .delete(linode_api::resources::nodebalancers::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionNodebalancersNodeBalancerIdApiVersionEnum::V4,
        node_balancer_id: 123,
    })
    .await;
```

    
### Delete a config
Deletes the Config for a port of this NodeBalancer.

__This cannot be undone.__

Once completed, this NodeBalancer will no longer respond to requests on the given port. This also deletes all associated NodeBalancerNodes, but the Linodes they were routing traffic to will be unchanged and will not be removed.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers config-delete \
  12345 4567
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .delete(linode_api::resources::nodebalancers::configs::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
    })
    .await;
```

    
### Delete a NodeBalancer's node
Deletes a Node from this Config. This backend will no longer receive traffic for the configured port of this NodeBalancer.

This does not change or remove the Linode whose address was used in the creation of this Node.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers node-delete \
  12345 4567 54321
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes/{nodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .nodes()
    .delete(linode_api::resources::nodebalancers::configs::nodes::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        node_id: "string".to_string(),
    })
    .await;
```

    
### Remove an Object Storage bucket
Removes a single bucket.

> 📘
>
> - You need to remove all objects from a bucket before you can delete it. While you *can* delete a bucket using the [s3cmd command-line tool](https://www.linode.com/docs/products/storage/object-storage/guides/s3cmd/#delete-a-bucket), this operation fails if the bucket contains too many objects. The best way to empty large buckets is to use the [S3 API to configure lifecycle policies](https://docs.ceph.com/en/latest/radosgw/bucketpolicy/#). Set a policy to remove all objects, wait a day or more for the system to remove all objects, then delete the bucket.
>
> - The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#delete-bucket) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/object-storage/buckets/{regionId}/{bucket}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .delete(linode_api::resources::object_storage::buckets::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionObjectStorageBucketsRegionIdBucketApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
    })
    .await;
```

    
### Delete an Object Storage TLS/SSL certificate
Deletes this Object Storage bucket's user uploaded TLS/SSL certificate and private key.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage ssl-delete \
  us-east-1 example-bucket
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/ssl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .ssl_resource()
    .delete(linode_api::resources::object_storage::buckets::ssl_resource::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionObjectStorageBucketsRegionIdBucketSslApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
    })
    .await;
```

    
### Revoke an Object Storage key
Revokes an Object Storage Key. This keypair will no longer be usable by third-party clients.

- A successful request triggers an `obj_access_key_delete` event.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage keys-delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/object-storage/keys/{keyId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .keys()
    .delete(linode_api::resources::object_storage::keys::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionObjectStorageKeysKeyIdApiVersionEnum::V4,
        key_id: 123,
    })
    .await;
```

    
### Delete a placement group
Deletes a placement group you have permission to `read_write`.

- Deleting a placement group can't be undone.
- All compute instances need to be [removed](https://techdocs.akamai.com/linode-api/reference/post-group-unassign) before you can delete a placement group.
- If your placement group is non-compliant, you can't delete it. You need to wait for our help to bring it [compliant](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#non-compliance-precedence).


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement group-delete 528
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/placement/groups/{groupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .delete(linode_api::resources::placement::groups::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionPlacementGroupsGroupIdApiVersionEnum::V4,
        group_id: 123,
    })
    .await;
```

    
### Revoke app access
Expires this app token. This token may no longer be used to access your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile app-delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/profile/apps/{appId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .apps()
    .delete(linode_api::resources::profile_resource::apps::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionProfileAppsAppIdApiVersionEnum::V4,
        app_id: 123,
    })
    .await;
```

    
### Revoke a trusted device
Revoke an active TrustedDevice for your User.  Once a TrustedDevice is revoked, this device will have to log in again before accessing your Linode account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile device-revoke 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/profile/devices/{deviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .devices()
    .delete(linode_api::resources::profile_resource::devices::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionProfileDevicesDeviceIdApiVersionEnum::V4,
        device_id: 123,
    })
    .await;
```

    
### Delete a phone number
Delete the verified phone number for the User making this request.

Use this operation to opt out of SMS messages for the requesting User after a phone number has been verified with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli phone delete
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/profile/phone-number`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .phone_number()
    .delete(linode_api::resources::profile_resource::phone_number::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionProfilePhoneNumberApiVersionEnum::V4,
    })
    .await;
```

    
### Delete an SSH key
Deletes an SSH Key you have access to.

__Note__. deleting an SSH Key will _not_ remove it from any Linode or Disk that was deployed with `authorized_keys`. In those cases, the keys must be manually deleted on the Linode or Disk. This operation will only delete the key's association from your Profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli sshkeys delete 42
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/profile/sshkeys/{sshKeyId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .sshkeys()
    .delete(linode_api::resources::profile_resource::sshkeys::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionProfileSshkeysSshKeyIdApiVersionEnum::V4,
        ssh_key_id: 123,
    })
    .await;
```

    
### Revoke a personal access token
Revokes a Personal Access Token. The token will be invalidated immediately, and requests using that token will fail with a 401. It is possible to revoke access to the token making the request to revoke a token, but keep in mind that doing so could lose you access to the api and require you to create a new token through some other means.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile token-delete 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/profile/tokens/{tokenId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tokens()
    .delete(linode_api::resources::profile_resource::tokens::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionProfileTokensTokenIdApiVersionEnum::V4,
        token_id: 123,
    })
    .await;
```

    
### Delete a tag
Remove a Tag from all objects and delete it.

__Important__. You must be an unrestricted User in order to access, add, or modify Tags information.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tags delete
linode-cli tags rm
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/tags/{tagLabel}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .tags()
    .delete(linode_api::resources::tags::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionTagsTagLabelApiVersionEnum::V4,
        tag_label: "string".to_string(),
    })
    .await;
```

    
### Delete a volume
Deletes a Volume you have permission to `read_write`.

- __Deleting a Volume is a destructive action and cannot be undone.__

- Deleting stops billing for the Volume. You will be billed for time used within the billing period the Volume was active.

- Volumes that are migrating cannot be deleted until the migration is finished.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes delete 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/volumes/{volumeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .delete(linode_api::resources::volumes::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionVolumesVolumeIdApiVersionEnum::V4,
        volume_id: 123,
    })
    .await;
```

    
### Delete a VPC
Delete a single VPC and all of its Subnets.

- The User accessing this operation must have `read_write` grants to the VPC.
- A successful request triggers a `vpc_delete` event and `subnet_delete` events for each deleted VPC Subnet.
- All of the VPC's Subnets must be eligible for deletion. Accordingly, all Configuration Profile Interfaces that each Subnet is assigned to must first be deleted. If those Interfaces are active, the associated Linodes must first be shut down before they can be removed. If any Subnet cannot be deleted, then neither the VPC nor any of its Subnets are deleted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs delete $vpcId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    vpc:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/vpcs/{vpcId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .delete(linode_api::resources::vpcs::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionVpcsVpcIdApiVersionEnum::V4,
        vpc_id: 123,
    })
    .await;
```

    
### Delete a VPC subnet
Delete a single VPC Subnet.

The user accessing this operation must have `read_write` grants to the VPC. A successful request triggers a `subnet_delete` event.

__Note__. You need to delete all the Configuration Profile Interfaces that this Subnet is assigned to before you can delete it. If those Interfaces are active, the associated Linode needs to be shut down before they can be removed.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs subnet-delete $vpcId $vpcSubnetId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    vpc:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `DELETE /{apiVersion}/vpcs/{vpcId}/subnets/{vpcSubnetId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .subnets()
    .delete(linode_api::resources::vpcs::subnets::DeleteRequest {
        api_version: linode_api::models::DeleteApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum::V4,
        vpc_id: 123,
        vpc_subnet_id: 123,
    })
    .await;
```

    
### Get your account
Returns the contact and billing information related to your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .list(linode_api::resources::account::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountApiVersionEnum::V4,
    })
    .await;
```

    
### List agreements
Returns all agreements and their acceptance status for your account.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/agreements`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .agreements()
    .list(linode_api::resources::account::agreements::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountAgreementsApiVersionEnum::V4,
    })
    .await;
```

    
### List available services
Returns a paginated list of the services available to you, for all Linode regions.

__Note__. Only authorized Users can run this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account get-availability
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/availability`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .availability()
    .list(linode_api::resources::account::availability::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountAvailabilityApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a region's service availability
View the available services for your account in a specific region.

__Note__. Only authorized users can access this.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account get-account-availability us-east
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/availability/{id}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .availability()
    .get(linode_api::resources::account::availability::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountAvailabilityIdApiVersionEnum::V4,
        id: "string".to_string(),
    })
    .await;
```

    
### List enrolled Beta programs
Display all enrolled Beta Programs for your Account. Includes inactive as well as active Beta Programs.

Only unrestricted Users can access this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli betas enrolled
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/betas`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .betas()
    .list(linode_api::resources::account::betas::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountBetasApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an enrolled Beta program
Display an enrolled Beta Program for your Account. The Beta Program may be inactive.

Only unrestricted Users can access this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli betas enrolled-view $betaId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/betas/{betaId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .betas()
    .get(linode_api::resources::account::betas::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountBetasBetaIdApiVersionEnum::V4,
        beta_id: "string".to_string(),
    })
    .await;
```

    
### List child accounts
Returns a paginated list of basic information for the child accounts that exist for your parent account. See [Parent and Child Accounts for Akamai Partners](https://www.linode.com/docs/guides/parent-child-accounts/) for details on these accounts.

__Note__. This operation can only be accessed by an unrestricted parent user, or restricted parent user with the `child_account_access` grant.


<<LB>>

---


- __CLI__.

    ```
    linode-cli child-account list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    child_account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/child-accounts`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .child_accounts()
    .list(linode_api::resources::account::child_accounts::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountChildAccountsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a child account
View a specific child account based on its `euuid`. See [Parent and Child Accounts for Akamai Partners](https://www.linode.com/docs/guides/parent-child-accounts/) for details on these accounts.

__Note__. This operation can only be accessed by an unrestricted user, or restricted user with the `child_account_access` grant.


<<LB>>

---


- __CLI__.

    ```
    linode-cli child-account view A1BC2DEF-34GH-567I-J890KLMN12O34P56
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    child_account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/child-accounts/{euuid}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .child_accounts()
    .get(linode_api::resources::account::child_accounts::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountChildAccountsEuuidApiVersionEnum::V4,
        euuid: "string".to_string(),
    })
    .await;
```

    
### List entity transfers
__Deprecated__ Please run [List service transfers](https://techdocs.akamai.com/linode-api/reference/get-service-transfers).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/entity-transfers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .entity_transfers()
    .list(linode_api::resources::account::entity_transfers::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountEntityTransfersApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an entity transfer
__Deprecated__ Please run [Get a service transfer request](https://techdocs.akamai.com/linode-api/reference/get-service-transfer).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/entity-transfers/{token}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .entity_transfers()
    .get(linode_api::resources::account::entity_transfers::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountEntityTransfersTokenApiVersionEnum::V4,
        token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### List events
Returns a collection of Event objects representing actions taken on your Account from the last 90 days. The Events returned depend on your grants.


<<LB>>

---


- __CLI__.

    ```
    linode-cli events list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    events:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/events`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .events()
    .list(linode_api::resources::account::events::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountEventsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an event
Returns a single Event object.


<<LB>>

---


- __CLI__.

    ```
    linode-cli events view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    events:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/events/{eventId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .events()
    .get(linode_api::resources::account::events::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountEventsEventIdApiVersionEnum::V4,
        event_id: 123,
    })
    .await;
```

    
### List invoices
Returns a paginated list of Invoices against your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account invoices-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/invoices`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .invoices()
    .list(linode_api::resources::account::invoices::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountInvoicesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an invoice
Returns a single Invoice object.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account invoice-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/invoices/{invoiceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .invoices()
    .get(linode_api::resources::account::invoices::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountInvoicesInvoiceIdApiVersionEnum::V4,
        invoice_id: 123,
    })
    .await;
```

    
### List invoice items
Returns a paginated list of Invoice items.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account invoice-items 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/invoices/{invoiceId}/items`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .invoices()
    .items()
    .list(linode_api::resources::account::invoices::items::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountInvoicesInvoiceIdItemsApiVersionEnum::V4,
        invoice_id: 123,
        ..Default::default()
    })
    .await;
```

    
### List user logins
Returns a collection of successful logins for all users on the account during the last 90 days. This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account logins-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/logins`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .logins()
    .list(linode_api::resources::account::logins::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountLoginsApiVersionEnum::V4,
    })
    .await;
```

    
### Get an account login
Returns a Login object that displays information about a successful login. The logins that can be viewed can be for any user on the account, and are not limited to only the logins of the user that is accessing this API endpoint. This operation can only be accessed by the unrestricted users of the account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account login-view 1234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/logins/{loginId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .logins()
    .get(linode_api::resources::account::logins::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountLoginsLoginIdApiVersionEnum::V4,
        login_id: 123,
    })
    .await;
```

    
### List maintenances
Returns a collection of Maintenance objects for any entity a user has permissions to view. Canceled Maintenance objects are not returned.

Currently, Linodes are the only entities available for viewing.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account maintenance-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/account/maintenance`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .maintenance()
    .list(linode_api::resources::account::maintenance::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountMaintenanceApiVersionEnum::V4,
    })
    .await;
```

    
### List notifications
Returns a collection of notification objects that represent important, often time-sensitive details about your account. You can't interact directly with notifications, and a notification disappears when the circumstances that caused it have been resolved. For example, if you have an important ticket open, you can respond to that ticket to dismiss its notification.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account notifications-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/notifications`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .notifications()
    .list(linode_api::resources::account::notifications::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountNotificationsApiVersionEnum::V4,
    })
    .await;
```

    
### List OAuth clients
Returns a paginated list of OAuth Clients registered to your Account.  OAuth Clients allow users to log into applications you write or host using their Linode Account, and may allow them to grant some level of access to their Linodes or other entities to your application.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account clients-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/oauth-clients`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .list(linode_api::resources::account::oauth_clients::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountOauthClientsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an OAuth client
Returns information about a single OAuth client.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account client-view \
  edc6790ea9db4d224c5c
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/oauth-clients/{clientId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .get(linode_api::resources::account::oauth_clients::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountOauthClientsClientIdApiVersionEnum::V4,
        client_id: "string".to_string(),
    })
    .await;
```

    
### Get the OAuth client's thumbnail
Returns the PNG thumbnail for this OAuth Client.  This is a publicly viewable endpoint, and can be accessed without authentication.

**API Endpoint**: `GET /{apiVersion}/account/oauth-clients/{clientId}/thumbnail`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .thumbnail()
    .list(linode_api::resources::account::oauth_clients::thumbnail::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountOauthClientsClientIdThumbnailApiVersionEnum::V4,
        client_id: "string".to_string(),
    })
    .await;
```

    
### List payment methods
Returns a paginated list of Payment Methods for this Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli payment-methods list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/payment-methods`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payment_methods()
    .list(linode_api::resources::account::payment_methods::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountPaymentMethodsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a payment method
View the details of the specified Payment Method.


<<LB>>

---


- __CLI__.

    ```
    linode-cli payment-methods view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/payment-methods/{paymentMethodId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payment_methods()
    .get(linode_api::resources::account::payment_methods::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountPaymentMethodsPaymentMethodIdApiVersionEnum::V4,
        payment_method_id: 123,
    })
    .await;
```

    
### List payments
Returns a paginated list of Payments made on this Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account payments-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/payments`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payments()
    .list(linode_api::resources::account::payments::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountPaymentsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a payment
Returns information about a specific Payment.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account payment-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/payments/{paymentId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payments()
    .get(linode_api::resources::account::payments::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountPaymentsPaymentIdApiVersionEnum::V4,
        payment_id: 123,
    })
    .await;
```

    
### List service transfers
Returns a collection of all created and accepted Service Transfers for this account, regardless of the user that created or accepted the transfer.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli service-transfers \
  list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/service-transfers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .service_transfers()
    .list(linode_api::resources::account::service_transfers::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountServiceTransfersApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a service transfer request
Returns the details of the Service Transfer for the provided token.

While a transfer is pending, any unrestricted user _of any account_ can access this operation. After a transfer has been accepted, it can only be viewed by unrestricted users of the accounts that created and accepted the transfer. If canceled or expired, only unrestricted users of the account that created the transfer can view it.


<<LB>>

---


- __CLI__.

    ```
    linode-cli service-transfers \
  view 123E4567-E89B-12D3-A456-426614174000
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/service-transfers/{token}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .service_transfers()
    .get(linode_api::resources::account::service_transfers::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountServiceTransfersTokenApiVersionEnum::V4,
        token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Get account settings
Returns information related to your Account settings: Managed service subscription, Longview subscription, and network helper.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account settings
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/settings`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .settings()
    .list(linode_api::resources::account::settings::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountSettingsApiVersionEnum::V4,
    })
    .await;
```

    
### Get network usage
Returns a Transfer object showing your network utilization, in GB, for the current month.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account transfer
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/transfer`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .transfer()
    .list(linode_api::resources::account::transfer::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountTransferApiVersionEnum::V4,
    })
    .await;
```

    
### List users
Returns a paginated list of all users on your account.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.

A user can access all or part of an account based on their access status and grants:

- __Unrestricted access__. These users can access everything on an account.

- __Restricted access__. These users can only access entities or perform actions they've been given specific grants to.


<<LB>>

---


- __CLI__.

    ```
    linode-cli users list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/users`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .users()
    .list(linode_api::resources::account::users::ListRequest {
        api_version: linode_api::models::GetApiVersionAccountUsersApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a user
Returns information about a single user on your account.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.


<<LB>>

---


- __CLI__.

    ```
    linode-cli users view example_user
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/users/{username}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .users()
    .get(linode_api::resources::account::users::GetRequest {
        api_version: linode_api::models::GetApiVersionAccountUsersUsernameApiVersionEnum::V4,
        username: "string".to_string(),
    })
    .await;
```

    
### List a user's grants
Returns the full grants structure for an account username you specify. This includes all entities on the account, and the level of access this user has to each of them.

This doesn't apply to the account owner or the current authenticated user. You can run the [List grants](https://techdocs.akamai.com/linode-api/reference/get-profile-grants) operation to view those grants. However, this doesn't show the entities that they _don't_ have access to.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/account/users/{username}/grants`


### List Beta programs
Display all active Beta Programs.

Only unrestricted Users can access this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli betas list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/betas`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .betas()
    .list(linode_api::resources::betas::ListRequest {
        api_version: linode_api::models::GetApiVersionBetasApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a Beta program
Display information about a Beta Program. This operation can be used to access inactive as well as active Beta Programs.

Only unrestricted Users can access this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli betas view $betaId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/betas/{betaId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .betas()
    .get(linode_api::resources::betas::GetRequest {
        api_version: linode_api::models::GetApiVersionBetasBetaIdApiVersionEnum::V4,
        beta_id: "string".to_string(),
    })
    .await;
```

    
### List managed database engines
__This operation is currently only available for customers who already have an active Managed Database.__

Display all available Managed Database engine types and versions. Engine IDs are used when creating new Managed Databases.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases engines
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/databases/engines`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .engines()
    .list(linode_api::resources::databases::engines::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesEnginesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed database engine
__This operation is currently only available for customers who already have an active Managed Database.__

Display information for a single Managed Database engine type and version.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases engine-view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/databases/engines/{engineId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .engines()
    .get(linode_api::resources::databases::engines::GetRequest {
        api_version: linode_api::models::GetApiVersionDatabasesEnginesEngineIdApiVersionEnum::V4,
        engine_id: "string".to_string(),
        ..Default::default()
    })
    .await;
```

    
### List managed databases
__This operation is currently only available for customers who already have an active Managed Database.__

Display all Managed Databases that are accessible by your User, regardless of engine type.

For more detailed information on a particular Database instance, make a request to its `instance_uri`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .instances()
    .list(linode_api::resources::databases::instances::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesInstancesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List managed MySQL databases
__This operation is currently only available for customers who already have an active Managed Database.__

Display all accessible Managed MySQL Databases.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/mysql/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .list(linode_api::resources::databases::mysql::instances::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed MySQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Display information for a single, accessible Managed MySQL Database.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/mysql/instances/{instanceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .get(linode_api::resources::databases::mysql::instances::GetRequest {
        api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### List managed MySQL database backups
__This operation is currently only available for customers who already have an active Managed Database.__

Display all backups for an accessible Managed MySQL Database.

The Database must not be provisioning to perform this operation.

Database `auto` type backups are created every 24 hours at 0:00 UTC. Each `auto` backup is retained for 7 days.

Database `snapshot` type backups are created by accessing the [Create a managed MySQL database backup snapshot](https://techdocs.akamai.com/linode-api/reference/post-databases-mysql-instance-backup) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-backups-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/mysql/instances/{instanceId}/backups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .backups()
    .list(linode_api::resources::databases::mysql::instances::backups::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdBackupsApiVersionEnum::V4,
        instance_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a managed MySQL database backup
__This operation is currently only available for customers who already have an active Managed Database.__

Display information for a single backup for an accessible Managed MySQL Database.

The Database must not be provisioning to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-backup-view 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/mysql/instances/{instanceId}/backups/{backupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .backups()
    .get(linode_api::resources::databases::mysql::instances::backups::GetRequest {
        api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
        instance_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### Get managed MySQL database credentials
__This operation is currently only available for customers who already have an active Managed Database.__

Display the root username and password for an accessible Managed MySQL Database.

The Database must have an `active` status to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-creds-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/mysql/instances/{instanceId}/credentials`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .credentials()
    .list(linode_api::resources::databases::mysql::instances::credentials::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdCredentialsApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Get a managed MySQL database SSL certificate
__This operation is currently only available for customers who already have an active Managed Database.__

Display the SSL CA certificate for an accessible Managed MySQL Database.

The Database must have an `active` status to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-ssl-cert 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/mysql/instances/{instanceId}/ssl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .ssl_resource()
    .list(linode_api::resources::databases::mysql::instances::ssl_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdSslApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### List managed PostgreSQL databases
__This operation is currently only available for customers who already have an active Managed Database.__

Display all accessible Managed PostgreSQL Databases.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/postgresql/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .list(linode_api::resources::databases::postgresql::instances::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed PostgreSQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Display information for a single, accessible Managed PostgreSQL Database.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/postgresql/instances/{instanceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .get(linode_api::resources::databases::postgresql::instances::GetRequest {
        api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### List managed PostgreSQL database backups
__This operation is currently only available for customers who already have an active Managed Database.__

Display all backups for an accessible Managed PostgreSQL Database.

The Database must not be provisioning to perform this operation.

Database `auto` type backups are created every 24 hours at 0:00 UTC. Each `auto` backup is retained for 7 days.

Database `snapshot` type backups are created by accessing the [Create a managed PostgreSQL database backup snapshot](https://techdocs.akamai.com/linode-api/reference/post-databases-postgre-sql-instance-backup) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-backups-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/postgresql/instances/{instanceId}/backups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .backups()
    .list(linode_api::resources::databases::postgresql::instances::backups::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsApiVersionEnum::V4,
        instance_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a managed PostgreSQL database backup
__This operation is currently only available for customers who already have an active Managed Database.__

Display information for a single backup for an accessible Managed PostgreSQL Database.

The Database must not be provisioning to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-backup-view 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/postgresql/instances/{instanceId}/backups/{backupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .backups()
    .get(linode_api::resources::databases::postgresql::instances::backups::GetRequest {
        api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
        instance_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### Get managed PostgreSQL database credentials
__This operation is currently only available for customers who already have an active Managed Database.__

Display the root username and password for an accessible Managed PostgreSQL Database.

The Database must have an `active` status to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-creds-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/postgresql/instances/{instanceId}/credentials`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .credentials()
    .list(linode_api::resources::databases::postgresql::instances::credentials::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdCredentialsApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Get a managed PostgreSQL database SSL certificate
__This operation is currently only available for customers who already have an active Managed Database.__

Display the SSL CA certificate for an accessible Managed PostgreSQL Database.

The Database must have an `active` status to perform this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-ssl-cert 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/databases/postgresql/instances/{instanceId}/ssl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .ssl_resource()
    .list(linode_api::resources::databases::postgresql::instances::ssl_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdSslApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### List managed database types
__This operation is currently only available for customers who already have an active Managed Database.__

Display all Managed Database node types. The type and number of nodes determine the resources and price of a Managed Database instance.

Each Managed Database can have one node type. In the case of a high availability Database, all nodes are provisioned according to the chosen type.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/databases/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .types_resource()
    .list(linode_api::resources::databases::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionDatabasesTypesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed database type
__This operation is currently only available for customers who already have an active Managed Database.__

Display the details of a single Managed Database type. The type and number of nodes determine the resources and price of a Managed Database instance.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases type-view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/databases/types/{typeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .types_resource()
    .get(linode_api::resources::databases::types_resource::GetRequest {
        api_version: linode_api::models::GetApiVersionDatabasesTypesTypeIdApiVersionEnum::V4,
        type_id: "string".to_string(),
        ..Default::default()
    })
    .await;
```

    
### List domains
This is a collection of Domains that you have registered in Linode's DNS Manager.  Linode is not a registrar, and in order for these to work you must own the domains and point your registrar at Linode's nameservers.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/domains`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .list(linode_api::resources::domains::ListRequest {
        api_version: linode_api::models::GetApiVersionDomainsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a domain
This is a single Domain that you have registered in Linode's DNS Manager. Linode is not a registrar, and in order for this Domain record to work you must own the domain and point your registrar at Linode's nameservers.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/domains/{domainId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .get(linode_api::resources::domains::GetRequest {
        api_version: linode_api::models::GetApiVersionDomainsDomainIdApiVersionEnum::V4,
        domain_id: 123,
    })
    .await;
```

    
### List domain records
Returns a paginated list of Records configured on a Domain in Linode's DNS Manager.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains records-list 1234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/domains/{domainId}/records`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .records()
    .list(linode_api::resources::domains::records::ListRequest {
        api_version: linode_api::models::GetApiVersionDomainsDomainIdRecordsApiVersionEnum::V4,
        domain_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a domain record
View a single Record on this Domain.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains records-view 123 234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/domains/{domainId}/records/{recordId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .records()
    .get(linode_api::resources::domains::records::GetRequest {
        api_version: linode_api::models::GetApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum::V4,
        domain_id: 123,
        record_id: 123,
    })
    .await;
```

    
### Get a domain zone file
Returns the zone file for the last rendered zone for the specified domain.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains zone-file 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/domains/{domainId}/zone-file`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .zone_file()
    .list(linode_api::resources::domains::zone_file::ListRequest {
        api_version: linode_api::models::GetApiVersionDomainsDomainIdZoneFileApiVersionEnum::V4,
        domain_id: "string".to_string(),
    })
    .await;
```

    
### List images
Returns a paginated list of images. An image can be one of two types:

- **Public image**. The `id` for these images begins with `linode/`. These images are generally available to all users. To limit the response to public images, don't include [authentication](https://techdocs.akamai.com/linode-api/reference/get-started#authentication) when calling this operation.

- **Private image**. The `id` for these images begins with `private/`. These images are account-specific and only accessible to users with appropriate [grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants). To view private images, you need to include authentication when calling this operation. The response includes both private and public images.


<<LB>>

---


- __CLI__.

    ```
    linode-cli images list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/images`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .list(linode_api::resources::images::ListRequest {
        api_version: linode_api::models::GetApiVersionImagesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an image
Get information about a single image. An image can be one of two types:

- **Public image**. The `id` for these images begins with `linode/`. These images are generally available to all users. To limit the response to public images, don't include [authentication](https://techdocs.akamai.com/linode-api/reference/get-started#authentication) when calling this operation.

- **Private image**. The `id` for these images begins with `private/`. These images are account-specific and only accessible to users with appropriate [grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants). To view private images, you need to include authentication when calling this operation. The response will also include public images.


<<LB>>

---


- __CLI__.

    ```
    linode-cli images view linode/debian9
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/images/{imageId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .get(linode_api::resources::images::GetRequest {
        api_version: linode_api::models::GetApiVersionImagesImageIdApiVersionEnum::V4,
        image_id: "linode/debian11".to_string(),
    })
    .await;
```

    
### List Linodes
Returns a paginated list of Linodes you have permission to view.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .list(linode_api::resources::linode::instances::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a Linode
Get a specific Linode by ID.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .get(linode_api::resources::linode::instances::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### List backups
Returns information about this Linode's available backups.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes backups-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/backups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .backups()
    .list(linode_api::resources::linode::instances::backups::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdBackupsApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Get a backup
Returns information about a Backup.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes backup-view 123 123456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/backups/{backupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .backups()
    .get(linode_api::resources::linode::instances::backups::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdApiVersionEnum::V4,
        linode_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### List config profiles
Lists Configuration profiles associated with a Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes configs-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/configs`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .list(linode_api::resources::linode::instances::configs::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsApiVersionEnum::V4,
        linode_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a config profile
Returns information about a specific Configuration profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-view 123 23456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/configs/{configId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .get(linode_api::resources::linode::instances::configs::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
    })
    .await;
```

    
### List configuration profile interfaces
Returns an ordered array of all Interfaces associated with this Configuration Profile.

- The User accessing this operation must have at least `read_only` grants to the Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-interfaces-list $linodeId $configId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .interfaces()
    .list(linode_api::resources::linode::instances::configs::interfaces::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
    })
    .await;
```

    
### Get a configuration profile interface
Returns a single Configuration Profile Interface.

- The User accessing this operation must have at least `read_only` grants to the Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-interface-view $linodeId $configId $interfaceId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/{interfaceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .interfaces()
    .get(linode_api::resources::linode::instances::configs::interfaces::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
        interface_id: 123,
    })
    .await;
```

    
### List disks
View Disk information for Disks associated with this Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disks-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/disks`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .list(linode_api::resources::linode::instances::disks::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdDisksApiVersionEnum::V4,
        linode_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a disk
View Disk information for a Disk associated with this Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-view 123 25674
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .get(linode_api::resources::linode::instances::disks::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdDisksDiskIdApiVersionEnum::V4,
        linode_id: 123,
        disk_id: 123,
    })
    .await;
```

    
### List a Linode's firewalls
View Firewall information for Firewalls assigned to this Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes firewalls-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/firewalls`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .firewalls()
    .list(linode_api::resources::linode::instances::firewalls::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdFirewallsApiVersionEnum::V4,
        linode_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get networking information
Returns networking information for a single Linode.

__Note__. If the target Linode has several configuration profiles that include a Virtual Private Cloud (VPC) interface, address information for all of VPCs will be listed in the response.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes ips-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/ips`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .ips()
    .list(linode_api::resources::linode::instances::ips::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdIpsApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Get a Linode's IP address
View information about the specified IP address associated with the specified Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes ip-view 123 97.107.143.141
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/ips/{address}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .ips()
    .get(linode_api::resources::linode::instances::ips::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum::V4,
        linode_id: 123,
        address: "string".to_string(),
    })
    .await;
```

    
### List Linode NodeBalancers
Returns a list of NodeBalancers that are assigned to this Linode and readable by the requesting User.

Read permission to a NodeBalancer can be given to a User by accessing the [Update a user's grants](https://techdocs.akamai.com/linode-api/reference/put-user-grants) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes nodebalancers 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/nodebalancers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .nodebalancers()
    .list(linode_api::resources::linode::instances::nodebalancers::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdNodebalancersApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Get Linode statistics
Returns CPU, IO, IPv4, and IPv6 statistics for your Linode for the past 24 hours.


<<LB>>

---


- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/stats`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .stats()
    .list(linode_api::resources::linode::instances::stats::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdStatsApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Get monthly statistics
Returns statistics for a specific month. The year/month values must be either a date in the past, or the current month. If the current month, statistics will be retrieved for the past 30 days.


<<LB>>

---


- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/stats/{year}/{month}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .stats()
    .get(linode_api::resources::linode::instances::stats::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdStatsYearMonthApiVersionEnum::V4,
        linode_id: 123,
        year: 123,
        month: 123,
    })
    .await;
```

    
### Get a network transfer
Returns a Linode's network transfer pool statistics for the current month.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes transfer-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/transfer`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .transfer()
    .list(linode_api::resources::linode::instances::transfer::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdTransferApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Get monthly network transfer stats
Returns a Linode's network transfer statistics for a specific month. The year/month values must be either a date in the past, or the current month.


<<LB>>

---


- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/transfer/{year}/{month}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .transfer()
    .get(linode_api::resources::linode::instances::transfer::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdTransferYearMonthApiVersionEnum::V4,
        linode_id: 123,
        year: 123,
        month: 123,
    })
    .await;
```

    
### List a Linode's volumes
View Block Storage Volumes attached to this Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes volumes 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/instances/{linodeId}/volumes`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .volumes()
    .list(linode_api::resources::linode::instances::volumes::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdVolumesApiVersionEnum::V4,
        linode_id: 123,
        ..Default::default()
    })
    .await;
```

    
### List kernels
Lists available Kernels.

Due to the extensive list of available kernels, please keep [pagination](https://techdocs.akamai.com/linode-api/reference/pagination) controls in mind when managing responses to this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli kernels list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/linode/kernels`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .kernels()
    .list(linode_api::resources::linode::kernels::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeKernelsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a kernel
Returns information about a single Kernel.


<<LB>>

---


- __CLI__.

    ```
    linode-cli kernels view latest-64bit
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/linode/kernels/{kernelId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .kernels()
    .get(linode_api::resources::linode::kernels::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeKernelsKernelIdApiVersionEnum::V4,
        kernel_id: "string".to_string(),
    })
    .await;
```

    
### List StackScripts
If the request is not authenticated, only public StackScripts are returned.

For more information on StackScripts, please read our [StackScripts documentation](https://www.linode.com/docs/products/tools/stackscripts/).


<<LB>>

---


- __CLI__.

    ```
    linode-cli stackscripts list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    stackscripts:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/stackscripts`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .stackscripts()
    .list(linode_api::resources::linode::stackscripts::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeStackscriptsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a StackScript
Returns all of the information about a specified StackScript, including the contents of the script.


<<LB>>

---


- __CLI__.

    ```
    linode-cli stackscripts view 10079
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    stackscripts:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/linode/stackscripts/{stackscriptId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .stackscripts()
    .get(linode_api::resources::linode::stackscripts::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeStackscriptsStackscriptIdApiVersionEnum::V4,
        stackscript_id: "string".to_string(),
    })
    .await;
```

    
### List types
Returns Linode Types, including pricing and specifications for each Type. Use these when [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) or [resizing](https://techdocs.akamai.com/linode-api/reference/post-resize-linode-instance) Linodes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/linode/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .types_resource()
    .list(linode_api::resources::linode::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionLinodeTypesApiVersionEnum::V4,
    })
    .await;
```

    
### Get a type
Returns information about a specific Linode Type, including pricing and specifications. This is used when [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) or [resizing](https://techdocs.akamai.com/linode-api/reference/post-resize-linode-instance) Linodes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes type-view g6-standard-2
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/linode/types/{typeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .types_resource()
    .get(linode_api::resources::linode::types_resource::GetRequest {
        api_version: linode_api::models::GetApiVersionLinodeTypesTypeIdApiVersionEnum::V4,
        type_id: "string".to_string(),
    })
    .await;
```

    
### List Kubernetes clusters
Lists current Kubernetes clusters available on your account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke clusters-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .list(linode_api::resources::lke::clusters::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersApiVersionEnum::V4,
    })
    .await;
```

    
### Get a Kubernetes cluster
Get a specific Cluster by ID.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-view 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .get(linode_api::resources::lke::clusters::GetRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### List Kubernetes API endpoints
List the Kubernetes API server endpoints for this cluster. Please note that it often takes 2-5 minutes before the endpoint is ready after first [creating a new cluster](https://techdocs.akamai.com/linode-api/reference/post-lke-cluster).


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke api-endpoints-list 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/api-endpoints`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .api_endpoints()
    .list(linode_api::resources::lke::clusters::api_endpoints::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdApiEndpointsApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Get the control plane access control list
Get a specific cluster's control plane access control List. __Note__: control plane ACLs may not currently be available to all users.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-acl-view 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/control_plane_acl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .control_plane_acl()
    .list(linode_api::resources::lke::clusters::control_plane_acl::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdControlPlaneAclApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Get a Kubernetes cluster dashboard URL
Get a [Kubernetes Dashboard](https://github.com/kubernetes/dashboard) access URL for this Cluster, which enables performance of administrative tasks through a web interface.

Dashboards are installed for Clusters by default.

To access the Cluster Dashboard login prompt, enter the URL in a web browser. Select either __Token__ or __Kubeconfig__ authentication, then select __Sign in__.

For additional guidance on using the Cluster Dashboard, see the [Navigating the Cluster Dashboard](https://www.linode.com/docs/guides/using-the-kubernetes-dashboard-on-lke/#navigating-the-cluster-dashboard) section of our guide on [Using the Kubernetes Dashboard on LKE](https://www.linode.com/docs/guides/using-the-kubernetes-dashboard-on-lke/).


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-dashboard-url 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/dashboard`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .dashboard()
    .list(linode_api::resources::lke::clusters::dashboard::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdDashboardApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Get a Kubeconfig
Get the Kubeconfig file for a Cluster. Please note that it often takes 2-5 minutes before the Kubeconfig file is ready after first [creating a new cluster](https://techdocs.akamai.com/linode-api/reference/post-lke-cluster).


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke kubeconfig-view 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/kubeconfig`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .kubeconfig()
    .list(linode_api::resources::lke::clusters::kubeconfig::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdKubeconfigApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Get a node
Returns the values for a specified node object.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke node-view 123456 12345-6aa78910bc
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/nodes/{nodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .nodes()
    .get(linode_api::resources::lke::clusters::nodes::GetRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdNodesNodeIdApiVersionEnum::V4,
        cluster_id: 123,
        node_id: "string".to_string(),
    })
    .await;
```

    
### List node pools
Returns all active Node Pools on a Kubernetes cluster.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke pools-list 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/pools`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .pools()
    .list(linode_api::resources::lke::clusters::pools::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdPoolsApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Get a node pool
Get a specific Node Pool by ID.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke pool-view 12345 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .pools()
    .get(linode_api::resources::lke::clusters::pools::GetRequest {
        api_version: linode_api::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdApiVersionEnum::V4,
        cluster_id: 123,
        pool_id: 123,
    })
    .await;
```

    
### List Kubernetes types
Returns Kubernetes types and prices, including any region-specific rates.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/lke/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .types_resource()
    .list(linode_api::resources::lke::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeTypesApiVersionEnum::V4,
    })
    .await;
```

    
### List Kubernetes versions
List the Kubernetes versions available for deployment to a Kubernetes cluster.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke versions-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/versions`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .versions()
    .list(linode_api::resources::lke::versions::ListRequest {
        api_version: linode_api::models::GetApiVersionLkeVersionsApiVersionEnum::V4,
    })
    .await;
```

    
### Get a Kubernetes version
View a Kubernetes version available for deployment to a Kubernetes cluster.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke version-view 1.27
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/lke/versions/{version}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .versions()
    .get(linode_api::resources::lke::versions::GetRequest {
        api_version: linode_api::models::GetApiVersionLkeVersionsVersionApiVersionEnum::V4,
        version: "string".to_string(),
    })
    .await;
```

    
### List Longview clients
Returns a paginated list of Longview Clients you have access to. Longview Client is used to monitor stats on your Linode with the help of the Longview Client application.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/longview/clients`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .clients()
    .list(linode_api::resources::longview::clients::ListRequest {
        api_version: linode_api::models::GetApiVersionLongviewClientsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a Longview client
Returns a single Longview Client you can access.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview view 789
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/longview/clients/{clientId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .clients()
    .get(linode_api::resources::longview::clients::GetRequest {
        api_version: linode_api::models::GetApiVersionLongviewClientsClientIdApiVersionEnum::V4,
        client_id: 123,
    })
    .await;
```

    
### Get a Longview plan
Get the details of your current Longview plan. This returns a `LongviewSubscription` object for your current Longview Pro plan, or an empty set `{}` if your current plan is Longview Free.

You must have at least one of the following `global` [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) in order to run this operation:

  - `"account_access": read_write`
  - `"account_access": read_only`
  - `"longview_subscription": true`
  - `"add_longview": true`

To update your subscription plan, send a request to [Update a Longview plan](https://techdocs.akamai.com/linode-api/reference/put-longview-plan).


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview plan-view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/longview/plan`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .plan()
    .list(linode_api::resources::longview::plan::ListRequest {
        api_version: linode_api::models::GetApiVersionLongviewPlanApiVersionEnum::V4,
    })
    .await;
```

    
### List Longview subscriptions
Returns a paginated list of available Longview Subscriptions. This is a public endpoint and requires no authentication.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview subscriptions-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/longview/subscriptions`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .subscriptions()
    .list(linode_api::resources::longview::subscriptions::ListRequest {
        api_version: linode_api::models::GetApiVersionLongviewSubscriptionsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a Longview subscription
Get the Longview plan details as a single `LongviewSubscription` object for the provided subscription ID. This is a public endpoint and requires no authentication.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview subscription-view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/longview/subscriptions/{subscriptionId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .subscriptions()
    .get(linode_api::resources::longview::subscriptions::GetRequest {
        api_version: linode_api::models::GetApiVersionLongviewSubscriptionsSubscriptionIdApiVersionEnum::V4,
        subscription_id: "string".to_string(),
    })
    .await;
```

    
### List Longview types
Returns Longview types and prices, including any region-specific rates.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/longview/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .types_resource()
    .list(linode_api::resources::longview::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionLongviewTypesApiVersionEnum::V4,
    })
    .await;
```

    
### List managed contacts
Returns a paginated list of Managed Contacts on your Account.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed contacts-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/contacts`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .contacts()
    .list(linode_api::resources::managed::contacts::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedContactsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed contact
Returns a single Managed Contact.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed contact-view 567
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/contacts/{contactId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .contacts()
    .get(linode_api::resources::managed::contacts::GetRequest {
        api_version: linode_api::models::GetApiVersionManagedContactsContactIdApiVersionEnum::V4,
        contact_id: 123,
    })
    .await;
```

    
### List managed credentials
Returns a paginated list of Managed Credentials on your Account.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credentials-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/credentials`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .list(linode_api::resources::managed::credentials::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedCredentialsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed SSH key
Returns the unique SSH public key assigned to your Linode account's Managed service. If you [add this public key](https://www.linode.com/docs/products/services/managed/get-started/#adding-the-public-key) to a Linode on your account, Linode special forces will be able to log in to the Linode with this key when attempting to resolve issues.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credential-sshkey-view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/credentials/sshkey`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .sshkey()
    .list(linode_api::resources::managed::credentials::sshkey::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedCredentialsSshkeyApiVersionEnum::V4,
    })
    .await;
```

    
### Get a managed credential
Returns a single Managed Credential.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credential-view 9991
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/credentials/{credentialId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .get(linode_api::resources::managed::credentials::GetRequest {
        api_version: linode_api::models::GetApiVersionManagedCredentialsCredentialIdApiVersionEnum::V4,
        credential_id: 123,
    })
    .await;
```

    
### List managed issues
Returns a paginated list of recent and ongoing issues detected on your Managed Services.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed issues-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/issues`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .issues()
    .list(linode_api::resources::managed::issues::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedIssuesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a managed issue
Returns a single Issue that is impacting or did impact one of your Managed Services.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed issue-view 823
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/issues/{issueId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .issues()
    .get(linode_api::resources::managed::issues::GetRequest {
        api_version: linode_api::models::GetApiVersionManagedIssuesIssueIdApiVersionEnum::V4,
        issue_id: 123,
    })
    .await;
```

    
### List managed Linode settings
Returns a paginated list of Managed Settings for your Linodes. There will be one entry per Linode on your Account.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed linode-settings-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/linode-settings`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .linode_settings()
    .list(linode_api::resources::managed::linode_settings::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedLinodeSettingsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a Linode's managed settings
Returns a single Linode's Managed settings.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed linode-setting-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/linode-settings/{linodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .linode_settings()
    .get(linode_api::resources::managed::linode_settings::GetRequest {
        api_version: linode_api::models::GetApiVersionManagedLinodeSettingsLinodeIdApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### List managed services
Returns a paginated list of Managed Services on your Account. These are the services Linode Managed is monitoring and will report and attempt to resolve issues with.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed services-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/services`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .list(linode_api::resources::managed::services::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedServicesApiVersionEnum::V4,
    })
    .await;
```

    
### Get a managed service
Returns information about a single Managed Service on your Account.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed service-view 9994
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/services/{serviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .get(linode_api::resources::managed::services::GetRequest {
        api_version: linode_api::models::GetApiVersionManagedServicesServiceIdApiVersionEnum::V4,
        service_id: 123,
    })
    .await;
```

    
### List managed stats
Returns a list of Managed Stats on your Account in the form of x and y data points. You can use these data points to plot your own graph visualizations. These stats reflect the last 24 hours of combined usage across all managed Linodes on your account giving you a high-level snapshot of data for the following:

- cpu
- disk
- swap
- network in
- network out

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed stats-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/managed/stats`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .stats()
    .list(linode_api::resources::managed::stats::ListRequest {
        api_version: linode_api::models::GetApiVersionManagedStatsApiVersionEnum::V4,
    })
    .await;
```

    
### List network transfer prices
Returns collection of network transfer prices, including any region-specific rates.


<<LB>>

---


- __CLI__.

    ```
    linode-cli network-transfer prices
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/network-transfer/prices`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .network_transfer()
    .prices()
    .list(linode_api::resources::network_transfer::prices::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkTransferPricesApiVersionEnum::V4,
    })
    .await;
```

    
### List firewalls
Returns a paginated list of accessible Firewalls.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .list(linode_api::resources::networking::firewalls::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a firewall
Get a specific Firewall resource by its ID. The Firewall's Devices will not be returned in the response. Instead, run the [List firewall devices](https://techdocs.akamai.com/linode-api/reference/get-firewall-devices) operation to review them.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls/{firewallId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .get(linode_api::resources::networking::firewalls::GetRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdApiVersionEnum::V4,
        firewall_id: 123,
    })
    .await;
```

    
### List firewall devices
Returns a paginated list of a Firewall's Devices. A Firewall Device assigns a Firewall to a service (referred to as the Device's `entity`).


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls devices-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls/{firewallId}/devices`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .devices()
    .list(linode_api::resources::networking::firewalls::devices::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesApiVersionEnum::V4,
        firewall_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a firewall device
Returns information for a Firewall Device, which assigns a Firewall to a service (referred to as the Device's `entity`).


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls device-view \
  123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls/{firewallId}/devices/{deviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .devices()
    .get(linode_api::resources::networking::firewalls::devices::GetRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdApiVersionEnum::V4,
        firewall_id: 123,
        device_id: 123,
    })
    .await;
```

    
### List firewall rule versions
Lists the current and historical rules of the firewall (that is not deleted), using `version`. Whenever rules update, the `version` increments from `1`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls versions-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls/{firewallId}/history`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .history()
    .list(linode_api::resources::networking::firewalls::history::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryApiVersionEnum::V4,
        firewall_id: 123,
    })
    .await;
```

    
### Get a firewall rule version
Get a specific firewall rule version for an `enabled` or `disabled` firewall.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls version-view \
  123 2
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls/{firewallId}/history/rules/{version}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .history()
    .rules()
    .get(linode_api::resources::networking::firewalls::history::rules::GetRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionApiVersionEnum::V4,
        firewall_id: 123,
        version: 3,
    })
    .await;
```

    
### List firewall rules
Returns the inbound and outbound Rules for a Firewall.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls rules-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/firewalls/{firewallId}/rules`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .rules()
    .list(linode_api::resources::networking::firewalls::rules::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdRulesApiVersionEnum::V4,
        firewall_id: 123,
    })
    .await;
```

    
### List IP addresses
Returns a paginated list of IP addresses on your account, excluding private addresses.

__Note__. Use the `skip_ipv6_rdns` query string to improve performance if your application frequently accesses this operation and doesn't require IPv6 RDNS data.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking ips-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/ips`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ips()
    .list(linode_api::resources::networking::ips::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingIpsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an IP address
Returns information about a single IP Address on your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking ip-view 97.107.143.141
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/ips/{address}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ips()
    .get(linode_api::resources::networking::ips::GetRequest {
        api_version: linode_api::models::GetApiVersionNetworkingIpsAddressApiVersionEnum::V4,
        address: "string".to_string(),
    })
    .await;
```

    
### List IPv6 pools
Displays the IPv6 pools on your Account. A pool of IPv6 addresses are routed to all of your Linodes in a single [region](https://techdocs.akamai.com/linode-api/reference/get-regions). Any Linode on your Account may bring up any address in this pool at any time, with no external configuration required.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking v6-pools
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/ipv6/pools`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv6()
    .pools()
    .list(linode_api::resources::networking::ipv6::pools::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingIpv6PoolsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List IPv6 ranges
Displays the IPv6 ranges on your Account.

  - An IPv6 range is a `/64` or `/56` block of IPv6 addresses routed to a single Linode in a given [region](https://techdocs.akamai.com/linode-api/reference/get-regions).

  - Your Linode is responsible for routing individual addresses in the range, or handling traffic for all the addresses in the range.

  - Run the [Create an IPv6 range](https://techdocs.akamai.com/linode-api/reference/post-ipv6-range) operation to add a `/64` or `/56` block of IPv6 addresses to your account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking v6-ranges
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/ipv6/ranges`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv6()
    .ranges()
    .list(linode_api::resources::networking::ipv6::ranges::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingIpv6RangesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an IPv6 range
View IPv6 range information.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking v6-range-view 2001:0db8::
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/ipv6/ranges/{range}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv6()
    .ranges()
    .get(linode_api::resources::networking::ipv6::ranges::GetRequest {
        api_version: linode_api::models::GetApiVersionNetworkingIpv6RangesRangeApiVersionEnum::V4,
        range: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
    })
    .await;
```

    
### List VLANs
Returns a list of all Virtual Local Area Networks (VLANs) on your Account. VLANs provide a mechanism for secure communication between two or more Linodes that are assigned to the same VLAN and are both within the same Layer 2 broadcast domain.

VLANs are created and attached to Linodes by using the `interfaces` property for the following operations:

- [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance)
- [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config)
- [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config)

There are several ways to detach a VLAN from a Linode:

- [Update](https://techdocs.akamai.com/linode-api/reference/put-linode-config) the active Configuration Profile to remove the VLAN Interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode.
- [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) without the VLAN Interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode into the new Configuration Profile.
- [Delete](https://techdocs.akamai.com/linode-api/reference/delete-linode-instance) the Linode.

__Note__. Only Next Generation Network (NGN) data centers support VLANs. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support.

__Note__. See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) to view additional specifications and limitations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vlans list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/networking/vlans`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .vlans()
    .list(linode_api::resources::networking::vlans::ListRequest {
        api_version: linode_api::models::GetApiVersionNetworkingVlansApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List NodeBalancers
Returns a paginated list of NodeBalancers you have access to.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .list(linode_api::resources::nodebalancers::ListRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List NodeBalancer types
Returns NodeBalancer types and prices, including any region-specific rates.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .types_resource()
    .list(linode_api::resources::nodebalancers::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersTypesApiVersionEnum::V4,
    })
    .await;
```

    
### Get a NodeBalancer
Returns a single NodeBalancer you can access.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers view 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .get(linode_api::resources::nodebalancers::GetRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdApiVersionEnum::V4,
        node_balancer_id: 123,
    })
    .await;
```

    
### List configs
Returns a paginated list of NodeBalancer Configs associated with this NodeBalancer. NodeBalancer Configs represent individual ports that this NodeBalancer will accept traffic on, one Config per port.

For example, if you wanted to accept standard HTTP traffic, you would need a Config listening on port 80.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers configs-list 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}/configs`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .list(linode_api::resources::nodebalancers::configs::ListRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsApiVersionEnum::V4,
        node_balancer_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a config
Returns configuration information for a single port of this NodeBalancer.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers config-view \
  12345 4567
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .get(linode_api::resources::nodebalancers::configs::GetRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
    })
    .await;
```

    
### List nodes
Returns a paginated list of NodeBalancer nodes associated with this Config. These are the backends that will be sent traffic for this port.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers nodes-list 12345 4567
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .nodes()
    .list(linode_api::resources::nodebalancers::configs::nodes::ListRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a NodeBalancer's node
Returns information about a single Node, a backend for this NodeBalancer's configured port.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers node-view 12345 4567 54321
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes/{nodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .nodes()
    .get(linode_api::resources::nodebalancers::configs::nodes::GetRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        node_id: "string".to_string(),
    })
    .await;
```

    
### List NodeBalancer firewalls
View information for Firewalls assigned to this NodeBalancer.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers firewalls $nodeBalancerId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}/firewalls`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .firewalls()
    .list(linode_api::resources::nodebalancers::firewalls::ListRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdFirewallsApiVersionEnum::V4,
        node_balancer_id: 123,
    })
    .await;
```

    
### Get NodeBalancer statistics
Returns detailed statistics about the requested NodeBalancer.


<<LB>>

---


- __OAuth scopes__.

    ```
    nodebalancers:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/nodebalancers/{nodeBalancerId}/stats`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .stats()
    .list(linode_api::resources::nodebalancers::stats::ListRequest {
        api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdStatsApiVersionEnum::V4,
        node_balancer_id: 123,
    })
    .await;
```

    
### List Object Storage buckets
Returns a paginated list of all Object Storage buckets available in your account.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/serviceops/#list-buckets) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/buckets`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .list(linode_api::resources::object_storage::buckets::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageBucketsApiVersionEnum::V4,
    })
    .await;
```

    
### List Object Storage buckets per region
Returns a list of buckets on your account, in the specified region.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#get-bucket) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/buckets/{regionId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .get_by_region_id(linode_api::resources::object_storage::buckets::GetByRegionIdRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdApiVersionEnum::V4,
        region_id: "string".to_string(),
    })
    .await;
```

    
### Get an Object Storage bucket
Returns a single Object Storage bucket.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#get-bucket) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/buckets/{regionId}/{bucket}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .get_by_bucket(linode_api::resources::object_storage::buckets::GetByBucketRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
    })
    .await;
```

    
### Get an Object Storage object ACL config
View an Object's configured Access Control List (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects and specify the level of access granted to those users.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object-acl) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-acl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .object_acl()
    .list(linode_api::resources::object_storage::buckets::object_acl::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectAclApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        name: "string".to_string(),
    })
    .await;
```

    
### List Object Storage bucket contents
Returns the contents of a bucket. The contents are paginated using a `marker`, that's the name of the last object on the previous page.  Objects can also be filtered by `prefix` and `delimiter`. See [Filtering and sorting](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) for more information.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-list`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .object_list()
    .list(linode_api::resources::object_storage::buckets::object_list::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        ..Default::default()
    })
    .await;
```

    
### Get an Object Storage TLS/SSL certificate
Returns a boolean value indicating if this bucket has a corresponding TLS/SSL certificate that was uploaded by an Account user.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage ssl-view \
  us-east-1 example-bucket
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/ssl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .ssl_resource()
    .list(linode_api::resources::object_storage::buckets::ssl_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketSslApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
    })
    .await;
```

    
### List clusters
Returns a paginated list of available Object Storage legacy clusters.

> 📘
>
> This displays deprecated `clusterId` values that represent regions used with older versions of the API. It's maintained for backward compatibility. Run [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region), instead.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage clusters-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/object-storage/clusters`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .clusters()
    .list(linode_api::resources::object_storage::clusters::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageClustersApiVersionEnum::V4,
    })
    .await;
```

    
### Get a cluster
__Deprecated__ Returns a single Object Storage cluster.

> 📘
>
> This displays deprecated `clusterId` values that represent regions used with older versions of the API. It's maintained for backward compatibility. Run [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region), instead.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage clusters-view us-east-1
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/object-storage/clusters/{clusterId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .clusters()
    .get(linode_api::resources::object_storage::clusters::GetRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageClustersClusterIdApiVersionEnum::V4,
        cluster_id: "us-east-1".to_string(),
    })
    .await;
```

    
### List Object Storage keys
Returns a paginated list of Object Storage keys for authenticating to the Object Storage S3 API.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage keys-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/keys`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .keys()
    .list(linode_api::resources::object_storage::keys::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageKeysApiVersionEnum::V4,
    })
    .await;
```

    
### Get an Object Storage key
Returns a single Object Storage key provisioned for your account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage keys-view \
  --keyId 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/keys/{keyId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .keys()
    .get(linode_api::resources::object_storage::keys::GetRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageKeysKeyIdApiVersionEnum::V4,
        key_id: 123,
    })
    .await;
```

    
### Get Object Storage transfer data
The amount of outbound data transfer used by your account's Object Storage buckets. Object Storage adds 1 terabyte of outbound data transfer to your data transfer pool. See the [Object Storage Overview](https://www.linode.com/docs/products/storage/object-storage/#pricing) guide for details on Object Storage transfer quotas.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/object-storage/transfer`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .transfer()
    .list(linode_api::resources::object_storage::transfer::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageTransferApiVersionEnum::V4,
    })
    .await;
```

    
### List Object Storage types
Returns Object Storage types and prices, including any region-specific rates.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/object-storage/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .types_resource()
    .list(linode_api::resources::object_storage::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionObjectStorageTypesApiVersionEnum::V4,
    })
    .await;
```

    
### List placement groups
Returns a paginated list of placement groups you have permission to view.


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement groups-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    placement:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/placement/groups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .list(linode_api::resources::placement::groups::ListRequest {
        api_version: linode_api::models::GetApiVersionPlacementGroupsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a placement group
View a specific placement group by ID.


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement group-view 528
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/placement/groups/{groupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .get(linode_api::resources::placement::groups::GetRequest {
        api_version: linode_api::models::GetApiVersionPlacementGroupsGroupIdApiVersionEnum::V4,
        group_id: 123,
    })
    .await;
```

    
### Get a profile
Returns information about the current User. This can be used to see who is acting in applications where more than one token is managed. For example, in third-party OAuth applications.

This operation is always accessible, no matter what OAuth scopes the acting token has.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile view
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/profile`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .list(linode_api::resources::profile_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileApiVersionEnum::V4,
    })
    .await;
```

    
### List authorized apps
This is a collection of OAuth apps that you've given access to your Account, and includes the level of access granted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile apps-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/apps`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .apps()
    .list(linode_api::resources::profile_resource::apps::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileAppsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an authorized app
Returns information about a single app you've authorized to access your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile app-view 1234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/apps/{appId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .apps()
    .get(linode_api::resources::profile_resource::apps::GetRequest {
        api_version: linode_api::models::GetApiVersionProfileAppsAppIdApiVersionEnum::V4,
        app_id: 123,
    })
    .await;
```

    
### List trusted devices
Returns a paginated list of active TrustedDevices for your User. Browsers with an active Remember Me Session are logged into your account until the session expires or is revoked.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile devices-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/devices`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .devices()
    .list(linode_api::resources::profile_resource::devices::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileDevicesApiVersionEnum::V4,
    })
    .await;
```

    
### Get a trusted device
Returns a single active TrustedDevice for your User.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile device-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/devices/{deviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .devices()
    .get(linode_api::resources::profile_resource::devices::GetRequest {
        api_version: linode_api::models::GetApiVersionProfileDevicesDeviceIdApiVersionEnum::V4,
        device_id: 123,
    })
    .await;
```

    
### List grants
This returns a GrantsResponse describing what the acting User has been granted access to.  For unrestricted users, this will return a  204 and no body because unrestricted users have access to everything without grants.  This will not return information about entities you do not have access to.  This operation is useful when writing third-party OAuth applications to see what options you should present to the acting User.

For example, if they do not have `global.add_linodes`, you might not display a button to deploy a new Linode.

Any client may run this operation; no OAuth scopes are required.

**API Endpoint**: `GET /{apiVersion}/profile/grants`


### List logins
Returns a collection of successful account logins from this user during the last 90 days.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile logins-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/logins`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .logins()
    .list(linode_api::resources::profile_resource::logins::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileLoginsApiVersionEnum::V4,
    })
    .await;
```

    
### Get a profile's login
Returns a login object displaying information about a successful account login from this user.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile login-view 1234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/logins/{loginId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .logins()
    .get(linode_api::resources::profile_resource::logins::GetRequest {
        api_version: linode_api::models::GetApiVersionProfileLoginsLoginIdApiVersionEnum::V4,
        login_id: 123,
    })
    .await;
```

    
### Get user preferences
View a list of user preferences tied to the OAuth client that generated the token making the request. The user preferences endpoints allow consumers of the API to store arbitrary JSON data, such as a user's font size preference or preferred display name. User preferences are available for each OAuth client registered to your account, and as such an account can have multiple user preferences.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/preferences`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .preferences()
    .list(linode_api::resources::profile_resource::preferences::ListRequest {
        api_version: linode_api::models::GetApiVersionProfilePreferencesApiVersionEnum::V4,
    })
    .await;
```

    
### List security questions
Returns a collection of security questions and their responses, if any, for your User Profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli security-questions list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/security-questions`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .security_questions()
    .list(linode_api::resources::profile_resource::security_questions::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileSecurityQuestionsApiVersionEnum::V4,
    })
    .await;
```

    
### List SSH keys
Returns a collection of SSH Keys you've added to your Profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli sshkeys list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/sshkeys`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .sshkeys()
    .list(linode_api::resources::profile_resource::sshkeys::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileSshkeysApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get an SSH key
Returns a single SSH Key object identified by `id` that you have access to view.


<<LB>>

---


- __CLI__.

    ```
    linode-cli sshkeys view 42
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/sshkeys/{sshKeyId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .sshkeys()
    .get(linode_api::resources::profile_resource::sshkeys::GetRequest {
        api_version: linode_api::models::GetApiVersionProfileSshkeysSshKeyIdApiVersionEnum::V4,
        ssh_key_id: 123,
    })
    .await;
```

    
### List personal access tokens
Returns a paginated list of Personal Access Tokens currently active for your User.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile tokens-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/tokens`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tokens()
    .list(linode_api::resources::profile_resource::tokens::ListRequest {
        api_version: linode_api::models::GetApiVersionProfileTokensApiVersionEnum::V4,
    })
    .await;
```

    
### Get a personal access token
Returns a single Personal Access Token.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile token-view 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/profile/tokens/{tokenId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tokens()
    .get(linode_api::resources::profile_resource::tokens::GetRequest {
        api_version: linode_api::models::GetApiVersionProfileTokensTokenIdApiVersionEnum::V4,
        token_id: 123,
    })
    .await;
```

    
### List regions
Lists the Regions available for Linode services. Not all services are guaranteed to be available in all Regions.


<<LB>>

---


- __CLI__.

    ```
    linode-cli regions list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/regions`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .regions()
    .list(linode_api::resources::regions::ListRequest {
        api_version: linode_api::models::GetApiVersionRegionsApiVersionEnum::V4,
    })
    .await;
```

    
### List regions' availability
Returns availability data for all Regions.

Currently, this operation returns availability of select premium and GPU plans for select regions.


<<LB>>

---


- __CLI__.

    ```
    linode-cli regions list-avail
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/regions/availability`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .regions()
    .availability()
    .list(linode_api::resources::regions::availability::ListRequest {
        api_version: linode_api::models::GetApiVersionRegionsAvailabilityApiVersionEnum::V4,
    })
    .await;
```

    
### Get a region
Returns a single Region.


<<LB>>

---


- __CLI__.

    ```
    linode-cli regions view us-east
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/regions/{regionId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .regions()
    .get(linode_api::resources::regions::GetRequest {
        api_version: linode_api::models::GetApiVersionRegionsRegionIdApiVersionEnum::V4,
        region_id: "string".to_string(),
    })
    .await;
```

    
### Get a region's availability
Returns availability data for a single Region.


<<LB>>

---


- __CLI__.

    ```
    linode-cli regions view-avail us-east
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/regions/{regionId}/availability`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .regions()
    .availability()
    .list_1(linode_api::resources::regions::availability::List1Request {
        api_version: linode_api::models::GetApiVersionRegionsRegionIdAvailabilityApiVersionEnum::V4,
        region_id: "string".to_string(),
    })
    .await;
```

    
### List support tickets
Returns a collection of Support Tickets on your Account. Support Tickets can be both tickets you open with Linode for support, as well as tickets generated by Linode regarding your Account. This collection includes all Support Tickets generated on your Account, with open tickets returned first.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tickets list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/support/tickets`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .list(linode_api::resources::support::tickets::ListRequest {
        api_version: linode_api::models::GetApiVersionSupportTicketsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a support ticket
Returns a Support Ticket under your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tickets view 11223344
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/support/tickets/{ticketId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .get(linode_api::resources::support::tickets::GetRequest {
        api_version: linode_api::models::GetApiVersionSupportTicketsTicketIdApiVersionEnum::V4,
        ticket_id: 123,
    })
    .await;
```

    
### List replies
Returns a collection of replies to a Support Ticket on your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tickets replies 11223344
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/support/tickets/{ticketId}/replies`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .replies()
    .list(linode_api::resources::support::tickets::replies::ListRequest {
        api_version: linode_api::models::GetApiVersionSupportTicketsTicketIdRepliesApiVersionEnum::V4,
        ticket_id: 123,
        ..Default::default()
    })
    .await;
```

    
### List tags
Tags are User-defined labels attached to objects in your Account, such as Linodes. They are used for specifying and grouping attributes of objects that are relevant to the User.

This operation returns a paginated list of Tags on your account.

__Important__. You must be an unrestricted User in order to access, add, or modify Tags information.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tags list
linode-cli tags ls
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/tags`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .tags()
    .list(linode_api::resources::tags::ListRequest {
        api_version: linode_api::models::GetApiVersionTagsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List tagged objects
Returns a paginated list of all objects you've tagged with the requested Tag. This is a mixed collection of all object types.

__Important__. You must be an unrestricted User in order to access, add, or modify Tags information.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/tags/{tagLabel}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .tags()
    .get(linode_api::resources::tags::GetRequest {
        api_version: linode_api::models::GetApiVersionTagsTagLabelApiVersionEnum::V4,
        tag_label: "string".to_string(),
        ..Default::default()
    })
    .await;
```

    
### List volumes
Returns a paginated list of Volumes you have permission to view.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/volumes`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .list(linode_api::resources::volumes::ListRequest {
        api_version: linode_api::models::GetApiVersionVolumesApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List volume types
Returns volume types and prices, including any region-specific rates.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes types
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/volumes/types`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .types_resource()
    .list(linode_api::resources::volumes::types_resource::ListRequest {
        api_version: linode_api::models::GetApiVersionVolumesTypesApiVersionEnum::V4,
    })
    .await;
```

    
### Get a volume
Get information about a single Volume.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes view 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/volumes/{volumeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .get(linode_api::resources::volumes::GetRequest {
        api_version: linode_api::models::GetApiVersionVolumesVolumeIdApiVersionEnum::V4,
        volume_id: 123,
        ..Default::default()
    })
    .await;
```

    
### List VPCs
Display all VPCs on your account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/vpcs`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .list(linode_api::resources::vpcs::ListRequest {
        api_version: linode_api::models::GetApiVersionVpcsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### List VPC IP addresses
Returns a paginated list of all VPC IP addresses and address ranges on your account.

__Note__. If a Linode has several configuration profiles that include a VPC interface, address information for all of them is listed in the response. Since VPCs can use the same address space, you may see duplicate IP addresses.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs ips-all-list
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/vpcs/ips`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .ips()
    .list(linode_api::resources::vpcs::ips::ListRequest {
        api_version: linode_api::models::GetApiVersionVpcsIpsApiVersionEnum::V4,
        ..Default::default()
    })
    .await;
```

    
### Get a VPC
Get information about a single VPC.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs view $vpcId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/vpcs/{vpcId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .get(linode_api::resources::vpcs::GetRequest {
        api_version: linode_api::models::GetApiVersionVpcsVpcIdApiVersionEnum::V4,
        vpc_id: 123,
    })
    .await;
```

    
### List a VPC's IP addresses
Returns a paginated list of IP addresses for a single VPC.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs ip-list 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `GET /{apiVersion}/vpcs/{vpcId}/ips`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .ips()
    .list_1(linode_api::resources::vpcs::ips::List1Request {
        api_version: linode_api::models::GetApiVersionVpcsVpcIdIpsApiVersionEnum::V4,
        vpc_id: 123,
        ..Default::default()
    })
    .await;
```

    
### List VPC subnets
Get information about all VPC Subnets associated with a VPC.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs subnets-list $vpcId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/vpcs/{vpcId}/subnets`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .subnets()
    .list(linode_api::resources::vpcs::subnets::ListRequest {
        api_version: linode_api::models::GetApiVersionVpcsVpcIdSubnetsApiVersionEnum::V4,
        vpc_id: 123,
        ..Default::default()
    })
    .await;
```

    
### Get a VPC subnet
Get information about a single VPC Subnet.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs subnet-view $vpcId $vpcSubnetId
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

**API Endpoint**: `GET /{apiVersion}/vpcs/{vpcId}/subnets/{vpcSubnetId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .subnets()
    .get(linode_api::resources::vpcs::subnets::GetRequest {
        api_version: linode_api::models::GetApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum::V4,
        vpc_id: 123,
        vpc_subnet_id: 123,
    })
    .await;
```

    
### Acknowledge agreements
Accept required agreements by setting them to `true`. This remains until the content of the agreement changes. If it does, you need to run this operation again to accept it. If you set this to `false`, the API rejects the request and you need to open a [support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to reset the agreement. Omitted agreements are left unchanged.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/agreements`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .agreements()
    .create(linode_api::resources::account::agreements::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountAgreementsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountAgreementsBody {
            eu_model: Some(true),
            master_service_agreement: Some(true),
            privacy_policy: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Enroll in a Beta program
Enroll your Account in an active Beta Program.

Only unrestricted Users can access this operation.

To view active Beta Programs, run the [List beta programs](https://techdocs.akamai.com/linode-api/reference/get-beta-programs) operation.

Active Beta Programs may have a limited number of enrollments. If a Beta Program has reached is maximum number of enrollments, an error is returned even though the request is successful.

Beta Programs with `"greenlight_only": true` can only be enrolled by Accounts that participate in the [Greenlight](https://www.linode.com/green-light/) program.


<<LB>>

---


- __CLI__.

    ```
    linode-cli betas enroll --id example_open
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/betas`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .betas()
    .create(linode_api::resources::account::betas::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountBetasApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountBetasBody {
            id: "example_open".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Cancel your account
Cancels an active Linode account. Akamai attempts to charge the credit card on file for any remaining balance. An error occurs if this charge fails.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.

__Parent and child accounts__ In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- A child account user can't cancel a child account.
- You can't cancel a parent account if it has an active child account.
- You need to work with your Akamai account team to dissolve any parent-child account relationships before you can fully cancel a child or parent account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account cancel \
  --comments "I'm consolidating my accounts"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/cancel`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .cancel()
    .create(linode_api::resources::account::cancel::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountCancelApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountCancelBody {
            comments: Some(
                "I'm consolidating multiple accounts into one.".to_string(),
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a proxy user token
Create a short-lived bearer token for a parent user on a child account, using the `euuid` of that child account. In the context of the API, a parent user on a child account is referred to as a "proxy user." When Akamai provisions your parent-child account environment, a proxy user is automatically set in the child account. It follows a specific naming convention:

    <Parent account `company` name>_<SHA256 hash of parent `company` name and child account `euuid`>

__Note__. The variables above use only the first 15 and 16 characters of these values, respectively.

The token lets a parent account run API operations through the proxy user, as if they are a child user in the child account.

These points apply to the use of this operation:

- To create a token, a parent account user needs the `child_account_access` grant. This lets them use the proxy user on the child account. You can run [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) on a parent account user to check its `child_account_access` setting. To add this access, you can [update](https://techdocs.akamai.com/linode-api/reference/put-user-grants) the parent account user.

- The created token inherits the permissions of the proxy user. It will never have less.

- The API returns the raw token in the response. You can't get it again, so be sure to store it.

Example workflow:

1. [List child accounts](https://techdocs.akamai.com/linode-api/reference/get-child-accounts) and store the `euuid` for the applicable one.
2. Run this operation and store the `token` that's created for the proxy user.
3. As a parent account user with access to the proxy user in the child account, use this `token` to authenticate API operations, as if you were a child user.


<<LB>>

---


- __CLI__.

    ```
    linode-cli child-account create A1BC2DEF-34GH-567I-J890KLMN12O34P56
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    child_account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/child-accounts/{euuid}/token`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .child_accounts()
    .token_resource()
    .create(linode_api::resources::account::child_accounts::token_resource::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountChildAccountsEuuidTokenApiVersionEnum::V4,
        euuid: "string".to_string(),
    })
    .await;
```

    
### Add or edit a credit card
__Deprecated__ Please run [Add a payment method](https://techdocs.akamai.com/linode-api/reference/post-payment-method).

Adds a credit card Payment Method to your account and sets it as the default method.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/credit-card`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .credit_card()
    .create(linode_api::resources::account::credit_card::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountCreditCardApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountCreditCardBody {
            card_number: "string".to_string(),
            cvv: "123".to_string(),
            expiry_month: 12,
            expiry_year: 2020,
            ..Default::default()
        },
    })
    .await;
```

    
### Create an entity transfer
__Deprecated__ Please run [Request a service transfer](https://techdocs.akamai.com/linode-api/reference/post-service-transfer).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/entity-transfers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .entity_transfers()
    .create(linode_api::resources::account::entity_transfers::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountEntityTransfersApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountEntityTransfersBody {
            entities: linode_api::models::PostApiVersionAccountEntityTransfersBodyEntities {
                linodes: Some(vec![111, 222]),
                ..Default::default()
            },
            ..Default::default()
        },
    })
    .await;
```

    
### Accept an entity transfer
__Deprecated__ Please run [Accept a service transfer](https://techdocs.akamai.com/linode-api/reference/post-accept-service-transfer).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/entity-transfers/{token}/accept`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .entity_transfers()
    .accept()
    .create(linode_api::resources::account::entity_transfers::accept::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountEntityTransfersTokenAcceptApiVersionEnum::V4,
        token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Mark an event as read
Marks a single Event as read.


<<LB>>

---


- __CLI__.

    ```
    linode-cli events mark-read 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    events:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/events/{eventId}/read`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .events()
    .read()
    .create(linode_api::resources::account::events::read::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountEventsEventIdReadApiVersionEnum::V4,
        event_id: 123,
    })
    .await;
```

    
### Mark an event as seen
Marks all Events up to and including this Event by ID as seen.


<<LB>>

---


- __CLI__.

    ```
    linode-cli events mark-seen 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    events:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/events/{eventId}/seen`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .events()
    .seen()
    .create(linode_api::resources::account::events::seen::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountEventsEventIdSeenApiVersionEnum::V4,
        event_id: 123,
    })
    .await;
```

    
### Create an OAuth client
Creates an OAuth Client, which can be used to allow users (using their Linode account) to log in to your own application, and optionally grant your application some amount of access to their Linodes or other entities.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account client-create \
  --label Test_Client_1 \
  --redirect_uri https://example.org/callback
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/oauth-clients`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .create(linode_api::resources::account::oauth_clients::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountOauthClientsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountOauthClientsBody {
            id: Some("2737bf16b39ab5d7b4a1".to_string()),
            label: Some("Test_Client_1".to_string()),
            public: Some(false),
            redirect_uri: Some("https://example.org/oauth/callback".to_string()),
            secret: Some("<REDACTED>".to_string()),
            status: Some(
                linode_api::models::PostApiVersionAccountOauthClientsBodyStatusEnum::Active,
            ),
            thumbnail_url: linode_api::Patch::new(
                "https://api.linode.com/v4/account/clients/2737bf16b39ab5d7b4a1/thumbnail"
                    .to_string(),
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Reset an OAuth client secret
Resets the OAuth Client secret for a client you own, and returns the OAuth Client with the plaintext secret. This secret is not supposed to be publicly known or disclosed anywhere. This can be used to generate a new secret in case the one you have has been leaked, or to get a new secret if you lost the original. The old secret is expired immediately, and logins to your client with the old secret will fail.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account client-reset-secret \
  edc6790ea9db4d224c5c
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/oauth-clients/{clientId}/reset-secret`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .reset_secret()
    .create(linode_api::resources::account::oauth_clients::reset_secret::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountOauthClientsClientIdResetSecretApiVersionEnum::V4,
        client_id: "string".to_string(),
    })
    .await;
```

    
### Add a payment method
Adds a Payment Method to your Account with the option to set it as the default method.

- Adding a default Payment Method removes the default status from any other Payment Method.

- An Account can have up to 6 active Payment Methods.

- Up to 60 Payment Methods can be added each day.

- Prior to adding a Payment Method, ensure that your billing address information is up-to-date with a valid `zip` by running the [Update your account](https://techdocs.akamai.com/linode-api/reference/put-account) operation.

- A `payment_method_add` event is generated when a payment is successfully submitted.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- Child account users can't run this operation. These users don't have access to billing-related operations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli payment-methods add \
  --type credit_card \
  --is_default true \
  --data.card_number 4111111111111111 \
  --data.expiry_month 11 \
  --data.expiry_year 2020 \
  --data.cvv 111
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/payment-methods`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payment_methods()
    .create(linode_api::resources::account::payment_methods::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountPaymentMethodsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountPaymentMethodsBody {
            data: linode_api::models::PostApiVersionAccountPaymentMethodsBodyData {
                card_number: "string".to_string(),
                cvv: "123".to_string(),
                expiry_month: 12,
                expiry_year: 2020,
                ..Default::default()
            },
            is_default: true,
            type_field: linode_api::models::PostApiVersionAccountPaymentMethodsBodyTypeEnum::CreditCard,
            ..Default::default()
        },
    })
    .await;
```

    
### Set a default payment method
Make the specified Payment Method the default method for automatically processing payments. Removes the default status from any other Payment Method.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- Child account users can't run this operation. These users don't have access to billing-related operations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli payment-methods default 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/payment-methods/{paymentMethodId}/make-default`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .payment_methods()
    .make_default()
    .create(linode_api::resources::account::payment_methods::make_default::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountPaymentMethodsPaymentMethodIdMakeDefaultApiVersionEnum::V4,
        payment_method_id: 123,
    })
    .await;
```

    
### Make a payment
Makes a Payment to your Account.

- The requested amount is charged to the default Payment Method if no `payment_method_id` is specified.

- A `payment_submitted` event is generated when a payment is successfully submitted.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- Child account users can't run this operation. These users don't have access to billing-related operations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account payment-create \
  --usd 120.50 \
  --payment_method_id 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/payments`


### Stage a PayPal payment
__Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/payments/paypal`


### Execute a PayPal payment
__Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/payments/paypal/execute`


### Add a promo credit
Adds an expiring Promo Credit to your account. The following restrictions apply:

- Your account needs to be less than 90 days old.

- You can't already have a Promo Credit on your account.

- The user making the request needs to be unrestricted. You can run the [Update a user](https://techdocs.akamai.com/linode-api/reference/put-user) operation to change a user's restricted status.

- The `promo_code` needs to be valid and unexpired.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- Child account users can't run this operation. These users don't have access to billing-related operations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account \
  promo-add \
  --promo-code abcdefABCDEF1234567890
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/promo-codes`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .promo_codes()
    .create(linode_api::resources::account::promo_codes::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountPromoCodesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountPromoCodesBody {
            promo_code: "string".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Request a service transfer
Creates a transfer request for the specified services. A request can contain any of the specified service types and any number of each service type. At this time, only Linodes can be transferred.

When created successfully, a confirmation email is sent to the account that created this transfer containing a transfer token and instructions on completing the transfer.

When a transfer is [accepted](https://techdocs.akamai.com/linode-api/reference/post-accept-service-transfer), the requested services are moved to the receiving account. Linode services will not experience interruptions due to the transfer process. Backups for Linodes are transferred as well.

DNS records that are associated with requested services will not be transferred or updated. Please ensure that associated DNS records have been updated or communicated to the recipient prior to the transfer.

A transfer can take up to three hours to complete once accepted. When a transfer is completed, billing for transferred services ends for the sending account and begins for the receiving account.

This operation can only be accessed by the unrestricted users of an account.

There are several conditions that you need to meet to successfully create a transfer request:

1. The account creating the transfer can't have a past due balance or active Terms of Service violation.

1. The service needs to be owned by the account that is creating the transfer.

1. The service can't be assigned to another Service Transfer that is pending or that's been accepted and is incomplete.

1. Linodes can't:

    - be assigned to a NodeBalancer, Firewall, VLAN, VPC, or Managed Service.

    - have any attached Block Storage Volumes.

    - have any shared IP addresses.

    - have any assigned /56, /64, or /116 IPv6 ranges.


<<LB>>

---


- __CLI__.

    ```
    linode-cli service-transfers \
  create \
  --entities.linodes 111 \
  --entities.linodes 222
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/service-transfers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .service_transfers()
    .create(linode_api::resources::account::service_transfers::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountServiceTransfersApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountServiceTransfersBody {
            entities: linode_api::models::PostApiVersionAccountServiceTransfersBodyEntities {
                linodes: Some(vec![111, 222]),
                ..Default::default()
            },
            ..Default::default()
        },
    })
    .await;
```

    
### Accept a service transfer
Accept a Service Transfer for the provided token to receive the services included in the transfer to your account. At this time, only Linodes can be transferred.

When accepted, email confirmations are sent to the accounts that created and accepted the transfer. A transfer can take up to three hours to complete once accepted. Once a transfer is completed, billing for transferred services ends for the sending account and begins for the receiving account.

This operation can only be accessed by the unrestricted users of the account that receives the transfer. Users of the same account that created a transfer cannot accept the transfer.

There are several conditions that must be met in order to accept a transfer request:

1. Only transfers with a `pending` status can be accepted.

1. The account accepting the transfer must have a registered payment method and must not have a past due balance or other account limitations for the services to be transferred.

1. Both the account that created the transfer and the account that is accepting the transfer must not have any active Terms of Service violations.

1. The service must still be owned by the account that created the transfer.

1. Linodes must not:

    - be assigned to a NodeBalancer, Firewall, VLAN, or Managed Service.

    - have any attached Block Storage Volumes.

    - have any shared IP addresses.

    - have any assigned /56, /64, or /116 IPv6 ranges.

Any and all of the above conditions must be cured and maintained by the relevant account prior to the transfer's expiration to allow the transfer to be accepted by the receiving account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli service-transfers \
  accept 123E4567-E89B-12D3-A456-426614174000
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/service-transfers/{token}/accept`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .service_transfers()
    .accept()
    .create(linode_api::resources::account::service_transfers::accept::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountServiceTransfersTokenAcceptApiVersionEnum::V4,
        token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Enable Linode Managed
Enables Linode Managed for the entire account and sends a welcome email to the account's associated email address. Linode Managed can monitor any service or software stack reachable over TCP or HTTP. See our [Linode Managed guide](https://www.linode.com/docs/guides/linode-managed/) to learn more.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account enable-managed
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/settings/managed-enable`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .settings()
    .managed_enable()
    .create(linode_api::resources::account::settings::managed_enable::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountSettingsManagedEnableApiVersionEnum::V4,
    })
    .await;
```

    
### Create a user
Creates a user on your account. You determine the new user's account access by setting it to restricted or unrestricted and by defining its grants. After completion, the API sends a confirmation message containing password creation and login instructions to the user's `email` address.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- A parent account user can create new parent account users.

- A child account can [update](https://techdocs.akamai.com/linode-api/reference/put-user) the child account parent user (proxy user) to `unrestricted`. This gives the proxy user access to create new child account users.

- A child account user can create new child account users.

- You can't create a proxy user. The proxy user in a child account is predefined when you initially provision the parent-child relationship.


<<LB>>

---


- __CLI__.

    ```
    linode-cli users create \
  --username example_user \
  --email example_user@linode.com \
  --restricted true
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/account/users`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .users()
    .create(linode_api::resources::account::users::CreateRequest {
        api_version: linode_api::models::PostApiVersionAccountUsersApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionAccountUsersBody {
            email: Some("example_user@linode.com".to_string()),
            last_login: linode_api::Patch::new(linode_api::models::PostApiVersionAccountUsersBodyLastLogin {
                login_datetime: Some("2018-01-01T01:01:01".to_string()),
                status: Some(
                    linode_api::models::PostApiVersionAccountUsersBodyLastLoginStatusEnum::Successful,
                ),
                ..Default::default()
            }),
            password_created: linode_api::Patch::new(
                "2018-01-01T01:01:01".to_string(),
            ),
            restricted: Some(true),
            ssh_keys: Some(vec!["home-pc".to_string(), "laptop".to_string()]),
            tfa_enabled: Some(true),
            username: Some("example_user".to_string()),
            verified_phone_number: linode_api::Patch::new("+5555555555".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a managed MySQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Provision a Managed MySQL Database.

Restricted Users must have the `add_databases` grant to use this operation.

New instances can take approximately 15 to 30 minutes to provision.

The `allow_list` is used to control access to the Managed Database.

- IP addresses and ranges in this list can access the Managed Database. All other sources are blocked.

- If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.

- Entering an empty array (`[]`) blocks all connections (both public and private) to the Managed Database.

All Managed Databases include automatic, daily backups. Up to seven backups are automatically stored for each Managed Database, providing restore points for each day of the past week.

All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed MySQL Database during configurable maintenance windows.

- If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.

- __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then [migrate your databases](https://www.linode.com/docs/products/databases/managed-databases/guides/migrate-mysql/) from the original Managed Database cluster to the new one.

- To modify update the maintenance window for a Database, run the [Update a managed MySQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-mysql-instance) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-create \
  --label example-db1 \
  --region us-east \
  --type g6-dedicated-2 \
  --cluster_size 3 \
  --engine mysql/8.0.26 \
  --encrypted false \
  --ssl_connection false \
  --replication_type semi_synch \
  --allow_list 203.0.113.1 \
  --allow_list 192.0.1.0/24
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/mysql/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .create(linode_api::resources::databases::mysql::instances::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionDatabasesMysqlInstancesBody {
            allow_list: Some(
                vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
            ),
            cluster_size: Some(3),
            encrypted: Some(false),
            engine: "mysql/8.0.26".to_string(),
            label: "example-db".to_string(),
            region: "us-east".to_string(),
            replication_type: Some(
                linode_api::models::PostApiVersionDatabasesMysqlInstancesBodyReplicationTypeEnum::SemiSynch,
            ),
            ssl_connection: Some(true),
            type_field: "g6-dedicated-2".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a managed MySQL database backup snapshot
__This operation is currently only available for customers who already have an active Managed Database.__

Creates a snapshot backup of a Managed MySQL Database.

Requires `read_write` access to the Database.

Up to 3 snapshot backups for each Database can be stored at a time. If 3 snapshots have been created for a Database, one must be deleted before another can be made.

Backups generated by this operation have the type `snapshot`. Snapshot backups may take several minutes to complete, after which they will be accessible to view or restore.

The Database must have an `active` status to perform this operation. If another backup is in progress, it must complete before a new backup can be initiated.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-backup-snapshot 123 \
  --label snapshot1 \
  --target primary
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/mysql/instances/{instanceId}/backups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .backups()
    .create(linode_api::resources::databases::mysql::instances::backups::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsApiVersionEnum::V4,
        instance_id: 123,
        data: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBody {
            label: "db-snapshot".to_string(),
            target: Some(
                linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBodyTargetEnum::Primary,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Restore a managed MySQL database backup
__This operation is currently only available for customers who already have an active Managed Database.__

Restore a backup to a Managed MySQL Database on your Account.

Requires `read_write` access to the Database.

The Database must have an `active`, `degraded`, or `failed` status to perform this operation.

__Note__. Restoring from a backup will erase all existing data on the database instance and replace it with backup data.

__Note__. Currently, restoring a backup after resetting Managed Database credentials results in a failed cluster. Please contact Customer Support if this occurs.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-backup-restore 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/mysql/instances/{instanceId}/backups/{backupId}/restore`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .backups()
    .restore()
    .create(linode_api::resources::databases::mysql::instances::backups::restore::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdRestoreApiVersionEnum::V4,
        instance_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### Reset managed MySQL database credentials
__This operation is currently only available for customers who already have an active Managed Database.__

Reset the root password for a Managed MySQL Database.

Requires `read_write` access to the Database.

A new root password is randomly generated and accessible with the [Get managed MySQL database credentials](https://techdocs.akamai.com/linode-api/reference/get-databases-mysql-instance-credentials) operation.

Only unrestricted Users can access this operation, and have access regardless of the acting token's OAuth scopes.

__Note__. It may take several seconds for credentials to reset.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-creds-reset 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/mysql/instances/{instanceId}/credentials/reset`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .credentials()
    .reset()
    .create(linode_api::resources::databases::mysql::instances::credentials::reset::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdCredentialsResetApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Patch a managed MySQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Apply security patches and updates to the underlying operating system of the Managed MySQL Database. This function runs during regular maintenance windows, which are configurable with the [Update a managed MySQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-mysql-instance) operation.

Requires `read_write` access to the Database.

The Database must have an `active` status to perform this operation.

__Note__

- If your database cluster is configured with a single node, you will experience downtime during this maintenance. Consider upgrading to a high availability plan to avoid any downtime due to maintenance.

- __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then [migrate your databases](https://www.linode.com/docs/products/databases/managed-databases/guides/migrate-mysql/) from the original Managed Database cluster to the new one.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-patch 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/mysql/instances/{instanceId}/patch`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .patch()
    .create(linode_api::resources::databases::mysql::instances::patch::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdPatchApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Create a managed PostgreSQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Provision a Managed PostgreSQL Database.

Restricted Users must have the `add_databases` grant to use this operation.

New instances can take approximately 15 to 30 minutes to provision.

The `allow_list` is used to control access to the Managed Database.

- IP addresses and ranges in this list can access the Managed Database. All other sources are blocked.

- If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.

- Entering an empty array (`[]`) blocks all connections (both public and private) to the Managed Database.

All Managed Databases include automatic, daily backups. Up to seven backups are automatically stored for each Managed Database, providing restore points for each day of the past week.

All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed PostgreSQL Database during configurable maintenance windows.

- If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.

- __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then migrate your databases from the original Managed Database cluster to the new one.

- To modify update the maintenance window for a Database, run the [Update a managed PostgreSQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-postgre-sql-instance) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-create \
  --label example-db \
  --region us-east \
  --type g6-dedicated-2 \
  --cluster_size 3 \
  --engine postgresql/13.2 \
  --encrypted false \
  --ssl_connection false \
  --replication_type asynch \
  --replication_commit_type local \
  --allow_list 203.0.113.1 \
  --allow_list 192.0.1.0/24
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/postgresql/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .create(linode_api::resources::databases::postgresql::instances::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesBody {
            allow_list: Some(
                vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
            ),
            cluster_size: Some(3),
            encrypted: Some(false),
            engine: "postgresql/13.2".to_string(),
            label: "example-db".to_string(),
            region: "us-east".to_string(),
            replication_commit_type: Some(
                linode_api::models::PostApiVersionDatabasesPostgresqlInstancesBodyReplicationCommitTypeEnum::Local,
            ),
            replication_type: Some(
                linode_api::models::PostApiVersionDatabasesPostgresqlInstancesBodyReplicationTypeEnum::Asynch,
            ),
            ssl_connection: Some(true),
            type_field: "g6-dedicated-2".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a managed PostgreSQL database backup snapshot
__This operation is currently only available for customers who already have an active Managed Database.__

Creates a snapshot backup of a Managed PostgreSQL Database.

Requires `read_write` access to the Database.

Up to 3 snapshot backups for each Database can be stored at a time. If 3 snapshots have been created for a Database, one must be deleted before another can be made.

Backups generated by this operation have the type `snapshot`. Snapshot backups may take several minutes to complete, after which they will be accessible to view or restore.

The Database must have an `active` status to perform this operation. If another backup is in progress, it must complete before a new backup can be initiated.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-backup-snapshot 123 \
  --label snapshot1 \
  --target primary
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/postgresql/instances/{instanceId}/backups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .backups()
    .create(linode_api::resources::databases::postgresql::instances::backups::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsApiVersionEnum::V4,
        instance_id: 123,
        data: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBody {
            label: "db-snapshot".to_string(),
            target: Some(
                linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBodyTargetEnum::Primary,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Restore a managed PostgreSQL database backup
__This operation is currently only available for customers who already have an active Managed Database.__

Restore a backup to a Managed PostgreSQL Database on your Account.

Requires `read_write` access to the Database.

The Database must have an `active`, `degraded`, or `failed` status to perform this operation.

__Note__. Restoring from a backup will erase all existing data on the database instance and replace it with backup data.

__Note__. Currently, restoring a backup after resetting Managed Database credentials results in a failed cluster. Please contact Customer Support if this occurs.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-backup-restore 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/postgresql/instances/{instanceId}/backups/{backupId}/restore`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .backups()
    .restore()
    .create(linode_api::resources::databases::postgresql::instances::backups::restore::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdRestoreApiVersionEnum::V4,
        instance_id: 123,
        backup_id: 123,
    })
    .await;
```

    
### Reset managed PostgreSQL database credentials
__This operation is currently only available for customers who already have an active Managed Database.__

Reset the root password for a Managed PostgreSQL Database.

Requires `read_write` access to the Database.

A new root password is randomly generated and accessible with the [Get managed PostgreSQL database credentials](https://techdocs.akamai.com/linode-api/reference/get-databases-postgre-sql-instance-credentials) operation.

Only unrestricted Users can access this operation, and have access regardless of the acting token's OAuth scopes.

__Note__. It may take several seconds for credentials to reset.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-creds-reset 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/postgresql/instances/{instanceId}/credentials/reset`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .credentials()
    .reset()
    .create(linode_api::resources::databases::postgresql::instances::credentials::reset::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdCredentialsResetApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Patch a managed PostgreSQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Apply security patches and updates to the underlying operating system of the Managed PostgreSQL Database. This function runs during regular maintenance windows, which are configurable with the [Update a managed PostgreSQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-postgre-sql-instance) operation.

Requires `read_write` access to the Database.

The Database must have an `active` status to perform this operation.

__Note__

- If your database cluster is configured with a single node, you will experience downtime during this maintenance. Consider upgrading to a high availability plan to avoid any downtime due to maintenance.

- __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then migrate your databases from the original Managed Database cluster to the new one.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-patch 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/databases/postgresql/instances/{instanceId}/patch`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .patch()
    .create(linode_api::resources::databases::postgresql::instances::patch::CreateRequest {
        api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdPatchApiVersionEnum::V4,
        instance_id: 123,
    })
    .await;
```

    
### Create a domain
Adds a new Domain to Linode's DNS Manager. Linode is not a registrar, and you must own the domain before adding it here. Be sure to point your registrar to Linode's nameservers so that the records hosted here are used.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains create \
  --type master \
  --domain example.org \
  --soa_email admin@example.org
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/domains`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .create(linode_api::resources::domains::CreateRequest {
        api_version: linode_api::models::PostApiVersionDomainsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionDomainsBody {
            axfr_ips: Some(vec!["string".to_string()]),
            description: Some("string".to_string()),
            domain: Some("example.org".to_string()),
            expire_sec: Some(300),
            group: Some("string".to_string()),
            id: Some(1234),
            master_ips: Some(vec!["string".to_string()]),
            refresh_sec: Some(300),
            retry_sec: Some(300),
            soa_email: Some("admin@example.org".to_string()),
            status: Some(
                linode_api::models::PostApiVersionDomainsBodyStatusEnum::Active,
            ),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            ttl_sec: Some(300),
            type_field: Some(
                linode_api::models::PostApiVersionDomainsBodyTypeEnum::Master,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Import a domain
Imports a domain zone from a remote nameserver. Your nameserver must allow zone transfers (AXFR) from the following IPs:

- 96.126.114.97
- 96.126.114.98
- 2600:3c00::5e
- 2600:3c00::5f


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains import --domain example.com --remote_nameserver examplenameserver.com
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/domains/import`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .import_resource()
    .create(linode_api::resources::domains::import_resource::CreateRequest {
        api_version: linode_api::models::PostApiVersionDomainsImportApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionDomainsImportBody {
            domain: "example.com".to_string(),
            remote_nameserver: "examplenameserver.com".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Clone a domain
Clones a Domain and all associated DNS records from a Domain that is registered in Linode's DNS manager.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains clone 123 --domain example.com
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/domains/{domainId}/clone`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .clone()
    .create(linode_api::resources::domains::clone::CreateRequest {
        api_version: linode_api::models::PostApiVersionDomainsDomainIdCloneApiVersionEnum::V4,
        domain_id: "string".to_string(),
        data: linode_api::models::PostApiVersionDomainsDomainIdCloneBody {
            domain: "example.org".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a domain record
Adds a new Domain Record to the zonefile this Domain represents.

Each domain can have up to 12,000 active records.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains records-create 123 \
  --type A \
  --name test \
  --target 203.0.113.1 \
  --priority 50 \
  --weight 50 \
  --port 80 \
  --ttl_sec 604800
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/domains/{domainId}/records`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .records()
    .create(linode_api::resources::domains::records::CreateRequest {
        api_version: linode_api::models::PostApiVersionDomainsDomainIdRecordsApiVersionEnum::V4,
        domain_id: 123,
        data: linode_api::models::PostApiVersionDomainsDomainIdRecordsBody {
            created: Some("2018-01-01T00:01:01".to_string()),
            id: Some(123456),
            name: Some("test".to_string()),
            port: Some(80),
            priority: Some(50),
            protocol: linode_api::Patch::new("string".to_string()),
            service: linode_api::Patch::new("string".to_string()),
            tag: linode_api::Patch::new(
                linode_api::models::PostApiVersionDomainsDomainIdRecordsBodyTagEnum::Iodef,
            ),
            target: Some("192.0.2.0".to_string()),
            ttl_sec: Some(604800),
            type_field: Some(
                linode_api::models::PostApiVersionDomainsDomainIdRecordsBodyTypeEnum::A,
            ),
            updated: Some("2018-01-01T00:01:01".to_string()),
            weight: Some(50),
            ..Default::default()
        },
    })
    .await;
```

    
### Create an image
Captures a private, gold-master image from a Linode disk.

> 📘
>
> - Captured images are stored using our Object Storage service. The `region` where the target image exists determines where the [resulting image is stored](https://techdocs.akamai.com/cloud-computing/docs/images#regions-and-captured-custom-images).
>
> - If you create an image from an encrypted disk, the API doesn't encrypt the image. When you rebuild a compute instance from that image, the resulting disk will be automatically encrypted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli images create \
  --label this_is_a_label \
  --description "A longer description \
    of the image" \
  --disk_id 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_write
linodes:read_only
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/images`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .create(linode_api::resources::images::CreateRequest {
        api_version: linode_api::models::PostApiVersionImagesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionImagesBody {
            cloud_init: Some(true),
            description: Some("string".to_string()),
            disk_id: 42,
            label: Some("string".to_string()),
            tags: Some(vec!["repair-image".to_string(), "fix-1".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Upload an image
Creates a new private image container and returns a URL as the `upload_to` object in the response. Use this URL to upload your own disk image to the container.

1. Ensure the disk image is raw disk image (`.img`) format.

2. Compress the disk image using gzip (`.gz`) format. Compressed, the file can be up to 5 GB and decompressed it can be up to 6 GB.

3. Upload the file in a separate PUT request that includes the `Content-type: application/octet-stream` header:

  ```
  curl -v \
    -H "Content-Type: application/octet-stream" \
    --upload-file example.img.gz \
    $UPLOAD_URL \
    --progress-bar \
    --output /dev/null
  ```

> 📘
>
> - You need to upload image data within 24 hours of creation or the API cancels the upload and deletes the image container.
>
> - Only core regions that support our [Object Storage](https://techdocs.akamai.com/cloud-computing/reference/how-to-choose-a-data-center#product-availability) service can store an uploaded image.
>
> - If you create an image from an encrypted disk, the API doesn't encrypt the image. When you rebuild a compute instance from that image, the resulting disk will be automatically encrypted.
>
> - You can create a new image and upload image data using a single process through [Cloud Manager](https://www.linode.com/docs/products/tools/images/guides/upload-an-image/#uploading-an-image-file-through-the-cloud-manager) or the [Linode CLI](https://www.linode.com/docs/products/tools/images/guides/upload-an-image/#uploading-an-image-file-through-the-linode-cli).


<<LB>>

---


- __CLI__.

    ```
    # Run the operation to just get the upload_to URL
linode-cli images upload \
  --description "Optional details about the Image" \
  --label "Example Image" \
  --region us-east

# Upload the image file in a single step
linode-cli image-upload \
  --description "Optional details about the Image" \
  --label "Example Image" \
  --region us-east \
  /path/to/image-file.img.gz
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/images/upload`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .upload()
    .create(linode_api::resources::images::upload::CreateRequest {
        api_version: linode_api::models::PostApiVersionImagesUploadApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionImagesUploadBody {
            cloud_init: Some(true),
            description: Some("This is an example image in the docs.".to_string()),
            label: "my-image-label".to_string(),
            region: "eu-central".to_string(),
            tags: Some(vec!["repair-image".to_string(), "fix-1".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Replicate an image
__Limited availability__ Target an existing image and replicate it to another compute region.

- This operation is in Limited Availability. Talk to your account team about access to it.

- This is only available in a `region` that supports Object Storage, which stores the replicated image. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to review a region's `capabilities`.

- To replicate an image, it needs to have a `status` of `available`. Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation to view an image's `status`.

- To also keep the target image in its original compute region, you need to include that `region` in the request's data. If you leave it out, the API removes the image from that region. Run the [Get an image](https://techdocs.akamai.com/linode-api/reference/get-image) operation to see the `regions` where an image currently exists.

- You can't include an empty array to delete all images. You need to provide at least one compute `region` where the image is `available`. Use the [Delete an image](https://techdocs.akamai.com/linode-api/reference/delete-image) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli images replicate private/12345 \
  --regions "us-mia" \
  --regions "us-east"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/images/{imageId}/regions`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .regions()
    .create(linode_api::resources::images::regions::CreateRequest {
        api_version: linode_api::models::PostApiVersionImagesImageIdRegionsApiVersionEnum::V4,
        image_id: "linode/debian11".to_string(),
        data: linode_api::models::PostApiVersionImagesImageIdRegionsBody {
            regions: Some(vec!["us-iad".to_string(), "us-west".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a Linode
Creates a Linode Instance on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Creating a new Linode will incur a charge on your Account.

Linodes can be created using one of the available Types. Run [List Linode types](https://techdocs.akamai.com/linode-api/reference/get-linode-types) to get more information about each Type's specs and cost.

Linodes can be created in any one of our available Regions, which are accessible from the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation.

In an effort to fight spam, Linode restricts outbound connections on ports 25, 465, and 587 on all Linodes for new accounts created after November 5th, 2019. For more information, see our guide on [Running a Mail Server](https://www.linode.com/docs/guides/running-a-mail-server/).

__Important__. You must be an unrestricted User in order to add or modify tags on Linodes.

Linodes can be created in a number of ways:

- Using a Linode Public Image distribution or a Private Image you created based on another Linode.

  - Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation with authentication to view all available Images.

  - The Linode will be `running` after it completes `provisioning`.
  - A default config with two Disks, one being a 512 swap disk, is created.
    - `swap_size` can be used to customize the swap disk size.
  - Requires a `root_pass` be supplied to use for the root User's Account.
  - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.
  - You may also supply a list of usernames via the `authorized_users` field.
    - These users must have an SSH Key associated with your Profile first. See the [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key)) operation for more information.

- Using cloud-init with [Metadata](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata/).
  - Automate system configuration and software installation by providing a base-64 encoded [cloud-config](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata-cloud-config/) file.
  - Requires a compatible Image. You can determine compatible Images by checking for `cloud-init` under `capabilities` when running [List images](https://techdocs.akamai.com/linode-api/reference/get-images).
  - Requires a compatible Region.  You can determine compatible Regions by checking for `Metadata` under `capabilities` when running [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions).

- Using a StackScript.

  - Run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts) for a list of available StackScripts.
  - The Linode will be `running` after it completes `provisioning`.
  - Requires a compatible Image to be supplied.
    - Run [Get a StackScript](https://techdocs.akamai.com/linode-api/reference/get-stack-script) for compatible Images.
  - Requires a `root_pass` be supplied to use for the root User's Account.
  - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.
  - You may also supply a list of usernames via the `authorized_users` field.
    - These users must have an SSH Key associated with your Profile first. See [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) for more information.

- Using one of your other Linode's backups.
  - You must create a Linode large enough to accommodate the Backup's size.
  - The Disks and Config will match that of the Linode that was backed up.
  - The `root_pass` will match that of the Linode that was backed up.

- Attached to a private VLAN.
  - Review the `interfaces` property of the [Request Body Schema](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) for details.
  - For more information, see our guide on [Getting Started with VLANs](https://www.linode.com/docs/products/networking/vlans/get-started/).

- Create an empty Linode.
  - The Linode will remain `offline` and must be manually started.
    - Run [Boot a Linode](https://techdocs.akamai.com/linode-api/reference/post-boot-linode-instance).
  - Disks and Configs must be created manually.
  - This is only recommended for advanced use cases.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes create \
  --label linode123 \
  --root_pass aComplex@Password \
  --booted true \
  --stackscript_id 10079 \
  --stackscript_data '{"gh_username": "linode"}' \
  --region us-east \
  --disk_encryption enabled\
  --placement_group.id 528 \
  --type g6-standard-2 \
  --authorized_keys "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer" \
  --authorized_users "myUser" \
  --authorized_users "secondaryUser" \
  --metadata.user_data "I2Nsb3VkLWNvbmZpZw==" \
  --firewall_id 9000
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .create(linode_api::resources::linode::instances::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesApiVersionEnum::V4,
        data: "could be anything",
    })
    .await;
```

    
### Create a snapshot
Creates a snapshot backup of a Linode.

__Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.

__Important__. If you already have a snapshot of this Linode, this is a destructive action. The previous snapshot will be deleted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes snapshot 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/backups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .backups()
    .create(linode_api::resources::linode::instances::backups::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsBody {
            label: "SnapshotLabel".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Cancel backups
Cancels the Backup service on the given Linode. Deletes all of this Linode's existing backups forever.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes backups-cancel 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/backups/cancel`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .backups()
    .cancel()
    .create(linode_api::resources::linode::instances::backups::cancel::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsCancelApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Enable backups
Enables backups for the specified Linode.

__Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes backups-enable 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/backups/enable`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .backups()
    .enable()
    .create(linode_api::resources::linode::instances::backups::enable::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsEnableApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Restore a backup
Restores a Linode's Backup to the specified Linode.

The following conditions apply:

  - Backups may not be restored across Regions.
  - Only successfully completed Backups that are not undergoing maintenance can be restored.
  - The Linode that the Backup is being restored to must not itself be in the process of creating a Backup.

__Note__. When you restore a backup, the restored disk is assigned the same [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) as the original disk. In most cases, this is acceptable and does not cause issues. However, if you attempt to mount both the original disk and the corresponding restore disk at the same time (by assigning them both to devices in your Configuration Profile's __Block Device Assignment__), you will encounter a UUID "collision".

When this happens, the system selects, and mounts, only one of the disks at random. This is due to both disks sharing the same UUID, and your instance _may fail to boot_ since it will not be clear which disk is root. If your system does boot, you will not see any immediate indication if you are booted into the restored disk or the original disk, and you will be unable to access both disks at the same time.

To avoid this, we recommend only restoring a backup to the same Compute Instance if you do not intend on mounting them at the same time or are comfortable modifying UUIDs. If you need access to files on both the original disk and the restored disk simultaneously (such as needing to copy files between them), we suggest either restoring the backup to a separate Compute Instance or [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) a new Compute Instance with the desired `backup_id`.

To learn more about block device assignments and viewing your disks' UUIDs, see our guide on [Configuration Profiles](https://www.linode.com/docs/products/compute/compute-instances/guides/configuration-profiles/#block-device-assignment).

__Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes backup-restore 123 123456 \
  --linode_id 234 \
  --overwrite true
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/backups/{backupId}/restore`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .backups()
    .restore()
    .create(linode_api::resources::linode::instances::backups::restore::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsBackupIdRestoreApiVersionEnum::V4,
        linode_id: 123,
        backup_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsBackupIdRestoreBody {
            linode_id: 234,
            overwrite: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Boot a Linode
Boots a Linode you have permission to modify. If no parameters are given, a Config profile will be chosen for this boot based on the following criteria:

- If there is only one Config profile for this Linode, it will be used.
- If there is more than one Config profile, the last booted config will be used.
- If there is more than one Config profile and none were the last to be booted (because the Linode was never booted or the last booted config was deleted) an error will be returned.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes boot 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/boot`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .boot()
    .create(linode_api::resources::linode::instances::boot::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBootApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBootBody {
            config_id: Some(123),
            ..Default::default()
        },
    })
    .await;
```

    
### Clone a Linode
You can clone your Linode's existing Disks or Configuration profiles to another Linode on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Cloning to a new Linode will incur a charge on your Account.

If cloning to an existing Linode, any actions currently running or queued must be completed first before you can clone to it.

Up to five clone operations from any given source Linode can be run concurrently. If more concurrent clones are attempted, an HTTP 400 error will be returned by this operation.

Any [tags](https://techdocs.akamai.com/linode-api/reference/get-tags) existing on the source Linode will be cloned to the target Linode.

Linodes utilizing Metadata (`"has_user_data": true`) must be cloned to a new Linode with `metadata.user_data` included with the clone request.

`vpc` details

- If the Linode you are cloning has a `vpc` purpose Interface on its active Configuration Profile that includes a 1:1 NAT, the resulting clone is configured with an `any` 1:1 NAT.
- See the [VPC documentation](https://www.linode.com/docs/products/networking/vpc/#technical-specifications) guide for its specifications and limitations.

`vlan` details

- Only Next Generation Network (NGN) data centers support VLANs. If a VLAN is attached to your Linode and you attempt clone it to a non-NGN data center, the cloning will not initiate. If a Linode cannot be cloned because of an incompatibility, you will be prompted to select a different data center or contact support.
- See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes clone 123 \
  --linode_id 124 \
  --region us-east \
  --type g6-standard-2 \
  --label cloned-linode \
  --backups_enabled true \
  --placement_group.id 528 \
  --disks 25674 \
  --configs 23456 \
  --private_ip true \
  --metadata.user_data I2Nsb3VkLWNvbmZpZw==
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/clone`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .clone()
    .create(linode_api::resources::linode::instances::clone::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneBody {
            backups_enabled: Some(true),
            configs: Some(vec![23456]),
            disks: Some(vec![25674]),
            group: Some("Linode-Group".to_string()),
            label: Some("cloned-linode".to_string()),
            linode_id: Some(124),
            metadata: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyMetadata {
                user_data: Some(
                    "I2Nsb3VkLWNvbmZpZwpwYWNrYWdlX3VwZGF0ZTogdHJ1ZQpwYWNrYWdlX3VwZ3JhZGU6IHRydWU="
                        .to_string(),
                ),
                ..Default::default()
            }),
            placement_group: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyPlacementGroup {
                id: 528,
                ..Default::default()
            }),
            private_ip: Some(true),
            region: Some("us-east".to_string()),
            type_field: Some("g6-standard-2".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a config profile
Adds a new Configuration profile to a Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-create 7590910 \
  --label "My Config" \
  --devices.sda.disk_id 123456 \
  --devices.sdb.disk_id 123457
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/configs`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .create(linode_api::resources::linode::instances::configs::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBody {
            comments: linode_api::Patch::new("This is my main Config".to_string()),
            devices: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevices {
                sda: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSda {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdb: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdb {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdc: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdc {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdd: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdd {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sde: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSde {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdf: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdf {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdg: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdg {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdh: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdh {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            helpers: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyHelpers {
                devtmpfs_automount: Some(false),
                distro: Some(true),
                modules_dep: Some(true),
                network: Some(true),
                updatedb_disabled: Some(true),
                ..Default::default()
            }),
            id: Some(23456),
            interfaces: Some(
                vec![
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItem
                    { id : Some(101), ipam_address : linode_api::Patch::new(None),
                    ipv4 :
                    Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemIpv4
                    { ..Default::default() }), label : linode_api::Patch::new(None),
                    primary : Some(false), purpose :
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Public,
                    subnet_id : linode_api::Patch::new(None), vpc_id :
                    linode_api::Patch::new(None), ..Default::default() },
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItem
                    { id : Some(102), ipam_address :
                    linode_api::Patch::new("10.0.0.1/24".to_string()), ipv4 :
                    Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemIpv4
                    { nat_1_1 : linode_api::Patch::new(None), vpc :
                    linode_api::Patch::new("10.0.0.2".to_string()),
                    ..Default::default() }), label : linode_api::Patch::new("vlan-1"
                    .to_string()), primary : Some(false), purpose :
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Vlan,
                    subnet_id : linode_api::Patch::new(None), vpc_id :
                    linode_api::Patch::new(None), ..Default::default() },
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItem
                    { id : Some(103), ipam_address : linode_api::Patch::new(None),
                    ipv4 :
                    Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemIpv4
                    { nat_1_1 : linode_api::Patch::new("203.0.113.2".to_string()),
                    vpc : linode_api::Patch::new("10.0.1.2".to_string()),
                    ..Default::default() }), label : linode_api::Patch::new(None),
                    primary : Some(true), purpose :
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Vpc,
                    subnet_id : linode_api::Patch::new(101), vpc_id :
                    linode_api::Patch::new(111), ..Default::default() }
                ],
            ),
            kernel: Some("linode/latest-64bit".to_string()),
            label: Some("My Config".to_string()),
            memory_limit: Some(2048),
            root_device: Some("/dev/sda".to_string()),
            run_level: Some(
                linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyRunLevelEnum::Default,
            ),
            virt_mode: Some(
                linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyVirtModeEnum::Paravirt,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Add a configuration profile interface
Creates and appends a single Interface to the end of the `interfaces` array for an existing Configuration Profile.

- The User accessing this operation must have `read_write` grants to the Linode.
- A successful request triggers a `linode_config_update` event.
- If the new Interface is added with `"primary": true`, then any existing primary Interface is changed to `"primary": false`.

Reboot the Linode with this Configuration Profile to activate an Interface that was added with this operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-interface-add $linodeId $configId \
  --purpose vpc \
  --primary false \
  --subnet_id 101 \
  --ipv4.vpc "10.0.1.2" \
  --ipv4.nat_1_1 "203.0.113.2"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .interfaces()
    .create(linode_api::resources::linode::instances::configs::interfaces::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
        data: "could be anything",
    })
    .await;
```

    
### Reorder configuration profile interfaces
Reorders the existing Interfaces of a Configuration Profile.

- The User accessing this operation must have `read_write` grants to the Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-interfaces-order $linodeId $configId \
  --ids 101 --ids 102 --ids 103
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/order`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .interfaces()
    .order()
    .create(linode_api::resources::linode::instances::configs::interfaces::order::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesOrderApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesOrderBody {
            ids: vec![101],
            ..Default::default()
        },
    })
    .await;
```

    
### Create a disk
Add a new disk to an existing Linode. You can create an empty disk to manually configure it later. You can also target a stored `image` to build the disk using a pre-configured file system.

- A Linode can have up to 50 disks.

- When creating an empty disk, you need to provide a `label` for it. If you don't include a `label`, you need to target an `image` instead.

- When you create a disk from an `image`, you need to set a `root_pass` for the disk.

- The default file system for a new disk is `ext4`. If you're creating one from an `image`, the disk inherits the file system of that `image`, is unless you specify otherwise.

- When you deploy a StackScript on a disk:

  - You can run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts) to review available StackScripts.

  - You need to include a compatible `image` when creating the disk. Run [Get a StackScript](https://techdocs.akamai.com/linode-api/reference/get-stack-script) to review compatible images.

  - You should supply SSH keys for the disk's root user, using the `authorized_keys` field.

  - You can include individual users via the `authorized_users` field. Before you can add a user, it needs an SSH key assigned to its profile. See [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) for more information.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-create 123 \
  --size 1300 \
  --authorized_keys "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer" \
  --authorized_users "myUser" \
  --authorized_users "secondaryUser" \
  --root_pass aComplex@Password \
  --image "linode/debian9" \
  --stackscript_id 10079 \
  --stackscript_data '{"gh_username": "linode"}'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/disks`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .create(linode_api::resources::linode::instances::disks::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksBody {
            authorized_keys: Some(
                vec![
                    "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer"
                    .to_string()
                ],
            ),
            authorized_users: Some(
                vec!["myUser".to_string(), "secondaryUser".to_string()],
            ),
            filesystem: Some(
                linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksBodyFilesystemEnum::Ext4,
            ),
            image: Some("linode/debian9".to_string()),
            label: Some("Debian 9 Disk".to_string()),
            root_pass: Some("aComplexP@ssword".to_string()),
            size: Some(48640),
            stackscript_data: Some(serde_json::json!({ "gh_username" : "linode" })),
            stackscript_id: Some(10079),
            ..Default::default()
        },
    })
    .await;
```

    
### Clone a disk
Copies a disk, byte-for-byte, into a new disk on the same Linode. The operation fails if the target doesn't have enough storage space. A Linode can have up to 50 disks.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-clone
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}/clone`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .clone()
    .create(linode_api::resources::linode::instances::disks::clone::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneApiVersionEnum::V4,
        linode_id: 123,
        disk_id: 123,
    })
    .await;
```

    
### Reset a disk root password
Resets the password of a Disk you have permission to `read_write`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-reset-password \
  123 25674 \
  --password aComplex@Password
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}/password`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .password()
    .create(linode_api::resources::linode::instances::disks::password::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdPasswordApiVersionEnum::V4,
        linode_id: 123,
        disk_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdPasswordBody {
            password: "another@complex^Password123".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Resize a disk
Resizes a Disk you have permission to `read_write`.

The Disk must not be in use. If the Disk is in use, the request will succeed but the resize will ultimately fail. For a request to succeed, the Linode must be shut down prior to resizing the Disk, or the Disk must not be assigned to the Linode's active Configuration Profile.

If you are resizing the Disk to a smaller size, it cannot be made smaller than what is required by the total size of the files current on the Disk.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-resize 123 25674 \
  --size 2048
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}/resize`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .resize()
    .create(linode_api::resources::linode::instances::disks::resize::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdResizeApiVersionEnum::V4,
        linode_id: 123,
        disk_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdResizeBody {
            size: 2048,
            ..Default::default()
        },
    })
    .await;
```

    
### Apply a Linode's firewalls
Reapply assigned firewalls to a Linode in case they were not applied successfully.

The `firewall_apply` event indicates if the firewalls were applied.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes apply-firewalls 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/firewalls/apply`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .firewalls()
    .apply()
    .create(linode_api::resources::linode::instances::firewalls::apply::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdFirewallsApplyApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Allocate an IPv4 address
Allocates a public or private IPv4 address to a Linode. Public IP Addresses, after the one included with each Linode, incur an additional monthly charge. If you need an additional public IP Address you must request one - please [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket). You may not add more than one private IPv4 address to a single Linode.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes ip-add 123 \
  --type ipv4 \
  --public true
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/ips`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .ips()
    .create(linode_api::resources::linode::instances::ips::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdIpsApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdIpsBody {
            public: true,
            type_field: linode_api::models::PostApiVersionLinodeInstancesLinodeIdIpsBodyTypeEnum::Ipv4,
            ..Default::default()
        },
    })
    .await;
```

    
### Initiate a DC migration/pending host migration
Initiate a pending host migration that has been scheduled by Linode or initiate a cross data center (DC) migration.  A list of pending migrations, if any, can be accessed from [List notifications](https://techdocs.akamai.com/linode-api/reference/get-notifications). When the migration begins, your Linode will be shutdown if not already off. If the migration initiated the shutdown, it will reboot the Linode when completed.

To initiate a cross DC migration, you must pass a `region` parameter to the request body specifying the target data center region. You can view a list of all available regions and their feature capabilities from [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions). See our [Pricing Page](https://www.linode.com/pricing/) for Region-specific pricing, which applies after migration is complete. If your Linode has a DC migration already queued or you have initiated a previously scheduled migration, you will not be able to initiate a DC migration until it has completed.

`vpc` details

- Cross DC migrations are not allowed for Linodes that have a `vpc` purpose Configuration Profile Interface. Host migrations within the same DC are permitted.
- See the [VPC documentation](https://www.linode.com/docs/products/networking/vpc/#technical-specifications) guide for its specifications and limitations.

`vlan` details

- Only Next Generation Network (NGN) data centers support VLANs. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated or cloned because of an incompatibility, you will be prompted to select a different data center or contact support.
- Next Generation Network (NGN) data centers do not support IPv6 `/116` pools or IP Failover. If you have these features enabled on your Linode and attempt to migrate to an NGN data center, the migration will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support.
- See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes migrate 123 \
  --region us-central \
  --placement_group.id 528
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/migrate`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .migrate()
    .create(linode_api::resources::linode::instances::migrate::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdMigrateApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdMigrateBody {
            placement_group: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdMigrateBodyPlacementGroup {
                id: 528,
                ..Default::default()
            }),
            region: Some("us-east".to_string()),
            type_field: Some(
                linode_api::models::PostApiVersionLinodeInstancesLinodeIdMigrateBodyTypeEnum::Warm,
            ),
            upgrade: Some(false),
            ..Default::default()
        },
    })
    .await;
```

    
### Upgrade a Linode
Linodes created with now-deprecated Types are entitled to a free upgrade to the next generation. A mutating Linode will be allocated any new resources the upgraded Type provides, and will be subsequently restarted if it was currently running. If any actions are currently running or queued, those actions must be completed first before you can initiate a mutate.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes upgrade 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/mutate`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .mutate()
    .create(linode_api::resources::linode::instances::mutate::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdMutateApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdMutateBody {
            allow_auto_disk_resize: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Reset a Linode's root password
Resets the root password for this Linode.

- Your Linode must be [shut down](https://techdocs.akamai.com/linode-api/reference/post-shutdown-linode-instance) for a password reset to complete.
- If your Linode has more than one disk (not counting its swap disk), run the [Reset a disk root password](https://techdocs.akamai.com/linode-api/reference/post-reset-disk-password) operation to update a specific disk's root password.
- A `password_reset` event is generated when a root password reset is successful.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes linode-reset-password 123 a$eCure4assw0rd!43v51
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/password`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .password()
    .create(linode_api::resources::linode::instances::password::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdPasswordApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdPasswordBody {
            root_pass: "a$eCure4assw0rd!43v51".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Reboot a Linode
Reboots a Linode you have permission to modify. If any actions are currently running or queued, those actions must be completed first before you can initiate a reboot.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes reboot 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/reboot`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .reboot()
    .create(linode_api::resources::linode::instances::reboot::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRebootApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRebootBody {
            config_id: Some(123),
            ..Default::default()
        },
    })
    .await;
```

    
### Rebuild a Linode
Rebuilds a Linode you have the `read_write` permission to modify.

A rebuild will first shut down the Linode, delete all disks and configs on the Linode, and then deploy a new `image` to the Linode with the given attributes. Additionally:

  - Requires an `image` be supplied.
  - Requires a `root_pass` be supplied to use for the root User's Account.
  - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.
  - Linodes utilizing Metadata (`"has_user_data": true`) should include `metadata.user_data` in the rebuild request to continue using the service.

During a rebuild, you can `enable` or `disable` local disk encryption. If disk encryption is not included in the request, the previous `disk_encryption` value is used. Disk encryption cannot be disabled if the compute instance is attached to a LKE nodepool.

You also have the option to resize the Linode to a different plan by including the `type` parameter with your request. Note that resizing involves migrating the Linode to a new hardware host, while rebuilding without resizing maintains the same hardware host. Resizing also requires significantly more time for completion of this operation. The following additional conditions apply:

  - The Linode must not have a pending migration.
  - Your Account cannot have an outstanding balance.
  - The Linode must not have more disk allocation than the new Type allows.
    - In that situation, you must first delete or resize the disk to be smaller.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes rebuild 123 \
  --image "linode/debian9" \
  --root_pass aComplex@Password \
  --disk_encryption disabled \
  --authorized_keys "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer" \
  --authorized_users "myUsername" \
  --authorized_users "secondaryUsername" \
  --booted true \
  --stackscript_id 10079 \
  --stackscript_data '{"gh_username": "linode"}' \
  --type "g6-standard-2" \
  --metadata.userdata "I2Nsb3VkLWNvbmZpZw=="
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/rebuild`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .rebuild()
    .create(linode_api::resources::linode::instances::rebuild::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRebuildApiVersionEnum::V4,
        linode_id: 123,
        data: "could be anything",
    })
    .await;
```

    
### Boot a Linode into rescue mode
Rescue Mode is a safe environment for performing many system recovery and disk management tasks. Rescue Mode is based on the Finnix recovery distribution, a self-contained and bootable Linux distribution. You can also use Rescue Mode for tasks other than disaster recovery, such as formatting disks to use different filesystems, copying data between disks, and downloading files from a disk via SSH and SFTP.

- Note that `sdh` is reserved and unavailable during rescue.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes rescue 123 \
  --devices.sda.disk_id 124458
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/rescue`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .rescue()
    .create(linode_api::resources::linode::instances::rescue::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBody {
            devices: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevices {
                sda: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSda {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdb: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdb {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdc: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdc {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdd: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdd {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sde: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSde {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdf: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdf {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdg: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdg {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    })
    .await;
```

    
### Resize a Linode
Resizes a Linode you have the `read_write` permission to a different Type. If any actions are currently running or queued, those actions must be completed first before you can initiate a resize. Additionally, the following criteria must be met in order to resize a Linode:

  - The Linode must not have a pending migration.
  - Your Account cannot have an outstanding balance.
  - The Linode must not have more disk allocation than the new Type allows.
    - In that situation, you must first delete or resize the disk to be smaller.

You can also resize a Linode when using the [Rebuild a Linode](https://techdocs.akamai.com/linode-api/reference/post-rebuild-linode-instance) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes resize 123 \
  --type g6-standard-2
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/resize`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .resize()
    .create(linode_api::resources::linode::instances::resize::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdResizeApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdResizeBody {
            allow_auto_disk_resize: Some(true),
            migration_type: Some(
                linode_api::models::PostApiVersionLinodeInstancesLinodeIdResizeBodyMigrationTypeEnum::Warm,
            ),
            type_field: "g6-standard-2".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Shut down a Linode
Shuts down a Linode you have permission to modify. If any actions are currently running or queued, those actions must be completed first before you can initiate a shutdown.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes shutdown 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/instances/{linodeId}/shutdown`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .shutdown()
    .create(linode_api::resources::linode::instances::shutdown::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdShutdownApiVersionEnum::V4,
        linode_id: 123,
    })
    .await;
```

    
### Create a StackScript
Creates a StackScript in your Account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli stackscripts create \
  --label a-stackscript \
  --description "This StackScript install and configures MySQL" \
  --images "linode/debian9" \
  --images "linode/debian8" \
  --is_public true \
  --rev_note "Set up MySQL" \
  --script '#!/bin/bash'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    stackscripts:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/linode/stackscripts`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .stackscripts()
    .create(linode_api::resources::linode::stackscripts::CreateRequest {
        api_version: linode_api::models::PostApiVersionLinodeStackscriptsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionLinodeStackscriptsBody {
            ..Default::default()
        },
    })
    .await;
```

    
### Create a Kubernetes cluster
Creates a Kubernetes cluster. The Kubernetes cluster will be created asynchronously. You can use the events system to determine when the Kubernetes cluster is ready to use. Please note that it often takes 2-5 minutes before the [Kubernetes API endpoints](https://techdocs.akamai.com/linode-api/reference/get-lke-cluster-api-endpoints) and the [Kubeconfig file](https://techdocs.akamai.com/linode-api/reference/get-lke-cluster-kubeconfig) for the new cluster are ready.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-create \
  --label cluster12345 \
  --region us-central \
  --k8s_version 1.27 \
  --control_plane.high_availability true \
  --node_pools.type g6-standard-4 --node_pools.count 6 \
  --node_pools.type g6-standard-8 --node_pools.count 3 \
  --node_pools.autoscaler.enabled true \
  --node_pools.autoscaler.max 12 \
  --node_pools.autoscaler.min 3 \
  --tags ecomm
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/lke/clusters`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .create(linode_api::resources::lke::clusters::CreateRequest {
        api_version: linode_api::models::PostApiVersionLkeClustersApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionLkeClustersBody {
            control_plane: Some(linode_api::models::PostApiVersionLkeClustersBodyControlPlane {
                acl: Some(linode_api::models::PostApiVersionLkeClustersBodyControlPlaneAcl {
                    addresses: Some(linode_api::models::PostApiVersionLkeClustersBodyControlPlaneAclAddresses {
                        ipv4: Some(
                            vec!["203.0.113.1".to_string(), "192.0.2.0/24".to_string()],
                        ),
                        ipv6: Some(vec!["2001:db8:1234:abcd::/64".to_string()]),
                        ..Default::default()
                    }),
                    enabled: Some(true),
                    revision_id: Some("20240127r001".to_string()),
                    ..Default::default()
                }),
                high_availability: Some(true),
                ..Default::default()
            }),
            k8s_version: "1.27".to_string(),
            label: "lkecluster12345".to_string(),
            node_pools: vec![
                linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItem {
                autoscaler :
                Some(linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemAutoscaler
                { enabled : Some(true), max : Some(12), min : Some(3),
                ..Default::default() }), count : 6, disks :
                Some(vec![linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemDisksItem
                { size : Some(1024), type_field :
                Some(linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemDisksItemTypeEnum::Ext4),
                ..Default::default() }]), labels :
                Some(linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemLabels
                { key : Some("example.com/my-app".to_string()), value : Some("teams"
                .to_string()), ..Default::default() }), tags :
                Some(vec!["example tag".to_string(), "another example".to_string()]),
                taints :
                Some(vec![linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItem
                { effect :
                linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItemEffectEnum::NoSchedule,
                key : "example.com/my-app".to_string(), value : "teamA".to_string(),
                ..Default::default() },
                linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItem
                { effect :
                linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItemEffectEnum::NoExecute,
                key : "myapp.io/team".to_string(), value : "teamB".to_string(),
                ..Default::default() }]), type_field : "g6-standard-4".to_string(),
                ..Default::default() }
            ],
            region: "us-central".to_string(),
            tags: Some(vec!["ecomm".to_string(), "blogs".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Recycle a node
Recycles an individual Node in the designated Kubernetes Cluster. The Node will be deleted and replaced with a new Linode, which may take a few minutes. Replacement Nodes are installed with the latest available patch for the Cluster's Kubernetes Version.

__Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke node-recycle 12345 12345-6aa78910bc
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/lke/clusters/{clusterId}/nodes/{nodeId}/recycle`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .nodes()
    .recycle()
    .create(linode_api::resources::lke::clusters::nodes::recycle::CreateRequest {
        api_version: linode_api::models::PostApiVersionLkeClustersClusterIdNodesNodeIdRecycleApiVersionEnum::V4,
        cluster_id: 123,
        node_id: "string".to_string(),
    })
    .await;
```

    
### Create a node pool
Creates a new Node Pool for the designated Kubernetes cluster.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke pool-create 12345 \
  --type g6-standard-4 \
  --count 6 \
  --tags example-tag \
  --autoscaler.enabled true \
  --autoscaler.max 12 \
  --autoscaler.min 3 \
  --labels.key "example.com/my-app" \
  --labels.value "teams" \
  --taints.effect "NoSchedule" \
  --taints.key "example.com/my-app" \
  --taints.value "teamA"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/lke/clusters/{clusterId}/pools`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .pools()
    .create(linode_api::resources::lke::clusters::pools::CreateRequest {
        api_version: linode_api::models::PostApiVersionLkeClustersClusterIdPoolsApiVersionEnum::V4,
        cluster_id: 123,
        data: linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBody {
            autoscaler: Some(linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyAutoscaler {
                enabled: Some(true),
                max: Some(12),
                min: Some(3),
                ..Default::default()
            }),
            count: 6,
            disks: Some(
                vec![
                    linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyDisksItem
                    { size : Some(1024), type_field :
                    Some(linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyDisksItemTypeEnum::Ext4),
                    ..Default::default() }
                ],
            ),
            labels: Some(linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyLabels {
                key: Some("example.com/my-app".to_string()),
                value: Some("teams".to_string()),
                ..Default::default()
            }),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            taints: Some(
                vec![
                    linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItem
                    { effect :
                    linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItemEffectEnum::NoSchedule,
                    key : "example.com/my-app".to_string(), value : "teamA"
                    .to_string(), ..Default::default() },
                    linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItem
                    { effect :
                    linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItemEffectEnum::NoExecute,
                    key : "myapp.io/team".to_string(), value : "teamB".to_string(),
                    ..Default::default() }
                ],
            ),
            type_field: "g6-standard-4".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Recycle a node pool
Recycles a Node Pool for the designated Kubernetes Cluster. All Linodes within the Node Pool will be deleted and replaced with new Linodes on a rolling basis, which may take several minutes. Replacement Nodes are installed with the latest available patch for the Cluster's Kubernetes Version.

__Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke pool-recycle 12345 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}/recycle`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .pools()
    .recycle()
    .create(linode_api::resources::lke::clusters::pools::recycle::CreateRequest {
        api_version: linode_api::models::PostApiVersionLkeClustersClusterIdPoolsPoolIdRecycleApiVersionEnum::V4,
        cluster_id: 123,
        pool_id: 123,
    })
    .await;
```

    
### Recycle cluster nodes
Recycles all nodes in all pools of a designated Kubernetes Cluster. All Linodes within the Cluster will be deleted and replaced with new Linodes on a rolling basis, which may take several minutes. Replacement Nodes are installed with the latest available patch version for the Cluster's current Kubernetes minor release.

__Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-nodes-recycle 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/lke/clusters/{clusterId}/recycle`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .recycle()
    .create(linode_api::resources::lke::clusters::recycle::CreateRequest {
        api_version: linode_api::models::PostApiVersionLkeClustersClusterIdRecycleApiVersionEnum::V4,
        cluster_id: 123,
    })
    .await;
```

    
### Regenerate a Kubernetes cluster
Regenerate the Kubeconfig file and/or the service account token for a Cluster.

This is a helper operation that allows performing both the [Delete a Kubeconfig](https://techdocs.akamai.com/linode-api/reference/delete-lke-cluster-kubeconfig) and the [Delete a service token](https://techdocs.akamai.com/linode-api/reference/delete-lke-service-token) operations with a single request.

When using this operation, at least one of `kubeconfig` or `servicetoken` is required.

__Note__. When regenerating a service account token, the Cluster's control plane components and Linode CSI drivers are also restarted and configured with the new token. High Availability Clusters should not experience any disruption, while standard Clusters may experience brief control plane downtime while components are restarted.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke regenerate 12345 \
    --kubeconfig true \
    --servicetoken true
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/lke/clusters/{clusterId}/regenerate`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .regenerate()
    .create(linode_api::resources::lke::clusters::regenerate::CreateRequest {
        api_version: linode_api::models::PostApiVersionLkeClustersClusterIdRegenerateApiVersionEnum::V4,
        cluster_id: 123,
        data: linode_api::models::PostApiVersionLkeClustersClusterIdRegenerateBody {
            kubeconfig: Some(true),
            servicetoken: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a Longview client
Creates a Longview Client.  This Client will not begin monitoring the status of your server until you configure the Longview Client application on your Linode using the returning `install_code` and `api_key`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview create \
  --label client789
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/longview/clients`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .clients()
    .create(linode_api::resources::longview::clients::CreateRequest {
        api_version: linode_api::models::PostApiVersionLongviewClientsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionLongviewClientsBody {
            api_key: Some("BD1B4B54-D752-A76D-5A9BD8A17F39DB61".to_string()),
            apps: Some(linode_api::models::PostApiVersionLongviewClientsBodyApps {
                apache: Some(true),
                mysql: Some(true),
                nginx: Some(false),
                ..Default::default()
            }),
            created: Some("2018-01-01T00:01:01".to_string()),
            id: Some(789),
            install_code: Some("BD1B5605-BF5E-D385-BA07AD518BE7F321".to_string()),
            label: Some("client789".to_string()),
            updated: Some("2018-01-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a managed contact
Creates a Managed Contact.  A Managed Contact is someone Linode special forces can contact in the course of attempting to resolve an issue with a Managed Service.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed contact-create \
  --name "John Doe" \
  --email "john.doe@example.org" \
  --phone.primary "123-456-7890"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/contacts`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .contacts()
    .create(linode_api::resources::managed::contacts::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedContactsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionManagedContactsBody {
            email: Some("john.doe@example.org".to_string()),
            group: linode_api::Patch::new("on-call".to_string()),
            id: Some(567),
            name: Some("John Doe".to_string()),
            phone: Some(linode_api::models::PostApiVersionManagedContactsBodyPhone {
                primary: linode_api::Patch::new("123-456-7890".to_string()),
                secondary: linode_api::Patch::new("string".to_string()),
                ..Default::default()
            }),
            updated: Some("2018-01-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a managed credential
Creates a Managed Credential. A Managed Credential is stored securely to allow Linode special forces to access your Managed Services and resolve issues.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credential-create \
  --label prod-password-1 \
  --username johndoe \
  --password s3cur3P@ssw0rd
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/credentials`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .create(linode_api::resources::managed::credentials::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedCredentialsApiVersionEnum::V4,
        data: "could be anything",
    })
    .await;
```

    
### Delete a managed credential
Deletes a Managed Credential.  Linode special forces will no longer have access to this Credential when attempting to resolve issues.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credential-revoke 9991
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/credentials/{credentialId}/revoke`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .revoke()
    .create(linode_api::resources::managed::credentials::revoke::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedCredentialsCredentialIdRevokeApiVersionEnum::V4,
        credential_id: 123,
    })
    .await;
```

    
### Update a managed credential's username and password
Updates the username and password for a Managed Credential.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credential-update-username-password 9991 \
  --username johndoe \
  --password s3cur3P@ssw0rd
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/credentials/{credentialId}/update`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .update()
    .create(linode_api::resources::managed::credentials::update::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedCredentialsCredentialIdUpdateApiVersionEnum::V4,
        credential_id: 123,
        data: linode_api::models::PostApiVersionManagedCredentialsCredentialIdUpdateBody {
            password: "s3cur3P@ssw0rd".to_string(),
            username: Some("johndoe".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a managed service
Creates a Managed Service. Linode Managed will begin monitoring this service and reporting and attempting to resolve any Issues.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed service-create \
  --service_type url \
  --label prod-1 \
  --address "https://example.org" \
  --timeout 30 \
  --body "it worked" \
  --consultation_group on-call \
  --notes "The service name is \
    my-cool-application" \
  --credentials 9991
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/services`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .create(linode_api::resources::managed::services::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedServicesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionManagedServicesBody {
            address: Some("https://example.org".to_string()),
            body: linode_api::Patch::new("it worked".to_string()),
            consultation_group: Some("on-call".to_string()),
            created: Some("2018-01-01T00:01:01".to_string()),
            credentials: Some(vec![9991]),
            id: Some(9944),
            label: Some("prod-1".to_string()),
            notes: linode_api::Patch::new(
                "The service name is my-cool-application".to_string(),
            ),
            region: Some("string".to_string()),
            service_type: Some(
                linode_api::models::PostApiVersionManagedServicesBodyServiceTypeEnum::Url,
            ),
            status: Some(
                linode_api::models::PostApiVersionManagedServicesBodyStatusEnum::Ok,
            ),
            timeout: Some(30),
            updated: Some("2018-03-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Disable a managed service
Temporarily disables monitoring of a Managed Service.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed service-disable 9994
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/services/{serviceId}/disable`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .disable()
    .create(linode_api::resources::managed::services::disable::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedServicesServiceIdDisableApiVersionEnum::V4,
        service_id: 123,
    })
    .await;
```

    
### Enable a managed service
Enables monitoring of a Managed Service.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed service-enable 9994
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/managed/services/{serviceId}/enable`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .enable()
    .create(linode_api::resources::managed::services::enable::CreateRequest {
        api_version: linode_api::models::PostApiVersionManagedServicesServiceIdEnableApiVersionEnum::V4,
        service_id: 123,
    })
    .await;
```

    
### Create a firewall
Creates a Firewall to filter network traffic.

- Use `rules` to create inbound and outbound access rules. Rule versions increment from `1` whenever the firewall's `rules` change.

- Use `devices` to assign the firewall to a service and apply its rules to the device. Requires `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) to the device. Currently, firewalls can be assigned to Linode compute instances and NodeBalancers.

- A Firewall can be assigned to multiple services at a time.

- Use `firewall_id` to assign a firewall when [creating a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance).

- A service can have one assigned Firewall at a time.

- Firewalls apply to all of a Linode's non-`vlan` purpose Configuration Profile Interfaces.

- Assigned Linodes must not have any ongoing live migrations.

- A `firewall_create` Event is generated when this operation succeeds.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls create \
  --label example-firewall \
  --rules.outbound_policy ACCEPT \
  --rules.inbound_policy DROP \
  --rules.inbound '[{"protocol": "TCP", "ports": "22, 80, 8080, 443", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"], "ipv6": ["2001:DB8::/128"]}, "action": "ACCEPT"}]' \
  --rules.outbound '[{"protocol": "TCP", "ports": "49152-65535", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"],"ipv6": ["2001:DB8::/128"]}, "action": "DROP", "label": "outbound-rule123", "description": "An example outbound rule description."}]'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/firewalls`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .create(linode_api::resources::networking::firewalls::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingFirewallsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingFirewallsBody {
            created: Some("2018-01-01T00:01:01".to_string()),
            id: Some(123),
            label: Some("firewall123".to_string()),
            rules: Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRules {
                fingerprint: Some("997dd135".to_string()),
                inbound: Some(
                    vec![
                        linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItem
                        { action :
                        Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItemActionEnum::Accept),
                        addresses :
                        Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItemAddresses
                        { ipv4 : Some(vec!["192.0.2.0/24".to_string(),
                        "198.51.100.2/32".to_string()]), ipv6 :
                        Some(vec!["2001:DB8::/128".to_string()]),
                        ..Default::default() }), description :
                        Some("An example firewall rule description.".to_string()),
                        label : Some("firewallrule123".to_string()), ports :
                        linode_api::Patch::new("22-24, 80, 443".to_string()),
                        protocol :
                        Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItemProtocolEnum::Tcp),
                        ..Default::default() }
                    ],
                ),
                inbound_policy: Some(
                    linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundPolicyEnum::Drop,
                ),
                outbound: Some(
                    vec![
                        linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItem
                        { action :
                        Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItemActionEnum::Accept),
                        addresses :
                        Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItemAddresses
                        { ipv4 : Some(vec!["192.0.2.0/24".to_string(),
                        "198.51.100.2/32".to_string()]), ipv6 :
                        Some(vec!["2001:DB8::/128".to_string()]),
                        ..Default::default() }), description :
                        Some("An example firewall rule description.".to_string()),
                        label : Some("firewallrule123".to_string()), ports :
                        linode_api::Patch::new("22-24, 80, 443".to_string()),
                        protocol :
                        Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItemProtocolEnum::Tcp),
                        ..Default::default() }
                    ],
                ),
                outbound_policy: Some(
                    linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundPolicyEnum::Drop,
                ),
                version: Some(1),
                ..Default::default()
            }),
            status: Some(
                linode_api::models::PostApiVersionNetworkingFirewallsBodyStatusEnum::Enabled,
            ),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            updated: Some("2018-01-02T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a firewall device
Creates a Firewall Device, which assigns a Firewall to a service (referred to as the Device's `entity`) and applies the Firewall's Rules to the device.

- Currently, Devices with `linode` and `nodebalancer` entity types are accepted.

- Firewalls only apply to inbound TCP traffic to NodeBalancers.

- A Firewall can be assigned to multiple services at a time.

- A service can have one assigned Firewall at a time.

- Assigned Linodes must not have any ongoing live migrations.

- A `firewall_device_add` Event is generated when the Firewall Device is added successfully.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls device-create 123 \
  --id 456 \
  --type "linode"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/firewalls/{firewallId}/devices`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .devices()
    .create(linode_api::resources::networking::firewalls::devices::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesApiVersionEnum::V4,
        firewall_id: 123,
        data: linode_api::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesBody {
            id: Some(123),
            label: Some("my-linode".to_string()),
            type_field: Some(
                linode_api::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesBodyTypeEnum::Linode,
            ),
            url: Some("/v4/linode/instances/123".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Allocate an IP address
Allocates a new IPv4 Address on your Account. The Linode must be configured to support additional addresses - please [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) requesting additional addresses before attempting allocation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking ip-add \
  --type ipv4 \
  --public true \
  --linode_id 123
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/ips`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ips()
    .create(linode_api::resources::networking::ips::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingIpsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingIpsBody {
            linode_id: 123,
            public: true,
            type_field: linode_api::models::PostApiVersionNetworkingIpsBodyTypeEnum::Ipv4,
            ..Default::default()
        },
    })
    .await;
```

    
### Assign IP addresses
Assign multiple IPv4 addresses and/or IPv6 ranges to multiple Linodes in one Region. This allows swapping, shuffling, or otherwise reorganizing IPs to your Linodes.

The following restrictions apply:

- All Linodes involved must have at least one public IPv4 address after assignment.
- Linodes may have no more than one assigned private IPv4 address.
- Linodes may have no more than one assigned IPv6 range.
- Shared IP addresses cannot be swapped between Linodes.

[Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request additional IPv4 addresses or IPv6 ranges beyond standard account limits.

__Note__. Removing an IP address that has been set as a Managed Linode's `ssh.ip` causes the Managed Linode's SSH access settings to reset to their default values.

To view and configure Managed Linode SSH settings, use the following operations:

- [Get a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/get-managed-linode-setting)
- [Update a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/put-managed-linode-setting)

__Note__. Addresses with an active 1:1 NAT to a VPC Interface address cannot be assigned to other Linodes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking ip-assign \
  --region us-east \
  --assignments.address 192.0.2.1 \
  --assignments.linode_id 123 \
  --assignments.address 2001:db8:3c4d:15::/64 \
  --assignments.linode_id 234
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/ips/assign`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ips()
    .assign()
    .create(linode_api::resources::networking::ips::assign::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingIpsAssignApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingIpsAssignBody {
            assignments: vec![
                linode_api::models::PostApiVersionNetworkingIpsAssignBodyAssignmentsItem
                { address : "192.0.2.1".to_string(), linode_id : 123,
                ..Default::default() }
            ],
            region: "us-east".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Share IP addresses
Configure shared IPs.

IP sharing allows IP address reassignment (also referred to as IP failover) from one Linode to another if the primary Linode becomes unresponsive. This means that requests to the primary Linode's IP address can be automatically rerouted to secondary Linodes at the configured shared IP addresses.

IP failover requires configuration of a [BGP based failover service](https://techdocs.akamai.com/cloud-computing/docs/configure-failover-on-a-compute-instance) within the internal system of the primary Linode.

__Note__. A public IPv4 address cannot be shared if it is configured for a 1:1 NAT on a `vpc` purpose Configuration Profile Interface.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking ip-share \
  --linode_id 123 \
  --ips 192.0.2.1 \
  --ips 2001:db8:3c4d:15::
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/ips/share`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ips()
    .share()
    .create(linode_api::resources::networking::ips::share::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingIpsShareApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingIpsShareBody {
            ips: vec!["192.0.2.1".to_string(), "2001:db8:3c4d:15::".to_string()],
            linode_id: 123,
            ..Default::default()
        },
    })
    .await;
```

    
### Assign IPv4s to Linodes
This operation is equivalent to [Assign IP addresses](https://techdocs.akamai.com/linode-api/reference/post-assign-ips).

Assign multiple IPv4 addresses and/or IPv6 ranges to multiple Linodes in one Region. This allows swapping, shuffling, or otherwise reorganizing IPs to your Linodes.

The following restrictions apply:

- All Linodes involved must have at least one public IPv4 address after assignment.
- Linodes may have no more than one assigned private IPv4 address.
- Linodes may have no more than one assigned IPv6 range.

[Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request additional IPv4 addresses or IPv6 ranges beyond standard account limits.

__Note__. Removing an IP address that has been set as a Managed Linode's `ssh.ip` causes the Managed Linode's SSH access settings to reset to their default values.

To view and configure Managed Linode SSH settings, use the following operations:
- [Get a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/get-managed-linode-setting)
- [Update a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/put-managed-linode-setting)


<<LB>>

---


- __OAuth scopes__.

    ```
    ips:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/ipv4/assign`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv4()
    .assign()
    .create(linode_api::resources::networking::ipv4::assign::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingIpv4AssignApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingIpv4AssignBody {
            assignments: vec![
                linode_api::models::PostApiVersionNetworkingIpv4AssignBodyAssignmentsItem
                { address : "192.0.2.1".to_string(), linode_id : 123,
                ..Default::default() }
            ],
            region: "us-east".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Configure IPv4 sharing
This operation is equivalent to [Share IP addresses](https://techdocs.akamai.com/linode-api/reference/post-share-ips).

Configure shared IPs.

IP sharing allows IP address reassignment (also referred to as IP failover) from one Linode to another if the primary Linode becomes unresponsive. This means that requests to the primary Linode's IP address can be automatically rerouted to secondary Linodes at the configured shared IP addresses.

IP failover requires configuration of a [BGP based failover service](https://techdocs.akamai.com/cloud-computing/docs/configure-failover-on-a-compute-instance) within the internal system of the primary Linode.


<<LB>>

---


- __OAuth scopes__.

    ```
    ips:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/ipv4/share`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv4()
    .share()
    .create(linode_api::resources::networking::ipv4::share::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingIpv4ShareApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingIpv4ShareBody {
            ips: vec!["192.0.2.1".to_string(), "2001:db8:3c4d:15::".to_string()],
            linode_id: 123,
            ..Default::default()
        },
    })
    .await;
```

    
### Create an IPv6 range
Creates an IPv6 Range and assigns it based on the provided Linode or route target IPv6 SLAAC address. See the `ipv6` property when running the [Get a Linode](https://techdocs.akamai.com/linode-api/reference/get-linode-instance) operation to view a Linode's IPv6 SLAAC address.

- Either `linode_id` or `route_target` is required in a request.
- `linode_id` and `route_target` are mutually exclusive. Submitting values for both properties in a request results in an error.
- Upon a successful request, an IPv6 range is created in the [region](https://techdocs.akamai.com/linode-api/reference/get-regions) that corresponds to the provided `linode_id` or `route_target`.
- Your Linode is responsible for routing individual addresses in the range, or handling traffic for all the addresses in the range.
- Run the [Assign IP addresses](https://techdocs.akamai.com/linode-api/reference/post-assign-ips) operation to re-assign IPv6 Ranges to your Linodes.

__Note__. The following restrictions apply:

  - A Linode can only have one IPv6 range targeting its SLAAC address.
  - An account can only have one IPv6 range in each [region](https://techdocs.akamai.com/linode-api/reference/get-regions).
  - [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request expansion of these restrictions.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking v6-range-create \
  --linode_id 123 \
  --prefix_length 64
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/networking/ipv6/ranges`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ipv6()
    .ranges()
    .create(linode_api::resources::networking::ipv6::ranges::CreateRequest {
        api_version: linode_api::models::PostApiVersionNetworkingIpv6RangesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNetworkingIpv6RangesBody {
            linode_id: Some(123),
            prefix_length: 123,
            route_target: Some("2001:0db8::1".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a NodeBalancer
Creates a NodeBalancer in the requested Region. Only available in [regions](https://techdocs.akamai.com/linode-api/reference/get-regions) with "NodeBalancers" in their `capabilities`.

NodeBalancers require a port Config with at least one backend Node to start serving requests.

When using the Linode CLI to create a NodeBalancer, first create a NodeBalancer without any Configs. Then, create Configs and Nodes for that NodeBalancer with the respective [Create a config](https://techdocs.akamai.com/linode-api/reference/post-node-balancer-config) and [Create a node](https://techdocs.akamai.com/linode-api/reference/post-node-balancer-node) operations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers create \
  --region us-east \
  --label balancer12345 \
  --client_conn_throttle 0
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/nodebalancers`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .create(linode_api::resources::nodebalancers::CreateRequest {
        api_version: linode_api::models::PostApiVersionNodebalancersApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionNodebalancersBody {
            client_conn_throttle: Some(0),
            configs: Some(
                vec![
                    linode_api::models::PostApiVersionNodebalancersBodyConfigsItem {
                    algorithm :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemAlgorithmEnum::Roundrobin),
                    check :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemCheckEnum::HttpBody),
                    check_attempts : Some(3), check_body : Some("it works"
                    .to_string()), check_interval : Some(90), check_passive :
                    Some(true), check_path : Some("/test".to_string()), check_timeout
                    : Some(10), cipher_suite :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemCipherSuiteEnum::Recommended),
                    nodes :
                    vec![linode_api::models::PostApiVersionNodebalancersBodyConfigsItemNodesItem
                    { address : Some("192.168.210.120:80".to_string()), config_id :
                    Some(4567), id : Some(54321), label : Some("node54321"
                    .to_string()), mode :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemNodesItemModeEnum::Accept),
                    nodebalancer_id : Some(12345), status :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum::Up),
                    weight : Some(50), ..Default::default() }], port : Some(80),
                    protocol :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemProtocolEnum::Http),
                    proxy_protocol :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemProxyProtocolEnum::None),
                    ssl_cert : linode_api::Patch::new("<REDACTED>".to_string()),
                    ssl_key : linode_api::Patch::new("<REDACTED>".to_string()),
                    stickiness :
                    Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemStickinessEnum::HttpCookie),
                    ..Default::default() }
                ],
            ),
            firewall_id: Some(123),
            label: Some("balancer12345".to_string()),
            region: "us-east".to_string(),
            tags: Some(vec!["test".to_string(), "web-dev-team".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a config
Creates a NodeBalancer Config, which allows the NodeBalancer to accept traffic on a new port. You will need to add NodeBalancer Nodes to the new Config before it can actually serve requests.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers config-create 12345 \
  --port 443 \
  --protocol https \
  --algorithm roundrobin \
  --stickiness http_cookie \
  --check http_body \
  --check_interval 90 \
  --check_timeout 10 \
  --check_attempts 3 \
  --check_path "/test" \
  --check_body "it works" \
  --check_passive true \
  --proxy_protocol "none" \
  --ssl_cert "-----BEGIN CERTIFICATE-----
              CERTIFICATE_INFORMATION
              -----END CERTIFICATE-----" \
  --ssl_key "-----BEGIN PRIVATE KEY-----
             PRIVATE_KEY_INFORMATION
             -----END PRIVATE KEY-----" \
  --cipher_suite recommended
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/nodebalancers/{nodeBalancerId}/configs`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .create(linode_api::resources::nodebalancers::configs::CreateRequest {
        api_version: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsApiVersionEnum::V4,
        node_balancer_id: 123,
        data: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBody {
            algorithm: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyAlgorithmEnum::Roundrobin,
            ),
            check: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum::HttpBody,
            ),
            check_attempts: Some(3),
            check_body: Some("it works".to_string()),
            check_interval: Some(90),
            check_passive: Some(true),
            check_path: Some("/test".to_string()),
            check_timeout: Some(10),
            cipher_suite: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCipherSuiteEnum::Recommended,
            ),
            id: Some(4567),
            nodebalancer_id: Some(12345),
            nodes_status: Some(linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyNodesStatus {
                down: Some(0),
                up: Some(4),
                ..Default::default()
            }),
            port: Some(80),
            protocol: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyProtocolEnum::Http,
            ),
            proxy_protocol: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyProxyProtocolEnum::None,
            ),
            ssl_cert: linode_api::Patch::new("<REDACTED>".to_string()),
            ssl_commonname: Some("www.example.com".to_string()),
            ssl_fingerprint: Some(
                "00:01:02:03:04:05:06:07:08:09:0A:0B:0C:0D:0E:0F:10:11:12:13"
                    .to_string(),
            ),
            ssl_key: linode_api::Patch::new("<REDACTED>".to_string()),
            stickiness: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyStickinessEnum::HttpCookie,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a node
Creates a NodeBalancer Node, a backend that can accept traffic for this NodeBalancer Config. Nodes are routed requests on the configured port based on their status.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers node-create \
  12345 4567 \
  --address 192.168.210.120:80 \
  --label node54321 \
  --weight 50 \
  --mode accept
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .nodes()
    .create(linode_api::resources::nodebalancers::configs::nodes::CreateRequest {
        api_version: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        data: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesBody {
            address: Some("192.168.210.120:80".to_string()),
            config_id: Some(4567),
            id: Some(54321),
            label: Some("node54321".to_string()),
            mode: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesBodyModeEnum::Accept,
            ),
            nodebalancer_id: Some(12345),
            status: Some(
                linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesBodyStatusEnum::Up,
            ),
            weight: Some(50),
            ..Default::default()
        },
    })
    .await;
```

    
### Rebuild a config
Rebuilds a NodeBalancer Config and its Nodes that you have permission to modify.

Use this operation to update a NodeBalancer's Config and Nodes with a single request.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers config-rebuild \
  12345 4567 \
  --port 443 \
  --protocol https \
  --algorithm roundrobin \
  --stickiness http_cookie \
  --check http_body \
  --check_interval 90 \
  --check_timeout 10 \
  --check_attempts 3 \
  --check_path "/test" \
  --check_body "it works" \
  --check_passive true \
  --proxy_protocol "none" \
  --ssl_cert "-----BEGIN CERTIFICATE-----
              CERTIFICATE_INFORMATION
              -----END CERTIFICATE-----" \
  --ssl_key "-----BEGIN PRIVATE KEY-----
             PRIVATE_KEY_INFORMATION
             -----END PRIVATE KEY-----" \
  --cipher_suite recommended \
  --nodes '{"address":"192.168.210.120:80","label":"node1","weight":50,"mode":"accept"}' \
  --nodes '{"address":"192.168.210.122:80","label":"node2","weight":50,"mode":"accept"}'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/rebuild`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .rebuild()
    .create(linode_api::resources::nodebalancers::configs::rebuild::CreateRequest {
        api_version: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        data: "could be anything",
    })
    .await;
```

    
### Create an Object Storage bucket
Creates an Object Storage bucket in the specified data center ([region](https://techdocs.akamai.com/linode-api/reference/get-regions)). If the bucket already exists on your account, this operation returns a 200 response with that bucket as if the API just created it.

> 📘
>
> - Accounts with negative balances can't access this operation.
>
> - The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket) equivalent operation offers more detail.
>
> - The API still supports the `clusterId` equivalent (`us-west-1`) when setting a `region` for a new bucket, but you should use the `regionId` (`us-west`), instead.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/object-storage/buckets`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .create(linode_api::resources::object_storage::buckets::CreateRequest {
        api_version: linode_api::models::PostApiVersionObjectStorageBucketsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionObjectStorageBucketsBody {
            acl: Some(
                linode_api::models::PostApiVersionObjectStorageBucketsBodyAclEnum::Private,
            ),
            cors_enabled: Some(false),
            label: "example-bucket".to_string(),
            region: Some("us-east".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Modify access to an Object Storage bucket
Apply basic Cross-origin Resource Sharing (CORS) and Access Control Level (ACL) settings. You can configure CORS for all origins and set canned ACL settings.

> 📘
>
> For more fine-grained control of both systems, use the [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket-acl).


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/access`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .access()
    .create(linode_api::resources::object_storage::buckets::access::CreateRequest {
        api_version: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        data: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessBody {
            acl: Some(
                linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::Private,
            ),
            cors_enabled: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a URL for an object
Creates a pre-signed URL to access a single object in a bucket. Use it to share, create, or delete objects by using the appropriate HTTP method in your request body's `method` parameter.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-url`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .object_url()
    .create(linode_api::resources::object_storage::buckets::object_url::CreateRequest {
        api_version: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        data: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlBody {
            content_type: Some("string".to_string()),
            expires_in: Some(123),
            method: "GET".to_string(),
            name: "example".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Upload an Object Storage TLS/SSL certificate
Upload a TLS/SSL certificate and private key to be served when you visit your Object Storage bucket via HTTPS. Your TLS/SSL certificate and private key are stored encrypted at rest.

To replace an expired certificate, [delete your current certificates](https://techdocs.akamai.com/linode-api/reference/delete-object-storage-ssl) and upload a new one.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage ssl-upload \
  us-east-1 example-bucket \
  --certificate "-----BEGIN CERTIFICATE-----
                 CERTIFICATE_INFORMATION
                 -----END CERTIFICATE-----" \
  --private_key "-----BEGIN PRIVATE KEY-----
                 PRIVATE_KEY_INFORMATION
                 -----END PRIVATE KEY-----"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/ssl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .ssl_resource()
    .create(linode_api::resources::object_storage::buckets::ssl_resource::CreateRequest {
        api_version: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketSslApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        data: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketSslBody {
            certificate: "-----BEGIN CERTIFICATE-----\nCERTIFICATE_INFORMATION\n-----END CERTIFICATE-----"
                .to_string(),
            private_key: "-----BEGIN PRIVATE KEY-----\nPRIVATE_KEY_INFORMATION\n-----END PRIVATE KEY-----"
                .to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Cancel Object Storage
Cancel Object Storage on an Account.

__Warning__. This removes all buckets and their contents from your Account. This data is irretrievable once removed.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage cancel
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/object-storage/cancel`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .cancel()
    .create(linode_api::resources::object_storage::cancel::CreateRequest {
        api_version: linode_api::models::PostApiVersionObjectStorageCancelApiVersionEnum::V4,
    })
    .await;
```

    
### Create an Object Storage key
Provisions a new Object Storage key for authenticating to the Object Storage S3 API. A successful request triggers an `obj_access_key_create` [event](https://techdocs.akamai.com/linode-api/reference/get-events).

> 📘
>
> Accounts with negative balances can't access this operation.

**The `regions` and `region` parameters**

When creating an Object Storage key, specify one or more data centers ([regions](https://techdocs.akamai.com/linode-api/reference/get-regions)) where you want to create and manage Object Storage buckets.

- **The `regions` array**. Populate it with `regionId` values. The resulting Object Storage key grants access to list and create new buckets in these regions. This *doesn't* give access to manage content in these buckets. To address this, you can:

  - Use the `bucket_access` array instead to grant management access, per bucket.

  - Use [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/) to change the access for this key.

- **The `bucket_access` array**. This optional array lets you set up limited keys. Include individual objects naming a `regionId`, the target `bucket_name`, and the `permissions` for the Object Storage key. Use the resulting key to manage content in the `bucket_name`, based on the permission level set. You can also use the key to create new buckets in the named region. However, the key doesn't have access to manage content in the newly created bucket. You can grant it this access using [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/).

- **Combine the two to apply varying levels of access in the key**. For example, set `regions` to `us-west` to give the key bucket list and create access in that region. Then, set up the `bucket_access` array to give access to a specific `bucket_name` in the `us-east` region. The key has access to manage content in that `bucket_name` and list and create buckets in the `us-east` region, too. If you include the same region in both, the settings applied in the `bucket_access` array take precedence. For example, assume you include `us-east` in the `regions` array, expecting to only give bucket list and creation access to that region. If you also set `us-east` as a `region` in the `bucket_access` array, the Object Storage key gives access to manage content in the specified `bucket_name`, and lets you list and create buckets in that region.

**The `cluster` parameter (legacy)**

For backward compatibility, include the `cluster` parameter to create an Object Storage key. Use the `clusterId` equivalent (us-west-1) instead of the `regionId` (us-west). Leave the `regions` array out. If including the `bucket_access` array to limit access, omit `region` from each object. Use the resulting key in clusters in all supported regions.

> 📘
>
> While the API supports this method, you should use the `regions` parameters, instead.

- **Unlimited access**. Omit the `bucket_access` array. The Object Storage key has unlimited cluster access to all buckets, with all permissions.

- **Limited access**. Include the `bucket_access` array. Set the target `bucket_name` and the level of `permissions` for access to that bucket. Use the resulting key to manage content in the named bucket. A limited Object Storage key can [list all buckets](https://techdocs.akamai.com/linode-api/reference/get-object-storage-buckets) and [create a new bucket](https://techdocs.akamai.com/linode-api/reference/post-object-storage-bucket). However, you can't use the key to perform any actions on a bucket, unless the key has access to it. You can use [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/) to modify a key's access.


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage keys-create \
  --label "my-object-storage-key" \
  --bucket_access '[{"region": "ap-south", "bucket_name": "bucket-example-1", "permissions": "read_write" }]'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/object-storage/keys`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .keys()
    .create(linode_api::resources::object_storage::keys::CreateRequest {
        api_version: linode_api::models::PostApiVersionObjectStorageKeysApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionObjectStorageKeysBody {
            bucket_access: Some(
                vec![
                    linode_api::models::PostApiVersionObjectStorageKeysBodyBucketAccessItem
                    { bucket_name : Some("example-bucket".to_string()), permissions :
                    Some(linode_api::models::PostApiVersionObjectStorageKeysBodyBucketAccessItemPermissionsEnum::ReadWrite),
                    region : Some("us-iad".to_string()), ..Default::default() }
                ],
            ),
            label: Some("my-key".to_string()),
            regions: Some(vec!["us-iad".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Create placement group
Creates a placement group on your account. Placement groups disperse your compute instances across physical machines (hosts) in one of our compute regions. Depending on your workload requirements, you may need your compute instances to follow specific strategies:

- __Grouped together__. You may want them placed close together to reduce latency between compute instances that are used for an application or workload.
- __Spread apart__. You may want to disperse them across several hosts to support high availability, for example when required for failover.

<<LB>>

> 📘
>
> - For this request to complete successfully, your user needs to have the `add_placement` grant.
> - We offer an [example API workflow](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#create-a-placement-group) you can follow to create a placement group.


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement group-create \
  --label PG_Miami_failover \
  --region us-mia \
  --placement_group_type "anti-affinity:local" \
  --placement_group_policy strict
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/placement/groups`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .create(linode_api::resources::placement::groups::CreateRequest {
        api_version: linode_api::models::PostApiVersionPlacementGroupsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionPlacementGroupsBody {
            label: Some("PG_Miami_failover".to_string()),
            placement_group_policy: Some(
                linode_api::models::PostApiVersionPlacementGroupsBodyPlacementGroupPolicyEnum::Strict,
            ),
            placement_group_type: Some(
                linode_api::models::PostApiVersionPlacementGroupsBodyPlacementGroupTypeEnum::AntiAffinityLocal,
            ),
            region: Some("us-iad".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Assign a placement group
Add one or more compute instances to your placement group. Check out our [example API workflow](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#create-a-placement-group) to create a placement group and add compute instances.


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement assign-linode 528 \
  --linodes 123 456 \
  --non-compliant true
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/placement/groups/{groupId}/assign`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .assign()
    .create(linode_api::resources::placement::groups::assign::CreateRequest {
        api_version: linode_api::models::PostApiVersionPlacementGroupsGroupIdAssignApiVersionEnum::V4,
        group_id: 123,
        data: linode_api::models::PostApiVersionPlacementGroupsGroupIdAssignBody {
            ..Default::default()
        },
    })
    .await;
```

    
### Unassign a placement group
Remove one or more compute instances from your placement group.


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement unassign-linode 528 \
  --linode 123 456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/placement/groups/{groupId}/unassign`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .unassign()
    .create(linode_api::resources::placement::groups::unassign::CreateRequest {
        api_version: linode_api::models::PostApiVersionPlacementGroupsGroupIdUnassignApiVersionEnum::V4,
        group_id: 123,
        data: linode_api::models::PostApiVersionPlacementGroupsGroupIdUnassignBody {
            ..Default::default()
        },
    })
    .await;
```

    
### Send a phone number verification code
Send a one-time verification code via SMS message to the submitted phone number. Providing your phone number helps ensure you can securely access your Account in case other ways to connect are lost. Your phone number is only used to verify your identity by sending an SMS message. Standard carrier messaging fees may apply.

- By accessing this operation you are opting in to receive SMS messages. You can opt out of SMS messages by running the [Delete a phone number](https://techdocs.akamai.com/linode-api/reference/delete-profile-phone-number) operation after your phone number is verified.

- Verification codes are valid for 10 minutes after they are sent.

- Subsequent requests made prior to code expiration result in sending the same code.

Once a verification code is received, verify your phone number with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli phone sms-code-send \
  --iso-code US \
  --phone-number 555-555-5555
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/phone-number`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .phone_number()
    .create(linode_api::resources::profile_resource::phone_number::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfilePhoneNumberApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionProfilePhoneNumberBody {
            iso_code: "US".to_string(),
            phone_number: "555-555-5555".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Verify a phone number
Verify a phone number by confirming the one-time code received via SMS message after running the [Send a phone number verification code](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number) operation.

- Verification codes are valid for 10 minutes after they are sent.

- Only the same User that made the verification code request can use that code with this operation.

Once completed, the verified phone number is assigned to the User making the request. To change the verified phone number for a User, first run the [Delete a phone number](https://techdocs.akamai.com/linode-api/reference/delete-profile-phone-number) operation, then begin the verification process again with the [Send a phone number verification code](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli phone verify \
  --otp_code 123456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/phone-number/verify`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .phone_number()
    .verify()
    .create(linode_api::resources::profile_resource::phone_number::verify::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfilePhoneNumberVerifyApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionProfilePhoneNumberVerifyBody {
            otp_code: "US".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Answer security questions
Adds security question responses for your User.

Requires exactly three unique questions.

Previous responses are overwritten if answered or reset to `null` if unanswered.

__Note__. Security questions must be answered for your User prior to accessing the [Create a two factor secret](https://techdocs.akamai.com/linode-api/reference/post-tfa-enable) operation.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/security-questions`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .security_questions()
    .create(linode_api::resources::profile_resource::security_questions::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfileSecurityQuestionsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionProfileSecurityQuestionsBody {
            security_questions: Some(
                vec![
                    linode_api::models::PostApiVersionProfileSecurityQuestionsBodySecurityQuestionsItem
                    { question_id : Some(1), response : Some("Gotham City"
                    .to_string()), security_question :
                    Some("In what city were you born?".to_string()),
                    ..Default::default() }
                ],
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Add an SSH key
Adds an SSH Key to your Account profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli sshkeys create \
  --label "My SSH Key" \
  --ssh_key "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/sshkeys`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .sshkeys()
    .create(linode_api::resources::profile_resource::sshkeys::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfileSshkeysApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionProfileSshkeysBody {
            created: Some("2018-01-01T00:01:01".to_string()),
            id: Some(42),
            label: Some("My SSH Key".to_string()),
            ssh_key: Some(
                "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer"
                    .to_string(),
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Disable two factor authentication
Disables Two Factor Authentication for your User. Once successful, login attempts from untrusted computers will only require a password before being successful. This is less secure, and is discouraged.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile tfa-disable
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/tfa-disable`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tfa_disable()
    .create(linode_api::resources::profile_resource::tfa_disable::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfileTfaDisableApiVersionEnum::V4,
    })
    .await;
```

    
### Create a two factor secret
Generates a Two Factor secret for your User. To enable TFA for your User, enter the secret obtained from this operation with the [Enable two factor authentication](https://techdocs.akamai.com/linode-api/reference/post-tfa-confirm) operation. Once enabled, logins from untrusted computers are required to provide a TFA code before they are successful.

Run the [Answer security questions](https://techdocs.akamai.com/linode-api/reference/post-security-questions) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile tfa-enable
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/tfa-enable`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tfa_enable()
    .create(linode_api::resources::profile_resource::tfa_enable::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfileTfaEnableApiVersionEnum::V4,
    })
    .await;
```

    
### Enable two factor authentication
Confirms that you can successfully generate Two Factor codes and enables TFA on your Account. Once this is complete, login attempts from untrusted computers will be required to provide a Two Factor code before they are successful.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile tfa-confirm \
  --tfa_code 213456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/tfa-enable-confirm`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tfa_enable_confirm()
    .create(linode_api::resources::profile_resource::tfa_enable_confirm::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfileTfaEnableConfirmApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionProfileTfaEnableConfirmBody {
            tfa_code: Some("213456".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a personal access token
Creates a Personal Access Token for your User. The raw token will be returned in the response, but will never be returned again afterward so be sure to take note of it. You may create a token with _at most_ the scopes of your current token. The created token will be able to access your Account until the given expiry, or until it is revoked. __Parent and child accounts__ In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- If you're using a child account parent user (proxy user), you can't create this form of token. The only token available to a proxy user is one that lets you run operations in a child account. These are created with the [Create a proxy user token](https://techdocs.akamai.com/linode-api/reference/post-child-account-token) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile token-create \
  --scopes '*' \
  --expiry '2018-01-01T13:46:32' \
  --label linode-cli
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/profile/tokens`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tokens()
    .create(linode_api::resources::profile_resource::tokens::CreateRequest {
        api_version: linode_api::models::PostApiVersionProfileTokensApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionProfileTokensBody {
            expiry: Some("1970-01-01T00:00:00".to_string()),
            label: Some("linode-cli".to_string()),
            scopes: Some("*".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Open a support ticket
Open a Support Ticket. Only one of the ID attributes (`linode_id`, `domain_id`, etc.) can be set on a single Support Ticket.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tickets create \
  --description "I'm having trouble setting the root password on my Linode. I tried following the instructions but something is not working and I'm not sure what I'm doing wrong. Can you please help me figure out how I can reset it?" \
  --linode_id 123 \
  --summary "Having trouble resetting root password on my Linode"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/support/tickets`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .create(linode_api::resources::support::tickets::CreateRequest {
        api_version: linode_api::models::PostApiVersionSupportTicketsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionSupportTicketsBody {
            database_id: Some(123),
            description: "I'm having trouble setting the root password on my Linode. I tried following the instructions but something is not working and I'm not sure what I'm doing wrong. Can you please help me figure out how I can reset it?"
                .to_string(),
            domain_id: Some(123),
            firewall_id: Some(123),
            linode_id: Some(123),
            lkecluster_id: Some(123),
            longviewclient_id: Some(123),
            managed_issue: Some(false),
            nodebalancer_id: Some(123),
            region: Some("string".to_string()),
            summary: "Having trouble resetting root password on my Linode"
                .to_string(),
            vlan: Some("string".to_string()),
            volume_id: Some(123),
            vpc_id: Some(123),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a support ticket attachment
Adds a file attachment to an existing Support Ticket on your Account. File attachments are used to assist our Support team in resolving your Ticket. Examples of attachments are screen shots and text files that provide additional information.

The file attachment is submitted in the request as multipart/form-data.

__Note__. Accepted file extensions include: .gif, .jpg, .jpeg, .pjpg, .pjpeg, .tif, .tiff, .png, .pdf, or .txt.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/support/tickets/{ticketId}/attachments`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .attachments()
    .create(linode_api::resources::support::tickets::attachments::CreateRequest {
        api_version: linode_api::models::PostApiVersionSupportTicketsTicketIdAttachmentsApiVersionEnum::V4,
        ticket_id: 123,
        data: linode_api::models::PostApiVersionSupportTicketsTicketIdAttachmentsBody {
            file: "/Users/LinodeGuy/pictures/screen_shot.jpg".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Close a support ticket
Closes a Support Ticket you have access to modify.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tickets close 11223344
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/support/tickets/{ticketId}/close`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .close()
    .create(linode_api::resources::support::tickets::close::CreateRequest {
        api_version: linode_api::models::PostApiVersionSupportTicketsTicketIdCloseApiVersionEnum::V4,
        ticket_id: 123,
    })
    .await;
```

    
### Create a reply
Adds a reply to an existing Support Ticket.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tickets reply 11223344 \
  --description "Thank you for your help. I was able to figure out what the problem was and I successfully reset my password. You guys are the best!"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/support/tickets/{ticketId}/replies`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .support()
    .tickets()
    .replies()
    .create(linode_api::resources::support::tickets::replies::CreateRequest {
        api_version: linode_api::models::PostApiVersionSupportTicketsTicketIdRepliesApiVersionEnum::V4,
        ticket_id: 123,
        data: linode_api::models::PostApiVersionSupportTicketsTicketIdRepliesBody {
            description: "Thank you for your help. I was able to figure out what the problem was and I successfully reset my password. You guys are the best!"
                .to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a tag
Creates a new Tag and optionally tags requested objects with it immediately.

__Important__. You must be an unrestricted User in order to access, add, or modify Tags information.


<<LB>>

---


- __CLI__.

    ```
    linode-cli tags create \
  --label 'example tag' \
  --linodes 123 \
  --linodes 456 \
  --volumes 9082 \
  --volumes 10003
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/tags`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .tags()
    .create(linode_api::resources::tags::CreateRequest {
        api_version: linode_api::models::PostApiVersionTagsApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionTagsBody {
            domains: Some(vec![564, 565]),
            label: "example tag".to_string(),
            linodes: Some(vec![123, 456]),
            nodebalancers: Some(vec![10301, 10501]),
            volumes: Some(vec![9082, 10003]),
            ..Default::default()
        },
    })
    .await;
```

    
### Create a volume
Creates a volume on your account. For this to complete, you need the `add_volumes` grant. Creating a new volume accrues additional charges on your account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes create \
  --label my-volume \
  --size 20 \
  --linode_id 12346 \
  --encryption enabled \
  --no-defaults
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/volumes`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .create(linode_api::resources::volumes::CreateRequest {
        api_version: linode_api::models::PostApiVersionVolumesApiVersionEnum::V4,
        data: linode_api::models::PostApiVersionVolumesBody {
            config_id: Some(23456),
            encryption: Some(
                linode_api::models::PostApiVersionVolumesBodyEncryptionEnum::Enabled,
            ),
            label: "my-volume".to_string(),
            linode_id: Some(123),
            region: Some("string".to_string()),
            size: Some(20),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Attach a volume
Attaches a Volume on your Account to an existing Linode on your Account. In order for this request to complete successfully, your User must have `read_write` permission to the Volume and `read_write` permission to the Linode. Additionally, the Volume and Linode must be located in the same Region.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes attach 12345 \
  --linode_id 12346 \
  --config_id 23456
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/volumes/{volumeId}/attach`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .attach()
    .create(linode_api::resources::volumes::attach::CreateRequest {
        api_version: linode_api::models::PostApiVersionVolumesVolumeIdAttachApiVersionEnum::V4,
        volume_id: 123,
        data: linode_api::models::PostApiVersionVolumesVolumeIdAttachBody {
            config_id: Some(23456),
            linode_id: 123,
            persist_across_boots: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Clone a volume
Creates a Volume on your Account. In order for this request to complete successfully, your User must have the `add_volumes` grant. The new Volume will have the same size and data as the source Volume. Creating a new Volume will incur a charge on your Account.

- Only Volumes with a `status` of `active` can be cloned.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes clone 12345 \
  --label my-volume
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/volumes/{volumeId}/clone`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .clone()
    .create(linode_api::resources::volumes::clone::CreateRequest {
        api_version: linode_api::models::PostApiVersionVolumesVolumeIdCloneApiVersionEnum::V4,
        volume_id: 123,
        data: linode_api::models::PostApiVersionVolumesVolumeIdCloneBody {
            label: "my-volume".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Detach a volume
Detaches a Volume on your Account from a Linode on your Account. In order for this request to complete successfully, your User must have `read_write` access to the Volume and `read_write` access to the Linode.

Volumes are automatically detached from deleted Linodes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes detach 12345
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/volumes/{volumeId}/detach`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .detach()
    .create(linode_api::resources::volumes::detach::CreateRequest {
        api_version: linode_api::models::PostApiVersionVolumesVolumeIdDetachApiVersionEnum::V4,
        volume_id: 123,
    })
    .await;
```

    
### Resize a volume
Resize an existing Volume on your Account. In order for this request to complete successfully, your User must have the `read_write` permissions to the Volume.

- Volumes can only be resized up.
- Only Volumes with a `status` of "active" can be resized.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes resize 12345 \
  --size 30
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/volumes/{volumeId}/resize`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .resize()
    .create(linode_api::resources::volumes::resize::CreateRequest {
        api_version: linode_api::models::PostApiVersionVolumesVolumeIdResizeApiVersionEnum::V4,
        volume_id: 123,
        data: linode_api::models::PostApiVersionVolumesVolumeIdResizeBody {
            size: 30,
            ..Default::default()
        },
    })
    .await;
```

    
### Create a VPC
Create a new VPC and optionally associated VPC Subnets.

- Users must have the `add_vpc` grant to access this operation.
- A successful request triggers a `vpc_create` event and `subnet_create` events for any created VPC Subnets.

Once a VPC is created, it can be attached to a Linode by assigning a VPC Subnet to one of the Linode's Configuration Profile Interfaces. This step can be accomplished with the following operations:

- [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance)
- [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config)
- [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config)
- [Add a configuration profile interface](https://techdocs.akamai.com/linode-api/reference/post-linode-config-interface)


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs create \
  --description "A description of my VPC." \
  --label cool-vpc \
  --region us-east \
  --subnets.label cool-vpc-subnet \
  --subnets.ipv4 10.0.1.0/24
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    vpc:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/vpcs`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .create(linode_api::resources::vpcs::CreateRequest {
        api_version: linode_api::models::PostApiVersionVpcsApiVersionEnum::V4,
        data: "could be anything",
    })
    .await;
```

    
### Create a VPC subnet
Create a VPC Subnet.

- The User accessing this operation must have `read_write` grants to the VPC.
- A successful request triggers a `subnet_create` event.

Once a VPC Subnet is created, it can be attached to a Linode by assigning the Subnet to one of the Linode's Configuration Profile Interfaces. This step can be accomplished with the following operations:

- [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance)
- [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config)
- [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config)
- [Add a configuration profile interface](https://techdocs.akamai.com/linode-api/reference/post-linode-config-interface)


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs subnet-create $vpcId \
  --label cool-vpc-subnet \
  --ipv4 10.0.1.0/24
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    vpc:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `POST /{apiVersion}/vpcs/{vpcId}/subnets`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .subnets()
    .create(linode_api::resources::vpcs::subnets::CreateRequest {
        api_version: linode_api::models::PostApiVersionVpcsVpcIdSubnetsApiVersionEnum::V4,
        vpc_id: 123,
        data: linode_api::models::PostApiVersionVpcsVpcIdSubnetsBody {
            ipv4: "10.0.1.0/24".to_string(),
            label: "cool-vpc-subnet".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Update your account
Updates contact and billing information related to your account. If you exclude any properties from the request, the operation leaves them unchanged.

__Note__. When updating an account's `country` to `US`, you'll get an error if the account's `zip` is not a valid US zip code.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- You can't change the `company` for a parent account. Akamai uses this value to set the name for a child account parent user (proxy user) on any child account.

- Child account users can't run this operation. These users don't have access to billing-related operations.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account update \
  --address_1 "123 Main St." \
  --address_2 "Suite 101" \
  --city Philadelphia \
  --company My Company \ LLC \
  --country US \
  --email jsmith@mycompany.com \
  --first_name John \
  --last_name Smith \
  --phone 555-555-1212 \
  --state PA \
  --tax_id ATU99999999 \
  --zip 19102
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/account`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .put(linode_api::resources::account::PutRequest {
        api_version: linode_api::models::PutApiVersionAccountApiVersionEnum::V4,
        data: linode_api::models::PutApiVersionAccountBody {
            active_promotions: Some(
                vec![
                    linode_api::models::PutApiVersionAccountBodyActivePromotionsItem
                    { credit_monthly_cap : Some("10.00".to_string()),
                    credit_remaining : Some("50.00".to_string()), description :
                    Some("Receive up to $10 off your services every month for 6 months! Unused credits will expire once this promotion period ends."
                    .to_string()), expire_dt : Some("2018-01-31T23:59:59"
                    .to_string()), image_url :
                    Some("https://linode.com/10_a_month_promotion.svg".to_string()),
                    service_type :
                    Some(linode_api::models::PutApiVersionAccountBodyActivePromotionsItemServiceTypeEnum::All),
                    summary : Some("$10 off your Linode a month!".to_string()),
                    this_month_credit_remaining : Some("10.00".to_string()),
                    ..Default::default() }
                ],
            ),
            active_since: Some("2018-01-01T00:01:01".to_string()),
            address_1: Some("123 Main Street".to_string()),
            address_2: Some("Suite A".to_string()),
            balance: Some(200),
            balance_uninvoiced: Some(145),
            billing_source: Some(
                linode_api::models::PutApiVersionAccountBodyBillingSourceEnum::Akamai,
            ),
            capabilities: Some(
                vec![
                    "Linodes".to_string(), "NodeBalancers".to_string(),
                    "Block Storage".to_string(), "Object Storage".to_string(),
                    "Placement Groups".to_string(), "Block Storage Encryption"
                    .to_string()
                ],
            ),
            city: Some("Philadelphia".to_string()),
            company: Some("Linode LLC".to_string()),
            country: Some("US".to_string()),
            credit_card: Some(linode_api::models::PutApiVersionAccountBodyCreditCard {
                expiry: Some("11/2022".to_string()),
                last_four: Some("string".to_string()),
                ..Default::default()
            }),
            email: Some("john.smith@linode.com".to_string()),
            euuid: Some("E1AF5EEC-526F-487D-B317EBEB34C87D71".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Smith".to_string()),
            phone: Some("215-555-1212".to_string()),
            state: Some("PA".to_string()),
            tax_id: Some("ATU99999999".to_string()),
            zip: Some("19102-1234".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update an OAuth client
Update information about an OAuth Client on your Account. This can be especially useful to update the `redirect_uri` of your client in the event that the callback url changed in your application.


<<LB>>

---


- __CLI__.

    ```
    linode-cli account client-update \
  edc6790ea9db4d224c5c \
  --label Test_Client_1
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/account/oauth-clients/{clientId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .put(linode_api::resources::account::oauth_clients::PutRequest {
        api_version: linode_api::models::PutApiVersionAccountOauthClientsClientIdApiVersionEnum::V4,
        client_id: "string".to_string(),
        data: linode_api::models::PutApiVersionAccountOauthClientsClientIdBody {
            id: Some("2737bf16b39ab5d7b4a1".to_string()),
            label: Some("Test_Client_1".to_string()),
            public: Some(false),
            redirect_uri: Some("https://example.org/oauth/callback".to_string()),
            secret: Some("<REDACTED>".to_string()),
            status: Some(
                linode_api::models::PutApiVersionAccountOauthClientsClientIdBodyStatusEnum::Active,
            ),
            thumbnail_url: linode_api::Patch::new(
                "https://api.linode.com/v4/account/clients/2737bf16b39ab5d7b4a1/thumbnail"
                    .to_string(),
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update the OAuth client's thumbnail
Upload a thumbnail for a client you own.  You must upload a PNG image file that will be returned when the thumbnail is retrieved.  This image will be publicly viewable.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/account/oauth-clients/{clientId}/thumbnail`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .oauth_clients()
    .thumbnail()
    .put(linode_api::resources::account::oauth_clients::thumbnail::PutRequest {
        api_version: linode_api::models::PutApiVersionAccountOauthClientsClientIdThumbnailApiVersionEnum::V4,
        client_id: "string".to_string(),
        data: linode_api::UploadFile::from_path("uploads/file.pdf").unwrap(),
    })
    .await;
```

    
### Update account settings
Updates your account settings. For a Longview subscription plan, see [Update a Longview plan](https://techdocs.akamai.com/linode-api/reference/put-longview-plan).


<<LB>>

---


- __CLI__.

    ```
    linode-cli account settings-update \
  --network_helper false
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/account/settings`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .settings()
    .put(linode_api::resources::account::settings::PutRequest {
        api_version: linode_api::models::PutApiVersionAccountSettingsApiVersionEnum::V4,
        data: linode_api::models::PutApiVersionAccountSettingsBody {
            backups_enabled: Some(true),
            longview_subscription: Some("longview-3".to_string()),
            managed: Some(true),
            network_helper: Some(false),
            object_storage: Some(
                linode_api::models::PutApiVersionAccountSettingsBodyObjectStorageEnum::Active,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a user
Update information about a user on your account, including its restricted status. When setting a user to `restricted`, the API sets no grants for it. You need to set grants so that user can access things on the account.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- You can't edit the `username` or `email` values for the child account parent user (proxy user). These are predefined for the proxy user when you initially provision the parent-child relationship. Only a proxy user's `restricted` status can be modified. This can only be done by an unrestricted child account user.

- A parent account using an unrestricted proxy user in a child account can modify the `username`, `email`, and `restricted` status for an existing child account user.

- A restricted account user--parent or child--can't change their user to `unrestricted`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli users update example_user \
  --username example_user \
  --email example@linode.com \
  --restricted true
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/account/users/{username}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .users()
    .put(linode_api::resources::account::users::PutRequest {
        api_version: linode_api::models::PutApiVersionAccountUsersUsernameApiVersionEnum::V4,
        username: "string".to_string(),
        data: linode_api::models::PutApiVersionAccountUsersUsernameBody {
            email: Some("example_user@linode.com".to_string()),
            last_login: linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameBodyLastLogin {
                login_datetime: Some("2018-01-01T01:01:01".to_string()),
                status: Some(
                    linode_api::models::PutApiVersionAccountUsersUsernameBodyLastLoginStatusEnum::Successful,
                ),
                ..Default::default()
            }),
            password_created: linode_api::Patch::new(
                "2018-01-01T01:01:01".to_string(),
            ),
            restricted: Some(true),
            ssh_keys: Some(vec!["home-pc".to_string(), "laptop".to_string()]),
            tfa_enabled: Some(true),
            username: Some("example_user".to_string()),
            verified_phone_number: linode_api::Patch::new("+5555555555".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a user's grants
Update the grants a user has. This can be used to give a user access to new entities or actions, or take access away.  You don't need to include the grant for every entity on the account in this request. Any that are not included remain unchanged.

__Note__. This operation can only be accessed by account users with _unrestricted_ access.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- No child account user can modify the `account_access` grant for the child account parent user (proxy user).

- An unrestricted child account user can configure all other grants for the proxy user, via `global` object.

- An unrestricted child account user can enable the `account_access` grant for other child account users. However, enabled child users are still subject to child user restrictions--they can't perform write operations for any billing or account information.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/account/users/{username}/grants`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .account()
    .users()
    .grants()
    .put(linode_api::resources::account::users::grants::PutRequest {
        api_version: linode_api::models::PutApiVersionAccountUsersUsernameGrantsApiVersionEnum::V4,
        username: "string".to_string(),
        data: linode_api::models::PutApiVersionAccountUsersUsernameGrantsBody {
            database: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDatabaseItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDatabaseItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            domain: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDomainItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDomainItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            firewall: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyFirewallItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyFirewallItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            global: Some(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyGlobal {
                account_access: linode_api::Patch::new(
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyGlobalAccountAccessEnum::ReadOnly,
                ),
                add_databases: Some(true),
                add_domains: Some(true),
                add_firewalls: Some(true),
                add_images: Some(true),
                add_linodes: Some(true),
                add_loadbalancers: Some(true),
                add_longview: Some(true),
                add_nodebalancers: Some(true),
                add_placement_groups: Some(true),
                add_stackscripts: Some(true),
                add_volumes: Some(true),
                add_vpcs: Some(true),
                cancel_account: Some(false),
                child_account_access: linode_api::Patch::new(true),
                longview_subscription: Some(true),
                ..Default::default()
            }),
            image: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyImageItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyImageItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            linode: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLinodeItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLinodeItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            longview: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLongviewItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLongviewItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            nodebalancer: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyNodebalancerItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyNodebalancerItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            placement_group: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyPlacementGroupItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyPlacementGroupItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            stackscript: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyStackscriptItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyStackscriptItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            volume: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVolumeItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVolumeItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            vpc: Some(
                vec![
                    linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVpcItem
                    { id : Some(123), label : Some("example-entity".to_string()),
                    permissions :
                    linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVpcItemPermissionsEnum::ReadOnly),
                    ..Default::default() }
                ],
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a managed MySQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Update a Managed MySQL Database.

Requires `read_write` access to the Database.

The Database must have an `active` status to perform this operation.

Updating addresses in the `allow_list` overwrites any existing addresses.

- IP addresses and ranges in this list can access the Managed Database. All other sources are blocked.

- If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.

- Entering an empty array (`[]`) blocks all connections (both public and private) to the Managed Database.

- __Note__. Updates to the `allow_list` may take a short period of time to complete, making this operation inappropriate for rapid successive updates to this property.

All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed MySQL Database. The maintenance window for these updates is configured with the Managed Database's `updates` property.

- If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.

- __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then [migrate your databases](https://www.linode.com/docs/products/databases/managed-databases/guides/migrate-mysql/) from the original Managed Database cluster to the new one.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases mysql-update 123 \
  --label example-db \
  --allow_list 203.0.113.1 \
  --allow_list 192.0.1.0/24 \
  --type g6-standard-1 \
  --updates.frequency monthly \
  --updates.duration 3 \
  --updates.hour_of_day 12 \
  --updates.day_of_week 4 \
  --updates.week_of_month 3
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/databases/mysql/instances/{instanceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .mysql()
    .instances()
    .put(linode_api::resources::databases::mysql::instances::PutRequest {
        api_version: linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdApiVersionEnum::V4,
        instance_id: 123,
        data: linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdBody {
            allow_list: Some(
                vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
            ),
            label: Some("example-db".to_string()),
            type_field: Some("g6-standard-1".to_string()),
            updates: Some(linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdBodyUpdates {
                day_of_week: Some(1),
                duration: Some(3),
                frequency: Some(
                    linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdBodyUpdatesFrequencyEnum::Weekly,
                ),
                hour_of_day: Some(0),
                week_of_month: linode_api::Patch::new(123),
                ..Default::default()
            }),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a managed PostgreSQL database
__This operation is currently only available for customers who already have an active Managed Database.__

Update a Managed PostgreSQL Database.

Requires `read_write` access to the Database.

The Database must have an `active` status to perform this operation.

Updating addresses in the `allow_list` overwrites any existing addresses.

- IP addresses and ranges in this list can access the Managed Database. All other sources are blocked.

- If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.

- Entering an empty array (`[]`) blocks all connections (both public and private) to the Managed Database.

- __Note__. Updates to the `allow_list` may take a short period of time to complete, making this operation inappropriate for rapid successive updates to this property.

All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed PostgreSQL Database. The maintenance window for these updates is configured with the Managed Database's `updates` property.

- If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.

- __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then migrate your databases from the original Managed Database cluster to the new one.


<<LB>>

---


- __CLI__.

    ```
    linode-cli databases postgresql-update 123 \
  --label example-db \
  --allow_list 203.0.113.1 \
  --allow_list 192.0.1.0/24 \
  --type g6-standard-1 \
  --updates.frequency monthly \
  --updates.duration 3 \
  --updates.hour_of_day 12 \
  --updates.day_of_week 4 \
  --updates.week_of_month 3
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    databases:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/databases/postgresql/instances/{instanceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .databases()
    .postgresql()
    .instances()
    .put(linode_api::resources::databases::postgresql::instances::PutRequest {
        api_version: linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdApiVersionEnum::V4,
        instance_id: 123,
        data: linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdBody {
            allow_list: Some(
                vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
            ),
            label: Some("example-db".to_string()),
            type_field: Some("g6-standard-1".to_string()),
            updates: Some(linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdates {
                day_of_week: Some(1),
                duration: Some(3),
                frequency: Some(
                    linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdatesFrequencyEnum::Weekly,
                ),
                hour_of_day: Some(0),
                week_of_month: linode_api::Patch::new(123),
                ..Default::default()
            }),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a domain
Update information about a Domain in Linode's DNS Manager.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains update 1234 \
  --retry_sec 7200 \
  --ttl_sec 300
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/domains/{domainId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .put(linode_api::resources::domains::PutRequest {
        api_version: linode_api::models::PutApiVersionDomainsDomainIdApiVersionEnum::V4,
        domain_id: 123,
        data: linode_api::models::PutApiVersionDomainsDomainIdBody {
            axfr_ips: Some(vec!["string".to_string()]),
            description: Some("string".to_string()),
            domain: Some("example.org".to_string()),
            expire_sec: Some(300),
            group: Some("string".to_string()),
            id: Some(1234),
            master_ips: Some(vec!["string".to_string()]),
            refresh_sec: Some(300),
            retry_sec: Some(300),
            soa_email: Some("admin@example.org".to_string()),
            status: Some(
                linode_api::models::PutApiVersionDomainsDomainIdBodyStatusEnum::Active,
            ),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            ttl_sec: Some(300),
            type_field: Some(
                linode_api::models::PutApiVersionDomainsDomainIdBodyTypeEnum::Master,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a domain record
Updates a single Record on this Domain.


<<LB>>

---


- __CLI__.

    ```
    linode-cli domains records-update 123 234 \
  --name test \
  --target 203.0.113.1 \
  --priority 50 \
  --weight 50 \
  --port 80 \
  --ttl_sec 604800
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    domains:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/domains/{domainId}/records/{recordId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .domains()
    .records()
    .put(linode_api::resources::domains::records::PutRequest {
        api_version: linode_api::models::PutApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum::V4,
        domain_id: 123,
        record_id: 123,
        data: linode_api::models::PutApiVersionDomainsDomainIdRecordsRecordIdBody {
            name: Some("test".to_string()),
            port: Some(80),
            priority: Some(50),
            protocol: linode_api::Patch::new("string".to_string()),
            service: linode_api::Patch::new("string".to_string()),
            tag: linode_api::Patch::new(
                linode_api::models::PutApiVersionDomainsDomainIdRecordsRecordIdBodyTagEnum::Iodef,
            ),
            target: Some("192.0.2.0".to_string()),
            ttl_sec: Some(604800),
            weight: Some(50),
            ..Default::default()
        },
    })
    .await;
```

    
### Update an image
Updates a private image that you have permission to `read_write`.

> 📘
>
> You can't update the `regions` with this operation. Use the [Replicate an image](https://techdocs.akamai.com/linode-api/reference/post-replicate-image) operation to modify the existing regions for your image.


<<LB>>

---


- __CLI__.

    ```
    linode-cli images update private/12345 \
  --label "My gold-master image" \
  --description "The detailed description \
    of my image."
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    images:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/images/{imageId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .images()
    .put(linode_api::resources::images::PutRequest {
        api_version: linode_api::models::PutApiVersionImagesImageIdApiVersionEnum::V4,
        image_id: "linode/debian11".to_string(),
        data: linode_api::models::PutApiVersionImagesImageIdBody {
            capabilities: Some(
                vec!["cloud-init".to_string(), "distributed-sites".to_string()],
            ),
            created: Some("2021-08-14T22:44:02".to_string()),
            created_by: Some("linode".to_string()),
            deprecated: Some(false),
            description: linode_api::Patch::new(
                "Example image description.".to_string(),
            ),
            eol: linode_api::Patch::new("2026-07-01T04:00:00".to_string()),
            expiry: linode_api::Patch::new("1970-01-01T00:00:00".to_string()),
            id: Some("linode/debian11".to_string()),
            is_public: Some(true),
            label: Some("Debian 11".to_string()),
            regions: Some(
                vec![
                    linode_api::models::PutApiVersionImagesImageIdBodyRegionsItem {
                    region : Some("us-iad".to_string()), status :
                    Some(linode_api::models::PutApiVersionImagesImageIdBodyRegionsItemStatusEnum::Available),
                    ..Default::default() }
                ],
            ),
            size: Some(2500),
            status: Some(
                linode_api::models::PutApiVersionImagesImageIdBodyStatusEnum::Available,
            ),
            tags: Some(vec!["repair-image".to_string(), "fix-1".to_string()]),
            total_size: Some(1234567),
            type_field: Some(
                linode_api::models::PutApiVersionImagesImageIdBodyTypeEnum::Manual,
            ),
            updated: Some("2021-08-14T22:44:02".to_string()),
            vendor: linode_api::Patch::new("Debian".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a Linode
Updates a Linode that you have permission to `read_write`.

__Important__. You must be an unrestricted User in order to add or modify tags on Linodes.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes update 7833080 \
  --label linode123 \
  --backups.schedule.day "Saturday" \
  --backups.schedule.window "W22" \
  --alerts.cpu 180 \
  --alerts.network_in 10 \
  --alerts.network_out 10 \
  --alerts.transfer_quota 80 \
  --alerts.io 10000
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/linode/instances/{linodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .put(linode_api::resources::linode::instances::PutRequest {
        api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdBody {
            alerts: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyAlerts {
                cpu: Some(180),
                io: Some(10000),
                network_in: Some(10),
                network_out: Some(10),
                transfer_quota: Some(80),
                ..Default::default()
            }),
            backups: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackups {
                available: Some(true),
                enabled: Some(true),
                last_successful: Some("2018-01-01T00:01:01".to_string()),
                schedule: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackupsSchedule {
                    day: linode_api::Patch::new(
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackupsScheduleDayEnum::Saturday,
                    ),
                    window: linode_api::Patch::new(
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackupsScheduleWindowEnum::W22,
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            capabilities: Some(vec!["Block Storage Encryption".to_string()]),
            created: Some("2018-01-01T00:01:01".to_string()),
            disk_encryption: linode_api::Patch::new("disabled".to_string()),
            group: Some("Linode-Group".to_string()),
            has_user_data: Some(true),
            host_uuid: Some("3a3ddd59d9a78bb8de041391075df44de62bfec8".to_string()),
            hypervisor: Some(
                linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyHypervisorEnum::Kvm,
            ),
            id: Some(123),
            image: Some("linode/debian9".to_string()),
            ipv4: Some(vec!["203.0.113.1".to_string(), "192.0.2.1".to_string()]),
            ipv6: linode_api::Patch::new("c001:d00d::1337/128".to_string()),
            label: Some("linode123".to_string()),
            lke_cluster_id: linode_api::Patch::new(1),
            placement_group: linode_api::Patch::new(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroup {
                id: Some(528),
                label: Some("PG_Miami_failover".to_string()),
                placement_group_policy: Some(
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupPolicyEnum::Strict,
                ),
                placement_group_type: Some(
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupTypeEnum::AntiAffinityLocal,
                ),
                ..Default::default()
            }),
            region: Some("us-east".to_string()),
            specs: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodySpecs {
                disk: Some(81920),
                gpus: Some(0),
                memory: Some(4096),
                transfer: Some(4000),
                vcpus: Some(2),
                ..Default::default()
            }),
            status: Some(
                linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyStatusEnum::Running,
            ),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            type_field: Some("g6-standard-1".to_string()),
            updated: Some("2018-01-01T00:01:01".to_string()),
            watchdog_enabled: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a config profile
Updates a Configuration profile.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-update 123 23456 \
  --kernel "linode/latest-64bit" \
  --comments "This is my main Config" \
  --memory_limit 2048 \
  --run_level default \
  --virt_mode paravirt \
  --helpers.updatedb_disabled true \
  --helpers.distro true \
  --helpers.modules_dep true \
  --helpers.network true \
  --helpers.devtmpfs_automount false \
  --label "My Config" \
  --devices.sda.disk_id 123456 \
  --devices.sdb.disk_id 123457
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/linode/instances/{linodeId}/configs/{configId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .put(linode_api::resources::linode::instances::configs::PutRequest {
        api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
        data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBody {
            comments: linode_api::Patch::new("This is my main Config".to_string()),
            devices: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevices {
                sda: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSda {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdb: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdb {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdc: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdc {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdd: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdd {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sde: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSde {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdf: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdf {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdg: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdg {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                sdh: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdh {
                    disk_id: Some(124458),
                    volume_id: Some(123),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            helpers: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyHelpers {
                devtmpfs_automount: Some(false),
                distro: Some(true),
                modules_dep: Some(true),
                network: Some(true),
                updatedb_disabled: Some(true),
                ..Default::default()
            }),
            id: Some(23456),
            interfaces: Some(
                vec![
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItem
                    { id : Some(101), ipam_address : linode_api::Patch::new(None),
                    ipv4 :
                    Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemIpv4
                    { ..Default::default() }), label : linode_api::Patch::new(None),
                    primary : Some(false), purpose :
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemPurposeEnum::Public,
                    subnet_id : linode_api::Patch::new(None), vpc_id :
                    linode_api::Patch::new(None), ..Default::default() },
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItem
                    { id : Some(102), ipam_address :
                    linode_api::Patch::new("10.0.0.1/24".to_string()), ipv4 :
                    Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemIpv4
                    { nat_1_1 : linode_api::Patch::new(None), vpc :
                    linode_api::Patch::new("10.0.0.2".to_string()),
                    ..Default::default() }), label : linode_api::Patch::new("vlan-1"
                    .to_string()), primary : Some(false), purpose :
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemPurposeEnum::Vlan,
                    subnet_id : linode_api::Patch::new(None), vpc_id :
                    linode_api::Patch::new(None), ..Default::default() },
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItem
                    { id : Some(103), ipam_address : linode_api::Patch::new(None),
                    ipv4 :
                    Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemIpv4
                    { nat_1_1 : linode_api::Patch::new("203.0.113.2".to_string()),
                    vpc : linode_api::Patch::new("10.0.1.2".to_string()),
                    ..Default::default() }), label : linode_api::Patch::new(None),
                    primary : Some(true), purpose :
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemPurposeEnum::Vpc,
                    subnet_id : linode_api::Patch::new(101), vpc_id :
                    linode_api::Patch::new(111), ..Default::default() }
                ],
            ),
            kernel: Some("linode/latest-64bit".to_string()),
            label: Some("My Config".to_string()),
            memory_limit: Some(2048),
            root_device: Some("/dev/sda".to_string()),
            run_level: Some(
                linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum::Default,
            ),
            virt_mode: Some(
                linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyVirtModeEnum::Paravirt,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a configuration profile interface
Updates a `vpc` or `public` purpose Interface for this Configuration Profile.

- The User accessing this operation must have `read_write` grants to the Linode.
- A successful request triggers a `linode_config_update` event.
- The Interface `purpose` cannot be updated with this operation.
- VPC Subnets cannot be updated on an Interface. A new `vpc` purpose Interface must be created to assign a different Subnet to a Configuration Profile.
- Only `primary` can be updated for `public` purpose Interfaces.
- This operation not currently allowed for `vlan` purpose Interfaces.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes config-interface-update $linodeId $configId $interfaceId \
  --primary true \
  --ipv4.vpc "10.0.1.2" \
  --ipv4.nat_1_1 "203.0.113.2"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/{interfaceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .configs()
    .interfaces()
    .put(linode_api::resources::linode::instances::configs::interfaces::PutRequest {
        api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum::V4,
        linode_id: 123,
        config_id: 123,
        interface_id: 123,
        data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBody {
            ip_ranges: linode_api::Patch::new(
                vec![
                    "10.0.0.64/26".to_string(), "fd04:495a:691c:971c::1:0/112"
                    .to_string()
                ],
            ),
            ipv4: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBodyIpv4 {
                nat_1_1: linode_api::Patch::new("203.0.113.2".to_string()),
                vpc: linode_api::Patch::new("10.0.0.2".to_string()),
                ..Default::default()
            }),
            primary: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a disk
Updates a Disk that you have permission to `read_write`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes disk-update 123 25674 \
  --label "Debian 9 Disk"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .disks()
    .put(linode_api::resources::linode::instances::disks::PutRequest {
        api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdDisksDiskIdApiVersionEnum::V4,
        linode_id: 123,
        disk_id: 123,
        data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdDisksDiskIdBody {
            label: Some("Debian 9 Disk".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update an IP address's RDNS for a Linode
Updates the reverse DNS (RDNS) for a Linode's IP Address. This may be done for both IPv4 and IPv6 addresses.

Setting the RDNS to `null` for a public IPv4 address, resets it to the default `ip.linodeusercontent.com` RDNS value.


<<LB>>

---


- __CLI__.

    ```
    linode-cli linodes ip-update 123 \
  203.0.113.1 \
  --rdns test.example.org
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/linode/instances/{linodeId}/ips/{address}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .instances()
    .ips()
    .put(linode_api::resources::linode::instances::ips::PutRequest {
        api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum::V4,
        linode_id: 123,
        address: "string".to_string(),
        data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdIpsAddressBody {
            rdns: Some("test.example.org".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a StackScript
Updates a StackScript.

__Once a StackScript is made public, it cannot be made private.__


<<LB>>

---


- __CLI__.

    ```
    linode-cli stackscripts update 10079 \
  --label a-stackscript \
  --description "This StackScript installs \
    and configures MySQL" \
  --images "linode/debian9" \
  --images "linode/debian8" \
  --is_public true \
  --rev_note "Set up MySQL" \
  --script '#!/bin/bash'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    stackscripts:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/linode/stackscripts/{stackscriptId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .linode()
    .stackscripts()
    .put(linode_api::resources::linode::stackscripts::PutRequest {
        api_version: linode_api::models::PutApiVersionLinodeStackscriptsStackscriptIdApiVersionEnum::V4,
        stackscript_id: "string".to_string(),
        data: linode_api::models::PutApiVersionLinodeStackscriptsStackscriptIdBody {
            ..Default::default()
        },
    })
    .await;
```

    
### Update a Kubernetes cluster
Updates a Kubernetes cluster.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-update 12345 \
  --label lkecluster54321 \
  --control_plane.high_availability true \
  --k8s_version 1.27 \
  --tags ecomm \
  --tags blog \
  --tags prod \
  --tags monitoring
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/lke/clusters/{clusterId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .put(linode_api::resources::lke::clusters::PutRequest {
        api_version: linode_api::models::PutApiVersionLkeClustersClusterIdApiVersionEnum::V4,
        cluster_id: 123,
        data: linode_api::models::PutApiVersionLkeClustersClusterIdBody {
            control_plane: Some(linode_api::models::PutApiVersionLkeClustersClusterIdBodyControlPlane {
                acl: Some(linode_api::models::PutApiVersionLkeClustersClusterIdBodyControlPlaneAcl {
                    addresses: Some(linode_api::models::PutApiVersionLkeClustersClusterIdBodyControlPlaneAclAddresses {
                        ipv4: Some(
                            vec!["203.0.113.1".to_string(), "192.0.2.0/24".to_string()],
                        ),
                        ipv6: Some(vec!["2001:db8:1234:abcd::/64".to_string()]),
                        ..Default::default()
                    }),
                    enabled: Some(true),
                    revision_id: Some("20240127r001".to_string()),
                    ..Default::default()
                }),
                high_availability: Some(true),
                ..Default::default()
            }),
            k8s_version: Some("string".to_string()),
            label: Some("lkecluster12345".to_string()),
            tags: Some(
                vec![
                    "prod".to_string(), "monitoring".to_string(), "ecomm"
                    .to_string(), "blog".to_string()
                ],
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update the control plane access control list
Updates a specific cluster's control plane access control list. __Note__: control plane ACLs may not currently be available to all users.


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke cluster-acl-update 12345 \
      --acl.enabled true \
      --acl.addresses.ipv4 "203.0.113.1" \
      --acl.addresses.ipv6 "2001:db8:1234:abcd::/64"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/lke/clusters/{clusterId}/control_plane_acl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .control_plane_acl()
    .put(linode_api::resources::lke::clusters::control_plane_acl::PutRequest {
        api_version: linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclApiVersionEnum::V4,
        cluster_id: 123,
        data: linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBody {
            acl: Some(linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAcl {
                addresses: Some(linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAclAddresses {
                    ipv4: Some(
                        vec!["203.0.113.1".to_string(), "192.0.2.0/24".to_string()],
                    ),
                    ipv6: Some(vec!["2001:db8:1234:abcd::/64".to_string()]),
                    ..Default::default()
                }),
                enabled: Some(true),
                revision_id: Some("20240127r001".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a node pool
Updates a node pool's count, labels and taints, and autoscaler configuration.

Linodes are created or deleted to match changes to the Node Pool's count.

Specifying labels or taints on update overwrites any previous values, and updates existing nodes with the new values without a recycle.

__Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__


<<LB>>

---


- __CLI__.

    ```
    linode-cli lke pool-update 12345 456 \
  --count 6 \
  --autoscaler.enabled true \
  --autoscaler.max 12 \
  --autoscaler.min 3 \
  --labels.key "example.com/my-app" \
  --labels.value "teams" \
  --taints.effect "NoSchedule" \
  --taints.key "example.com/my-app" \
  --taints.value "teamA"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    lke:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .lke()
    .clusters()
    .pools()
    .put(linode_api::resources::lke::clusters::pools::PutRequest {
        api_version: linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdApiVersionEnum::V4,
        cluster_id: 123,
        pool_id: 123,
        data: linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBody {
            autoscaler: Some(linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyAutoscaler {
                enabled: Some(true),
                max: Some(12),
                min: Some(3),
                ..Default::default()
            }),
            count: Some(6),
            labels: Some(linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyLabels {
                key: Some("example.com/my-app".to_string()),
                value: Some("teams".to_string()),
                ..Default::default()
            }),
            taints: Some(
                vec![
                    linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItem
                    { effect :
                    linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItemEffectEnum::NoSchedule,
                    key : "example.com/my-app".to_string(), value : "teamA"
                    .to_string(), ..Default::default() },
                    linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItem
                    { effect :
                    linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItemEffectEnum::NoExecute,
                    key : "myapp.io/team".to_string(), value : "teamB".to_string(),
                    ..Default::default() }
                ],
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a Longview client
Updates a Longview Client.  This cannot update how it monitors your server; use the Longview Client application on your Linode for monitoring configuration.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview update 789 \
  --label client789
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/longview/clients/{clientId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .clients()
    .put(linode_api::resources::longview::clients::PutRequest {
        api_version: linode_api::models::PutApiVersionLongviewClientsClientIdApiVersionEnum::V4,
        client_id: 123,
        data: linode_api::models::PutApiVersionLongviewClientsClientIdBody {
            api_key: Some("BD1B4B54-D752-A76D-5A9BD8A17F39DB61".to_string()),
            apps: Some(linode_api::models::PutApiVersionLongviewClientsClientIdBodyApps {
                apache: Some(true),
                mysql: Some(true),
                nginx: Some(false),
                ..Default::default()
            }),
            created: Some("2018-01-01T00:01:01".to_string()),
            id: Some(789),
            install_code: Some("BD1B5605-BF5E-D385-BA07AD518BE7F321".to_string()),
            label: Some("client789".to_string()),
            updated: Some("2018-01-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a Longview plan
Update your Longview plan to that of the given subscription ID. This returns a `LongviewSubscription` object for the updated Longview Pro plan, or an empty set `{}` if the updated plan is Longview Free.

You must have `"longview_subscription": true` configured as a `global` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) in order to run this operation.

You can send a request to the [List Longview subscriptions](https://techdocs.akamai.com/linode-api/reference/get-longview-subscriptions) operation to receive the details, including `id`'s, of each plan.


<<LB>>

---


- __CLI__.

    ```
    linode-cli longview plan-update --longview_subscription longview-10
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    longview:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/longview/plan`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .longview()
    .plan()
    .put(linode_api::resources::longview::plan::PutRequest {
        api_version: linode_api::models::PutApiVersionLongviewPlanApiVersionEnum::V4,
        data: linode_api::models::PutApiVersionLongviewPlanBody {
            longview_subscription: linode_api::Patch::new(
                linode_api::models::PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum::Longview10,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a managed contact
Updates information about a Managed Contact. This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed contact-update 567 \
  --name "John Doe" \
  --email "john.doe@example.org" \
  --phone.primary "123-456-7890"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/managed/contacts/{contactId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .contacts()
    .put(linode_api::resources::managed::contacts::PutRequest {
        api_version: linode_api::models::PutApiVersionManagedContactsContactIdApiVersionEnum::V4,
        contact_id: 123,
        data: linode_api::models::PutApiVersionManagedContactsContactIdBody {
            email: Some("john.doe@example.org".to_string()),
            group: linode_api::Patch::new("on-call".to_string()),
            id: Some(567),
            name: Some("John Doe".to_string()),
            phone: Some(linode_api::models::PutApiVersionManagedContactsContactIdBodyPhone {
                primary: linode_api::Patch::new("123-456-7890".to_string()),
                secondary: linode_api::Patch::new("string".to_string()),
                ..Default::default()
            }),
            updated: Some("2018-01-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a managed credential
Updates the label of a Managed Credential. This operation does not update the username and password for a Managed Credential. To do this, run the [Update a managed credential's username and password](https://techdocs.akamai.com/linode-api/reference/post-managed-credential-username-password)) operation instead. This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed credential-update 9991 \
  --label prod-password-1
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/managed/credentials/{credentialId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .credentials()
    .put(linode_api::resources::managed::credentials::PutRequest {
        api_version: linode_api::models::PutApiVersionManagedCredentialsCredentialIdApiVersionEnum::V4,
        credential_id: 123,
        data: linode_api::models::PutApiVersionManagedCredentialsCredentialIdBody {
            id: Some(9991),
            label: Some("prod-password-1".to_string()),
            last_decrypted: Some("2018-01-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a Linode's managed settings
Updates a single Linode's Managed settings. This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed linode-setting-update \
  7833234 \
  --ssh.access true \
  --ssh.user linode \
  --ssh.port 22 \
  --ssh.ip 203.0.113.1
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/managed/linode-settings/{linodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .linode_settings()
    .put(linode_api::resources::managed::linode_settings::PutRequest {
        api_version: linode_api::models::PutApiVersionManagedLinodeSettingsLinodeIdApiVersionEnum::V4,
        linode_id: 123,
        data: linode_api::models::PutApiVersionManagedLinodeSettingsLinodeIdBody {
            group: Some("linodes".to_string()),
            id: Some(123),
            label: Some("linode123".to_string()),
            ssh: Some(linode_api::models::PutApiVersionManagedLinodeSettingsLinodeIdBodySsh {
                access: Some(true),
                ip: Some("203.0.113.1".to_string()),
                port: linode_api::Patch::new(22),
                user: linode_api::Patch::new("linode".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a managed service
Updates information about a Managed Service.

This operation can only be accessed by the unrestricted users of an account.


<<LB>>

---


- __CLI__.

    ```
    linode-cli managed service-update 9994 \
  --service_type url \
  --label prod-1 \
  --address "https://example.org" \
  --timeout 30 \
  --body "it worked" \
  --consultation_group on-call \
  --notes "The service name is my-cool-application" \
  --credentials 9991
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/managed/services/{serviceId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .managed()
    .services()
    .put(linode_api::resources::managed::services::PutRequest {
        api_version: linode_api::models::PutApiVersionManagedServicesServiceIdApiVersionEnum::V4,
        service_id: 123,
        data: linode_api::models::PutApiVersionManagedServicesServiceIdBody {
            address: Some("https://example.org".to_string()),
            body: linode_api::Patch::new("it worked".to_string()),
            consultation_group: Some("on-call".to_string()),
            created: Some("2018-01-01T00:01:01".to_string()),
            credentials: Some(vec![9991]),
            id: Some(9944),
            label: Some("prod-1".to_string()),
            notes: linode_api::Patch::new(
                "The service name is my-cool-application".to_string(),
            ),
            region: Some("string".to_string()),
            service_type: Some(
                linode_api::models::PutApiVersionManagedServicesServiceIdBodyServiceTypeEnum::Url,
            ),
            status: Some(
                linode_api::models::PutApiVersionManagedServicesServiceIdBodyStatusEnum::Ok,
            ),
            timeout: Some(30),
            updated: Some("2018-03-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a firewall
Updates information for a Firewall.

- Assigned Linodes must not have any ongoing live migrations.

- If a Firewall's status is changed with this operation, a corresponding `firewall_enable` or `firewall_disable` Event will be generated.

Some parts of a Firewall's configuration cannot be manipulated by this operation:

- A Firewall's Devices cannot be set with this operation. Instead, run the [Create a firewall device](https://techdocs.akamai.com/linode-api/reference/post-firewall-device) and [Delete a firewall device](https://techdocs.akamai.com/linode-api/reference/delete-firewall-device) operations to assign and remove this Firewall from services.

- A Firewall's Rules cannot be changed with this operation. Instead, run the [Update firewall rules](https://techdocs.akamai.com/linode-api/reference/put-firewall-rules) operation to update your Rules.

- A Firewall's status can be set to `enabled` or `disabled` by this operation, but it cannot be set to `deleted`. Instead, run the [Delete a firewall](https://techdocs.akamai.com/linode-api/reference/delete-firewall) operation to delete a Firewall.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls update 123 \
  --status disabled
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/networking/firewalls/{firewallId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .put(linode_api::resources::networking::firewalls::PutRequest {
        api_version: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdApiVersionEnum::V4,
        firewall_id: 123,
        data: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdBody {
            label: Some("firewall123".to_string()),
            status: Some(
                linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdBodyStatusEnum::Enabled,
            ),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update firewall rules
Updates the inbound and outbound Rules for a Firewall.

- Assigned Linodes must not have any ongoing live migrations.

- __Note__. This operation replaces all of a Firewall's `inbound` and `outbound` rulesets with the values specified in your request.


<<LB>>

---


- __CLI__.

    ```
    linode-cli firewalls rules-update 123 \
  --inbound '[{"action":"ACCEPT", "protocol": "TCP", "ports": "22, 80, 8080, 443", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"], "ipv6": ["2001:DB8::/128"]}}]' \
  --outbound '[{"action":"DROP","protocol": "TCP", "ports": "49152-65535", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"], "ipv6": ["2001:DB8::/128`"]}}]'
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    firewall:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/networking/firewalls/{firewallId}/rules`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .firewalls()
    .rules()
    .put(linode_api::resources::networking::firewalls::rules::PutRequest {
        api_version: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesApiVersionEnum::V4,
        firewall_id: 123,
        data: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBody {
            fingerprint: Some("997dd135".to_string()),
            inbound: Some(
                vec![
                    linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItem
                    { action :
                    Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItemActionEnum::Accept),
                    addresses :
                    Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItemAddresses
                    { ipv4 : Some(vec!["192.0.2.0/24".to_string(), "198.51.100.2/32"
                    .to_string()]), ipv6 : Some(vec!["2001:DB8::/128".to_string()]),
                    ..Default::default() }), description :
                    Some("An example firewall rule description.".to_string()), label
                    : Some("firewallrule123".to_string()), ports :
                    linode_api::Patch::new("22-24, 80, 443".to_string()), protocol :
                    Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItemProtocolEnum::Tcp),
                    ..Default::default() }
                ],
            ),
            inbound_policy: Some(
                linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundPolicyEnum::Drop,
            ),
            outbound: Some(
                vec![
                    linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItem
                    { action :
                    Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItemActionEnum::Accept),
                    addresses :
                    Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItemAddresses
                    { ipv4 : Some(vec!["192.0.2.0/24".to_string(), "198.51.100.2/32"
                    .to_string()]), ipv6 : Some(vec!["2001:DB8::/128".to_string()]),
                    ..Default::default() }), description :
                    Some("An example firewall rule description.".to_string()), label
                    : Some("firewallrule123".to_string()), ports :
                    linode_api::Patch::new("22-24, 80, 443".to_string()), protocol :
                    Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItemProtocolEnum::Tcp),
                    ..Default::default() }
                ],
            ),
            outbound_policy: Some(
                linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundPolicyEnum::Drop,
            ),
            version: Some(1),
            ..Default::default()
        },
    })
    .await;
```

    
### Update an IP address's RDNS
Sets RDNS on an IP Address. Forward DNS must already be set up for reverse DNS to be applied. If you set the RDNS to `null` for public IPv4 addresses, it will be reset to the default _ip.linodeusercontent.com_ RDNS value.


<<LB>>

---


- __CLI__.

    ```
    linode-cli networking ip-update \
  203.0.113.1 \
  --rdns "test.example.org"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    ips:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/networking/ips/{address}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .networking()
    .ips()
    .put(linode_api::resources::networking::ips::PutRequest {
        api_version: linode_api::models::PutApiVersionNetworkingIpsAddressApiVersionEnum::V4,
        address: "string".to_string(),
        data: linode_api::models::PutApiVersionNetworkingIpsAddressBody {
            rdns: Some("test.example.org".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a NodeBalancer
Updates information about a NodeBalancer you can access.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers update 12345 \
  --label balancer12345 \
  --client_conn_throttle 0
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/nodebalancers/{nodeBalancerId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .put(linode_api::resources::nodebalancers::PutRequest {
        api_version: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdApiVersionEnum::V4,
        node_balancer_id: 123,
        data: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdBody {
            client_conn_throttle: Some(0),
            created: Some("2018-01-01T00:01:01".to_string()),
            hostname: Some("192.0.2.1.ip.linodeusercontent.com".to_string()),
            id: Some(12345),
            ipv4: Some("203.0.113.1".to_string()),
            ipv6: linode_api::Patch::new("string".to_string()),
            label: Some("balancer12345".to_string()),
            region: Some("us-east".to_string()),
            tags: Some(
                vec!["example tag".to_string(), "another example".to_string()],
            ),
            transfer: Some(linode_api::models::PutApiVersionNodebalancersNodeBalancerIdBodyTransfer {
                in_field: linode_api::Patch::new(28.91200828552246),
                out: linode_api::Patch::new(3.5487728118896484),
                total: linode_api::Patch::new(32.46078109741211),
                ..Default::default()
            }),
            updated: Some("2018-03-01T00:01:01".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a config
Updates the configuration for a single port on a NodeBalancer.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers config-update \
  12345 4567 \
  --port 443 \
  --protocol https \
  --algorithm roundrobin \
  --stickiness http_cookie \
  --check http_body \
  --check_interval 90 \
  --check_timeout 10 \
  --check_attempts 3 \
  --check_path "/test" \
  --check_body "it works" \
  --check_passive true \
  --proxy_protocol "none" \
  --ssl_cert "-----BEGIN CERTIFICATE-----
              CERTIFICATE_INFORMATION
              -----END CERTIFICATE-----" \
  --ssl_key "-----BEGIN PRIVATE KEY-----
             PRIVATE_KEY_INFORMATION
             -----END PRIVATE KEY-----" \
  --cipher_suite recommended
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .put(linode_api::resources::nodebalancers::configs::PutRequest {
        api_version: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        data: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBody {
            algorithm: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyAlgorithmEnum::Roundrobin,
            ),
            check: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyCheckEnum::HttpBody,
            ),
            check_attempts: Some(3),
            check_body: Some("it works".to_string()),
            check_interval: Some(90),
            check_passive: Some(true),
            check_path: Some("/test".to_string()),
            check_timeout: Some(10),
            cipher_suite: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyCipherSuiteEnum::Recommended,
            ),
            id: Some(4567),
            nodebalancer_id: Some(12345),
            nodes_status: Some(linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyNodesStatus {
                down: Some(0),
                up: Some(4),
                ..Default::default()
            }),
            port: Some(80),
            protocol: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyProtocolEnum::Http,
            ),
            proxy_protocol: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyProxyProtocolEnum::None,
            ),
            ssl_cert: linode_api::Patch::new("<REDACTED>".to_string()),
            ssl_commonname: Some("www.example.com".to_string()),
            ssl_fingerprint: Some(
                "00:01:02:03:04:05:06:07:08:09:0A:0B:0C:0D:0E:0F:10:11:12:13"
                    .to_string(),
            ),
            ssl_key: linode_api::Patch::new("<REDACTED>".to_string()),
            stickiness: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyStickinessEnum::HttpCookie,
            ),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a node
Updates information about a Node, a backend for this NodeBalancer's configured port.


<<LB>>

---


- __CLI__.

    ```
    linode-cli nodebalancers node-update \
  12345 4567 54321 \
  --address 192.168.210.120:80 \
  --label node54321 \
  --weight 50 \
  --mode accept
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    nodebalancers:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes/{nodeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .nodebalancers()
    .configs()
    .nodes()
    .put(linode_api::resources::nodebalancers::configs::nodes::PutRequest {
        api_version: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdApiVersionEnum::V4,
        node_balancer_id: 123,
        config_id: 123,
        node_id: "string".to_string(),
        data: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdBody {
            address: Some("192.168.210.120:80".to_string()),
            config_id: Some(4567),
            id: Some(54321),
            label: Some("node54321".to_string()),
            mode: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdBodyModeEnum::Accept,
            ),
            nodebalancer_id: Some(12345),
            status: Some(
                linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdBodyStatusEnum::Up,
            ),
            weight: Some(50),
            ..Default::default()
        },
    })
    .await;
```

    
### Update access to an Object Storage bucket
Update basic Cross-origin Resource Sharing (CORS) and Access Control Level (ACL) settings. You can configure CORS for all origins and set canned ACL settings.

> 📘
>
> For more fine-grained control of both systems, use the [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket-acl).


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/access`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .access()
    .put(linode_api::resources::object_storage::buckets::access::PutRequest {
        api_version: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        data: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessBody {
            acl: Some(
                linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::Private,
            ),
            cors_enabled: Some(true),
            ..Default::default()
        },
    })
    .await;
```

    
### Update an object's ACL config
Update an object's configured access control level (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects and specify the level of access granted to those users.

> 📘
>
> The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#set-object-acl) equivalent operation offers more detail.


<<LB>>

---


- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-acl`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .buckets()
    .object_acl()
    .put(linode_api::resources::object_storage::buckets::object_acl::PutRequest {
        api_version: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclApiVersionEnum::V4,
        region_id: "string".to_string(),
        bucket: "string".to_string(),
        data: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBody {
            acl: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::PublicRead,
            name: "string".to_string(),
            ..Default::default()
        },
    })
    .await;
```

    
### Update an Object Storage key
Updates an Object Storage key on your account. A successful request triggers an `obj_access_key_update` [event](https://techdocs.akamai.com/linode-api/reference/get-events).


<<LB>>

---


- __CLI__.

    ```
    linode-cli object-storage keys-update \
  --keyId 12345
  --label "my-object-storage-key"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    object_storage:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/object-storage/keys/{keyId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .object_storage()
    .keys()
    .put(linode_api::resources::object_storage::keys::PutRequest {
        api_version: linode_api::models::PutApiVersionObjectStorageKeysKeyIdApiVersionEnum::V4,
        key_id: 123,
        data: linode_api::models::PutApiVersionObjectStorageKeysKeyIdBody {
            label: Some("my-key".to_string()),
            regions: Some(vec!["us-iad".to_string(), "fr-par".to_string()]),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a placement group
Change the `label` for a specific placement group. This is the only value you can update. However, you can [add](https://techdocs.akamai.com/linode-api/reference/post-group-add-linode) more compute instances or [remove](https://techdocs.akamai.com/linode-api/reference/post-group-unassign) existing ones.


<<LB>>

---


- __CLI__.

    ```
    linode-cli placement group-update 528 \
  --label PG_Miami_failover_update
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    linodes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/placement/groups/{groupId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .placement()
    .groups()
    .put(linode_api::resources::placement::groups::PutRequest {
        api_version: linode_api::models::PutApiVersionPlacementGroupsGroupIdApiVersionEnum::V4,
        group_id: 123,
        data: linode_api::models::PutApiVersionPlacementGroupsGroupIdBody {
            label: Some("PG_Miami_failover".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a profile
Update information in your Profile.  This operation requires the `account:read_write` OAuth Scope.

__Parent and child accounts__

In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:

- You can't edit the `email` for a child account parent user (proxy user). This value is fixed and set when you provision this environment.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile update \
  --email example-user@gmail.com \
  --timezone US/Eastern \
  --email_notifications true \
  --list_auth_method keys_only \
  --two_factor_auth true \
  --restricted false
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/profile`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .put(linode_api::resources::profile_resource::PutRequest {
        api_version: linode_api::models::PutApiVersionProfileApiVersionEnum::V4,
        data: linode_api::models::PutApiVersionProfileBody {
            authentication_type: Some(
                linode_api::models::PutApiVersionProfileBodyAuthenticationTypeEnum::Password,
            ),
            authorized_keys: linode_api::Patch::new(vec!["string".to_string()]),
            email: Some("example-user@gmail.com".to_string()),
            email_notifications: Some(true),
            ip_whitelist_enabled: Some(false),
            lish_auth_method: Some(
                linode_api::models::PutApiVersionProfileBodyLishAuthMethodEnum::KeysOnly,
            ),
            referrals: Some(linode_api::models::PutApiVersionProfileBodyReferrals {
                code: Some("871be32f49c1411b14f29f618aaf0c14637fb8d3".to_string()),
                completed: Some(0),
                credit: Some(0),
                pending: Some(0),
                total: Some(0),
                url: Some(
                    "https://www.linode.com/?r=871be32f49c1411b14f29f618aaf0c14637fb8d3"
                        .to_string(),
                ),
                ..Default::default()
            }),
            restricted: Some(false),
            timezone: Some("US/Eastern".to_string()),
            two_factor_auth: Some(true),
            uid: Some(1234),
            username: Some("exampleUser".to_string()),
            verified_phone_number: linode_api::Patch::new("+5555555555".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a user's preferences
Updates a user's preferences. These preferences are tied to the OAuth client that generated the token making the request. The user preferences endpoints allow consumers of the API to store arbitrary JSON data, such as a user's font size preference or preferred display name. An account may have multiple preferences. Preferences, and the pertaining request body, may contain any arbitrary JSON data that the user would like to store.


<<LB>>

---


- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/profile/preferences`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .preferences()
    .put(linode_api::resources::profile_resource::preferences::PutRequest {
        api_version: linode_api::models::PutApiVersionProfilePreferencesApiVersionEnum::V4,
        data: serde_json::json!({}),
    })
    .await;
```

    
### Update an SSH key
Updates an SSH Key that you have permission to `read_write`.

Only SSH key labels can be updated.


<<LB>>

---


- __CLI__.

    ```
    linode-cli sshkeys update 42 \
  --label "my laptop"
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/profile/sshkeys/{sshKeyId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .sshkeys()
    .put(linode_api::resources::profile_resource::sshkeys::PutRequest {
        api_version: linode_api::models::PutApiVersionProfileSshkeysSshKeyIdApiVersionEnum::V4,
        ssh_key_id: 123,
        data: linode_api::models::PutApiVersionProfileSshkeysSshKeyIdBody {
            label: Some("My SSH Key".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a personal access token
Updates a Personal Access Token.


<<LB>>

---


- __CLI__.

    ```
    linode-cli profile token-update 123 \
  --label linode-cli
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    account:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/profile/tokens/{tokenId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .profile_resource()
    .tokens()
    .put(linode_api::resources::profile_resource::tokens::PutRequest {
        api_version: linode_api::models::PutApiVersionProfileTokensTokenIdApiVersionEnum::V4,
        token_id: 123,
        data: linode_api::models::PutApiVersionProfileTokensTokenIdBody {
            created: Some("2018-01-01T00:01:01".to_string()),
            expiry: Some("2018-01-01T13:46:32".to_string()),
            id: Some(123),
            label: Some("linode-cli".to_string()),
            scopes: Some("*".to_string()),
            token: Some("abcdefghijklmnop".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a volume
Updates a Volume that you have permission to `read_write`.


<<LB>>

---


- __CLI__.

    ```
    linode-cli volumes update 12345 \
  --label my_volume
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    volumes:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/volumes/{volumeId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .volumes()
    .put(linode_api::resources::volumes::PutRequest {
        api_version: linode_api::models::PutApiVersionVolumesVolumeIdApiVersionEnum::V4,
        volume_id: 123,
        data: "could be anything",
    })
    .await;
```

    
### Update a VPC
Update an existing VPC.

- The User accessing this operation must have `read_write` grants to the VPC.
- A successful request triggers a `vpc_update` event.

To update a VPC's Subnet, run the [Update a VPC subnet](https://techdocs.akamai.com/linode-api/reference/put-vpc-subnet) operation.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs update $vpcId \
  --description "A description of my VPC."
  --label cool-vpc
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    vpc:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/vpcs/{vpcId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .put(linode_api::resources::vpcs::PutRequest {
        api_version: linode_api::models::PutApiVersionVpcsVpcIdApiVersionEnum::V4,
        vpc_id: 123,
        data: linode_api::models::PutApiVersionVpcsVpcIdBody {
            description: Some("A description of my VPC.".to_string()),
            label: Some("cool-vpc".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    
### Update a VPC subnet
Update a VPC Subnet.

- The User accessing this operation must have `read_write` grants to the VPC.
- A successful request triggers a `subnet_update` event.


<<LB>>

---


- __CLI__.

    ```
    linode-cli vpcs subnet-update $vpcId \
  --label cool-vpc-subnet
    ```

    [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)

- __OAuth scopes__.

    ```
    vpc:read_write
    ```

    [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

**API Endpoint**: `PUT /{apiVersion}/vpcs/{vpcId}/subnets/{vpcSubnetId}`


#### Example Snippet

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
let res = client
    .vpcs()
    .subnets()
    .put(linode_api::resources::vpcs::subnets::PutRequest {
        api_version: linode_api::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum::V4,
        vpc_id: 123,
        vpc_subnet_id: 123,
        data: linode_api::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdBody {
            label: Some("cool-vpc-subnet".to_string()),
            ..Default::default()
        },
    })
    .await;
```

    