//! SEO health checking and PageSpeed tracking.
//!
//! No API keys needed — uses public HTTP checks and the free PageSpeed API.

use serde::{Deserialize, Serialize};
use std::path::Path;

// --- SEO Health Check ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoHealthReport {
    pub domain: String,
    pub timestamp: f64,
    pub checks: Vec<SeoCheck>,
    pub passed: usize,
    pub failed: usize,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

/// Run all SEO health checks against a domain (no auth needed).
pub async fn check_seo_health(domain: &str) -> SeoHealthReport {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .unwrap_or_default();

    let base = format!("https://{domain}");
    let mut checks = Vec::new();

    // 1. Homepage returns 200
    checks.push(check_url_status(&client, &base, "Homepage returns 200").await);

    // 2. robots.txt accessible
    let robots_check = check_url_content(&client, &format!("{base}/robots.txt"), "robots.txt accessible", "Sitemap:").await;
    checks.push(robots_check);

    // 3. Sitemap accessible and valid XML
    checks.push(check_url_content(&client, &format!("{base}/sitemap-index.xml"), "Sitemap accessible", "<").await);

    // 4. llms.txt accessible
    checks.push(check_url_status(&client, &format!("{base}/llms.txt"), "llms.txt accessible").await);

    // 5. HTTPS redirect
    checks.push(check_https_redirect(&client, domain).await);

    // 6. Security headers
    checks.push(check_header(&client, &base, "HSTS header", "strict-transport-security").await);
    checks.push(check_header(&client, &base, "X-Content-Type-Options", "x-content-type-options").await);
    checks.push(check_header(&client, &base, "X-Frame-Options", "x-frame-options").await);

    // 7. Cloudflare proxy
    checks.push(check_header(&client, &base, "Cloudflare proxy active", "cf-ray").await);

    // 8. Key pages return 200
    let key_pages = ["/engine", "/methodology", "/vs/", "/guides/quick-install", "/blog/"];
    for page in &key_pages {
        checks.push(check_url_status(&client, &format!("{base}{page}"), &format!("{page} returns 200")).await);
    }

    // 9. Homepage has title and meta description
    checks.push(check_html_meta(&client, &base).await);

    // 10. Canonical URL present
    checks.push(check_url_content(&client, &base, "Canonical URL present", "rel=\"canonical\"").await);

    // 11. Internal link validation — check all sitemap URLs return 200
    let link_checks = check_sitemap_links(domain).await;
    let broken_count = link_checks.iter().filter(|c| !c.passed).count();
    if broken_count > 0 {
        for lc in &link_checks {
            if !lc.passed {
                checks.push(lc.clone());
            }
        }
    } else {
        checks.push(SeoCheck {
            name: format!("All {} sitemap URLs return 200", link_checks.len()),
            passed: true,
            detail: "OK".into(),
        });
    }

    let passed = checks.iter().filter(|c| c.passed).count();
    let failed = checks.iter().filter(|c| !c.passed).count();
    let total = checks.len();
    let score = if total > 0 { (passed as f64 / total as f64) * 100.0 } else { 0.0 };

    SeoHealthReport {
        domain: domain.to_string(),
        timestamp: now(),
        checks,
        passed,
        failed,
        score: (score * 10.0).round() / 10.0,
    }
}

async fn check_url_status(client: &reqwest::Client, url: &str, name: &str) -> SeoCheck {
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();
            SeoCheck {
                name: name.to_string(),
                passed: status.is_success(),
                detail: format!("HTTP {status}"),
            }
        }
        Err(e) => SeoCheck {
            name: name.to_string(),
            passed: false,
            detail: format!("Error: {e}"),
        },
    }
}

async fn check_url_content(client: &reqwest::Client, url: &str, name: &str, contains: &str) -> SeoCheck {
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                return SeoCheck { name: name.to_string(), passed: false, detail: format!("HTTP {status}") };
            }
            let body = resp.text().await.unwrap_or_default();
            let found = body.contains(contains);
            SeoCheck {
                name: name.to_string(),
                passed: found,
                detail: if found { "OK".into() } else { format!("Missing expected content: {contains}") },
            }
        }
        Err(e) => SeoCheck { name: name.to_string(), passed: false, detail: format!("Error: {e}") },
    }
}

async fn check_https_redirect(_client: &reqwest::Client, domain: &str) -> SeoCheck {
    let http_url = format!("http://{domain}");
    // Use a client that doesn't follow redirects
    let no_redirect = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_default();

    match no_redirect.get(&http_url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let location = resp.headers().get("location")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            let redirects_to_https = status.is_redirection() && location.starts_with("https://");
            SeoCheck {
                name: "HTTP→HTTPS redirect".to_string(),
                passed: redirects_to_https,
                detail: if redirects_to_https {
                    format!("{status} → {location}")
                } else {
                    format!("HTTP {status}, Location: {location}")
                },
            }
        }
        Err(e) => SeoCheck {
            name: "HTTP→HTTPS redirect".to_string(),
            passed: false,
            detail: format!("Error: {e}"),
        },
    }
}

async fn check_header(client: &reqwest::Client, url: &str, name: &str, header: &str) -> SeoCheck {
    match client.head(url).send().await {
        Ok(resp) => {
            let val = resp.headers().get(header).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
            SeoCheck {
                name: name.to_string(),
                passed: val.is_some(),
                detail: val.unwrap_or_else(|| "Missing".into()),
            }
        }
        Err(e) => SeoCheck { name: name.to_string(), passed: false, detail: format!("Error: {e}") },
    }
}

async fn check_html_meta(client: &reqwest::Client, url: &str) -> SeoCheck {
    match client.get(url).send().await {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_default();
            let has_title = body.contains("<title>") && !body.contains("<title></title>");
            let has_description = body.contains("name=\"description\"");
            SeoCheck {
                name: "Homepage has title + meta description".to_string(),
                passed: has_title && has_description,
                detail: format!("title: {}, description: {}", has_title, has_description),
            }
        }
        Err(e) => SeoCheck {
            name: "Homepage has title + meta description".to_string(),
            passed: false,
            detail: format!("Error: {e}"),
        },
    }
}

// --- PageSpeed ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSpeedReport {
    pub url: String,
    pub timestamp: f64,
    pub performance: f64,
    pub accessibility: f64,
    pub best_practices: f64,
    pub seo: f64,
    pub lcp_ms: f64,
    pub cls: f64,
    pub fcp_ms: f64,
    pub opportunities: Vec<String>,
}

/// Check PageSpeed for a URL (free API, no auth).
pub async fn check_pagespeed(url: &str, strategy: &str) -> Option<PageSpeedReport> {
    let api_url = format!(
        "https://www.googleapis.com/pagespeedonline/v5/runPagespeed?url={}&strategy={}&category=PERFORMANCE&category=ACCESSIBILITY&category=BEST_PRACTICES&category=SEO",
        urlencoding::encode(url),
        strategy
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .unwrap_or_default();

    let resp = client.get(&api_url).send().await.ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;

    let categories = json.pointer("/lighthouseResult/categories")?;
    let audits = json.pointer("/lighthouseResult/audits");

    let performance = categories.pointer("/performance/score")?.as_f64()? * 100.0;
    let accessibility = categories.pointer("/accessibility/score")?.as_f64()? * 100.0;
    let best_practices = categories.pointer("/best-practices/score")
        .or_else(|| categories.pointer("/bestPractices/score"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) * 100.0;
    let seo = categories.pointer("/seo/score")?.as_f64()? * 100.0;

    let lcp_ms = audits
        .and_then(|a| a.pointer("/largest-contentful-paint/numericValue"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let cls = audits
        .and_then(|a| a.pointer("/cumulative-layout-shift/numericValue"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let fcp_ms = audits
        .and_then(|a| a.pointer("/first-contentful-paint/numericValue"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    // Extract top opportunities
    let mut opportunities = Vec::new();
    if let Some(audits_obj) = audits.and_then(|a| a.as_object()) {
        for (_key, audit) in audits_obj {
            let is_opportunity = audit.pointer("/details/type")
                .and_then(|t| t.as_str()) == Some("opportunity");
            if let Some(title) = audit.get("title").and_then(|t| t.as_str()).filter(|_| is_opportunity) {
                let savings = audit.pointer("/details/overallSavingsMs")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                if savings > 100.0 {
                    opportunities.push(format!("{title} (save {savings:.0}ms)"));
                }
            }
        }
    }

    Some(PageSpeedReport {
        url: url.to_string(),
        timestamp: now(),
        performance: performance.round(),
        accessibility: accessibility.round(),
        best_practices: best_practices.round(),
        seo: seo.round(),
        lcp_ms: (lcp_ms * 10.0).round() / 10.0,
        cls: (cls * 1000.0).round() / 1000.0,
        fcp_ms: (fcp_ms * 10.0).round() / 10.0,
        opportunities,
    })
}

// --- Persistence ---

/// Append a report to a JSONL file (atomic).
pub fn append_report<T: Serialize>(path: &Path, report: &T) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(dir)?;
    let line = serde_json::to_string(report)?;
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{line}")
}

/// Load the last N reports from a JSONL file.
pub fn load_recent_reports<T: for<'de> Deserialize<'de>>(path: &Path, limit: usize) -> Vec<T> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut reports: Vec<T> = content
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let start = reports.len().saturating_sub(limit);
    reports.drain(..start);
    reports
}

/// Detect PageSpeed regression: score dropped > threshold from previous run.
pub fn detect_regression(current: &PageSpeedReport, previous: &PageSpeedReport, threshold: f64) -> Vec<String> {
    let mut regressions = Vec::new();
    if previous.performance - current.performance > threshold {
        regressions.push(format!("Performance: {:.0} → {:.0} (dropped {:.0})", previous.performance, current.performance, previous.performance - current.performance));
    }
    if previous.accessibility - current.accessibility > threshold {
        regressions.push(format!("Accessibility: {:.0} → {:.0} (dropped {:.0})", previous.accessibility, current.accessibility, previous.accessibility - current.accessibility));
    }
    if previous.seo - current.seo > threshold {
        regressions.push(format!("SEO: {:.0} → {:.0} (dropped {:.0})", previous.seo, current.seo, previous.seo - current.seo));
    }
    regressions
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

// --- Link Checker ---

fn extract_loc_urls(xml: &str) -> Vec<String> {
    xml.lines()
        .filter_map(|line| {
            let start = line.find("<loc>")? + 5;
            let end = line.find("</loc>")?;
            Some(line[start..end].to_string())
        })
        .collect()
}

/// Check all URLs in a sitemap for broken links.
pub async fn check_sitemap_links(domain: &str) -> Vec<SeoCheck> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_default();

    let sitemap_url = format!("https://{domain}/sitemap-index.xml");
    let body = match client.get(&sitemap_url).send().await {
        Ok(resp) => resp.text().await.unwrap_or_default(),
        Err(_) => return vec![SeoCheck { name: "Sitemap fetch".into(), passed: false, detail: "Failed to fetch sitemap".into() }],
    };

    // Extract URLs from sitemap XML
    let mut urls = extract_loc_urls(&body);

    // If this is a sitemap index, fetch child sitemaps
    if urls.iter().any(|u| u.ends_with(".xml")) {
        let mut page_urls = Vec::new();
        for sitemap in &urls {
            if let Ok(resp) = client.get(sitemap).send().await {
                let child_body = resp.text().await.unwrap_or_default();
                page_urls.extend(extract_loc_urls(&child_body));
            }
        }
        urls = page_urls;
    }

    let mut checks = Vec::new();
    for url in &urls {
        checks.push(check_url_status(&client, url, url).await);
    }
    checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_regression_flags_drop() {
        let current = PageSpeedReport {
            url: "https://example.com".into(),
            timestamp: 0.0,
            performance: 72.0,
            accessibility: 95.0,
            best_practices: 90.0,
            seo: 88.0,
            lcp_ms: 2500.0,
            cls: 0.1,
            fcp_ms: 1200.0,
            opportunities: vec![],
        };
        let previous = PageSpeedReport {
            url: "https://example.com".into(),
            timestamp: 0.0,
            performance: 85.0,
            accessibility: 95.0,
            best_practices: 90.0,
            seo: 92.0,
            lcp_ms: 2000.0,
            cls: 0.05,
            fcp_ms: 1000.0,
            opportunities: vec![],
        };
        let regressions = detect_regression(&current, &previous, 5.0);
        assert_eq!(regressions.len(), 1); // Performance dropped 13 points
        assert!(regressions[0].contains("Performance"));
    }

    #[test]
    fn test_detect_regression_no_drop() {
        let report = PageSpeedReport {
            url: "https://example.com".into(),
            timestamp: 0.0,
            performance: 90.0,
            accessibility: 95.0,
            best_practices: 90.0,
            seo: 92.0,
            lcp_ms: 2000.0,
            cls: 0.05,
            fcp_ms: 1000.0,
            opportunities: vec![],
        };
        let regressions = detect_regression(&report, &report, 5.0);
        assert!(regressions.is_empty());
    }

    #[test]
    fn test_append_and_load_reports() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.jsonl");
        let report = PageSpeedReport {
            url: "https://example.com".into(),
            timestamp: 1234.0,
            performance: 90.0,
            accessibility: 95.0,
            best_practices: 90.0,
            seo: 92.0,
            lcp_ms: 2000.0,
            cls: 0.05,
            fcp_ms: 1000.0,
            opportunities: vec!["Reduce JS".into()],
        };
        append_report(&path, &report).unwrap();
        append_report(&path, &report).unwrap();
        let loaded: Vec<PageSpeedReport> = load_recent_reports(&path, 5);
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].performance, 90.0);
    }

    #[test]
    fn test_load_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.jsonl");
        let loaded: Vec<PageSpeedReport> = load_recent_reports(&path, 5);
        assert!(loaded.is_empty());
    }
}
