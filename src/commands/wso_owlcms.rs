use clap::Parser;

use crate::types::wso_owlcms::{CarolinaTab, WsoOwlCmsParsedBlock, WsoOwlCmsReferenceMeta};

/// Export Carolina WSO records as an OWLCMS CSV.
///
/// Examples:
///   meetcal wsoOWLCMS
#[derive(Parser)]
#[command(name = "wsoOWLCMS")]
pub struct WsoOwlcmsArgs {}

pub fn run(_args: WsoOwlcmsArgs, _convex_url: &str) {}
