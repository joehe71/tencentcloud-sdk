// Generated exhaustive model round trips and operation routing checks.
#[test]
fn model_create_product_secret_request() {
    round_trip::<models::CreateProductSecretRequest>(r#"{"SecretName":"test-marker","UserNamePrefix":"test-marker","ProductName":"test-marker","InstanceID":"test-marker","Domains":["test-marker"],"PrivilegesList":[{"PrivilegeName":"test-marker","Privileges":["test-marker"],"Database":"test-marker","TableName":"test-marker","ColumnName":"test-marker","SchemaName":"test-marker","SequenceName":"test-marker","ProcedureName":"test-marker","TypeName":"test-marker","FunctionName":"test-marker","ViewName":"test-marker","MatviewName":"test-marker"}],"Description":"test-marker","KmsKeyId":"test-marker","Tags":[{"TagKey":"test-marker","TagValue":"test-marker"}],"RotationBeginTime":"test-marker","EnableRotation":true,"RotationFrequency":123,"KmsHsmClusterId":"test-marker","AccountRemark":"test-marker","AccountType":"test-marker","EncryptType":123}"#, r#"{"SecretName":null,"UserNamePrefix":null,"ProductName":null,"InstanceID":null,"Domains":null,"PrivilegesList":null,"Description":null,"KmsKeyId":null,"Tags":null,"RotationBeginTime":null,"EnableRotation":null,"RotationFrequency":null,"KmsHsmClusterId":null,"AccountRemark":null,"AccountType":null,"EncryptType":null}"#);
}
#[test]
fn model_create_product_secret_response() {
    round_trip::<models::CreateProductSecretResponse>(r#"{"SecretName":"test-marker","TagCode":123,"TagMsg":"test-marker","FlowID":123,"RequestId":"test-marker"}"#, r#"{"SecretName":null,"TagCode":null,"TagMsg":null,"FlowID":null,"RequestId":null}"#);
}
#[test]
fn model_create_ssh_key_pair_secret_request() {
    round_trip::<models::CreateSshKeyPairSecretRequest>(r#"{"SecretName":"test-marker","ProjectId":123,"Description":"test-marker","KmsKeyId":"test-marker","Tags":[{"TagKey":"test-marker","TagValue":"test-marker"}],"SSHKeyName":"test-marker","KmsHsmClusterId":"test-marker","EncryptType":123}"#, r#"{"SecretName":null,"ProjectId":null,"Description":null,"KmsKeyId":null,"Tags":null,"SSHKeyName":null,"KmsHsmClusterId":null,"EncryptType":null}"#);
}
#[test]
fn model_create_ssh_key_pair_secret_response() {
    round_trip::<models::CreateSshKeyPairSecretResponse>(r#"{"SecretName":"test-marker","SSHKeyID":"test-marker","SSHKeyName":"test-marker","TagCode":123,"TagMsg":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"SSHKeyID":null,"SSHKeyName":null,"TagCode":null,"TagMsg":null,"RequestId":null}"#);
}
#[test]
fn model_create_secret_request() {
    round_trip::<models::CreateSecretRequest>(r#"{"SecretName":"test-marker","VersionId":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","SecretType":123,"SecretBinary":"test-marker","SecretString":"test-marker","AdditionalConfig":"test-marker","Tags":[{"TagKey":"test-marker","TagValue":"test-marker"}],"KmsHsmClusterId":"test-marker","EncryptType":123}"#, r#"{"SecretName":null,"VersionId":null,"Description":null,"KmsKeyId":null,"SecretType":null,"SecretBinary":null,"SecretString":null,"AdditionalConfig":null,"Tags":null,"KmsHsmClusterId":null,"EncryptType":null}"#);
}
#[test]
fn model_create_secret_response() {
    round_trip::<models::CreateSecretResponse>(r#"{"SecretName":"test-marker","VersionId":"test-marker","TagCode":123,"TagMsg":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"TagCode":null,"TagMsg":null,"RequestId":null}"#);
}
#[test]
fn model_delete_secret_request() {
    round_trip::<models::DeleteSecretRequest>(r#"{"SecretName":"test-marker","RecoveryWindowInDays":123,"CleanSSHKey":true,"DeleteMode":123}"#, r#"{"SecretName":null,"RecoveryWindowInDays":null,"CleanSSHKey":null,"DeleteMode":null}"#);
}
#[test]
fn model_delete_secret_response() {
    round_trip::<models::DeleteSecretResponse>(r#"{"SecretName":"test-marker","DeleteTime":123,"FlowID":123,"RequestId":"test-marker"}"#, r#"{"SecretName":null,"DeleteTime":null,"FlowID":null,"RequestId":null}"#);
}
#[test]
fn model_delete_secret_version_request() {
    round_trip::<models::DeleteSecretVersionRequest>(r#"{"SecretName":"test-marker","VersionId":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null}"#);
}
#[test]
fn model_delete_secret_version_response() {
    round_trip::<models::DeleteSecretVersionResponse>(r#"{"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"RequestId":null}"#);
}
#[test]
fn model_describe_async_request_info_request() {
    round_trip::<models::DescribeAsyncRequestInfoRequest>(r#"{"FlowID":123}"#, r#"{"FlowID":null}"#);
}
#[test]
fn model_describe_async_request_info_response() {
    round_trip::<models::DescribeAsyncRequestInfoResponse>(r#"{"TaskStatus":123,"Description":"test-marker","RequestId":"test-marker"}"#, r#"{"TaskStatus":null,"Description":null,"RequestId":null}"#);
}
#[test]
fn model_describe_rotation_detail_request() {
    round_trip::<models::DescribeRotationDetailRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_describe_rotation_detail_response() {
    round_trip::<models::DescribeRotationDetailResponse>(r#"{"EnableRotation":true,"Frequency":123,"LatestRotateTime":"test-marker","NextRotateBeginTime":"test-marker","RequestId":"test-marker"}"#, r#"{"EnableRotation":null,"Frequency":null,"LatestRotateTime":null,"NextRotateBeginTime":null,"RequestId":null}"#);
}
#[test]
fn model_describe_rotation_history_request() {
    round_trip::<models::DescribeRotationHistoryRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_describe_rotation_history_response() {
    round_trip::<models::DescribeRotationHistoryResponse>(r#"{"VersionIDs":["test-marker"],"TotalCount":123,"AccountInfoList":[{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}],"RequestId":"test-marker"}"#, r#"{"VersionIDs":null,"TotalCount":null,"AccountInfoList":null,"RequestId":null}"#);
}
#[test]
fn model_describe_secret_request() {
    round_trip::<models::DescribeSecretRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_describe_secret_response() {
    round_trip::<models::DescribeSecretResponse>(r#"{"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"SecretType":123,"ProductName":"test-marker","ResourceID":"test-marker","RotationStatus":true,"RotationFrequency":123,"ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"AdditionalConfig":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker","AccountInfoList":[{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}],"NextRotationTime":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"Description":null,"KmsKeyId":null,"CreateUin":null,"Status":null,"DeleteTime":null,"CreateTime":null,"SecretType":null,"ProductName":null,"ResourceID":null,"RotationStatus":null,"RotationFrequency":null,"ResourceName":null,"ProjectID":null,"AssociatedInstanceIDs":null,"TargetUin":null,"AdditionalConfig":null,"EncryptType":null,"EncryptSwitching":null,"CreateUinString":null,"TargetUinString":null,"AccountInfoList":null,"NextRotationTime":null,"RequestId":null}"#);
}
#[test]
fn model_describe_supported_products_request() {
    round_trip::<models::DescribeSupportedProductsRequest>("{}", "{}");
}
#[test]
fn model_describe_supported_products_response() {
    round_trip::<models::DescribeSupportedProductsResponse>(r#"{"Products":["test-marker"],"TotalCount":123,"RequestId":"test-marker"}"#, r#"{"Products":null,"TotalCount":null,"RequestId":null}"#);
}
#[test]
fn model_disable_secret_request() {
    round_trip::<models::DisableSecretRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_disable_secret_response() {
    round_trip::<models::DisableSecretResponse>(r#"{"SecretName":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"RequestId":null}"#);
}
#[test]
fn model_enable_secret_request() {
    round_trip::<models::EnableSecretRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_enable_secret_response() {
    round_trip::<models::EnableSecretResponse>(r#"{"SecretName":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"RequestId":null}"#);
}
#[test]
fn model_get_regions_request() {
    round_trip::<models::GetRegionsRequest>("{}", "{}");
}
#[test]
fn model_get_regions_response() {
    round_trip::<models::GetRegionsResponse>(r#"{"Regions":["test-marker"],"RequestId":"test-marker"}"#, r#"{"Regions":null,"RequestId":null}"#);
}
#[test]
fn model_get_ssh_key_pair_value_request() {
    round_trip::<models::GetSshKeyPairValueRequest>(r#"{"SecretName":"test-marker","SSHKeyId":"test-marker"}"#, r#"{"SecretName":null,"SSHKeyId":null}"#);
}
#[test]
fn model_get_ssh_key_pair_value_response() {
    round_trip::<models::GetSshKeyPairValueResponse>(r#"{"SSHKeyID":"test-marker","PublicKey":"test-marker","PrivateKey":"test-marker","ProjectID":123,"SSHKeyDescription":"test-marker","SSHKeyName":"test-marker","RequestId":"test-marker"}"#, r#"{"SSHKeyID":null,"PublicKey":null,"PrivateKey":null,"ProjectID":null,"SSHKeyDescription":null,"SSHKeyName":null,"RequestId":null}"#);
}
#[test]
fn model_get_secret_value_request() {
    round_trip::<models::GetSecretValueRequest>(r#"{"SecretName":"test-marker","VersionId":"test-marker","EncryptionPublicKey":"test-marker","EncryptionAlgorithm":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"EncryptionPublicKey":null,"EncryptionAlgorithm":null}"#);
}
#[test]
fn model_get_secret_value_response() {
    round_trip::<models::GetSecretValueResponse>(r#"{"SecretName":"test-marker","VersionId":"test-marker","SecretBinary":"test-marker","SecretString":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"SecretBinary":null,"SecretString":null,"RequestId":null}"#);
}
#[test]
fn model_get_service_status_request() {
    round_trip::<models::GetServiceStatusRequest>("{}", "{}");
}
#[test]
fn model_get_service_status_response() {
    round_trip::<models::GetServiceStatusResponse>(r#"{"ServiceEnabled":true,"InvalidType":123,"AccessKeyEscrowEnabled":true,"ExpireTime":"test-marker","QPSLimit":123,"SecretLimit":123,"PayModel":"test-marker","RenewFlag":123,"ResourceId":"test-marker","TotalCount":123,"ResourceRegion":123,"RequestId":"test-marker"}"#, r#"{"ServiceEnabled":null,"InvalidType":null,"AccessKeyEscrowEnabled":null,"ExpireTime":null,"QPSLimit":null,"SecretLimit":null,"PayModel":null,"RenewFlag":null,"ResourceId":null,"TotalCount":null,"ResourceRegion":null,"RequestId":null}"#);
}
#[test]
fn model_list_secret_version_ids_request() {
    round_trip::<models::ListSecretVersionIdsRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_list_secret_version_ids_response() {
    round_trip::<models::ListSecretVersionIdsResponse>(r#"{"SecretName":"test-marker","Versions":[{"VersionId":"test-marker","CreateTime":123}],"RequestId":"test-marker"}"#, r#"{"SecretName":null,"Versions":null,"RequestId":null}"#);
}
#[test]
fn model_list_secrets_request() {
    round_trip::<models::ListSecretsRequest>(r#"{"Offset":123,"Limit":123,"OrderType":123,"State":123,"SearchSecretName":"test-marker","TagFilters":[{"TagKey":"test-marker","TagValue":["test-marker"]}],"SecretType":123,"ProductName":"test-marker","EncryptType":123,"InstanceID":"test-marker"}"#, r#"{"Offset":null,"Limit":null,"OrderType":null,"State":null,"SearchSecretName":null,"TagFilters":null,"SecretType":null,"ProductName":null,"EncryptType":null,"InstanceID":null}"#);
}
#[test]
fn model_list_secrets_response() {
    round_trip::<models::ListSecretsResponse>(r#"{"TotalCount":123,"SecretMetadatas":[{"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"KmsKeyType":"test-marker","RotationStatus":123,"NextRotationTime":123,"SecretType":123,"ProductName":"test-marker","ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"RotationFrequency":123,"ResourceID":"test-marker","RotationBeginTime":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker"}],"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"SecretMetadatas":null,"RequestId":null}"#);
}
#[test]
fn model_product_privilege_unit() {
    round_trip::<models::ProductPrivilegeUnit>(r#"{"PrivilegeName":"test-marker","Privileges":["test-marker"],"Database":"test-marker","TableName":"test-marker","ColumnName":"test-marker","SchemaName":"test-marker","SequenceName":"test-marker","ProcedureName":"test-marker","TypeName":"test-marker","FunctionName":"test-marker","ViewName":"test-marker","MatviewName":"test-marker"}"#, r#"{"PrivilegeName":null,"Privileges":null,"Database":null,"TableName":null,"ColumnName":null,"SchemaName":null,"SequenceName":null,"ProcedureName":null,"TypeName":null,"FunctionName":null,"ViewName":null,"MatviewName":null}"#);
}
#[test]
fn model_put_secret_value_request() {
    round_trip::<models::PutSecretValueRequest>(r#"{"SecretName":"test-marker","VersionId":"test-marker","SecretBinary":"test-marker","SecretString":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"SecretBinary":null,"SecretString":null}"#);
}
#[test]
fn model_put_secret_value_response() {
    round_trip::<models::PutSecretValueResponse>(r#"{"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"RequestId":null}"#);
}
#[test]
fn model_restore_secret_request() {
    round_trip::<models::RestoreSecretRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_restore_secret_response() {
    round_trip::<models::RestoreSecretResponse>(r#"{"SecretName":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"RequestId":null}"#);
}
#[test]
fn model_rotate_product_secret_request() {
    round_trip::<models::RotateProductSecretRequest>(r#"{"SecretName":"test-marker"}"#, r#"{"SecretName":null}"#);
}
#[test]
fn model_rotate_product_secret_response() {
    round_trip::<models::RotateProductSecretResponse>(r#"{"FlowID":123,"RequestId":"test-marker"}"#, r#"{"FlowID":null,"RequestId":null}"#);
}
#[test]
fn model_secret_account_info() {
    round_trip::<models::SecretAccountInfo>(r#"{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}"#, r#"{"AccountName":null,"Host":null,"Version":null,"RotatedTime":null}"#);
}
#[test]
fn model_secret_metadata() {
    round_trip::<models::SecretMetadata>(r#"{"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"KmsKeyType":"test-marker","RotationStatus":123,"NextRotationTime":123,"SecretType":123,"ProductName":"test-marker","ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"RotationFrequency":123,"ResourceID":"test-marker","RotationBeginTime":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker"}"#, r#"{"SecretName":null,"Description":null,"KmsKeyId":null,"CreateUin":null,"Status":null,"DeleteTime":null,"CreateTime":null,"KmsKeyType":null,"RotationStatus":null,"NextRotationTime":null,"SecretType":null,"ProductName":null,"ResourceName":null,"ProjectID":null,"AssociatedInstanceIDs":null,"TargetUin":null,"RotationFrequency":null,"ResourceID":null,"RotationBeginTime":null,"EncryptType":null,"EncryptSwitching":null,"CreateUinString":null,"TargetUinString":null}"#);
}
#[test]
fn model_tag() {
    round_trip::<models::Tag>(r#"{"TagKey":"test-marker","TagValue":"test-marker"}"#, r#"{"TagKey":null,"TagValue":null}"#);
}
#[test]
fn model_tag_filter() {
    round_trip::<models::TagFilter>(r#"{"TagKey":"test-marker","TagValue":["test-marker"]}"#, r#"{"TagKey":null,"TagValue":null}"#);
}
#[test]
fn model_update_description_request() {
    round_trip::<models::UpdateDescriptionRequest>(r#"{"SecretName":"test-marker","Description":"test-marker"}"#, r#"{"SecretName":null,"Description":null}"#);
}
#[test]
fn model_update_description_response() {
    round_trip::<models::UpdateDescriptionResponse>(r#"{"SecretName":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"RequestId":null}"#);
}
#[test]
fn model_update_rotation_status_request() {
    round_trip::<models::UpdateRotationStatusRequest>(r#"{"SecretName":"test-marker","EnableRotation":true,"Frequency":123,"RotationBeginTime":"test-marker"}"#, r#"{"SecretName":null,"EnableRotation":null,"Frequency":null,"RotationBeginTime":null}"#);
}
#[test]
fn model_update_rotation_status_response() {
    round_trip::<models::UpdateRotationStatusResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_update_secret_request() {
    round_trip::<models::UpdateSecretRequest>(r#"{"SecretName":"test-marker","VersionId":"test-marker","SecretBinary":"test-marker","SecretString":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"SecretBinary":null,"SecretString":null}"#);
}
#[test]
fn model_update_secret_response() {
    round_trip::<models::UpdateSecretResponse>(r#"{"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-marker"}"#, r#"{"SecretName":null,"VersionId":null,"RequestId":null}"#);
}
#[test]
fn model_version_info() {
    round_trip::<models::VersionInfo>(r#"{"VersionId":"test-marker","CreateTime":123}"#, r#"{"VersionId":null,"CreateTime":null}"#);
}
#[tokio::test]
async fn route_create_product_secret() {
      let (client, calls) = mock_client("CreateProductSecret", serde_json::json!({"Response": {"SecretName":"test-marker","TagCode":123,"TagMsg":"test-marker","FlowID":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_product_secret(&models::CreateProductSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","TagCode":123,"TagMsg":"test-marker","FlowID":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_ssh_key_pair_secret() {
      let (client, calls) = mock_client("CreateSSHKeyPairSecret", serde_json::json!({"Response": {"SecretName":"test-marker","SSHKeyID":"test-marker","SSHKeyName":"test-marker","TagCode":123,"TagMsg":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_ssh_key_pair_secret(&models::CreateSshKeyPairSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","SSHKeyID":"test-marker","SSHKeyName":"test-marker","TagCode":123,"TagMsg":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_secret() {
      let (client, calls) = mock_client("CreateSecret", serde_json::json!({"Response": {"SecretName":"test-marker","VersionId":"test-marker","TagCode":123,"TagMsg":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_secret(&models::CreateSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","VersionId":"test-marker","TagCode":123,"TagMsg":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_secret() {
      let (client, calls) = mock_client("DeleteSecret", serde_json::json!({"Response": {"SecretName":"test-marker","DeleteTime":123,"FlowID":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_secret(&models::DeleteSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","DeleteTime":123,"FlowID":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_secret_version() {
      let (client, calls) = mock_client("DeleteSecretVersion", serde_json::json!({"Response": {"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_secret_version(&models::DeleteSecretVersionRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_describe_async_request_info() {
      let (client, calls) = mock_client("DescribeAsyncRequestInfo", serde_json::json!({"Response": {"TaskStatus":123,"Description":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.describe_async_request_info(&models::DescribeAsyncRequestInfoRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TaskStatus":123,"Description":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_describe_rotation_detail() {
      let (client, calls) = mock_client("DescribeRotationDetail", serde_json::json!({"Response": {"EnableRotation":true,"Frequency":123,"LatestRotateTime":"test-marker","NextRotateBeginTime":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.describe_rotation_detail(&models::DescribeRotationDetailRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"EnableRotation":true,"Frequency":123,"LatestRotateTime":"test-marker","NextRotateBeginTime":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_describe_rotation_history() {
      let (client, calls) = mock_client("DescribeRotationHistory", serde_json::json!({"Response": {"VersionIDs":["test-marker"],"TotalCount":123,"AccountInfoList":[{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.describe_rotation_history(&models::DescribeRotationHistoryRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"VersionIDs":["test-marker"],"TotalCount":123,"AccountInfoList":[{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_describe_secret() {
      let (client, calls) = mock_client("DescribeSecret", serde_json::json!({"Response": {"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"SecretType":123,"ProductName":"test-marker","ResourceID":"test-marker","RotationStatus":true,"RotationFrequency":123,"ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"AdditionalConfig":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker","AccountInfoList":[{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}],"NextRotationTime":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.describe_secret(&models::DescribeSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"SecretType":123,"ProductName":"test-marker","ResourceID":"test-marker","RotationStatus":true,"RotationFrequency":123,"ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"AdditionalConfig":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker","AccountInfoList":[{"AccountName":"test-marker","Host":["test-marker"],"Version":"test-marker","RotatedTime":"test-marker"}],"NextRotationTime":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_describe_supported_products() {
      let (client, calls) = mock_client("DescribeSupportedProducts", serde_json::json!({"Response": {"Products":["test-marker"],"TotalCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.describe_supported_products(&models::DescribeSupportedProductsRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"Products":["test-marker"],"TotalCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_disable_secret() {
      let (client, calls) = mock_client("DisableSecret", serde_json::json!({"Response": {"SecretName":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.disable_secret(&models::DisableSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_enable_secret() {
      let (client, calls) = mock_client("EnableSecret", serde_json::json!({"Response": {"SecretName":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.enable_secret(&models::EnableSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_regions() {
      let (client, calls) = mock_client("GetRegions", serde_json::json!({"Response": {"Regions":["test-marker"],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_regions(&models::GetRegionsRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"Regions":["test-marker"],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_ssh_key_pair_value() {
      let (client, calls) = mock_client("GetSSHKeyPairValue", serde_json::json!({"Response": {"SSHKeyID":"test-marker","PublicKey":"test-marker","PrivateKey":"test-marker","ProjectID":123,"SSHKeyDescription":"test-marker","SSHKeyName":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_ssh_key_pair_value(&models::GetSshKeyPairValueRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SSHKeyID":"test-marker","PublicKey":"test-marker","PrivateKey":"test-marker","ProjectID":123,"SSHKeyDescription":"test-marker","SSHKeyName":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_secret_value() {
      let (client, calls) = mock_client("GetSecretValue", serde_json::json!({"Response": {"SecretName":"test-marker","VersionId":"test-marker","SecretBinary":"test-marker","SecretString":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_secret_value(&models::GetSecretValueRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","VersionId":"test-marker","SecretBinary":"test-marker","SecretString":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_service_status() {
      let (client, calls) = mock_client("GetServiceStatus", serde_json::json!({"Response": {"ServiceEnabled":true,"InvalidType":123,"AccessKeyEscrowEnabled":true,"ExpireTime":"test-marker","QPSLimit":123,"SecretLimit":123,"PayModel":"test-marker","RenewFlag":123,"ResourceId":"test-marker","TotalCount":123,"ResourceRegion":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_service_status(&models::GetServiceStatusRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"ServiceEnabled":true,"InvalidType":123,"AccessKeyEscrowEnabled":true,"ExpireTime":"test-marker","QPSLimit":123,"SecretLimit":123,"PayModel":"test-marker","RenewFlag":123,"ResourceId":"test-marker","TotalCount":123,"ResourceRegion":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_secret_version_ids() {
      let (client, calls) = mock_client("ListSecretVersionIds", serde_json::json!({"Response": {"SecretName":"test-marker","Versions":[{"VersionId":"test-marker","CreateTime":123}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_secret_version_ids(&models::ListSecretVersionIdsRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","Versions":[{"VersionId":"test-marker","CreateTime":123}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_secrets() {
      let (client, calls) = mock_client("ListSecrets", serde_json::json!({"Response": {"TotalCount":123,"SecretMetadatas":[{"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"KmsKeyType":"test-marker","RotationStatus":123,"NextRotationTime":123,"SecretType":123,"ProductName":"test-marker","ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"RotationFrequency":123,"ResourceID":"test-marker","RotationBeginTime":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker"}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_secrets(&models::ListSecretsRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"SecretMetadatas":[{"SecretName":"test-marker","Description":"test-marker","KmsKeyId":"test-marker","CreateUin":123,"Status":"test-marker","DeleteTime":123,"CreateTime":123,"KmsKeyType":"test-marker","RotationStatus":123,"NextRotationTime":123,"SecretType":123,"ProductName":"test-marker","ResourceName":"test-marker","ProjectID":123,"AssociatedInstanceIDs":["test-marker"],"TargetUin":123,"RotationFrequency":123,"ResourceID":"test-marker","RotationBeginTime":"test-marker","EncryptType":123,"EncryptSwitching":true,"CreateUinString":"test-marker","TargetUinString":"test-marker"}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_put_secret_value() {
      let (client, calls) = mock_client("PutSecretValue", serde_json::json!({"Response": {"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.put_secret_value(&models::PutSecretValueRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_restore_secret() {
      let (client, calls) = mock_client("RestoreSecret", serde_json::json!({"Response": {"SecretName":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.restore_secret(&models::RestoreSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_rotate_product_secret() {
      let (client, calls) = mock_client("RotateProductSecret", serde_json::json!({"Response": {"FlowID":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.rotate_product_secret(&models::RotateProductSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"FlowID":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_description() {
      let (client, calls) = mock_client("UpdateDescription", serde_json::json!({"Response": {"SecretName":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_description(&models::UpdateDescriptionRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_rotation_status() {
      let (client, calls) = mock_client("UpdateRotationStatus", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_rotation_status(&models::UpdateRotationStatusRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_secret() {
      let (client, calls) = mock_client("UpdateSecret", serde_json::json!({"Response": {"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_secret(&models::UpdateSecretRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"SecretName":"test-marker","VersionId":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
