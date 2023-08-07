/// Naming System
/// Specifies the naming system under which this risk metric is organized, which allows for the same names to be used in different systems controlled by different parties. This avoids the potential of a name clash.
/// $id: #assembly_oscal-assessment-common_characterization_facet_naming-system_naming-system
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum NamingSystem {
	// orig: http://fedramp.gov
#[serde(rename = "http://fedramp.gov")]
	HttpFedrampGov,
	// orig: http://fedramp.gov/ns/oscal
#[serde(rename = "http://fedramp.gov/ns/oscal")]
	HttpFedrampGovNsOscal,
	// orig: http://csrc.nist.gov/ns/oscal
#[serde(rename = "http://csrc.nist.gov/ns/oscal")]
	HttpCsrcNistGovNsOscal,
	// orig: http://csrc.nist.gov/ns/oscal/unknown
#[serde(rename = "http://csrc.nist.gov/ns/oscal/unknown")]
	HttpCsrcNistGovNsOscalUnknown,
	// orig: http://cve.mitre.org
#[serde(rename = "http://cve.mitre.org")]
	HttpCveMitreOrg,
	// orig: http://www.first.org/cvss/v2.0
#[serde(rename = "http://www.first.org/cvss/v2.0")]
	HttpWwwFirstOrgCvssV20,
	// orig: http://www.first.org/cvss/v3.0
#[serde(rename = "http://www.first.org/cvss/v3.0")]
	HttpWwwFirstOrgCvssV30,
	// orig: http://www.first.org/cvss/v3.1
#[serde(rename = "http://www.first.org/cvss/v3.1")]
	HttpWwwFirstOrgCvssV31,
}
