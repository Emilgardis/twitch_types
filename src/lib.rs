//! Twitch types

use serde::{Deserialize, Serialize};

/// A user ID.
#[aliri_braid::braid(serde)]
pub struct UserId;

/// A reward ID.
#[aliri_braid::braid(serde)]
pub struct RewardId;

/// A reward redemption ID.
#[aliri_braid::braid(serde)]
pub struct RedemptionId;

/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A reference to a borrowed [`UserName`], also specified as login. Should not be capitalized.
pub type UserNameRef = NicknameRef;

/// A users display name
#[aliri_braid::braid(serde)]
pub struct DisplayName;

/// A nickname, not capitalized.
#[aliri_braid::braid(serde)]
pub struct Nickname;

/// RFC3339 timestamp
#[aliri_braid::braid(serde, validator)]
pub struct Timestamp;

impl aliri_braid::Validator for Timestamp {
    type Error = TimestampParseError;

    fn validate(s: &str) -> Result<(), Self::Error> {
        #[cfg(feature = "time")]
        {
            let _ = time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)?;
            Ok(())
        }
        #[cfg(not(feature = "time"))]
        {
            // This validator is lacking some features for now
            if !s.chars().all(|c| {
                c.is_numeric()
                    || c == 'T'
                    || c == 'Z'
                    || c == '+'
                    || c == '.'
                    || c == '-'
                    || c == ':'
            }) {
                return Err(TimestampParseError::invalid());
            }
            // PSA: Don't do time stuff... it sucks
            if let Some(i) = s.find('T') {
                // if no `T`, then it's not a valid timestamp
                if i < 1 {
                    return Err(TimestampParseError::invalid());
                };
                let (full_date, full_time) = s.split_at(i);
                if full_date.len() != "1900-00-00".len() {
                    return Err(TimestampParseError::invalid_s(full_date));
                }
                if !full_date.chars().all(|c| c.is_numeric() || c == '-') {
                    return Err(TimestampParseError::invalid_s(full_date));
                }
                let partial_time = if let Some(stripped) = full_time.strip_suffix('Z') {
                    stripped
                } else {
                    return Err(TimestampParseError::Other("unsupported non-UTC timestamp, enable the `time` feature in `twitch_api2` to enable parsing these"));
                };
                if 2 != partial_time
                    .chars()
                    .into_iter()
                    .filter(|&b| b == ':')
                    .count()
                {
                    return Err(TimestampParseError::invalid_s(partial_time));
                };
                if !partial_time.contains('.') && partial_time.len() != "T00:00:00".len() {
                    return Err(TimestampParseError::invalid_s(partial_time));
                } else if partial_time.contains('.') {
                    let mut i = partial_time.split('.');
                    // if len not correct or next is none
                    if !i
                        .next()
                        .map(|s| s.len() == "T00:00:00".len())
                        .unwrap_or_default()
                    {
                        return Err(TimestampParseError::invalid_s(partial_time));
                    }
                }
            } else {
                return Err(TimestampParseError::invalid());
            }
            Ok(())
        }
    }
}

/// Errors that can occur when parsing a timestamp.
#[derive(Debug, thiserror::Error, displaydoc::Display)]
#[ignore_extra_doc_attributes]
#[non_exhaustive]
pub enum TimestampParseError {
    /// Could not parse the timestamp using `time`
    #[cfg(feature = "time")]
    #[cfg_attr(nightly, doc(cfg(feature = "time")))]
    TimeError(#[from] time::error::Parse),
    /// Could not format the timestamp using `time`
    #[cfg(feature = "time")]
    #[cfg_attr(nightly, doc(cfg(feature = "time")))]
    TimeFormatError(#[from] time::error::Format),
    /// {0}
    Other(&'static str),
    /// timestamp has an invalid format. {s:?} - {location}
    InvalidFormat {
        /// location of error
        location: &'static std::panic::Location<'static>,
        /// Thing that failed
        s: Option<String>,
    },
}

impl TimestampParseError {
    #[cfg(not(feature = "time"))]
    #[track_caller]
    fn invalid() -> Self {
        Self::InvalidFormat {
            location: std::panic::Location::caller(),
            s: None,
        }
    }

    #[cfg(not(feature = "time"))]
    #[track_caller]
    fn invalid_s(s: &str) -> Self {
        Self::InvalidFormat {
            location: std::panic::Location::caller(),
            s: Some(s.to_string()),
        }
    }
}

impl Timestamp {
    /// Set the partial-time component of the timestamp.
    ///
    /// # Panics
    ///
    /// Internally, without the `time` feature, this uses `unsafe` to deal with the raw string bytes. To ensure safety, the method will panic on invalid input and source.
    fn set_time(&mut self, hours: u8, minutes: u8, seconds: u8) {
        #[cfg(feature = "time")]
        {
            use std::convert::TryInto;
            let _ = std::mem::replace(
                self,
                self.to_fixed_offset()
                    .replace_time(
                        time::Time::from_hms(hours, minutes, seconds)
                            .expect("could not create time"),
                    )
                    .try_into()
                    .expect("could not make timestamp"),
            );
        }
        #[cfg(not(feature = "time"))]
        {
            const ERROR_MSG: &str = "malformed timestamp";
            assert!(hours < 24);
            assert!(minutes < 60);
            assert!(seconds < 60);

            #[inline]
            fn replace_len2(s: &mut str, replace: &str) {
                assert!(replace.as_bytes().len() == 2);
                assert!(s.as_bytes().len() == 2);

                let replace = replace.as_bytes();
                // Safety:
                // There are two things to make sure the replacement is valid.
                // 1. The length of the two slices are equal to two.
                // 2. `replace` slice does not contain any invalid characters.
                //    As a property of being a `&str` of len 2, start and end of the str slice are valid boundaries, start is index 0, end is index 1 == `replace.len()` => 2 iff 1.)
                let b = unsafe { s.as_bytes_mut() };
                b[0] = replace[0];
                b[1] = replace[1];
            }
            let t = self.0.find('T').expect(ERROR_MSG);
            let partial_time: &mut str = &mut self.0[t + 1..];
            // find the hours, minutes and seconds
            let mut matches = partial_time.match_indices(':');
            let (h, m, s) = (
                0,
                matches.next().expect(ERROR_MSG).0 + 1,
                matches.next().expect(ERROR_MSG).0 + 1,
            );
            assert!(matches.next().is_none());
            // RFC3339 requires partial-time components to be 2DIGIT
            partial_time
                .get_mut(h..h + 2)
                .map(|s| replace_len2(s, &format!("{:02}", hours)))
                .expect(ERROR_MSG);
            partial_time
                .get_mut(m..m + 2)
                .map(|s| replace_len2(s, &format!("{:02}", minutes)))
                .expect(ERROR_MSG);
            partial_time
                .get_mut(s..s + 2)
                .map(|s| replace_len2(s, &format!("{:02}", seconds)))
                .expect(ERROR_MSG);
        }
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl Timestamp {
    /// Create a timestamp corresponding to current time
    pub fn now() -> Timestamp {
        use std::convert::TryInto;
        time::OffsetDateTime::now_utc()
            .try_into()
            .expect("could not make timestamp")
    }

    /// Create a timestamp corresponding to the start of the current day. Timezone will always be UTC.
    pub fn today() -> Timestamp {
        use std::convert::TryInto;
        time::OffsetDateTime::now_utc()
            .replace_time(time::Time::MIDNIGHT)
            .try_into()
            .expect("could not make timestamp")
    }
}

impl TimestampRef {
    /// Normalize the timestamp into UTC time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api2::types::Timestamp;
    ///
    /// let time = Timestamp::new("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.normalize()?.as_ref(), &time);
    /// let time2 = Timestamp::new("2021-07-01T13:37:00-01:00").unwrap();
    /// assert_ne!(time2.normalize()?.as_ref(), &time2);
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error + 'static>>(())
    /// ```
    #[allow(unreachable_code)]
    pub fn normalize(&'_ self) -> Result<std::borrow::Cow<'_, TimestampRef>, TimestampParseError> {
        let s = self.as_str();
        if s.ends_with('Z') {
            Ok(self.into())
        } else {
            #[cfg(feature = "time")]
            {
                use std::convert::TryInto;
                let utc = self.to_utc();
                return Ok(std::borrow::Cow::Owned(utc.try_into()?));
            }
            panic!("non `Z` timestamps are not possible to use without the `time` feature enabled for `twitch_api2`")
        }
    }

    /// Compare another time and return `self < other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api2::types::Timestamp;
    ///
    /// let time2021 = Timestamp::new("2021-07-01T13:37:00Z").unwrap();
    /// let time2020 = Timestamp::new("2020-07-01T13:37:00Z").unwrap();
    /// assert!(time2020.is_before(&time2021));
    /// ```
    pub fn is_before<T>(&self, other: &T) -> bool
    where Self: PartialOrd<T> {
        self < other
    }

    /// Make a timestamp with the time component set to 00:00:00.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api2::types::Timestamp;
    ///
    /// let time = Timestamp::new("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.to_day().as_str(), "2021-07-01T00:00:00Z")
    /// ```  
    pub fn to_day(&self) -> Timestamp {
        let mut c = self.to_owned();
        c.set_time(0, 0, 0);
        c
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl TimestampRef {
    /// Construct into a [`OffsetDateTime`](time::OffsetDateTime) time with a guaranteed UTC offset.
    ///
    /// # Panics
    ///
    /// This method assumes the timestamp is a valid rfc3339 timestamp, and panics if not.
    pub fn to_utc(&self) -> time::OffsetDateTime {
        self.to_fixed_offset().to_offset(time::UtcOffset::UTC)
    }

    /// Construct into a [`OffsetDateTime`](time::OffsetDateTime) time.
    ///
    /// # Panics
    ///
    /// This method assumes the timestamp is a valid rfc3339 timestamp, and panics if not.
    pub fn to_fixed_offset(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::parse(&self.0, &time::format_description::well_known::Rfc3339)
            .expect("this should never fail")
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Defer to TimestampRef impl
        let this: &TimestampRef = self.as_ref();
        let other: &TimestampRef = other.as_ref();
        this.partial_cmp(other)
    }
}

impl PartialOrd<Timestamp> for TimestampRef {
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        // Defer to TimestampRef impl
        let other: &TimestampRef = other.as_ref();
        self.partial_cmp(other)
    }
}

impl PartialOrd for TimestampRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // to check ordering, we normalize offset, then do a lexicographic comparison if possible,
        // We can do this because the timestamp should always be RFC3339 with time-offset = 'Z' with normalize.
        // However, we need to make sure punctuation and length is correct. Without the `time` feature, it's impossible to get a non-UTC timestamp, so normalize will do nothing.
        let this = self
            .normalize()
            .expect("normalization failed, this is a bug");
        let other = other
            .normalize()
            .expect("normalization of other failed, this is a bug");
        // If a punctuation exists in only one, we can't order.
        #[allow(clippy::if_same_then_else)]
        if this.as_ref().as_str().contains('.') ^ other.as_ref().as_str().contains('.') {
            #[cfg(feature = "tracing")]
            tracing::trace!("comparing two `Timestamps` with differing punctuation");
            return None;
        } else if this.0.len() != other.0.len() {
            #[cfg(feature = "tracing")]
            tracing::trace!("comparing two `Timestamps` with differing length");
            return None;
        }
        this.as_str().partial_cmp(other.as_str())
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialEq<time::OffsetDateTime> for Timestamp {
    fn eq(&self, other: &time::OffsetDateTime) -> bool {
        // Defer to TimestampRef impl
        let this: &TimestampRef = self.as_ref();
        this.eq(other)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialOrd<time::OffsetDateTime> for Timestamp {
    fn partial_cmp(&self, other: &time::OffsetDateTime) -> Option<std::cmp::Ordering> {
        // Defer to TimestampRef impl
        let this: &TimestampRef = self.as_ref();
        this.partial_cmp(other)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialEq<time::OffsetDateTime> for TimestampRef {
    fn eq(&self, other: &time::OffsetDateTime) -> bool { &self.to_utc() == other }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialOrd<time::OffsetDateTime> for TimestampRef {
    fn partial_cmp(&self, other: &time::OffsetDateTime) -> Option<std::cmp::Ordering> {
        self.to_utc().partial_cmp(other)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl std::convert::TryFrom<time::OffsetDateTime> for Timestamp {
    type Error = time::error::Format;

    fn try_from(value: time::OffsetDateTime) -> Result<Self, Self::Error> {
        Ok(Timestamp(
            value.format(&time::format_description::well_known::Rfc3339)?,
        ))
    }
}

/// A blocked term ID
#[aliri_braid::braid(serde)]
pub struct BlockedTermId;

/// A game or category ID
#[aliri_braid::braid(serde)]
pub struct CategoryId;

/// A tag ID
#[aliri_braid::braid(serde)]
pub struct TagId;

/// A video ID
#[aliri_braid::braid(serde)]
pub struct VideoId;

/// An EventSub Subscription ID
#[aliri_braid::braid(serde)]
pub struct EventSubId;

/// A Team ID
#[aliri_braid::braid(serde)]
pub struct TeamId;

/// A Stream ID
#[aliri_braid::braid(serde)]
pub struct StreamId;

/// A message ID
#[aliri_braid::braid(serde)]
pub struct MsgId;

/// A poll ID
#[aliri_braid::braid(serde)]
pub struct PollId;

/// A poll choice ID
#[aliri_braid::braid(serde)]
pub struct PollChoiceId;

/// A prediction ID
#[aliri_braid::braid(serde)]
pub struct PredictionId;

/// A prediction choice ID
#[aliri_braid::braid(serde)]
pub struct PredictionOutcomeId;

/// A Badge set ID
#[aliri_braid::braid(serde)]
pub struct BadgeSetId;

/// A channel chat badge ID
#[aliri_braid::braid(serde)]
pub struct ChatBadgeId;

/// A chat Emote ID
#[aliri_braid::braid(serde)]
pub struct EmoteId;

impl EmoteIdRef {
    /// Generates url for this emote.
    ///
    /// Generated URL will be `"https://static-cdn.jtvnw.net/emoticons/v2/{emote_id}/default/light/1.0"`
    pub fn default_render(&self) -> String {
        EmoteUrlBuilder {
            id: self.into(),
            animation_setting: None,
            theme_mode: EmoteThemeMode::Light,
            scale: EmoteScale::Size1_0,
            template: EMOTE_V2_URL_TEMPLATE.into(),
        }
        .render()
    }

    /// Create a [`EmoteUrlBuilder`] for this emote
    pub fn url(&self) -> EmoteUrlBuilder<'_> { EmoteUrlBuilder::new(self) }
}

/// Emote url template
pub(crate) static EMOTE_V2_URL_TEMPLATE: &str =
    "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}";

/// Formats for an emote.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EmoteAnimationSetting {
    /// Static
    Static,
    /// Animated
    Animated,
}

impl std::fmt::Display for EmoteAnimationSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

/// Background themes available for an emote.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EmoteThemeMode {
    /// Light
    Light,
    /// Dark
    Dark,
}

impl Default for EmoteThemeMode {
    fn default() -> Self { Self::Light }
}

impl std::fmt::Display for EmoteThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

/// Scales available for an emote.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum EmoteScale {
    /// 1.0
    #[serde(rename = "1.0")]
    Size1_0,
    /// 2.0
    #[serde(rename = "2.0")]
    Size2_0,
    /// 3.0
    #[serde(rename = "3.0")]
    Size3_0,
}

impl Default for EmoteScale {
    fn default() -> Self { Self::Size1_0 }
}

impl std::fmt::Display for EmoteScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

/// Builder for [emote URLs](https://dev.twitch.tv/docs/irc/emotes#emote-cdn-url-format).
///
/// # Examples
///
/// ```rust
/// # use twitch_api2::types::EmoteId;
/// let emote_id = EmoteId::from("emotesv2_dc24652ada1e4c84a5e3ceebae4de709");
/// assert_eq!(emote_id.url().size_3x().dark_mode().render(), "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_dc24652ada1e4c84a5e3ceebae4de709/default/dark/3.0")
/// ```
#[derive(Debug, Clone)]
pub struct EmoteUrlBuilder<'a> {
    pub(crate) id: std::borrow::Cow<'a, EmoteIdRef>,
    pub(crate) animation_setting: Option<EmoteAnimationSetting>,
    pub(crate) theme_mode: EmoteThemeMode,
    pub(crate) scale: EmoteScale,
    pub(crate) template: std::borrow::Cow<'a, str>,
}

impl EmoteUrlBuilder<'_> {
    // FIXME: AsRef
    /// Construct a new [`EmoteUrlBuilder`] from a [`EmoteId`]
    ///
    /// Defaults to `1.0` scale, `default` animation and `light` theme.
    pub fn new(id: &EmoteIdRef) -> EmoteUrlBuilder<'_> {
        EmoteUrlBuilder {
            id: id.into(),
            animation_setting: <_>::default(),
            theme_mode: <_>::default(),
            scale: <_>::default(),
            template: EMOTE_V2_URL_TEMPLATE.into(),
        }
    }

    /// Set size to 1.0
    pub fn size_1x(mut self) -> Self {
        self.scale = EmoteScale::Size1_0;
        self
    }

    /// Set size to 2.0
    pub fn size_2x(mut self) -> Self {
        self.scale = EmoteScale::Size2_0;
        self
    }

    /// Set size to 3.0
    pub fn size_3x(mut self) -> Self {
        self.scale = EmoteScale::Size3_0;
        self
    }

    /// Set theme to dark mode
    pub fn dark_mode(mut self) -> Self {
        self.theme_mode = EmoteThemeMode::Dark;
        self
    }

    /// Set theme to light mode
    pub fn light_mode(mut self) -> Self {
        self.theme_mode = EmoteThemeMode::Light;
        self
    }

    /// Set animation mode to default
    pub fn animation_default(mut self) -> Self {
        self.animation_setting = None;
        self
    }

    /// Set animation mode to static
    pub fn animation_static(mut self) -> Self {
        self.animation_setting = Some(EmoteAnimationSetting::Static);
        self
    }

    /// Set animation mode to animate
    pub fn animation_animated(mut self) -> Self {
        self.animation_setting = Some(EmoteAnimationSetting::Animated);
        self
    }

    /// Create the URL for this emote.
    pub fn render(self) -> String {
        if self.template != "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}" {
            let custom_template = |builder: &EmoteUrlBuilder| -> Option<String> {
                let mut template = self.template.clone().into_owned();
                let emote_id_range = template.find("{{id}}")?;
                eprintln!("id");
                template.replace_range(emote_id_range..emote_id_range+"{{id}}".len(), builder.id.as_str());
                eprintln!("format");
                let format_range = template.find("{{format}}")?;
                template.replace_range(format_range..format_range+"{{format}}".len(), &builder.animation_setting.as_ref().map(|s| s.to_string()).unwrap_or_else(|| String::from("default")));
                eprintln!("theme_mode");
                let theme_mode_range = template.find("{{theme_mode}}")?;
                template.replace_range(theme_mode_range..theme_mode_range+"{{theme_mode}}".len(), &builder.theme_mode.to_string());
                eprintln!("scale");
                let scale_range = template.find("{{scale}}")?;
                template.replace_range(scale_range..scale_range+"{{scale}}".len(), &builder.scale.to_string());
                if template.contains("{{") || template.contains("}}") {
                    None
                } else {
                    Some(template)
                }
            };
            if let Some(template) = custom_template(&self) {
                return template
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(template = %self.template, "emote builder was supplied an invalid or unknown template url, falling back to standard builder");
            }
        }
        // fallback to known working template
        format!("https://static-cdn.jtvnw.net/emoticons/v2/{emote_id}/{animation_setting}/{theme_mode}/{scale}",
            emote_id = self.id,
            animation_setting = self.animation_setting.as_ref().map(|s| s.to_string()).unwrap_or_else(|| String::from("default")),
            theme_mode = self.theme_mode,
            scale = self.scale,
        )
    }
}

/// An Emote Set ID
#[aliri_braid::braid(serde)]
pub struct EmoteSetId;

/// A Stream Segment ID.
#[aliri_braid::braid(serde)]
pub struct StreamSegmentId;

/// A Hype Train ID
#[aliri_braid::braid(serde)]
pub struct HypeTrainId;

/// A Creator Goal ID
#[aliri_braid::braid(serde)]
pub struct CreatorGoalId;

/// An emote index as defined by eventsub, similar to IRC `emotes` twitch tag.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ResubscriptionEmote {
    /// The index of where the Emote starts in the text.
    pub begin: i64,
    /// The index of where the Emote ends in the text.
    pub end: i64,
    /// The emote ID.
    pub id: EmoteId,
}

impl std::fmt::Display for ResubscriptionEmote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.id, self.begin, self.end)
    }
}

/// A game or category as defined by Twitch
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TwitchCategory {
    /// Template URL for the game’s box art.
    pub box_art_url: String,
    /// Game or category ID.
    pub id: CategoryId,
    /// Game name.
    pub name: String,
}

/// Subscription tiers
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(field_identifier)]
pub enum SubscriptionTier {
    /// Tier 1. $4.99
    #[serde(rename = "1000")]
    Tier1,
    /// Tier 1. $9.99
    #[serde(rename = "2000")]
    Tier2,
    /// Tier 1. $24.99
    #[serde(rename = "3000")]
    Tier3,
    /// Prime subscription
    Prime,
    /// Other
    Other(String),
}

impl Serialize for SubscriptionTier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            SubscriptionTier::Tier1 => "1000",
            SubscriptionTier::Tier2 => "2000",
            SubscriptionTier::Tier3 => "3000",
            SubscriptionTier::Prime => "Prime",
            SubscriptionTier::Other(o) => o,
        })
    }
}

/// Broadcaster types: "partner", "affiliate", or "".
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum BroadcasterType {
    /// Partner
    #[serde(rename = "partner")]
    Partner,
    /// Affiliate
    #[serde(rename = "affiliate")]
    Affiliate,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for BroadcasterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            BroadcasterType::Partner => "partner",
            BroadcasterType::Affiliate => "affiliate",
            BroadcasterType::None => "",
        })
    }
}

/// User types: "staff", "admin", "global_mod", or "".
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum UserType {
    /// Staff
    #[serde(rename = "staff")]
    Staff,
    /// Admin
    #[serde(rename = "admin")]
    Admin,
    /// Global Moderator
    #[serde(rename = "global_mod")]
    GlobalMod,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            UserType::Staff => "staff",
            UserType::Admin => "admin",
            UserType::GlobalMod => "global_mod",
            UserType::None => "",
        })
    }
}

/// Period during which the video was created
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPeriod {
    /// Filter by all. Effectively a no-op
    All,
    /// Filter by from this day only
    Day,
    /// Filter by this week
    Week,
    /// Filter by this month
    Month,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoType {
    /// A live video
    Live,
    // FIXME: What is this?
    /// A playlist video
    Playlist,
    /// A uploaded video
    Upload,
    /// An archived video
    Archive,
    /// A highlight
    Highlight,
    /// A premiere
    Premiere,
    /// A rerun
    Rerun,
    /// A watch party
    WatchParty,
    /// A watchparty premiere,
    WatchPartyPremiere,
    /// A watchparty rerun
    WatchPartyRerun,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPrivacy {
    /// Video is public
    Public,
    /// Video is private
    Private,
}

/// Length of the commercial in seconds
#[derive(
    displaydoc::Display,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[repr(u64)]
#[non_exhaustive]
pub enum CommercialLength {
    /// 30s
    Length30 = 30,
    /// 60s
    Length60 = 60,
    /// 90s
    Length90 = 90,
    /// 120s
    Length120 = 120,
    /// 150s
    Length150 = 150,
    /// 180s
    Length180 = 180,
}

impl std::convert::TryFrom<u64> for CommercialLength {
    type Error = CommercialLengthParseError;

    fn try_from(l: u64) -> Result<Self, Self::Error> {
        match l {
            30 => Ok(CommercialLength::Length30),
            60 => Ok(CommercialLength::Length60),
            90 => Ok(CommercialLength::Length90),
            120 => Ok(CommercialLength::Length120),
            150 => Ok(CommercialLength::Length150),
            180 => Ok(CommercialLength::Length180),
            other => Err(CommercialLengthParseError::InvalidLength(other)),
        }
    }
}

/// Error for the `TryFrom` on [`CommercialLength`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum CommercialLengthParseError {
    /// invalid length of {0}
    InvalidLength(u64),
}

/// A user according to many endpoints
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct User {
    /// ID of the user
    #[serde(alias = "user_id")]
    pub id: UserId,
    /// Login name of the user, not capitalized
    #[serde(alias = "user_login")]
    pub login: UserName,
    /// Display name of user
    #[serde(alias = "user_display_name", alias = "user_name")]
    pub display_name: DisplayName,
    #[serde(default)]
    /// URL of the user's profile
    pub profile_image_url: Option<String>,
}

/// Links to the same image of different sizes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Image {
    /// URL to png of size 28x28
    pub url_1x: String,
    /// URL to png of size 56x56
    pub url_2x: String,
    /// URL to png of size 112x112
    pub url_4x: String,
}

/// Information about global cooldown
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalCooldown {
    /// Cooldown enabled
    pub is_enabled: bool,
    /// Cooldown amount
    #[serde(alias = "seconds")]
    pub global_cooldown_seconds: u32,
}

/// Reward redemption max
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(untagged)]
#[non_exhaustive]
pub enum Max {
    /// Max per stream
    MaxPerStream {
        /// Max per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per stream
        #[serde(alias = "value")]
        max_per_stream: u32,
    },
    /// Max per user per stream
    MaxPerUserPerStream {
        /// Max per user per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per user per stream
        #[serde(alias = "value")]
        max_per_user_per_stream: u32,
    },
}

/// Poll choice
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PollChoice {
    /// ID for the choice.
    pub id: String,
    /// Text displayed for the choice.
    pub title: String,
    /// Total number of votes received for the choice across all methods of voting.
    pub votes: Option<i64>,
    /// Number of votes received via Channel Points.
    pub channel_points_votes: Option<i64>,
    /// Number of votes received via Bits.
    pub bits_votes: Option<i64>,
}

// FIXME: Poll status has different name depending on if returned from helix or eventsub. See https://twitch.uservoice.com/forums/310213-developers/suggestions/43402176
/// Status of a poll
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum PollStatus {
    /// Poll is currently in progress.
    #[serde(alias = "active")]
    Active,
    /// Poll has reached its ended_at time.
    #[serde(alias = "completed")]
    Completed,
    /// Poll has been manually terminated before its ended_at time.
    #[serde(alias = "terminated")]
    Terminated,
    /// Poll is no longer visible on the channel.
    #[serde(alias = "archived")]
    Archived,
    /// Poll is no longer visible to any user on Twitch.
    #[serde(alias = "moderated")]
    Moderated,
    /// Something went wrong determining the state.
    #[serde(alias = "invalid")]
    Invalid,
}

// FIXME: Prediction status has different name depending on if returned from helix or eventsub. See https://twitch.uservoice.com/forums/310213-developers/suggestions/43402197
/// Status of the Prediction
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum PredictionStatus {
    /// A winning outcome has been chosen and the Channel Points have been distributed to the users who guessed the correct outcome.
    #[serde(alias = "resolved")]
    Resolved,
    /// The Prediction is active and viewers can make predictions.
    #[serde(alias = "active")]
    Active,
    /// The Prediction has been canceled and the Channel Points have been refunded to participants.
    #[serde(alias = "canceled")]
    Canceled,
    /// The Prediction has been locked and viewers can no longer make predictions.
    #[serde(alias = "locked")]
    Locked,
}

/// Outcome for the Prediction
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PredictionOutcome {
    /// ID for the outcome.
    pub id: String,
    /// Text displayed for outcome.
    pub title: String,
    /// Number of unique users that chose the outcome.
    pub users: Option<i64>,
    /// Number of Channel Points used for the outcome.
    pub channel_points: Option<i64>,
    /// Array of users who were the top predictors. null if none. Top 10
    pub top_predictors: Option<Vec<PredictionTopPredictors>>,
    /// Color for the outcome. Valid values: BLUE, PINK
    pub color: String,
}

// FIXME: eventsub adds prefix `user_*`. See https://discord.com/channels/325552783787032576/326772207844065290/842359030252437514
/// Users who were the top predictors.
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PredictionTopPredictors {
    /// ID of the user.
    #[serde(alias = "user_id")]
    pub id: UserId,
    /// Display name of the user.
    #[serde(alias = "user_name")]
    pub name: DisplayName,
    /// Login of the user.
    #[serde(alias = "user_login")]
    pub login: UserName,
    /// Number of Channel Points used by the user.
    pub channel_points_used: i64,
    /// Number of Channel Points won by the user.
    ///
    /// This value is always null in the event payload for Prediction progress and Prediction lock. This value is 0 if the outcome did not win or if the Prediction was canceled and Channel Points were refunded.
    pub channel_points_won: Option<i64>,
}

/// Status of a message that is or was in AutoMod queue
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum AutomodStatus {
    /// Message has been caught and pending moderation
    Pending,
    /// Message has been allowed
    Allowed,
    /// Message has been denied
    Denied,
    /// Automod message expired in queue
    Expired,
}

/// Type of creator goal
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum CreatorGoalType {
    /// Creator goal is for followers
    Follower,
    /// Creator goal is for subscriptions
    Subscription,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn time_test() {
        let mut time1 = Timestamp::new("2021-11-11T10:00:00Z").unwrap();
        time1.set_time(10, 0, 32);
        let time2 = Timestamp::new("2021-11-10T10:00:00Z").unwrap();
        assert!(time2.is_before(&time1));
        dbg!(time1.normalize().unwrap());
        #[cfg(feature = "time")]
        let time = Timestamp::new("2021-11-11T13:37:00-01:00").unwrap();
        #[cfg(feature = "time")]
        dbg!(time.normalize().unwrap());
    }
}
