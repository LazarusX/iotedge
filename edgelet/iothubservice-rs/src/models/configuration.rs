/*
 * IotHub Gateway Service APIs
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: Service
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "SchemaVersion", skip_serializing_if = "Option::is_none")]
    schema_version: Option<String>,
    #[serde(rename = "InternalVersion", skip_serializing_if = "Option::is_none")]
    internal_version: Option<String>,
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    content: Option<::models::ConfigurationContent>,
    #[serde(rename = "ContentType", skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
    #[serde(rename = "TargetCondition", skip_serializing_if = "Option::is_none")]
    target_condition: Option<String>,
    #[serde(rename = "CreatedTimeUtc", skip_serializing_if = "Option::is_none")]
    created_time_utc: Option<String>,
    #[serde(rename = "LastUpdatedTimeUtc", skip_serializing_if = "Option::is_none")]
    last_updated_time_utc: Option<String>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    priority: Option<i32>,
    #[serde(rename = "SystemMetrics", skip_serializing_if = "Option::is_none")]
    system_metrics: Option<::models::ConfigurationMetrics>,
    #[serde(rename = "Metrics", skip_serializing_if = "Option::is_none")]
    metrics: Option<::models::ConfigurationMetrics>,
    #[serde(rename = "ETag", skip_serializing_if = "Option::is_none")]
    e_tag: Option<String>,
    #[serde(rename = "ConfigurationType", skip_serializing_if = "Option::is_none")]
    configuration_type: Option<String>,
    #[serde(rename = "BodyQuery", skip_serializing_if = "Option::is_none")]
    body_query: Option<String>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(rename = "MetricBodyQueries", skip_serializing_if = "Option::is_none")]
    metric_body_queries:
        Option<::std::collections::HashMap<String, ::models::ConfigurationBodyQuery>>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    version: Option<i32>,
    #[serde(rename = "IsEdgeDeployment", skip_serializing_if = "Option::is_none")]
    is_edge_deployment: Option<bool>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            id: None,
            schema_version: None,
            internal_version: None,
            labels: None,
            content: None,
            content_type: None,
            target_condition: None,
            created_time_utc: None,
            last_updated_time_utc: None,
            priority: None,
            system_metrics: None,
            metrics: None,
            e_tag: None,
            configuration_type: None,
            body_query: None,
            target: None,
            metric_body_queries: None,
            version: None,
            is_edge_deployment: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: String) -> Configuration {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_schema_version(&mut self, schema_version: String) {
        self.schema_version = Some(schema_version);
    }

    pub fn with_schema_version(mut self, schema_version: String) -> Configuration {
        self.schema_version = Some(schema_version);
        self
    }

    pub fn schema_version(&self) -> Option<&String> {
        self.schema_version.as_ref()
    }

    pub fn reset_schema_version(&mut self) {
        self.schema_version = None;
    }

    pub fn set_internal_version(&mut self, internal_version: String) {
        self.internal_version = Some(internal_version);
    }

    pub fn with_internal_version(mut self, internal_version: String) -> Configuration {
        self.internal_version = Some(internal_version);
        self
    }

    pub fn internal_version(&self) -> Option<&String> {
        self.internal_version.as_ref()
    }

    pub fn reset_internal_version(&mut self) {
        self.internal_version = None;
    }

    pub fn set_labels(&mut self, labels: ::std::collections::HashMap<String, String>) {
        self.labels = Some(labels);
    }

    pub fn with_labels(
        mut self,
        labels: ::std::collections::HashMap<String, String>,
    ) -> Configuration {
        self.labels = Some(labels);
        self
    }

    pub fn labels(&self) -> Option<&::std::collections::HashMap<String, String>> {
        self.labels.as_ref()
    }

    pub fn reset_labels(&mut self) {
        self.labels = None;
    }

    pub fn set_content(&mut self, content: ::models::ConfigurationContent) {
        self.content = Some(content);
    }

    pub fn with_content(mut self, content: ::models::ConfigurationContent) -> Configuration {
        self.content = Some(content);
        self
    }

    pub fn content(&self) -> Option<&::models::ConfigurationContent> {
        self.content.as_ref()
    }

    pub fn reset_content(&mut self) {
        self.content = None;
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    pub fn with_content_type(mut self, content_type: String) -> Configuration {
        self.content_type = Some(content_type);
        self
    }

    pub fn content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }

    pub fn reset_content_type(&mut self) {
        self.content_type = None;
    }

    pub fn set_target_condition(&mut self, target_condition: String) {
        self.target_condition = Some(target_condition);
    }

    pub fn with_target_condition(mut self, target_condition: String) -> Configuration {
        self.target_condition = Some(target_condition);
        self
    }

    pub fn target_condition(&self) -> Option<&String> {
        self.target_condition.as_ref()
    }

    pub fn reset_target_condition(&mut self) {
        self.target_condition = None;
    }

    pub fn set_created_time_utc(&mut self, created_time_utc: String) {
        self.created_time_utc = Some(created_time_utc);
    }

    pub fn with_created_time_utc(mut self, created_time_utc: String) -> Configuration {
        self.created_time_utc = Some(created_time_utc);
        self
    }

    pub fn created_time_utc(&self) -> Option<&String> {
        self.created_time_utc.as_ref()
    }

    pub fn reset_created_time_utc(&mut self) {
        self.created_time_utc = None;
    }

    pub fn set_last_updated_time_utc(&mut self, last_updated_time_utc: String) {
        self.last_updated_time_utc = Some(last_updated_time_utc);
    }

    pub fn with_last_updated_time_utc(mut self, last_updated_time_utc: String) -> Configuration {
        self.last_updated_time_utc = Some(last_updated_time_utc);
        self
    }

    pub fn last_updated_time_utc(&self) -> Option<&String> {
        self.last_updated_time_utc.as_ref()
    }

    pub fn reset_last_updated_time_utc(&mut self) {
        self.last_updated_time_utc = None;
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = Some(priority);
    }

    pub fn with_priority(mut self, priority: i32) -> Configuration {
        self.priority = Some(priority);
        self
    }

    pub fn priority(&self) -> Option<&i32> {
        self.priority.as_ref()
    }

    pub fn reset_priority(&mut self) {
        self.priority = None;
    }

    pub fn set_system_metrics(&mut self, system_metrics: ::models::ConfigurationMetrics) {
        self.system_metrics = Some(system_metrics);
    }

    pub fn with_system_metrics(
        mut self,
        system_metrics: ::models::ConfigurationMetrics,
    ) -> Configuration {
        self.system_metrics = Some(system_metrics);
        self
    }

    pub fn system_metrics(&self) -> Option<&::models::ConfigurationMetrics> {
        self.system_metrics.as_ref()
    }

    pub fn reset_system_metrics(&mut self) {
        self.system_metrics = None;
    }

    pub fn set_metrics(&mut self, metrics: ::models::ConfigurationMetrics) {
        self.metrics = Some(metrics);
    }

    pub fn with_metrics(mut self, metrics: ::models::ConfigurationMetrics) -> Configuration {
        self.metrics = Some(metrics);
        self
    }

    pub fn metrics(&self) -> Option<&::models::ConfigurationMetrics> {
        self.metrics.as_ref()
    }

    pub fn reset_metrics(&mut self) {
        self.metrics = None;
    }

    pub fn set_e_tag(&mut self, e_tag: String) {
        self.e_tag = Some(e_tag);
    }

    pub fn with_e_tag(mut self, e_tag: String) -> Configuration {
        self.e_tag = Some(e_tag);
        self
    }

    pub fn e_tag(&self) -> Option<&String> {
        self.e_tag.as_ref()
    }

    pub fn reset_e_tag(&mut self) {
        self.e_tag = None;
    }

    pub fn set_configuration_type(&mut self, configuration_type: String) {
        self.configuration_type = Some(configuration_type);
    }

    pub fn with_configuration_type(mut self, configuration_type: String) -> Configuration {
        self.configuration_type = Some(configuration_type);
        self
    }

    pub fn configuration_type(&self) -> Option<&String> {
        self.configuration_type.as_ref()
    }

    pub fn reset_configuration_type(&mut self) {
        self.configuration_type = None;
    }

    pub fn set_body_query(&mut self, body_query: String) {
        self.body_query = Some(body_query);
    }

    pub fn with_body_query(mut self, body_query: String) -> Configuration {
        self.body_query = Some(body_query);
        self
    }

    pub fn body_query(&self) -> Option<&String> {
        self.body_query.as_ref()
    }

    pub fn reset_body_query(&mut self) {
        self.body_query = None;
    }

    pub fn set_target(&mut self, target: String) {
        self.target = Some(target);
    }

    pub fn with_target(mut self, target: String) -> Configuration {
        self.target = Some(target);
        self
    }

    pub fn target(&self) -> Option<&String> {
        self.target.as_ref()
    }

    pub fn reset_target(&mut self) {
        self.target = None;
    }

    pub fn set_metric_body_queries(
        &mut self,
        metric_body_queries: ::std::collections::HashMap<String, ::models::ConfigurationBodyQuery>,
    ) {
        self.metric_body_queries = Some(metric_body_queries);
    }

    pub fn with_metric_body_queries(
        mut self,
        metric_body_queries: ::std::collections::HashMap<String, ::models::ConfigurationBodyQuery>,
    ) -> Configuration {
        self.metric_body_queries = Some(metric_body_queries);
        self
    }

    pub fn metric_body_queries(
        &self,
    ) -> Option<&::std::collections::HashMap<String, ::models::ConfigurationBodyQuery>> {
        self.metric_body_queries.as_ref()
    }

    pub fn reset_metric_body_queries(&mut self) {
        self.metric_body_queries = None;
    }

    pub fn set_version(&mut self, version: i32) {
        self.version = Some(version);
    }

    pub fn with_version(mut self, version: i32) -> Configuration {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&i32> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }

    pub fn set_is_edge_deployment(&mut self, is_edge_deployment: bool) {
        self.is_edge_deployment = Some(is_edge_deployment);
    }

    pub fn with_is_edge_deployment(mut self, is_edge_deployment: bool) -> Configuration {
        self.is_edge_deployment = Some(is_edge_deployment);
        self
    }

    pub fn is_edge_deployment(&self) -> Option<&bool> {
        self.is_edge_deployment.as_ref()
    }

    pub fn reset_is_edge_deployment(&mut self) {
        self.is_edge_deployment = None;
    }
}