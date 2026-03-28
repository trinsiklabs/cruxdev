//! Per-platform format specifications for content repurposing.
//!
//! Each platform has different constraints: character limits, hashtag culture,
//! link placement, and formatting conventions. These specs drive the derivative generators.

use serde::{Deserialize, Serialize};

/// A platform's content format specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSpec {
    pub platform: Platform,
    pub max_chars: Option<usize>,
    pub optimal_chars: Option<usize>,
    pub max_hashtags: usize,
    pub links_allowed: LinkPlacement,
    pub format_notes: String,
    pub tone: Tone,
}

/// Supported platforms for content repurposing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    XSingle,
    XThread,
    Facebook,
    Instagram,
    LinkedIn,
    TikTokScript,
    YouTubeDescription,
    KoFi,
    Medium,
    CarouselSlides,
}

impl Platform {
    /// All platforms as a slice.
    pub fn all() -> &'static [Platform] {
        &[
            Platform::XSingle,
            Platform::XThread,
            Platform::Facebook,
            Platform::Instagram,
            Platform::LinkedIn,
            Platform::TikTokScript,
            Platform::YouTubeDescription,
            Platform::KoFi,
            Platform::Medium,
            Platform::CarouselSlides,
        ]
    }

    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            Platform::XSingle => "X (Single)",
            Platform::XThread => "X (Thread)",
            Platform::Facebook => "Facebook",
            Platform::Instagram => "Instagram",
            Platform::LinkedIn => "LinkedIn",
            Platform::TikTokScript => "TikTok Script",
            Platform::YouTubeDescription => "YouTube Description",
            Platform::KoFi => "Ko-Fi",
            Platform::Medium => "Medium",
            Platform::CarouselSlides => "Carousel Slides",
        }
    }
}

/// Where links can be placed on a platform.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LinkPlacement {
    Inline,
    LastOnly,
    BioOnly,
    Multiple,
    None,
}

/// The expected tone for a platform.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tone {
    Punchy,
    Conversational,
    Visual,
    Professional,
    Spoken,
    SeoOptimized,
    Community,
    Editorial,
}

/// Get the spec for a specific platform.
pub fn spec_for(platform: Platform) -> PlatformSpec {
    match platform {
        Platform::XSingle => PlatformSpec {
            platform,
            max_chars: Some(280),
            optimal_chars: Some(200),
            max_hashtags: 2,
            links_allowed: LinkPlacement::Inline,
            format_notes: "Punchy, no fluff. One key insight or quote.".into(),
            tone: Tone::Punchy,
        },
        Platform::XThread => PlatformSpec {
            platform,
            max_chars: Some(280), // per tweet
            optimal_chars: Some(240),
            max_hashtags: 2,
            links_allowed: LinkPlacement::LastOnly,
            format_notes: "3-15 tweets. Hook first tweet, value in middle, CTA last.".into(),
            tone: Tone::Punchy,
        },
        Platform::Facebook => PlatformSpec {
            platform,
            max_chars: Some(63206),
            optimal_chars: Some(200),
            max_hashtags: 3,
            links_allowed: LinkPlacement::Inline,
            format_notes: "Conversational, question hooks to drive comments.".into(),
            tone: Tone::Conversational,
        },
        Platform::Instagram => PlatformSpec {
            platform,
            max_chars: Some(2200),
            optimal_chars: Some(1500),
            max_hashtags: 30,
            links_allowed: LinkPlacement::BioOnly,
            format_notes: "Line breaks, emojis, story-like. Hashtags at end or in comment.".into(),
            tone: Tone::Visual,
        },
        Platform::LinkedIn => PlatformSpec {
            platform,
            max_chars: Some(3000),
            optimal_chars: Some(1300),
            max_hashtags: 5,
            links_allowed: LinkPlacement::Inline,
            format_notes: "Professional, insight-led. Use line breaks for readability.".into(),
            tone: Tone::Professional,
        },
        Platform::TikTokScript => PlatformSpec {
            platform,
            max_chars: None, // spoken word, measured in seconds
            optimal_chars: Some(900), // ~60-90 sec spoken
            max_hashtags: 5,
            links_allowed: LinkPlacement::BioOnly,
            format_notes: "Spoken word, hook in first 3 seconds, 60-90 sec total. Visual cues in brackets.".into(),
            tone: Tone::Spoken,
        },
        Platform::YouTubeDescription => PlatformSpec {
            platform,
            max_chars: Some(5000),
            optimal_chars: Some(2000),
            max_hashtags: 5,
            links_allowed: LinkPlacement::Multiple,
            format_notes: "SEO front-loaded. Timestamps section. Links section at bottom.".into(),
            tone: Tone::SeoOptimized,
        },
        Platform::KoFi => PlatformSpec {
            platform,
            max_chars: None,
            optimal_chars: Some(500),
            max_hashtags: 2,
            links_allowed: LinkPlacement::Inline,
            format_notes: "Community tone, gratitude, behind-the-scenes angle.".into(),
            tone: Tone::Community,
        },
        Platform::Medium => PlatformSpec {
            platform,
            max_chars: None,
            optimal_chars: None,
            max_hashtags: 5,
            links_allowed: LinkPlacement::Inline,
            format_notes: "Different title from source. SEO-optimized for Medium's audience.".into(),
            tone: Tone::Editorial,
        },
        Platform::CarouselSlides => PlatformSpec {
            platform,
            max_chars: Some(200), // per slide
            optimal_chars: Some(100),
            max_hashtags: 0,
            links_allowed: LinkPlacement::None,
            format_notes: "5-12 slides. One key point per slide. Visual direction in brackets.".into(),
            tone: Tone::Visual,
        },
    }
}

/// Validate that content fits within a platform's constraints.
pub fn validate_content(content: &str, spec: &PlatformSpec) -> ContentValidation {
    let char_count = content.chars().count();
    let hashtag_count = content.matches('#').count();

    let within_max = spec.max_chars.map(|max| char_count <= max).unwrap_or(true);
    let within_optimal = spec.optimal_chars.map(|opt| char_count <= opt).unwrap_or(true);
    let hashtags_ok = hashtag_count <= spec.max_hashtags;

    ContentValidation {
        platform: spec.platform,
        char_count,
        within_max,
        within_optimal,
        hashtags_ok,
        hashtag_count,
    }
}

/// Result of content validation against a platform spec.
#[derive(Debug, Clone)]
pub struct ContentValidation {
    pub platform: Platform,
    pub char_count: usize,
    pub within_max: bool,
    pub within_optimal: bool,
    pub hashtags_ok: bool,
    pub hashtag_count: usize,
}

impl ContentValidation {
    /// Whether the content passes all hard constraints.
    pub fn is_valid(&self) -> bool {
        self.within_max && self.hashtags_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_platforms_defined() {
        let all = Platform::all();
        assert_eq!(all.len(), 10);
        // Verify each has a spec
        for platform in all {
            let spec = spec_for(*platform);
            assert_eq!(spec.platform, *platform);
        }
    }

    #[test]
    fn test_x_single_spec() {
        let spec = spec_for(Platform::XSingle);
        assert_eq!(spec.max_chars, Some(280));
        assert_eq!(spec.max_hashtags, 2);
        assert_eq!(spec.tone, Tone::Punchy);
    }

    #[test]
    fn test_validate_within_limits() {
        let spec = spec_for(Platform::XSingle);
        let content = "This is a short tweet about our new feature.";
        let result = validate_content(content, &spec);
        assert!(result.is_valid());
        assert!(result.within_optimal);
    }

    #[test]
    fn test_validate_exceeds_max() {
        let spec = spec_for(Platform::XSingle);
        let content = "a".repeat(300);
        let result = validate_content(&content, &spec);
        assert!(!result.within_max);
        assert!(!result.is_valid());
    }

    #[test]
    fn test_validate_too_many_hashtags() {
        let spec = spec_for(Platform::XSingle);
        let content = "Great post #one #two #three #four";
        let result = validate_content(content, &spec);
        assert!(!result.hashtags_ok);
        assert!(!result.is_valid());
    }

    #[test]
    fn test_no_max_chars_always_valid() {
        let spec = spec_for(Platform::Medium);
        let content = "a".repeat(50000);
        let result = validate_content(&content, &spec);
        assert!(result.within_max);
    }

    #[test]
    fn test_platform_names() {
        assert_eq!(Platform::XSingle.name(), "X (Single)");
        assert_eq!(Platform::LinkedIn.name(), "LinkedIn");
        assert_eq!(Platform::TikTokScript.name(), "TikTok Script");
    }

    #[test]
    fn test_linkedin_optimal_chars() {
        let spec = spec_for(Platform::LinkedIn);
        assert_eq!(spec.optimal_chars, Some(1300));
        assert_eq!(spec.max_chars, Some(3000));
    }

    #[test]
    fn test_carousel_no_links() {
        let spec = spec_for(Platform::CarouselSlides);
        assert_eq!(spec.links_allowed, LinkPlacement::None);
        assert_eq!(spec.max_hashtags, 0);
    }
}
