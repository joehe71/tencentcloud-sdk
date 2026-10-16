// Generated official action-to-request/response bindings.
operations! {
    create_product_secret: CreateProductSecretRequest => CreateProductSecretResponse = "CreateProductSecret";
    create_ssh_key_pair_secret: CreateSshKeyPairSecretRequest => CreateSshKeyPairSecretResponse = "CreateSSHKeyPairSecret";
    create_secret: CreateSecretRequest => CreateSecretResponse = "CreateSecret";
    delete_secret: DeleteSecretRequest => DeleteSecretResponse = "DeleteSecret";
    delete_secret_version: DeleteSecretVersionRequest => DeleteSecretVersionResponse = "DeleteSecretVersion";
    describe_async_request_info: DescribeAsyncRequestInfoRequest => DescribeAsyncRequestInfoResponse = "DescribeAsyncRequestInfo";
    describe_rotation_detail: DescribeRotationDetailRequest => DescribeRotationDetailResponse = "DescribeRotationDetail";
    describe_rotation_history: DescribeRotationHistoryRequest => DescribeRotationHistoryResponse = "DescribeRotationHistory";
    describe_secret: DescribeSecretRequest => DescribeSecretResponse = "DescribeSecret";
    describe_supported_products: DescribeSupportedProductsRequest => DescribeSupportedProductsResponse = "DescribeSupportedProducts";
    disable_secret: DisableSecretRequest => DisableSecretResponse = "DisableSecret";
    enable_secret: EnableSecretRequest => EnableSecretResponse = "EnableSecret";
    get_regions: GetRegionsRequest => GetRegionsResponse = "GetRegions";
    get_ssh_key_pair_value: GetSshKeyPairValueRequest => GetSshKeyPairValueResponse = "GetSSHKeyPairValue";
    get_secret_value: GetSecretValueRequest => GetSecretValueResponse = "GetSecretValue";
    get_service_status: GetServiceStatusRequest => GetServiceStatusResponse = "GetServiceStatus";
    list_secret_version_ids: ListSecretVersionIdsRequest => ListSecretVersionIdsResponse = "ListSecretVersionIds";
    list_secrets: ListSecretsRequest => ListSecretsResponse = "ListSecrets";
    put_secret_value: PutSecretValueRequest => PutSecretValueResponse = "PutSecretValue";
    restore_secret: RestoreSecretRequest => RestoreSecretResponse = "RestoreSecret";
    rotate_product_secret: RotateProductSecretRequest => RotateProductSecretResponse = "RotateProductSecret";
    update_description: UpdateDescriptionRequest => UpdateDescriptionResponse = "UpdateDescription";
    update_rotation_status: UpdateRotationStatusRequest => UpdateRotationStatusResponse = "UpdateRotationStatus";
    update_secret: UpdateSecretRequest => UpdateSecretResponse = "UpdateSecret";
}
