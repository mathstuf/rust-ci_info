//! # ci
//!
//! Loads CI environment information.
//!

#[cfg(test)]
#[path = "./ci_test.rs"]
mod ci_test;

use crate::types::{CiInfo, Vendor};
use std::env;

fn is_env_equal(key: &str, validation: &str) -> bool {
    match env::var(key) {
        Ok(value) => validation == value,
        _ => false,
    }
}

fn is_env_defined(key: &str) -> bool {
    match env::var(key) {
        Ok(_) => true,
        _ => false,
    }
}

fn get_vendor() -> Option<Vendor> {
    if is_env_defined("TRAVIS") {
        Some(Vendor::TRAVIS)
    } else if is_env_defined("CIRCLECI") {
        Some(Vendor::CIRCLE)
    } else if is_env_defined("GITLAB_CI") {
        Some(Vendor::GITLAB)
    } else if is_env_defined("APPVEYOR") {
        Some(Vendor::APPVEYOR)
    } else if is_env_equal("CI_NAME", "codeship") {
        Some(Vendor::CODESHIP)
    } else if is_env_defined("DRONE") {
        Some(Vendor::DRONE)
    } else if is_env_defined("MAGNUM") {
        Some(Vendor::MAGNUM)
    } else if is_env_defined("SEMAPHORE") {
        Some(Vendor::SEMAPHORE)
    } else if is_env_defined("JENKINS_URL") {
        Some(Vendor::JENKINS)
    } else if is_env_defined("bamboo_planKey") {
        Some(Vendor::BAMBOO)
    } else if is_env_defined("TF_BUILD") {
        Some(Vendor::TFS)
    } else if is_env_defined("TEAMCITY_VERSION") {
        Some(Vendor::TEAMCITY)
    } else if is_env_defined("BUILDKITE") {
        Some(Vendor::BUILDKITE)
    } else if is_env_defined("HUDSON_URL") {
        Some(Vendor::HUDSON)
    } else if is_env_defined("TASK_ID") || is_env_defined("RUN_ID") {
        Some(Vendor::TASKCLUSTER)
    } else if is_env_defined("GO_PIPELINE_LABEL") {
        Some(Vendor::GOCD)
    } else if is_env_defined("BITBUCKET_COMMIT") {
        Some(Vendor::BITBUCKET)
    } else if is_env_defined("CODEBUILD_BUILD_ARN") {
        Some(Vendor::CODEBUILD)
    } else {
        None
    }
}

fn check_if_ci(vendor: &Option<Vendor>) -> bool {
    vendor.is_some()
        || is_env_defined("CI")
        || is_env_defined("CONTINUOUS_INTEGRATION")
        || is_env_defined("BUILD_NUMBER")
}

/// Loads and returns the CI info of the current environment.
pub(crate) fn get() -> CiInfo {
    let mut info = CiInfo::new();

    info.vendor = get_vendor();

    info.ci = check_if_ci(&info.vendor);

    info
}

/// Returns true if a CI environment is detected.
pub(crate) fn is_ci() -> bool {
    let info = get();

    info.ci
}
