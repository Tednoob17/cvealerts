



/**
  * Cve struct - cve attributes
  * @description: 
  * @id:
  * @metrics:
  * @last:
  * @published:
  * @references:
  * @source:
  * @vuln
  * weaknesses:

  * Description: The attributes of Cve.
  */

#[derive(Debug, Deserialize)]
pub struct CVE {
    descriptions: Vec<CVEDescription>,
    id: String,
    lastModified: String,
    metrics: Vec<CVSSMetric>,
    published: String,
    references: Vec<Reference>,
    sourceIdentifier: String,
    vulnStatus: String,
    weaknesses: Vec<Weakness>,
}
