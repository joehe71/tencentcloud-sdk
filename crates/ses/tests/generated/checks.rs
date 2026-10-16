// Generated exhaustive model round trips and operation routing checks.
#[test]
fn model_abuse_report() {
    round_trip::<models::AbuseReport>(r#"{"DeliverTime":"test-marker","OriginalMailFrom":"test-marker","OriginalRcptTo":"test-marker","FromDomain":"test-marker","ComplainTime":"test-marker","Mta":"test-marker","SourceIp":"test-marker","InsertTime":"test-marker","TemplateId":"test-marker","BulkId":"test-marker","MessageId":"test-marker","AbuseTime":"test-marker","Subject":"test-marker"}"#, r#"{"DeliverTime":null,"OriginalMailFrom":null,"OriginalRcptTo":null,"FromDomain":null,"ComplainTime":null,"Mta":null,"SourceIp":null,"InsertTime":null,"TemplateId":null,"BulkId":null,"MessageId":null,"AbuseTime":null,"Subject":null}"#);
}
#[test]
fn model_address_unsubscribe_config_data() {
    round_trip::<models::AddressUnsubscribeConfigData>(r#"{"Address":"test-marker","UnsubscribeConfig":"test-marker","Status":123}"#, r#"{"Address":null,"UnsubscribeConfig":null,"Status":null}"#);
}
#[test]
fn model_attachment() {
    round_trip::<models::Attachment>(r#"{"FileName":"test-marker","Content":"test-marker","FileURL":"test-marker"}"#, r#"{"FileName":null,"Content":null,"FileURL":null}"#);
}
#[test]
fn model_batch_send_email_request() {
    round_trip::<models::BatchSendEmailRequest>(r#"{"FromEmailAddress":"test-marker","ReceiverId":123,"Subject":"test-marker","TaskType":123,"ReplyToAddresses":"test-marker","Template":{"TemplateID":123,"TemplateData":"test-marker"},"Simple":{"Html":"test-marker","Text":"test-marker"},"Attachments":[{"FileName":"test-marker","Content":"test-marker","FileURL":"test-marker"}],"CycleParam":{"BeginTime":"test-marker","IntervalTime":123,"TermCycle":123},"TimedParam":{"BeginTime":"test-marker"},"Unsubscribe":"test-marker","ADLocation":123}"#, r#"{"FromEmailAddress":null,"ReceiverId":null,"Subject":null,"TaskType":null,"ReplyToAddresses":null,"Template":null,"Simple":null,"Attachments":null,"CycleParam":null,"TimedParam":null,"Unsubscribe":null,"ADLocation":null}"#);
}
#[test]
fn model_batch_send_email_response() {
    round_trip::<models::BatchSendEmailResponse>(r#"{"TaskId":123,"RequestId":"test-marker"}"#, r#"{"TaskId":null,"RequestId":null}"#);
}
#[test]
fn model_black_address_detail() {
    round_trip::<models::BlackAddressDetail>(r#"{"Id":123,"Email":"test-marker","CreateTime":"test-marker","ExpireDate":"test-marker","Status":123}"#, r#"{"Id":null,"Email":null,"CreateTime":null,"ExpireDate":null,"Status":null}"#);
}
#[test]
fn model_black_email_address() {
    round_trip::<models::BlackEmailAddress>(r#"{"BounceTime":"test-marker","EmailAddress":"test-marker","IspDesc":"test-marker"}"#, r#"{"BounceTime":null,"EmailAddress":null,"IspDesc":null}"#);
}
#[test]
fn model_create_address_unsubscribe_config_request() {
    round_trip::<models::CreateAddressUnsubscribeConfigRequest>(r#"{"Address":"test-marker","UnsubscribeConfig":"test-marker","Status":123}"#, r#"{"Address":null,"UnsubscribeConfig":null,"Status":null}"#);
}
#[test]
fn model_create_address_unsubscribe_config_response() {
    round_trip::<models::CreateAddressUnsubscribeConfigResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_create_custom_blacklist_request() {
    round_trip::<models::CreateCustomBlacklistRequest>(r#"{"Emails":["test-marker"],"ExpireDate":"test-marker"}"#, r#"{"Emails":null,"ExpireDate":null}"#);
}
#[test]
fn model_create_custom_blacklist_response() {
    round_trip::<models::CreateCustomBlacklistResponse>(r#"{"TotalCount":123,"ValidCount":123,"TooLongCount":123,"RepeatCount":123,"InvalidCount":123,"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"ValidCount":null,"TooLongCount":null,"RepeatCount":null,"InvalidCount":null,"RequestId":null}"#);
}
#[test]
fn model_create_email_address_request() {
    round_trip::<models::CreateEmailAddressRequest>(r#"{"EmailAddress":"test-marker","EmailSenderName":"test-marker"}"#, r#"{"EmailAddress":null,"EmailSenderName":null}"#);
}
#[test]
fn model_create_email_address_response() {
    round_trip::<models::CreateEmailAddressResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_create_email_identity_request() {
    round_trip::<models::CreateEmailIdentityRequest>(r#"{"EmailIdentity":"test-marker","DKIMOption":123,"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}]}"#, r#"{"EmailIdentity":null,"DKIMOption":null,"TagList":null}"#);
}
#[test]
fn model_create_email_identity_response() {
    round_trip::<models::CreateEmailIdentityResponse>(r#"{"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"RequestId":"test-marker"}"#, r#"{"IdentityType":null,"VerifiedForSendingStatus":null,"Attributes":null,"DKIMOption":null,"RequestId":null}"#);
}
#[test]
fn model_create_email_template_request() {
    round_trip::<models::CreateEmailTemplateRequest>(r#"{"TemplateName":"test-marker","TemplateContent":{"Html":"test-marker","Text":"test-marker"}}"#, r#"{"TemplateName":null,"TemplateContent":null}"#);
}
#[test]
fn model_create_email_template_response() {
    round_trip::<models::CreateEmailTemplateResponse>(r#"{"TemplateID":123,"RequestId":"test-marker"}"#, r#"{"TemplateID":null,"RequestId":null}"#);
}
#[test]
fn model_create_receiver_detail_request() {
    round_trip::<models::CreateReceiverDetailRequest>(r#"{"ReceiverId":123,"Emails":["test-marker"]}"#, r#"{"ReceiverId":null,"Emails":null}"#);
}
#[test]
fn model_create_receiver_detail_response() {
    round_trip::<models::CreateReceiverDetailResponse>(r#"{"TotalCount":123,"ValidCount":123,"TooLongCount":123,"EmptyEmailCount":123,"RepeatCount":123,"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"ValidCount":null,"TooLongCount":null,"EmptyEmailCount":null,"RepeatCount":null,"RequestId":null}"#);
}
#[test]
fn model_create_receiver_detail_with_data_request() {
    round_trip::<models::CreateReceiverDetailWithDataRequest>(r#"{"ReceiverId":123,"Datas":[{"Email":"test-marker","TemplateData":"test-marker"}]}"#, r#"{"ReceiverId":null,"Datas":null}"#);
}
#[test]
fn model_create_receiver_detail_with_data_response() {
    round_trip::<models::CreateReceiverDetailWithDataResponse>(r#"{"TotalCount":123,"ValidCount":123,"TooLongCount":123,"EmptyEmailCount":123,"RepeatCount":123,"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"ValidCount":null,"TooLongCount":null,"EmptyEmailCount":null,"RepeatCount":null,"RequestId":null}"#);
}
#[test]
fn model_create_receiver_request() {
    round_trip::<models::CreateReceiverRequest>(r#"{"ReceiversName":"test-marker","Desc":"test-marker"}"#, r#"{"ReceiversName":null,"Desc":null}"#);
}
#[test]
fn model_create_receiver_response() {
    round_trip::<models::CreateReceiverResponse>(r#"{"ReceiverId":123,"RequestId":"test-marker"}"#, r#"{"ReceiverId":null,"RequestId":null}"#);
}
#[test]
fn model_cycle_email_param() {
    round_trip::<models::CycleEmailParam>(r#"{"BeginTime":"test-marker","IntervalTime":123,"TermCycle":123}"#, r#"{"BeginTime":null,"IntervalTime":null,"TermCycle":null}"#);
}
#[test]
fn model_dns_attributes() {
    round_trip::<models::DnsAttributes>(r#"{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}"#, r#"{"Type":null,"SendDomain":null,"ExpectedValue":null,"CurrentValue":null,"Status":null}"#);
}
#[test]
fn model_delete_address_unsubscribe_config_request() {
    round_trip::<models::DeleteAddressUnsubscribeConfigRequest>(r#"{"Address":"test-marker"}"#, r#"{"Address":null}"#);
}
#[test]
fn model_delete_address_unsubscribe_config_response() {
    round_trip::<models::DeleteAddressUnsubscribeConfigResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_delete_blacklist_request() {
    round_trip::<models::DeleteBlacklistRequest>(r#"{"EmailAddressList":["test-marker"]}"#, r#"{"EmailAddressList":null}"#);
}
#[test]
fn model_delete_blacklist_response() {
    round_trip::<models::DeleteBlacklistResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_delete_custom_blacklist_request() {
    round_trip::<models::DeleteCustomBlacklistRequest>(r#"{"Emails":["test-marker"]}"#, r#"{"Emails":null}"#);
}
#[test]
fn model_delete_custom_blacklist_response() {
    round_trip::<models::DeleteCustomBlacklistResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_delete_email_address_request() {
    round_trip::<models::DeleteEmailAddressRequest>(r#"{"EmailAddress":"test-marker"}"#, r#"{"EmailAddress":null}"#);
}
#[test]
fn model_delete_email_address_response() {
    round_trip::<models::DeleteEmailAddressResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_delete_email_identity_request() {
    round_trip::<models::DeleteEmailIdentityRequest>(r#"{"EmailIdentity":"test-marker"}"#, r#"{"EmailIdentity":null}"#);
}
#[test]
fn model_delete_email_identity_response() {
    round_trip::<models::DeleteEmailIdentityResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_delete_email_template_request() {
    round_trip::<models::DeleteEmailTemplateRequest>(r#"{"TemplateID":123}"#, r#"{"TemplateID":null}"#);
}
#[test]
fn model_delete_email_template_response() {
    round_trip::<models::DeleteEmailTemplateResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_delete_receiver_request() {
    round_trip::<models::DeleteReceiverRequest>(r#"{"ReceiverId":123}"#, r#"{"ReceiverId":null}"#);
}
#[test]
fn model_delete_receiver_response() {
    round_trip::<models::DeleteReceiverResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_email_identity() {
    round_trip::<models::EmailIdentity>(r#"{"IdentityName":"test-marker","IdentityType":"test-marker","SendingEnabled":true,"CurrentReputationLevel":123,"DailyQuota":123,"SendIp":["test-marker"],"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"DKIMOption":123}"#, r#"{"IdentityName":null,"IdentityType":null,"SendingEnabled":null,"CurrentReputationLevel":null,"DailyQuota":null,"SendIp":null,"TagList":null,"DKIMOption":null}"#);
}
#[test]
fn model_email_sender() {
    round_trip::<models::EmailSender>(r#"{"EmailAddress":"test-marker","EmailSenderName":"test-marker","CreatedTimestamp":123,"SmtpPwdType":123}"#, r#"{"EmailAddress":null,"EmailSenderName":null,"CreatedTimestamp":null,"SmtpPwdType":null}"#);
}
#[test]
fn model_get_abuse_report_request() {
    round_trip::<models::GetAbuseReportRequest>(r#"{"StartTime":"test-marker","EndTime":"test-marker","Offset":123,"Limit":123,"FromDomain":"test-marker","FromAddress":"test-marker","Mta":"test-marker","ToAddress":"test-marker","TemplateId":"test-marker"}"#, r#"{"StartTime":null,"EndTime":null,"Offset":null,"Limit":null,"FromDomain":null,"FromAddress":null,"Mta":null,"ToAddress":null,"TemplateId":null}"#);
}
#[test]
fn model_get_abuse_report_response() {
    round_trip::<models::GetAbuseReportResponse>(r#"{"Data":[{"DeliverTime":"test-marker","OriginalMailFrom":"test-marker","OriginalRcptTo":"test-marker","FromDomain":"test-marker","ComplainTime":"test-marker","Mta":"test-marker","SourceIp":"test-marker","InsertTime":"test-marker","TemplateId":"test-marker","BulkId":"test-marker","MessageId":"test-marker","AbuseTime":"test-marker","Subject":"test-marker"}],"TotalCount":123,"RequestId":"test-marker"}"#, r#"{"Data":null,"TotalCount":null,"RequestId":null}"#);
}
#[test]
fn model_get_email_identity_request() {
    round_trip::<models::GetEmailIdentityRequest>(r#"{"EmailIdentity":"test-marker"}"#, r#"{"EmailIdentity":null}"#);
}
#[test]
fn model_get_email_identity_response() {
    round_trip::<models::GetEmailIdentityResponse>(r#"{"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"RequestId":"test-marker"}"#, r#"{"IdentityType":null,"VerifiedForSendingStatus":null,"Attributes":null,"DKIMOption":null,"TagList":null,"RequestId":null}"#);
}
#[test]
fn model_get_email_template_request() {
    round_trip::<models::GetEmailTemplateRequest>(r#"{"TemplateID":123}"#, r#"{"TemplateID":null}"#);
}
#[test]
fn model_get_email_template_response() {
    round_trip::<models::GetEmailTemplateResponse>(r#"{"TemplateContent":{"Html":"test-marker","Text":"test-marker"},"TemplateStatus":123,"TemplateName":"test-marker","RequestId":"test-marker"}"#, r#"{"TemplateContent":null,"TemplateStatus":null,"TemplateName":null,"RequestId":null}"#);
}
#[test]
fn model_get_send_email_status_request() {
    round_trip::<models::GetSendEmailStatusRequest>(r#"{"RequestDate":"test-marker","Offset":123,"Limit":123,"MessageId":"test-marker","ToEmailAddress":"test-marker"}"#, r#"{"RequestDate":null,"Offset":null,"Limit":null,"MessageId":null,"ToEmailAddress":null}"#);
}
#[test]
fn model_get_send_email_status_response() {
    round_trip::<models::GetSendEmailStatusResponse>(r#"{"EmailStatusList":[{"MessageId":"test-marker","ToEmailAddress":"test-marker","FromEmailAddress":"test-marker","SendStatus":123,"DeliverStatus":123,"DeliverMessage":"test-marker","RequestTime":123,"DeliverTime":123,"UserOpened":true,"UserClicked":true,"UserUnsubscribed":true,"UserComplainted":true,"UserComplained":true}],"RequestId":"test-marker"}"#, r#"{"EmailStatusList":null,"RequestId":null}"#);
}
#[test]
fn model_get_statistics_report_request() {
    round_trip::<models::GetStatisticsReportRequest>(r#"{"StartDate":"test-marker","EndDate":"test-marker","Domain":"test-marker","ReceivingMailboxType":"test-marker"}"#, r#"{"StartDate":null,"EndDate":null,"Domain":null,"ReceivingMailboxType":null}"#);
}
#[test]
fn model_get_statistics_report_response() {
    round_trip::<models::GetStatisticsReportResponse>(r#"{"DailyVolumes":[{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123}],"OverallVolume":{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123},"RequestId":"test-marker"}"#, r#"{"DailyVolumes":null,"OverallVolume":null,"RequestId":null}"#);
}
#[test]
fn model_list_address_unsubscribe_config_request() {
    round_trip::<models::ListAddressUnsubscribeConfigRequest>(r#"{"Offset":123,"Limit":"test-marker"}"#, r#"{"Offset":null,"Limit":null}"#);
}
#[test]
fn model_list_address_unsubscribe_config_response() {
    round_trip::<models::ListAddressUnsubscribeConfigResponse>(r#"{"AddressUnsubscribeConfigList":[{"Address":"test-marker","UnsubscribeConfig":"test-marker","Status":123}],"Total":123,"RequestId":"test-marker"}"#, r#"{"AddressUnsubscribeConfigList":null,"Total":null,"RequestId":null}"#);
}
#[test]
fn model_list_black_email_address_request() {
    round_trip::<models::ListBlackEmailAddressRequest>(r#"{"StartDate":"test-marker","EndDate":"test-marker","Limit":123,"Offset":123,"EmailAddress":"test-marker","TaskID":"test-marker"}"#, r#"{"StartDate":null,"EndDate":null,"Limit":null,"Offset":null,"EmailAddress":null,"TaskID":null}"#);
}
#[test]
fn model_list_black_email_address_response() {
    round_trip::<models::ListBlackEmailAddressResponse>(r#"{"BlackList":[{"BounceTime":"test-marker","EmailAddress":"test-marker","IspDesc":"test-marker"}],"TotalCount":123,"RequestId":"test-marker"}"#, r#"{"BlackList":null,"TotalCount":null,"RequestId":null}"#);
}
#[test]
fn model_list_custom_blacklist_request() {
    round_trip::<models::ListCustomBlacklistRequest>(r#"{"Offset":123,"Limit":123,"Status":123,"Email":"test-marker"}"#, r#"{"Offset":null,"Limit":null,"Status":null,"Email":null}"#);
}
#[test]
fn model_list_custom_blacklist_response() {
    round_trip::<models::ListCustomBlacklistResponse>(r#"{"TotalCount":123,"Data":[{"Id":123,"Email":"test-marker","CreateTime":"test-marker","ExpireDate":"test-marker","Status":123}],"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"Data":null,"RequestId":null}"#);
}
#[test]
fn model_list_email_address_request() {
    round_trip::<models::ListEmailAddressRequest>("{}", "{}");
}
#[test]
fn model_list_email_address_response() {
    round_trip::<models::ListEmailAddressResponse>(r#"{"EmailSenders":[{"EmailAddress":"test-marker","EmailSenderName":"test-marker","CreatedTimestamp":123,"SmtpPwdType":123}],"RequestId":"test-marker"}"#, r#"{"EmailSenders":null,"RequestId":null}"#);
}
#[test]
fn model_list_email_identities_request() {
    round_trip::<models::ListEmailIdentitiesRequest>(r#"{"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"Limit":123,"Offset":123}"#, r#"{"TagList":null,"Limit":null,"Offset":null}"#);
}
#[test]
fn model_list_email_identities_response() {
    round_trip::<models::ListEmailIdentitiesResponse>(r#"{"EmailIdentities":[{"IdentityName":"test-marker","IdentityType":"test-marker","SendingEnabled":true,"CurrentReputationLevel":123,"DailyQuota":123,"SendIp":["test-marker"],"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"DKIMOption":123}],"MaxReputationLevel":123,"MaxDailyQuota":123,"Total":123,"RequestId":"test-marker"}"#, r#"{"EmailIdentities":null,"MaxReputationLevel":null,"MaxDailyQuota":null,"Total":null,"RequestId":null}"#);
}
#[test]
fn model_list_email_templates_request() {
    round_trip::<models::ListEmailTemplatesRequest>(r#"{"Limit":123,"Offset":123}"#, r#"{"Limit":null,"Offset":null}"#);
}
#[test]
fn model_list_email_templates_response() {
    round_trip::<models::ListEmailTemplatesResponse>(r#"{"TemplatesMetadata":[{"CreatedTimestamp":123,"TemplateName":"test-marker","TemplateStatus":123,"TemplateID":123,"ReviewReason":"test-marker"}],"TotalCount":123,"RequestId":"test-marker"}"#, r#"{"TemplatesMetadata":null,"TotalCount":null,"RequestId":null}"#);
}
#[test]
fn model_list_receiver_details_request() {
    round_trip::<models::ListReceiverDetailsRequest>(r#"{"ReceiverId":123,"Offset":123,"Limit":123,"Email":"test-marker","CreateTimeBegin":"test-marker","CreateTimeEnd":"test-marker","Status":123}"#, r#"{"ReceiverId":null,"Offset":null,"Limit":null,"Email":null,"CreateTimeBegin":null,"CreateTimeEnd":null,"Status":null}"#);
}
#[test]
fn model_list_receiver_details_response() {
    round_trip::<models::ListReceiverDetailsResponse>(r#"{"TotalCount":123,"Data":[{"Email":"test-marker","CreateTime":"test-marker","TemplateData":"test-marker","Reason":"test-marker","Status":123,"EmailId":123}],"ValidCount":123,"InvalidCount":123,"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"Data":null,"ValidCount":null,"InvalidCount":null,"RequestId":null}"#);
}
#[test]
fn model_list_receivers_request() {
    round_trip::<models::ListReceiversRequest>(r#"{"Offset":123,"Limit":123,"Status":123,"KeyWord":"test-marker"}"#, r#"{"Offset":null,"Limit":null,"Status":null,"KeyWord":null}"#);
}
#[test]
fn model_list_receivers_response() {
    round_trip::<models::ListReceiversResponse>(r#"{"TotalCount":123,"Data":[{"ReceiverId":123,"ReceiversName":"test-marker","Count":123,"Desc":"test-marker","ReceiversStatus":123,"CreateTime":"test-marker","InvalidCount":123}],"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"Data":null,"RequestId":null}"#);
}
#[test]
fn model_list_send_tasks_request() {
    round_trip::<models::ListSendTasksRequest>(r#"{"Offset":123,"Limit":123,"Status":123,"ReceiverId":123,"TaskType":123}"#, r#"{"Offset":null,"Limit":null,"Status":null,"ReceiverId":null,"TaskType":null}"#);
}
#[test]
fn model_list_send_tasks_response() {
    round_trip::<models::ListSendTasksResponse>(r#"{"TotalCount":123,"Data":[{"TaskId":123,"FromEmailAddress":"test-marker","ReceiverId":123,"TaskStatus":123,"TaskType":123,"RequestCount":123,"SendCount":123,"CacheCount":123,"CreateTime":"test-marker","UpdateTime":"test-marker","Subject":"test-marker","Template":{"TemplateID":123,"TemplateData":"test-marker"},"CycleParam":{"BeginTime":"test-marker","IntervalTime":123,"TermCycle":123},"TimedParam":{"BeginTime":"test-marker"},"ErrMsg":"test-marker","ReceiversName":"test-marker"}],"RequestId":"test-marker"}"#, r#"{"TotalCount":null,"Data":null,"RequestId":null}"#);
}
#[test]
fn model_receiver_data() {
    round_trip::<models::ReceiverData>(r#"{"ReceiverId":123,"ReceiversName":"test-marker","Count":123,"Desc":"test-marker","ReceiversStatus":123,"CreateTime":"test-marker","InvalidCount":123}"#, r#"{"ReceiverId":null,"ReceiversName":null,"Count":null,"Desc":null,"ReceiversStatus":null,"CreateTime":null,"InvalidCount":null}"#);
}
#[test]
fn model_receiver_detail() {
    round_trip::<models::ReceiverDetail>(r#"{"Email":"test-marker","CreateTime":"test-marker","TemplateData":"test-marker","Reason":"test-marker","Status":123,"EmailId":123}"#, r#"{"Email":null,"CreateTime":null,"TemplateData":null,"Reason":null,"Status":null,"EmailId":null}"#);
}
#[test]
fn model_receiver_input_data() {
    round_trip::<models::ReceiverInputData>(r#"{"Email":"test-marker","TemplateData":"test-marker"}"#, r#"{"Email":null,"TemplateData":null}"#);
}
#[test]
fn model_send_email_request() {
    round_trip::<models::SendEmailRequest>(r#"{"FromEmailAddress":"test-marker","Subject":"test-marker","Destination":["test-marker"],"ReplyToAddresses":"test-marker","Cc":["test-marker"],"Bcc":["test-marker"],"Template":{"TemplateID":123,"TemplateData":"test-marker"},"Simple":{"Html":"test-marker","Text":"test-marker"},"Attachments":[{"FileName":"test-marker","Content":"test-marker","FileURL":"test-marker"}],"Unsubscribe":"test-marker","TriggerType":123,"SmtpMessageId":"test-marker","SmtpHeaders":"test-marker","HeaderFrom":"test-marker"}"#, r#"{"FromEmailAddress":null,"Subject":null,"Destination":null,"ReplyToAddresses":null,"Cc":null,"Bcc":null,"Template":null,"Simple":null,"Attachments":null,"Unsubscribe":null,"TriggerType":null,"SmtpMessageId":null,"SmtpHeaders":null,"HeaderFrom":null}"#);
}
#[test]
fn model_send_email_response() {
    round_trip::<models::SendEmailResponse>(r#"{"MessageId":"test-marker","RequestId":"test-marker"}"#, r#"{"MessageId":null,"RequestId":null}"#);
}
#[test]
fn model_send_email_status() {
    round_trip::<models::SendEmailStatus>(r#"{"MessageId":"test-marker","ToEmailAddress":"test-marker","FromEmailAddress":"test-marker","SendStatus":123,"DeliverStatus":123,"DeliverMessage":"test-marker","RequestTime":123,"DeliverTime":123,"UserOpened":true,"UserClicked":true,"UserUnsubscribed":true,"UserComplainted":true,"UserComplained":true}"#, r#"{"MessageId":null,"ToEmailAddress":null,"FromEmailAddress":null,"SendStatus":null,"DeliverStatus":null,"DeliverMessage":null,"RequestTime":null,"DeliverTime":null,"UserOpened":null,"UserClicked":null,"UserUnsubscribed":null,"UserComplainted":null,"UserComplained":null}"#);
}
#[test]
fn model_send_task_data() {
    round_trip::<models::SendTaskData>(r#"{"TaskId":123,"FromEmailAddress":"test-marker","ReceiverId":123,"TaskStatus":123,"TaskType":123,"RequestCount":123,"SendCount":123,"CacheCount":123,"CreateTime":"test-marker","UpdateTime":"test-marker","Subject":"test-marker","Template":{"TemplateID":123,"TemplateData":"test-marker"},"CycleParam":{"BeginTime":"test-marker","IntervalTime":123,"TermCycle":123},"TimedParam":{"BeginTime":"test-marker"},"ErrMsg":"test-marker","ReceiversName":"test-marker"}"#, r#"{"TaskId":null,"FromEmailAddress":null,"ReceiverId":null,"TaskStatus":null,"TaskType":null,"RequestCount":null,"SendCount":null,"CacheCount":null,"CreateTime":null,"UpdateTime":null,"Subject":null,"Template":null,"CycleParam":null,"TimedParam":null,"ErrMsg":null,"ReceiversName":null}"#);
}
#[test]
fn model_simple() {
    round_trip::<models::Simple>(r#"{"Html":"test-marker","Text":"test-marker"}"#, r#"{"Html":null,"Text":null}"#);
}
#[test]
fn model_tag_list() {
    round_trip::<models::TagList>(r#"{"TagKey":"test-marker","TagValue":"test-marker"}"#, r#"{"TagKey":null,"TagValue":null}"#);
}
#[test]
fn model_template() {
    round_trip::<models::Template>(r#"{"TemplateID":123,"TemplateData":"test-marker"}"#, r#"{"TemplateID":null,"TemplateData":null}"#);
}
#[test]
fn model_template_content() {
    round_trip::<models::TemplateContent>(r#"{"Html":"test-marker","Text":"test-marker"}"#, r#"{"Html":null,"Text":null}"#);
}
#[test]
fn model_templates_metadata() {
    round_trip::<models::TemplatesMetadata>(r#"{"CreatedTimestamp":123,"TemplateName":"test-marker","TemplateStatus":123,"TemplateID":123,"ReviewReason":"test-marker"}"#, r#"{"CreatedTimestamp":null,"TemplateName":null,"TemplateStatus":null,"TemplateID":null,"ReviewReason":null}"#);
}
#[test]
fn model_timed_email_param() {
    round_trip::<models::TimedEmailParam>(r#"{"BeginTime":"test-marker"}"#, r#"{"BeginTime":null}"#);
}
#[test]
fn model_update_address_unsubscribe_config_request() {
    round_trip::<models::UpdateAddressUnsubscribeConfigRequest>(r#"{"Address":"test-marker","UnsubscribeConfig":"test-marker","Status":123}"#, r#"{"Address":null,"UnsubscribeConfig":null,"Status":null}"#);
}
#[test]
fn model_update_address_unsubscribe_config_response() {
    round_trip::<models::UpdateAddressUnsubscribeConfigResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_update_custom_blacklist_request() {
    round_trip::<models::UpdateCustomBlacklistRequest>(r#"{"Id":123,"Email":"test-marker","ExpireDate":"test-marker"}"#, r#"{"Id":null,"Email":null,"ExpireDate":null}"#);
}
#[test]
fn model_update_custom_blacklist_response() {
    round_trip::<models::UpdateCustomBlacklistResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_update_email_identity_request() {
    round_trip::<models::UpdateEmailIdentityRequest>(r#"{"EmailIdentity":"test-marker","NewAPI":true,"DKIMOption":123}"#, r#"{"EmailIdentity":null,"NewAPI":null,"DKIMOption":null}"#);
}
#[test]
fn model_update_email_identity_response() {
    round_trip::<models::UpdateEmailIdentityResponse>(r#"{"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"RequestId":"test-marker"}"#, r#"{"IdentityType":null,"VerifiedForSendingStatus":null,"Attributes":null,"DKIMOption":null,"RequestId":null}"#);
}
#[test]
fn model_update_email_smtp_password_request() {
    round_trip::<models::UpdateEmailSmtpPasswordRequest>(r#"{"Password":"test-marker","EmailAddress":"test-marker"}"#, r#"{"Password":null,"EmailAddress":null}"#);
}
#[test]
fn model_update_email_smtp_password_response() {
    round_trip::<models::UpdateEmailSmtpPasswordResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_update_email_template_request() {
    round_trip::<models::UpdateEmailTemplateRequest>(r#"{"TemplateContent":{"Html":"test-marker","Text":"test-marker"},"TemplateID":123,"TemplateName":"test-marker"}"#, r#"{"TemplateContent":null,"TemplateID":null,"TemplateName":null}"#);
}
#[test]
fn model_update_email_template_response() {
    round_trip::<models::UpdateEmailTemplateResponse>(r#"{"RequestId":"test-marker"}"#, r#"{"RequestId":null}"#);
}
#[test]
fn model_volume() {
    round_trip::<models::Volume>(r#"{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123}"#, r#"{"SendDate":null,"RequestCount":null,"AcceptedCount":null,"DeliveredCount":null,"OpenedCount":null,"ClickedCount":null,"BounceCount":null,"UnsubscribeCount":null}"#);
}
#[tokio::test]
async fn route_batch_send_email() {
      let (client, calls) = mock_client("BatchSendEmail", serde_json::json!({"Response": {"TaskId":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.batch_send_email(&models::BatchSendEmailRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TaskId":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_address_unsubscribe_config() {
      let (client, calls) = mock_client("CreateAddressUnsubscribeConfig", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_address_unsubscribe_config(&models::CreateAddressUnsubscribeConfigRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_custom_blacklist() {
      let (client, calls) = mock_client("CreateCustomBlacklist", serde_json::json!({"Response": {"TotalCount":123,"ValidCount":123,"TooLongCount":123,"RepeatCount":123,"InvalidCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_custom_blacklist(&models::CreateCustomBlacklistRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"ValidCount":123,"TooLongCount":123,"RepeatCount":123,"InvalidCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_email_address() {
      let (client, calls) = mock_client("CreateEmailAddress", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_email_address(&models::CreateEmailAddressRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_email_identity() {
      let (client, calls) = mock_client("CreateEmailIdentity", serde_json::json!({"Response": {"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_email_identity(&models::CreateEmailIdentityRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_email_template() {
      let (client, calls) = mock_client("CreateEmailTemplate", serde_json::json!({"Response": {"TemplateID":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_email_template(&models::CreateEmailTemplateRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TemplateID":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_receiver() {
      let (client, calls) = mock_client("CreateReceiver", serde_json::json!({"Response": {"ReceiverId":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_receiver(&models::CreateReceiverRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"ReceiverId":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_receiver_detail() {
      let (client, calls) = mock_client("CreateReceiverDetail", serde_json::json!({"Response": {"TotalCount":123,"ValidCount":123,"TooLongCount":123,"EmptyEmailCount":123,"RepeatCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_receiver_detail(&models::CreateReceiverDetailRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"ValidCount":123,"TooLongCount":123,"EmptyEmailCount":123,"RepeatCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_create_receiver_detail_with_data() {
      let (client, calls) = mock_client("CreateReceiverDetailWithData", serde_json::json!({"Response": {"TotalCount":123,"ValidCount":123,"TooLongCount":123,"EmptyEmailCount":123,"RepeatCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.create_receiver_detail_with_data(&models::CreateReceiverDetailWithDataRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"ValidCount":123,"TooLongCount":123,"EmptyEmailCount":123,"RepeatCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_address_unsubscribe_config() {
      let (client, calls) = mock_client("DeleteAddressUnsubscribeConfig", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_address_unsubscribe_config(&models::DeleteAddressUnsubscribeConfigRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_blacklist() {
      let (client, calls) = mock_client("DeleteBlackList", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_blacklist(&models::DeleteBlacklistRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_custom_blacklist() {
      let (client, calls) = mock_client("DeleteCustomBlackList", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_custom_blacklist(&models::DeleteCustomBlacklistRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_email_address() {
      let (client, calls) = mock_client("DeleteEmailAddress", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_email_address(&models::DeleteEmailAddressRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_email_identity() {
      let (client, calls) = mock_client("DeleteEmailIdentity", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_email_identity(&models::DeleteEmailIdentityRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_email_template() {
      let (client, calls) = mock_client("DeleteEmailTemplate", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_email_template(&models::DeleteEmailTemplateRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_delete_receiver() {
      let (client, calls) = mock_client("DeleteReceiver", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.delete_receiver(&models::DeleteReceiverRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_abuse_report() {
      let (client, calls) = mock_client("GetAbuseReport", serde_json::json!({"Response": {"Data":[{"DeliverTime":"test-marker","OriginalMailFrom":"test-marker","OriginalRcptTo":"test-marker","FromDomain":"test-marker","ComplainTime":"test-marker","Mta":"test-marker","SourceIp":"test-marker","InsertTime":"test-marker","TemplateId":"test-marker","BulkId":"test-marker","MessageId":"test-marker","AbuseTime":"test-marker","Subject":"test-marker"}],"TotalCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_abuse_report(&models::GetAbuseReportRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"Data":[{"DeliverTime":"test-marker","OriginalMailFrom":"test-marker","OriginalRcptTo":"test-marker","FromDomain":"test-marker","ComplainTime":"test-marker","Mta":"test-marker","SourceIp":"test-marker","InsertTime":"test-marker","TemplateId":"test-marker","BulkId":"test-marker","MessageId":"test-marker","AbuseTime":"test-marker","Subject":"test-marker"}],"TotalCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_email_identity() {
      let (client, calls) = mock_client("GetEmailIdentity", serde_json::json!({"Response": {"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_email_identity(&models::GetEmailIdentityRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_email_template() {
      let (client, calls) = mock_client("GetEmailTemplate", serde_json::json!({"Response": {"TemplateContent":{"Html":"test-marker","Text":"test-marker"},"TemplateStatus":123,"TemplateName":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_email_template(&models::GetEmailTemplateRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TemplateContent":{"Html":"test-marker","Text":"test-marker"},"TemplateStatus":123,"TemplateName":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_send_email_status() {
      let (client, calls) = mock_client("GetSendEmailStatus", serde_json::json!({"Response": {"EmailStatusList":[{"MessageId":"test-marker","ToEmailAddress":"test-marker","FromEmailAddress":"test-marker","SendStatus":123,"DeliverStatus":123,"DeliverMessage":"test-marker","RequestTime":123,"DeliverTime":123,"UserOpened":true,"UserClicked":true,"UserUnsubscribed":true,"UserComplainted":true,"UserComplained":true}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_send_email_status(&models::GetSendEmailStatusRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"EmailStatusList":[{"MessageId":"test-marker","ToEmailAddress":"test-marker","FromEmailAddress":"test-marker","SendStatus":123,"DeliverStatus":123,"DeliverMessage":"test-marker","RequestTime":123,"DeliverTime":123,"UserOpened":true,"UserClicked":true,"UserUnsubscribed":true,"UserComplainted":true,"UserComplained":true}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_get_statistics_report() {
      let (client, calls) = mock_client("GetStatisticsReport", serde_json::json!({"Response": {"DailyVolumes":[{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123}],"OverallVolume":{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123},"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.get_statistics_report(&models::GetStatisticsReportRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"DailyVolumes":[{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123}],"OverallVolume":{"SendDate":"test-marker","RequestCount":123,"AcceptedCount":123,"DeliveredCount":123,"OpenedCount":123,"ClickedCount":123,"BounceCount":123,"UnsubscribeCount":123},"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_address_unsubscribe_config() {
      let (client, calls) = mock_client("ListAddressUnsubscribeConfig", serde_json::json!({"Response": {"AddressUnsubscribeConfigList":[{"Address":"test-marker","UnsubscribeConfig":"test-marker","Status":123}],"Total":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_address_unsubscribe_config(&models::ListAddressUnsubscribeConfigRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"AddressUnsubscribeConfigList":[{"Address":"test-marker","UnsubscribeConfig":"test-marker","Status":123}],"Total":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_black_email_address() {
      let (client, calls) = mock_client("ListBlackEmailAddress", serde_json::json!({"Response": {"BlackList":[{"BounceTime":"test-marker","EmailAddress":"test-marker","IspDesc":"test-marker"}],"TotalCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_black_email_address(&models::ListBlackEmailAddressRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"BlackList":[{"BounceTime":"test-marker","EmailAddress":"test-marker","IspDesc":"test-marker"}],"TotalCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_custom_blacklist() {
      let (client, calls) = mock_client("ListCustomBlacklist", serde_json::json!({"Response": {"TotalCount":123,"Data":[{"Id":123,"Email":"test-marker","CreateTime":"test-marker","ExpireDate":"test-marker","Status":123}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_custom_blacklist(&models::ListCustomBlacklistRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"Data":[{"Id":123,"Email":"test-marker","CreateTime":"test-marker","ExpireDate":"test-marker","Status":123}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_email_address() {
      let (client, calls) = mock_client("ListEmailAddress", serde_json::json!({"Response": {"EmailSenders":[{"EmailAddress":"test-marker","EmailSenderName":"test-marker","CreatedTimestamp":123,"SmtpPwdType":123}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_email_address(&models::ListEmailAddressRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"EmailSenders":[{"EmailAddress":"test-marker","EmailSenderName":"test-marker","CreatedTimestamp":123,"SmtpPwdType":123}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_email_identities() {
      let (client, calls) = mock_client("ListEmailIdentities", serde_json::json!({"Response": {"EmailIdentities":[{"IdentityName":"test-marker","IdentityType":"test-marker","SendingEnabled":true,"CurrentReputationLevel":123,"DailyQuota":123,"SendIp":["test-marker"],"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"DKIMOption":123}],"MaxReputationLevel":123,"MaxDailyQuota":123,"Total":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_email_identities(&models::ListEmailIdentitiesRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"EmailIdentities":[{"IdentityName":"test-marker","IdentityType":"test-marker","SendingEnabled":true,"CurrentReputationLevel":123,"DailyQuota":123,"SendIp":["test-marker"],"TagList":[{"TagKey":"test-marker","TagValue":"test-marker"}],"DKIMOption":123}],"MaxReputationLevel":123,"MaxDailyQuota":123,"Total":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_email_templates() {
      let (client, calls) = mock_client("ListEmailTemplates", serde_json::json!({"Response": {"TemplatesMetadata":[{"CreatedTimestamp":123,"TemplateName":"test-marker","TemplateStatus":123,"TemplateID":123,"ReviewReason":"test-marker"}],"TotalCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_email_templates(&models::ListEmailTemplatesRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TemplatesMetadata":[{"CreatedTimestamp":123,"TemplateName":"test-marker","TemplateStatus":123,"TemplateID":123,"ReviewReason":"test-marker"}],"TotalCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_receiver_details() {
      let (client, calls) = mock_client("ListReceiverDetails", serde_json::json!({"Response": {"TotalCount":123,"Data":[{"Email":"test-marker","CreateTime":"test-marker","TemplateData":"test-marker","Reason":"test-marker","Status":123,"EmailId":123}],"ValidCount":123,"InvalidCount":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_receiver_details(&models::ListReceiverDetailsRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"Data":[{"Email":"test-marker","CreateTime":"test-marker","TemplateData":"test-marker","Reason":"test-marker","Status":123,"EmailId":123}],"ValidCount":123,"InvalidCount":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_receivers() {
      let (client, calls) = mock_client("ListReceivers", serde_json::json!({"Response": {"TotalCount":123,"Data":[{"ReceiverId":123,"ReceiversName":"test-marker","Count":123,"Desc":"test-marker","ReceiversStatus":123,"CreateTime":"test-marker","InvalidCount":123}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_receivers(&models::ListReceiversRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"Data":[{"ReceiverId":123,"ReceiversName":"test-marker","Count":123,"Desc":"test-marker","ReceiversStatus":123,"CreateTime":"test-marker","InvalidCount":123}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_list_send_tasks() {
      let (client, calls) = mock_client("ListSendTasks", serde_json::json!({"Response": {"TotalCount":123,"Data":[{"TaskId":123,"FromEmailAddress":"test-marker","ReceiverId":123,"TaskStatus":123,"TaskType":123,"RequestCount":123,"SendCount":123,"CacheCount":123,"CreateTime":"test-marker","UpdateTime":"test-marker","Subject":"test-marker","Template":{"TemplateID":123,"TemplateData":"test-marker"},"CycleParam":{"BeginTime":"test-marker","IntervalTime":123,"TermCycle":123},"TimedParam":{"BeginTime":"test-marker"},"ErrMsg":"test-marker","ReceiversName":"test-marker"}],"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.list_send_tasks(&models::ListSendTasksRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"TotalCount":123,"Data":[{"TaskId":123,"FromEmailAddress":"test-marker","ReceiverId":123,"TaskStatus":123,"TaskType":123,"RequestCount":123,"SendCount":123,"CacheCount":123,"CreateTime":"test-marker","UpdateTime":"test-marker","Subject":"test-marker","Template":{"TemplateID":123,"TemplateData":"test-marker"},"CycleParam":{"BeginTime":"test-marker","IntervalTime":123,"TermCycle":123},"TimedParam":{"BeginTime":"test-marker"},"ErrMsg":"test-marker","ReceiversName":"test-marker"}],"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_send_email() {
      let (client, calls) = mock_client("SendEmail", serde_json::json!({"Response": {"MessageId":"test-marker","RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.send_email(&models::SendEmailRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"MessageId":"test-marker","RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_address_unsubscribe_config() {
      let (client, calls) = mock_client("UpdateAddressUnsubscribeConfig", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_address_unsubscribe_config(&models::UpdateAddressUnsubscribeConfigRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_custom_blacklist() {
      let (client, calls) = mock_client("UpdateCustomBlackList", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_custom_blacklist(&models::UpdateCustomBlacklistRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_email_identity() {
      let (client, calls) = mock_client("UpdateEmailIdentity", serde_json::json!({"Response": {"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_email_identity(&models::UpdateEmailIdentityRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"IdentityType":"test-marker","VerifiedForSendingStatus":true,"Attributes":[{"Type":"test-marker","SendDomain":"test-marker","ExpectedValue":"test-marker","CurrentValue":"test-marker","Status":true}],"DKIMOption":123,"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_email_smtp_password() {
      let (client, calls) = mock_client("UpdateEmailSmtpPassWord", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_email_smtp_password(&models::UpdateEmailSmtpPasswordRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
#[tokio::test]
async fn route_update_email_template() {
      let (client, calls) = mock_client("UpdateEmailTemplate", serde_json::json!({"Response": {"RequestId":"test-request-id"}}), http::StatusCode::OK);
      let result = client.update_email_template(&models::UpdateEmailTemplateRequest::default()).await.unwrap();
      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));
      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({"RequestId":"test-request-id"}));
      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }
