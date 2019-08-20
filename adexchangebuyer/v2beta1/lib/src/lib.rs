pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AbsoluteDateRange {
        #[doc = "The end date of the range (inclusive).\nMust be within the 30 days leading up to current date, and must be equal to\nor after start_date."]
        #[serde(rename = "endDate", default)]
        pub end_date: Option<crate::schemas::Date>,
        #[doc = "The start date of the range (inclusive).\nMust be within the 30 days leading up to current date, and must be equal to\nor before end_date."]
        #[serde(rename = "startDate", default)]
        pub start_date: Option<crate::schemas::Date>,
    }
    impl ::field_selector::FieldSelector for AbsoluteDateRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AcceptProposalRequest {
        #[doc = "The last known client revision number of the proposal."]
        #[serde(rename = "proposalRevision", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision: Option<i64>,
    }
    impl ::field_selector::FieldSelector for AcceptProposalRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdSizeSizeType {
        #[doc = "A placeholder for an undefined size type."]
        SizeTypeUnspecified,
        #[doc = "Ad slot with size specified by height and width in pixels."]
        Pixel,
        #[doc = "Special size to describe an interstitial ad slot."]
        Interstitial,
        #[doc = "Native (mobile) ads rendered by the publisher."]
        Native,
        #[doc = "Fluid size (i.e., responsive size) can be resized automatically with the\nchange of outside environment."]
        Fluid,
    }
    impl AdSizeSizeType {
        pub fn as_str(self) -> &'static str {
            match self {
                AdSizeSizeType::SizeTypeUnspecified => "SIZE_TYPE_UNSPECIFIED",
                AdSizeSizeType::Pixel => "PIXEL",
                AdSizeSizeType::Interstitial => "INTERSTITIAL",
                AdSizeSizeType::Native => "NATIVE",
                AdSizeSizeType::Fluid => "FLUID",
            }
        }
    }
    impl ::std::fmt::Display for AdSizeSizeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdSizeSizeType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdSizeSizeType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SIZE_TYPE_UNSPECIFIED" => AdSizeSizeType::SizeTypeUnspecified,
                "PIXEL" => AdSizeSizeType::Pixel,
                "INTERSTITIAL" => AdSizeSizeType::Interstitial,
                "NATIVE" => AdSizeSizeType::Native,
                "FLUID" => AdSizeSizeType::Fluid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AdSize {
        #[doc = "The height of the ad slot in pixels.\nThis field will be present only when size type is `PIXEL`."]
        #[serde(rename = "height", default)]
        #[serde(with = "crate::parsed_string")]
        pub height: Option<i64>,
        #[doc = "The size type of the ad slot."]
        #[serde(rename = "sizeType", default)]
        pub size_type: Option<crate::schemas::AdSizeSizeType>,
        #[doc = "The width of the ad slot in pixels.\nThis field will be present only when size type is `PIXEL`."]
        #[serde(rename = "width", default)]
        #[serde(with = "crate::parsed_string")]
        pub width: Option<i64>,
    }
    impl ::field_selector::FieldSelector for AdSize {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AdTechnologyProviders {
        #[doc = "The detected ad technology provider IDs for this creative.\nSee https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for\nmapping of provider ID to provided name, a privacy policy URL, and a list\nof domains which can be attributed to the provider.\n\nIf the creative contains provider IDs that are outside of those listed in\nthe `BidRequest.adslot.consented_providers_settings.consented_providers`\nfield on the (Google bid\nprotocol)[https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto]\nand the\n`BidRequest.user.ext.consented_providers_settings.consented_providers`\nfield on the (OpenRTB\nprotocol)[https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto],\nand a bid is submitted with that creative for an impression that will\nserve to an EEA user, the bid will be filtered before the auction."]
        #[serde(rename = "detectedProviderIds", default)]
        pub detected_provider_ids: Option<Vec<i64>>,
        #[doc = "Whether the creative contains an unidentified ad technology provider.\n\nIf true for a given creative, any bid submitted with that creative for an\nimpression that will serve to an EEA user will be filtered before the\nauction."]
        #[serde(rename = "hasUnidentifiedProvider", default)]
        pub has_unidentified_provider: Option<bool>,
    }
    impl ::field_selector::FieldSelector for AdTechnologyProviders {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AddDealAssociationRequest {
        #[doc = "The association between a creative and a deal that should be added."]
        #[serde(rename = "association", default)]
        pub association: Option<crate::schemas::CreativeDealAssociation>,
    }
    impl ::field_selector::FieldSelector for AddDealAssociationRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AddNoteRequest {
        #[doc = "Details of the note to add."]
        #[serde(rename = "note", default)]
        pub note: Option<crate::schemas::Note>,
    }
    impl ::field_selector::FieldSelector for AddNoteRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AppContextAppTypesItems {}
    impl AppContextAppTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for AppContextAppTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AppContextAppTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AppContextAppTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppContext {
        #[doc = "The app types this restriction applies to."]
        #[serde(rename = "appTypes", default)]
        pub app_types: Option<Vec<crate::schemas::AppContextAppTypesItems>>,
    }
    impl ::field_selector::FieldSelector for AppContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuctionContextAuctionTypesItems {}
    impl AuctionContextAuctionTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for AuctionContextAuctionTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuctionContextAuctionTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuctionContextAuctionTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuctionContext {
        #[doc = "The auction types this restriction applies to."]
        #[serde(rename = "auctionTypes", default)]
        pub auction_types: Option<Vec<crate::schemas::AuctionContextAuctionTypesItems>>,
    }
    impl ::field_selector::FieldSelector for AuctionContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BidMetricsRow {
        #[doc = "The number of bids that Ad Exchange received from the buyer."]
        #[serde(rename = "bids", default)]
        pub bids: Option<crate::schemas::MetricValue>,
        #[doc = "The number of bids that were permitted to compete in the auction."]
        #[serde(rename = "bidsInAuction", default)]
        pub bids_in_auction: Option<crate::schemas::MetricValue>,
        #[doc = "The number of bids for which the buyer was billed."]
        #[serde(rename = "billedImpressions", default)]
        pub billed_impressions: Option<crate::schemas::MetricValue>,
        #[doc = "The number of bids that won an impression."]
        #[serde(rename = "impressionsWon", default)]
        pub impressions_won: Option<crate::schemas::MetricValue>,
        #[doc = "The number of bids for which the corresponding impression was measurable\nfor viewability (as defined by Active View)."]
        #[serde(rename = "measurableImpressions", default)]
        pub measurable_impressions: Option<crate::schemas::MetricValue>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
        #[doc = "The number of bids for which the corresponding impression was viewable (as\ndefined by Active View)."]
        #[serde(rename = "viewableImpressions", default)]
        pub viewable_impressions: Option<crate::schemas::MetricValue>,
    }
    impl ::field_selector::FieldSelector for BidMetricsRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BidResponseWithoutBidsStatusRowStatus {
        #[doc = "A placeholder for an undefined status.\nThis value will never be returned in responses."]
        StatusUnspecified,
        #[doc = "The response had no bids."]
        ResponsesWithoutBids,
        #[doc = "The response had no bids for the specified account, though it may have\nincluded bids on behalf of other accounts."]
        ResponsesWithoutBidsForAccount,
        #[doc = "The response had no bids for the specified deal, though it may have\nincluded bids on other deals on behalf of the account to which the deal\nbelongs."]
        ResponsesWithoutBidsForDeal,
    }
    impl BidResponseWithoutBidsStatusRowStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                BidResponseWithoutBidsStatusRowStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                BidResponseWithoutBidsStatusRowStatus::ResponsesWithoutBids => {
                    "RESPONSES_WITHOUT_BIDS"
                }
                BidResponseWithoutBidsStatusRowStatus::ResponsesWithoutBidsForAccount => {
                    "RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT"
                }
                BidResponseWithoutBidsStatusRowStatus::ResponsesWithoutBidsForDeal => {
                    "RESPONSES_WITHOUT_BIDS_FOR_DEAL"
                }
            }
        }
    }
    impl ::std::fmt::Display for BidResponseWithoutBidsStatusRowStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BidResponseWithoutBidsStatusRowStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BidResponseWithoutBidsStatusRowStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNSPECIFIED" => BidResponseWithoutBidsStatusRowStatus::StatusUnspecified,
                "RESPONSES_WITHOUT_BIDS" => {
                    BidResponseWithoutBidsStatusRowStatus::ResponsesWithoutBids
                }
                "RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT" => {
                    BidResponseWithoutBidsStatusRowStatus::ResponsesWithoutBidsForAccount
                }
                "RESPONSES_WITHOUT_BIDS_FOR_DEAL" => {
                    BidResponseWithoutBidsStatusRowStatus::ResponsesWithoutBidsForDeal
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BidResponseWithoutBidsStatusRow {
        #[doc = "The number of impressions for which there was a bid response with the\nspecified status."]
        #[serde(rename = "impressionCount", default)]
        pub impression_count: Option<crate::schemas::MetricValue>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
        #[doc = "The status specifying why the bid responses were considered to have no\napplicable bids."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::BidResponseWithoutBidsStatusRowStatus>,
    }
    impl ::field_selector::FieldSelector for BidResponseWithoutBidsStatusRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Buyer {
        #[doc = "Authorized Buyers account ID of the buyer."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Buyer {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CalloutStatusRow {
        #[doc = "The ID of the callout status.\nSee\n[callout-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/callout-status-codes)."]
        #[serde(rename = "calloutStatusId", default)]
        pub callout_status_id: Option<i32>,
        #[doc = "The number of impressions for which there was a bid request or bid response\nwith the specified callout status."]
        #[serde(rename = "impressionCount", default)]
        pub impression_count: Option<crate::schemas::MetricValue>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
    }
    impl ::field_selector::FieldSelector for CalloutStatusRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CancelNegotiationRequest;
    impl ::field_selector::FieldSelector for CancelNegotiationRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ClientEntityType {
        #[doc = "A placeholder for an undefined client entity type. Should not be used."]
        EntityTypeUnspecified,
        #[doc = "An advertiser."]
        Advertiser,
        #[doc = "A brand."]
        Brand,
        #[doc = "An advertising agency."]
        Agency,
        #[doc = "An explicit value for a client that was not yet classified\nas any particular entity."]
        EntityTypeUnclassified,
    }
    impl ClientEntityType {
        pub fn as_str(self) -> &'static str {
            match self {
                ClientEntityType::EntityTypeUnspecified => "ENTITY_TYPE_UNSPECIFIED",
                ClientEntityType::Advertiser => "ADVERTISER",
                ClientEntityType::Brand => "BRAND",
                ClientEntityType::Agency => "AGENCY",
                ClientEntityType::EntityTypeUnclassified => "ENTITY_TYPE_UNCLASSIFIED",
            }
        }
    }
    impl ::std::fmt::Display for ClientEntityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ClientEntityType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ClientEntityType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENTITY_TYPE_UNSPECIFIED" => ClientEntityType::EntityTypeUnspecified,
                "ADVERTISER" => ClientEntityType::Advertiser,
                "BRAND" => ClientEntityType::Brand,
                "AGENCY" => ClientEntityType::Agency,
                "ENTITY_TYPE_UNCLASSIFIED" => ClientEntityType::EntityTypeUnclassified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ClientRole {
        #[doc = "A placeholder for an undefined client role."]
        ClientRoleUnspecified,
        #[doc = "Users associated with this client can see publisher deal offers\nin the Marketplace.\nThey can neither negotiate proposals nor approve deals.\nIf this client is visible to publishers, they can send deal proposals\nto this client."]
        ClientDealViewer,
        #[doc = "Users associated with this client can respond to deal proposals\nsent to them by publishers. They can also initiate deal proposals\nof their own."]
        ClientDealNegotiator,
        #[doc = "Users associated with this client can approve eligible deals\non your behalf. Some deals may still explicitly require publisher\nfinalization. If this role is not selected, the sponsor buyer\nwill need to manually approve each of their deals."]
        ClientDealApprover,
    }
    impl ClientRole {
        pub fn as_str(self) -> &'static str {
            match self {
                ClientRole::ClientRoleUnspecified => "CLIENT_ROLE_UNSPECIFIED",
                ClientRole::ClientDealViewer => "CLIENT_DEAL_VIEWER",
                ClientRole::ClientDealNegotiator => "CLIENT_DEAL_NEGOTIATOR",
                ClientRole::ClientDealApprover => "CLIENT_DEAL_APPROVER",
            }
        }
    }
    impl ::std::fmt::Display for ClientRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ClientRole {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ClientRole {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLIENT_ROLE_UNSPECIFIED" => ClientRole::ClientRoleUnspecified,
                "CLIENT_DEAL_VIEWER" => ClientRole::ClientDealViewer,
                "CLIENT_DEAL_NEGOTIATOR" => ClientRole::ClientDealNegotiator,
                "CLIENT_DEAL_APPROVER" => ClientRole::ClientDealApprover,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ClientStatus {
        #[doc = "A placeholder for an undefined client status."]
        ClientStatusUnspecified,
        #[doc = "A client that is currently disabled."]
        Disabled,
        #[doc = "A client that is currently active."]
        Active,
    }
    impl ClientStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ClientStatus::ClientStatusUnspecified => "CLIENT_STATUS_UNSPECIFIED",
                ClientStatus::Disabled => "DISABLED",
                ClientStatus::Active => "ACTIVE",
            }
        }
    }
    impl ::std::fmt::Display for ClientStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ClientStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ClientStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLIENT_STATUS_UNSPECIFIED" => ClientStatus::ClientStatusUnspecified,
                "DISABLED" => ClientStatus::Disabled,
                "ACTIVE" => ClientStatus::Active,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Client {
        #[doc = "The globally-unique numerical ID of the client.\nThe value of this field is ignored in create and update operations."]
        #[serde(rename = "clientAccountId", default)]
        #[serde(with = "crate::parsed_string")]
        pub client_account_id: Option<i64>,
        #[doc = "Name used to represent this client to publishers.\nYou may have multiple clients that map to the same entity,\nbut for each client the combination of `clientName` and entity\nmust be unique.\nYou can specify this field as empty."]
        #[serde(rename = "clientName", default)]
        pub client_name: Option<String>,
        #[doc = "Numerical identifier of the client entity.\nThe entity can be an advertiser, a brand, or an agency.\nThis identifier is unique among all the entities with the same type.\n\nA list of all known advertisers with their identifiers is available in the\n[advertisers.txt](https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt)\nfile.\n\nA list of all known brands with their identifiers is available in the\n[brands.txt](https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt)\nfile.\n\nA list of all known agencies with their identifiers is available in the\n[agencies.txt](https://storage.googleapis.com/adx-rtb-dictionaries/agencies.txt)\nfile."]
        #[serde(rename = "entityId", default)]
        #[serde(with = "crate::parsed_string")]
        pub entity_id: Option<i64>,
        #[doc = "The name of the entity. This field is automatically fetched based on\nthe type and ID.\nThe value of this field is ignored in create and update operations."]
        #[serde(rename = "entityName", default)]
        pub entity_name: Option<String>,
        #[doc = "The type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`."]
        #[serde(rename = "entityType", default)]
        pub entity_type: Option<crate::schemas::ClientEntityType>,
        #[doc = "Optional arbitrary unique identifier of this client buyer from the\nstandpoint of its Ad Exchange sponsor buyer.\n\nThis field can be used to associate a client buyer with the identifier\nin the namespace of its sponsor buyer, lookup client buyers by that\nidentifier and verify whether an Ad Exchange counterpart of a given client\nbuyer already exists.\n\nIf present, must be unique among all the client buyers for its\nAd Exchange sponsor buyer."]
        #[serde(rename = "partnerClientId", default)]
        pub partner_client_id: Option<String>,
        #[doc = "The role which is assigned to the client buyer. Each role implies a set of\npermissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`,\n`CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`."]
        #[serde(rename = "role", default)]
        pub role: Option<crate::schemas::ClientRole>,
        #[doc = "The status of the client buyer."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::ClientStatus>,
        #[doc = "Whether the client buyer will be visible to sellers."]
        #[serde(rename = "visibleToSeller", default)]
        pub visible_to_seller: Option<bool>,
    }
    impl ::field_selector::FieldSelector for Client {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ClientUserStatus {
        #[doc = "A placeholder for an undefined user status."]
        UserStatusUnspecified,
        #[doc = "A user who was already created but hasn't accepted the invitation yet."]
        Pending,
        #[doc = "A user that is currently active."]
        Active,
        #[doc = "A user that is currently disabled."]
        Disabled,
    }
    impl ClientUserStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ClientUserStatus::UserStatusUnspecified => "USER_STATUS_UNSPECIFIED",
                ClientUserStatus::Pending => "PENDING",
                ClientUserStatus::Active => "ACTIVE",
                ClientUserStatus::Disabled => "DISABLED",
            }
        }
    }
    impl ::std::fmt::Display for ClientUserStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ClientUserStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ClientUserStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_STATUS_UNSPECIFIED" => ClientUserStatus::UserStatusUnspecified,
                "PENDING" => ClientUserStatus::Pending,
                "ACTIVE" => ClientUserStatus::Active,
                "DISABLED" => ClientUserStatus::Disabled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ClientUser {
        #[doc = "Numerical account ID of the client buyer\nwith which the user is associated; the\nbuyer must be a client of the current sponsor buyer.\nThe value of this field is ignored in an update operation."]
        #[serde(rename = "clientAccountId", default)]
        #[serde(with = "crate::parsed_string")]
        pub client_account_id: Option<i64>,
        #[doc = "User's email address. The value of this field\nis ignored in an update operation."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The status of the client user."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::ClientUserStatus>,
        #[doc = "The unique numerical ID of the client user\nthat has accepted an invitation.\nThe value of this field is ignored in an update operation."]
        #[serde(rename = "userId", default)]
        #[serde(with = "crate::parsed_string")]
        pub user_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector for ClientUser {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ClientUserInvitation {
        #[doc = "Numerical account ID of the client buyer\nthat the invited user is associated with.\nThe value of this field is ignored in create operations."]
        #[serde(rename = "clientAccountId", default)]
        #[serde(with = "crate::parsed_string")]
        pub client_account_id: Option<i64>,
        #[doc = "The email address to which the invitation is sent. Email\naddresses should be unique among all client users under each sponsor\nbuyer."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The unique numerical ID of the invitation that is sent to the user.\nThe value of this field is ignored in create operations."]
        #[serde(rename = "invitationId", default)]
        #[serde(with = "crate::parsed_string")]
        pub invitation_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector for ClientUserInvitation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CompleteSetupRequest;
    impl ::field_selector::FieldSelector for CompleteSetupRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContactInformation {
        #[doc = "Email address for the contact."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The name of the contact."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ContactInformation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CorrectionType {
        #[doc = "The correction type is unknown. Refer to the details for more information."]
        CorrectionTypeUnspecified,
        #[doc = "The ad's declared vendors did not match the vendors that were detected.\nThe detected vendors were added."]
        VendorIdsAdded,
        #[doc = "The ad had the SSL attribute declared but was not SSL-compliant.\nThe SSL attribute was removed."]
        SslAttributeRemoved,
        #[doc = "The ad was declared as Flash-free but contained Flash, so the Flash-free\nattribute was removed."]
        FlashFreeAttributeRemoved,
        #[doc = "The ad was not declared as Flash-free but it did not reference any flash\ncontent, so the Flash-free attribute was added."]
        FlashFreeAttributeAdded,
        #[doc = "The ad did not declare a required creative attribute.\nThe attribute was added."]
        RequiredAttributeAdded,
        #[doc = "The ad did not declare a required technology vendor.\nThe technology vendor was added."]
        RequiredVendorAdded,
        #[doc = "The ad did not declare the SSL attribute but was SSL-compliant, so the\nSSL attribute was added."]
        SslAttributeAdded,
        #[doc = "Properties consistent with In-banner video were found, so an\nIn-Banner Video attribute was added."]
        InBannerVideoAttributeAdded,
        #[doc = "The ad makes calls to the MRAID API so the MRAID attribute was added."]
        MraidAttributeAdded,
        #[doc = "The ad unnecessarily declared the Flash attribute, so the Flash attribute\nwas removed."]
        FlashAttributeRemoved,
        #[doc = "The ad contains video content."]
        VideoInSnippetAttributeAdded,
    }
    impl CorrectionType {
        pub fn as_str(self) -> &'static str {
            match self {
                CorrectionType::CorrectionTypeUnspecified => "CORRECTION_TYPE_UNSPECIFIED",
                CorrectionType::VendorIdsAdded => "VENDOR_IDS_ADDED",
                CorrectionType::SslAttributeRemoved => "SSL_ATTRIBUTE_REMOVED",
                CorrectionType::FlashFreeAttributeRemoved => "FLASH_FREE_ATTRIBUTE_REMOVED",
                CorrectionType::FlashFreeAttributeAdded => "FLASH_FREE_ATTRIBUTE_ADDED",
                CorrectionType::RequiredAttributeAdded => "REQUIRED_ATTRIBUTE_ADDED",
                CorrectionType::RequiredVendorAdded => "REQUIRED_VENDOR_ADDED",
                CorrectionType::SslAttributeAdded => "SSL_ATTRIBUTE_ADDED",
                CorrectionType::InBannerVideoAttributeAdded => "IN_BANNER_VIDEO_ATTRIBUTE_ADDED",
                CorrectionType::MraidAttributeAdded => "MRAID_ATTRIBUTE_ADDED",
                CorrectionType::FlashAttributeRemoved => "FLASH_ATTRIBUTE_REMOVED",
                CorrectionType::VideoInSnippetAttributeAdded => "VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED",
            }
        }
    }
    impl ::std::fmt::Display for CorrectionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CorrectionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CorrectionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CORRECTION_TYPE_UNSPECIFIED" => CorrectionType::CorrectionTypeUnspecified,
                "VENDOR_IDS_ADDED" => CorrectionType::VendorIdsAdded,
                "SSL_ATTRIBUTE_REMOVED" => CorrectionType::SslAttributeRemoved,
                "FLASH_FREE_ATTRIBUTE_REMOVED" => CorrectionType::FlashFreeAttributeRemoved,
                "FLASH_FREE_ATTRIBUTE_ADDED" => CorrectionType::FlashFreeAttributeAdded,
                "REQUIRED_ATTRIBUTE_ADDED" => CorrectionType::RequiredAttributeAdded,
                "REQUIRED_VENDOR_ADDED" => CorrectionType::RequiredVendorAdded,
                "SSL_ATTRIBUTE_ADDED" => CorrectionType::SslAttributeAdded,
                "IN_BANNER_VIDEO_ATTRIBUTE_ADDED" => CorrectionType::InBannerVideoAttributeAdded,
                "MRAID_ATTRIBUTE_ADDED" => CorrectionType::MraidAttributeAdded,
                "FLASH_ATTRIBUTE_REMOVED" => CorrectionType::FlashAttributeRemoved,
                "VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED" => CorrectionType::VideoInSnippetAttributeAdded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Correction {
        #[doc = "The contexts for the correction."]
        #[serde(rename = "contexts", default)]
        pub contexts: Option<Vec<crate::schemas::ServingContext>>,
        #[doc = "Additional details about what was corrected."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<String>>,
        #[doc = "The type of correction that was applied to the creative."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::CorrectionType>,
    }
    impl ::field_selector::FieldSelector for Correction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeAttributesItems {}
    impl CreativeAttributesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for CreativeAttributesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeAttributesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeAttributesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeDealsStatus {
        #[doc = "The status is unknown."]
        StatusUnspecified,
        #[doc = "The creative has not been checked."]
        NotChecked,
        #[doc = "The creative has been conditionally approved.\nSee serving_restrictions for details."]
        ConditionallyApproved,
        #[doc = "The creative has been approved."]
        Approved,
        #[doc = "The creative has been disapproved."]
        Disapproved,
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        PendingReview,
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        StatusTypeUnspecified,
    }
    impl CreativeDealsStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeDealsStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                CreativeDealsStatus::NotChecked => "NOT_CHECKED",
                CreativeDealsStatus::ConditionallyApproved => "CONDITIONALLY_APPROVED",
                CreativeDealsStatus::Approved => "APPROVED",
                CreativeDealsStatus::Disapproved => "DISAPPROVED",
                CreativeDealsStatus::PendingReview => "PENDING_REVIEW",
                CreativeDealsStatus::StatusTypeUnspecified => "STATUS_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for CreativeDealsStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeDealsStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeDealsStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNSPECIFIED" => CreativeDealsStatus::StatusUnspecified,
                "NOT_CHECKED" => CreativeDealsStatus::NotChecked,
                "CONDITIONALLY_APPROVED" => CreativeDealsStatus::ConditionallyApproved,
                "APPROVED" => CreativeDealsStatus::Approved,
                "DISAPPROVED" => CreativeDealsStatus::Disapproved,
                "PENDING_REVIEW" => CreativeDealsStatus::PendingReview,
                "STATUS_TYPE_UNSPECIFIED" => CreativeDealsStatus::StatusTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeOpenAuctionStatus {
        #[doc = "The status is unknown."]
        StatusUnspecified,
        #[doc = "The creative has not been checked."]
        NotChecked,
        #[doc = "The creative has been conditionally approved.\nSee serving_restrictions for details."]
        ConditionallyApproved,
        #[doc = "The creative has been approved."]
        Approved,
        #[doc = "The creative has been disapproved."]
        Disapproved,
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        PendingReview,
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        StatusTypeUnspecified,
    }
    impl CreativeOpenAuctionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeOpenAuctionStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                CreativeOpenAuctionStatus::NotChecked => "NOT_CHECKED",
                CreativeOpenAuctionStatus::ConditionallyApproved => "CONDITIONALLY_APPROVED",
                CreativeOpenAuctionStatus::Approved => "APPROVED",
                CreativeOpenAuctionStatus::Disapproved => "DISAPPROVED",
                CreativeOpenAuctionStatus::PendingReview => "PENDING_REVIEW",
                CreativeOpenAuctionStatus::StatusTypeUnspecified => "STATUS_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for CreativeOpenAuctionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeOpenAuctionStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeOpenAuctionStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNSPECIFIED" => CreativeOpenAuctionStatus::StatusUnspecified,
                "NOT_CHECKED" => CreativeOpenAuctionStatus::NotChecked,
                "CONDITIONALLY_APPROVED" => CreativeOpenAuctionStatus::ConditionallyApproved,
                "APPROVED" => CreativeOpenAuctionStatus::Approved,
                "DISAPPROVED" => CreativeOpenAuctionStatus::Disapproved,
                "PENDING_REVIEW" => CreativeOpenAuctionStatus::PendingReview,
                "STATUS_TYPE_UNSPECIFIED" => CreativeOpenAuctionStatus::StatusTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeRestrictedCategoriesItems {}
    impl CreativeRestrictedCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for CreativeRestrictedCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeRestrictedCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeRestrictedCategoriesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Creative {
        #[doc = "The account that this creative belongs to.\nCan be used to filter the response of the\ncreatives.list\nmethod."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The link to AdChoices destination page."]
        #[serde(rename = "adChoicesDestinationUrl", default)]
        pub ad_choices_destination_url: Option<String>,
        #[doc = "Output only. The detected ad technology providers."]
        #[serde(rename = "adTechnologyProviders", default)]
        pub ad_technology_providers: Option<crate::schemas::AdTechnologyProviders>,
        #[doc = "The name of the company being advertised in the creative."]
        #[serde(rename = "advertiserName", default)]
        pub advertiser_name: Option<String>,
        #[doc = "The agency ID for this creative."]
        #[serde(rename = "agencyId", default)]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: Option<i64>,
        #[doc = "Output only. The last update timestamp of the creative via API."]
        #[serde(rename = "apiUpdateTime", default)]
        pub api_update_time: Option<String>,
        #[doc = "All attributes for the ads that may be shown from this creative.\nCan be used to filter the response of the\ncreatives.list\nmethod."]
        #[serde(rename = "attributes", default)]
        pub attributes: Option<Vec<crate::schemas::CreativeAttributesItems>>,
        #[doc = "The set of destination URLs for the creative."]
        #[serde(rename = "clickThroughUrls", default)]
        pub click_through_urls: Option<Vec<String>>,
        #[doc = "Output only. Shows any corrections that were applied to this creative."]
        #[serde(rename = "corrections", default)]
        pub corrections: Option<Vec<crate::schemas::Correction>>,
        #[doc = "The buyer-defined creative ID of this creative.\nCan be used to filter the response of the\ncreatives.list\nmethod."]
        #[serde(rename = "creativeId", default)]
        pub creative_id: Option<String>,
        #[doc = "Output only. The top-level deals status of this creative.\nIf disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in\nserving_restrictions will also exist. Note\nthat this may be nuanced with other contextual restrictions, in which case,\nit may be preferable to read from serving_restrictions directly.\nCan be used to filter the response of the\ncreatives.list\nmethod."]
        #[serde(rename = "dealsStatus", default)]
        pub deals_status: Option<crate::schemas::CreativeDealsStatus>,
        #[doc = "The set of declared destination URLs for the creative."]
        #[serde(rename = "declaredClickThroughUrls", default)]
        pub declared_click_through_urls: Option<Vec<String>>,
        #[doc = "Output only. Detected advertiser IDs, if any."]
        #[serde(rename = "detectedAdvertiserIds", default)]
        pub detected_advertiser_ids: Option<Vec<i64>>,
        #[doc = "Output only. The detected domains for this creative."]
        #[serde(rename = "detectedDomains", default)]
        pub detected_domains: Option<Vec<String>>,
        #[doc = "Output only. The detected languages for this creative. The order is\narbitrary. The codes are 2 or 5 characters and are documented at\nhttps://developers.google.com/adwords/api/docs/appendix/languagecodes."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: Option<Vec<String>>,
        #[doc = "Output only. Detected product categories, if any.\nSee the ad-product-categories.txt file in the technical documentation\nfor a list of IDs."]
        #[serde(rename = "detectedProductCategories", default)]
        pub detected_product_categories: Option<Vec<i32>>,
        #[doc = "Output only. Detected sensitive categories, if any.\nSee the ad-sensitive-categories.txt file in the technical documentation for\na list of IDs. You should use these IDs along with the\nexcluded-sensitive-category field in the bid request to filter your bids."]
        #[serde(rename = "detectedSensitiveCategories", default)]
        pub detected_sensitive_categories: Option<Vec<i32>>,
        #[doc = "An HTML creative."]
        #[serde(rename = "html", default)]
        pub html: Option<crate::schemas::HtmlContent>,
        #[doc = "The set of URLs to be called to record an impression."]
        #[serde(rename = "impressionTrackingUrls", default)]
        pub impression_tracking_urls: Option<Vec<String>>,
        #[doc = "A native creative."]
        #[serde(rename = "native", default)]
        pub native: Option<crate::schemas::NativeContent>,
        #[doc = "Output only. The top-level open auction status of this creative.\nIf disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in\nserving_restrictions will also exist. Note\nthat this may be nuanced with other contextual restrictions, in which case,\nit may be preferable to read from serving_restrictions directly.\nCan be used to filter the response of the\ncreatives.list\nmethod."]
        #[serde(rename = "openAuctionStatus", default)]
        pub open_auction_status: Option<crate::schemas::CreativeOpenAuctionStatus>,
        #[doc = "All restricted categories for the ads that may be shown from this creative."]
        #[serde(rename = "restrictedCategories", default)]
        pub restricted_categories: Option<Vec<crate::schemas::CreativeRestrictedCategoriesItems>>,
        #[doc = "Output only. The granular status of this ad in specific contexts.\nA context here relates to where something ultimately serves (for example,\na physical location, a platform, an HTTPS vs HTTP request, or the type\nof auction)."]
        #[serde(rename = "servingRestrictions", default)]
        pub serving_restrictions: Option<Vec<crate::schemas::ServingRestriction>>,
        #[doc = "All vendor IDs for the ads that may be shown from this creative.\nSee https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt\nfor possible values."]
        #[serde(rename = "vendorIds", default)]
        pub vendor_ids: Option<Vec<i32>>,
        #[doc = "Output only. The version of this creative."]
        #[serde(rename = "version", default)]
        pub version: Option<i32>,
        #[doc = "A video creative."]
        #[serde(rename = "video", default)]
        pub video: Option<crate::schemas::VideoContent>,
    }
    impl ::field_selector::FieldSelector for Creative {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreativeDealAssociation {
        #[doc = "The account the creative belongs to."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The ID of the creative associated with the deal."]
        #[serde(rename = "creativeId", default)]
        pub creative_id: Option<String>,
        #[doc = "The externalDealId for the deal associated with the creative."]
        #[serde(rename = "dealsId", default)]
        pub deals_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativeDealAssociation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeRestrictionsCreativeFormat {
        #[doc = "A placeholder for an undefined creative format."]
        CreativeFormatUnspecified,
        #[doc = "A creative that will be displayed in environments such as a browser."]
        Display,
        #[doc = "A video creative that will be displayed in environments such as a video\nplayer."]
        Video,
    }
    impl CreativeRestrictionsCreativeFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeRestrictionsCreativeFormat::CreativeFormatUnspecified => {
                    "CREATIVE_FORMAT_UNSPECIFIED"
                }
                CreativeRestrictionsCreativeFormat::Display => "DISPLAY",
                CreativeRestrictionsCreativeFormat::Video => "VIDEO",
            }
        }
    }
    impl ::std::fmt::Display for CreativeRestrictionsCreativeFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeRestrictionsCreativeFormat {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeRestrictionsCreativeFormat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_FORMAT_UNSPECIFIED" => {
                    CreativeRestrictionsCreativeFormat::CreativeFormatUnspecified
                }
                "DISPLAY" => CreativeRestrictionsCreativeFormat::Display,
                "VIDEO" => CreativeRestrictionsCreativeFormat::Video,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeRestrictionsSkippableAdType {
        #[doc = "A placeholder for an undefined skippable ad type."]
        SkippableAdTypeUnspecified,
        #[doc = "This video ad can be skipped after 5 seconds."]
        Skippable,
        #[doc = "This video ad can be skipped after 5 seconds, and is counted as\nengaged view after 30 seconds. The creative is hosted on\nYouTube only, and viewcount of the YouTube video increments\nafter the engaged view."]
        InstreamSelect,
        #[doc = "This video ad is not skippable."]
        NotSkippable,
    }
    impl CreativeRestrictionsSkippableAdType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeRestrictionsSkippableAdType::SkippableAdTypeUnspecified => {
                    "SKIPPABLE_AD_TYPE_UNSPECIFIED"
                }
                CreativeRestrictionsSkippableAdType::Skippable => "SKIPPABLE",
                CreativeRestrictionsSkippableAdType::InstreamSelect => "INSTREAM_SELECT",
                CreativeRestrictionsSkippableAdType::NotSkippable => "NOT_SKIPPABLE",
            }
        }
    }
    impl ::std::fmt::Display for CreativeRestrictionsSkippableAdType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeRestrictionsSkippableAdType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeRestrictionsSkippableAdType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SKIPPABLE_AD_TYPE_UNSPECIFIED" => {
                    CreativeRestrictionsSkippableAdType::SkippableAdTypeUnspecified
                }
                "SKIPPABLE" => CreativeRestrictionsSkippableAdType::Skippable,
                "INSTREAM_SELECT" => CreativeRestrictionsSkippableAdType::InstreamSelect,
                "NOT_SKIPPABLE" => CreativeRestrictionsSkippableAdType::NotSkippable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreativeRestrictions {
        #[doc = "The format of the environment that the creatives will be displayed in."]
        #[serde(rename = "creativeFormat", default)]
        pub creative_format: Option<crate::schemas::CreativeRestrictionsCreativeFormat>,
        #[serde(rename = "creativeSpecifications", default)]
        pub creative_specifications: Option<Vec<crate::schemas::CreativeSpecification>>,
        #[doc = "Skippable video ads allow viewers to skip ads after 5 seconds."]
        #[serde(rename = "skippableAdType", default)]
        pub skippable_ad_type: Option<crate::schemas::CreativeRestrictionsSkippableAdType>,
    }
    impl ::field_selector::FieldSelector for CreativeRestrictions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeSizeAllowedFormatsItems {}
    impl CreativeSizeAllowedFormatsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for CreativeSizeAllowedFormatsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeSizeAllowedFormatsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeSizeAllowedFormatsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeSizeCreativeSizeType {
        #[doc = "A placeholder for an undefined creative size type."]
        CreativeSizeTypeUnspecified,
        #[doc = "The creative is a regular desktop creative."]
        Regular,
        #[doc = "The creative is an interstitial creative."]
        Interstitial,
        #[doc = "The creative is a video creative."]
        Video,
        #[doc = "The creative is a native (mobile) creative."]
        Native,
    }
    impl CreativeSizeCreativeSizeType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeSizeCreativeSizeType::CreativeSizeTypeUnspecified => {
                    "CREATIVE_SIZE_TYPE_UNSPECIFIED"
                }
                CreativeSizeCreativeSizeType::Regular => "REGULAR",
                CreativeSizeCreativeSizeType::Interstitial => "INTERSTITIAL",
                CreativeSizeCreativeSizeType::Video => "VIDEO",
                CreativeSizeCreativeSizeType::Native => "NATIVE",
            }
        }
    }
    impl ::std::fmt::Display for CreativeSizeCreativeSizeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeSizeCreativeSizeType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeSizeCreativeSizeType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_SIZE_TYPE_UNSPECIFIED" => {
                    CreativeSizeCreativeSizeType::CreativeSizeTypeUnspecified
                }
                "REGULAR" => CreativeSizeCreativeSizeType::Regular,
                "INTERSTITIAL" => CreativeSizeCreativeSizeType::Interstitial,
                "VIDEO" => CreativeSizeCreativeSizeType::Video,
                "NATIVE" => CreativeSizeCreativeSizeType::Native,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeSizeNativeTemplate {
        #[doc = "A placeholder for an undefined native template."]
        UnknownNativeTemplate,
        #[doc = "The creative is linked to native content ad."]
        NativeContentAd,
        #[doc = "The creative is linked to native app install ad."]
        NativeAppInstallAd,
        #[doc = "The creative is linked to native video content ad."]
        NativeVideoContentAd,
        #[doc = "The creative is linked to native video app install ad."]
        NativeVideoAppInstallAd,
    }
    impl CreativeSizeNativeTemplate {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeSizeNativeTemplate::UnknownNativeTemplate => "UNKNOWN_NATIVE_TEMPLATE",
                CreativeSizeNativeTemplate::NativeContentAd => "NATIVE_CONTENT_AD",
                CreativeSizeNativeTemplate::NativeAppInstallAd => "NATIVE_APP_INSTALL_AD",
                CreativeSizeNativeTemplate::NativeVideoContentAd => "NATIVE_VIDEO_CONTENT_AD",
                CreativeSizeNativeTemplate::NativeVideoAppInstallAd => {
                    "NATIVE_VIDEO_APP_INSTALL_AD"
                }
            }
        }
    }
    impl ::std::fmt::Display for CreativeSizeNativeTemplate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeSizeNativeTemplate {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeSizeNativeTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_NATIVE_TEMPLATE" => CreativeSizeNativeTemplate::UnknownNativeTemplate,
                "NATIVE_CONTENT_AD" => CreativeSizeNativeTemplate::NativeContentAd,
                "NATIVE_APP_INSTALL_AD" => CreativeSizeNativeTemplate::NativeAppInstallAd,
                "NATIVE_VIDEO_CONTENT_AD" => CreativeSizeNativeTemplate::NativeVideoContentAd,
                "NATIVE_VIDEO_APP_INSTALL_AD" => {
                    CreativeSizeNativeTemplate::NativeVideoAppInstallAd
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeSizeSkippableAdType {
        #[doc = "A placeholder for an undefined skippable ad type."]
        SkippableAdTypeUnspecified,
        #[doc = "This video ad can be skipped after 5 seconds."]
        Generic,
        #[doc = "This video ad can be skipped after 5 seconds, and count as\nengaged view after 30 seconds. The creative is hosted on\nYouTube only, and viewcount of the YouTube video increments\nafter the engaged view."]
        InstreamSelect,
        #[doc = "This video ad is not skippable."]
        NotSkippable,
    }
    impl CreativeSizeSkippableAdType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeSizeSkippableAdType::SkippableAdTypeUnspecified => {
                    "SKIPPABLE_AD_TYPE_UNSPECIFIED"
                }
                CreativeSizeSkippableAdType::Generic => "GENERIC",
                CreativeSizeSkippableAdType::InstreamSelect => "INSTREAM_SELECT",
                CreativeSizeSkippableAdType::NotSkippable => "NOT_SKIPPABLE",
            }
        }
    }
    impl ::std::fmt::Display for CreativeSizeSkippableAdType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeSizeSkippableAdType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeSizeSkippableAdType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SKIPPABLE_AD_TYPE_UNSPECIFIED" => {
                    CreativeSizeSkippableAdType::SkippableAdTypeUnspecified
                }
                "GENERIC" => CreativeSizeSkippableAdType::Generic,
                "INSTREAM_SELECT" => CreativeSizeSkippableAdType::InstreamSelect,
                "NOT_SKIPPABLE" => CreativeSizeSkippableAdType::NotSkippable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreativeSize {
        #[doc = "What formats are allowed by the publisher.\nIf this repeated field is empty then all formats are allowed.\nFor example, if this field contains AllowedFormatType.AUDIO then the\npublisher only allows an audio ad (without any video)."]
        #[serde(rename = "allowedFormats", default)]
        pub allowed_formats: Option<Vec<crate::schemas::CreativeSizeAllowedFormatsItems>>,
        #[doc = "For video creatives specifies the sizes of companion ads (if present).\nCompanion sizes may be filled in only when creative_size_type = VIDEO"]
        #[serde(rename = "companionSizes", default)]
        pub companion_sizes: Option<Vec<crate::schemas::Size>>,
        #[doc = "The creative size type."]
        #[serde(rename = "creativeSizeType", default)]
        pub creative_size_type: Option<crate::schemas::CreativeSizeCreativeSizeType>,
        #[doc = "Output only. The native template for this creative. It will have a value\nonly if creative_size_type = CreativeSizeType.NATIVE."]
        #[serde(rename = "nativeTemplate", default)]
        pub native_template: Option<crate::schemas::CreativeSizeNativeTemplate>,
        #[doc = "For regular or video creative size type, specifies the size\nof the creative"]
        #[serde(rename = "size", default)]
        pub size: Option<crate::schemas::Size>,
        #[doc = "The type of skippable ad for this creative. It will have a value only if\ncreative_size_type = CreativeSizeType.VIDEO."]
        #[serde(rename = "skippableAdType", default)]
        pub skippable_ad_type: Option<crate::schemas::CreativeSizeSkippableAdType>,
    }
    impl ::field_selector::FieldSelector for CreativeSize {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreativeSpecification {
        #[doc = "Companion sizes may be filled in only when this is a video creative."]
        #[serde(rename = "creativeCompanionSizes", default)]
        pub creative_companion_sizes: Option<Vec<crate::schemas::AdSize>>,
        #[doc = "The size of the creative."]
        #[serde(rename = "creativeSize", default)]
        pub creative_size: Option<crate::schemas::AdSize>,
    }
    impl ::field_selector::FieldSelector for CreativeSpecification {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreativeStatusRow {
        #[doc = "The number of bids with the specified status."]
        #[serde(rename = "bidCount", default)]
        pub bid_count: Option<crate::schemas::MetricValue>,
        #[doc = "The ID of the creative status.\nSee\n[creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes)."]
        #[serde(rename = "creativeStatusId", default)]
        pub creative_status_id: Option<i32>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
    }
    impl ::field_selector::FieldSelector for CreativeStatusRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CriteriaTargeting {
        #[doc = "A list of numeric IDs to be excluded."]
        #[serde(rename = "excludedCriteriaIds", default)]
        pub excluded_criteria_ids: Option<Vec<i64>>,
        #[doc = "A list of numeric IDs to be included."]
        #[serde(rename = "targetedCriteriaIds", default)]
        pub targeted_criteria_ids: Option<Vec<i64>>,
    }
    impl ::field_selector::FieldSelector for CriteriaTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Date {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month, or 0\nif specifying a year by itself or a year and month where the day is not\nsignificant."]
        #[serde(rename = "day", default)]
        pub day: Option<i32>,
        #[doc = "Month of year. Must be from 1 to 12, or 0 if specifying a year without a\nmonth and day."]
        #[serde(rename = "month", default)]
        pub month: Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999, or 0 if specifying a date without\na year."]
        #[serde(rename = "year", default)]
        pub year: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Date {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DayPartDayOfWeek {
        #[doc = "A placeholder for when the day of the week is not specified."]
        DayOfWeekUnspecified,
        #[doc = "Monday"]
        Monday,
        #[doc = "Tuesday"]
        Tuesday,
        #[doc = "Wednesday"]
        Wednesday,
        #[doc = "Thursday"]
        Thursday,
        #[doc = "Friday"]
        Friday,
        #[doc = "Saturday"]
        Saturday,
        #[doc = "Sunday"]
        Sunday,
    }
    impl DayPartDayOfWeek {
        pub fn as_str(self) -> &'static str {
            match self {
                DayPartDayOfWeek::DayOfWeekUnspecified => "DAY_OF_WEEK_UNSPECIFIED",
                DayPartDayOfWeek::Monday => "MONDAY",
                DayPartDayOfWeek::Tuesday => "TUESDAY",
                DayPartDayOfWeek::Wednesday => "WEDNESDAY",
                DayPartDayOfWeek::Thursday => "THURSDAY",
                DayPartDayOfWeek::Friday => "FRIDAY",
                DayPartDayOfWeek::Saturday => "SATURDAY",
                DayPartDayOfWeek::Sunday => "SUNDAY",
            }
        }
    }
    impl ::std::fmt::Display for DayPartDayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DayPartDayOfWeek {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DayPartDayOfWeek {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY_OF_WEEK_UNSPECIFIED" => DayPartDayOfWeek::DayOfWeekUnspecified,
                "MONDAY" => DayPartDayOfWeek::Monday,
                "TUESDAY" => DayPartDayOfWeek::Tuesday,
                "WEDNESDAY" => DayPartDayOfWeek::Wednesday,
                "THURSDAY" => DayPartDayOfWeek::Thursday,
                "FRIDAY" => DayPartDayOfWeek::Friday,
                "SATURDAY" => DayPartDayOfWeek::Saturday,
                "SUNDAY" => DayPartDayOfWeek::Sunday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DayPart {
        #[doc = "The day of the week to target. If unspecified, applicable to all days."]
        #[serde(rename = "dayOfWeek", default)]
        pub day_of_week: Option<crate::schemas::DayPartDayOfWeek>,
        #[doc = "The ending time of the day for the ad to show (minute level\ngranularity). The end time is exclusive. This field is not available\nfor filtering in PQL queries."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<crate::schemas::TimeOfDay>,
        #[doc = "The starting time of day for the ad to show (minute level granularity).\nThe start time is inclusive.\nThis field is not available for filtering in PQL queries."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<crate::schemas::TimeOfDay>,
    }
    impl ::field_selector::FieldSelector for DayPart {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DayPartTargetingTimeZoneType {
        #[doc = "A placeholder for an undefined time zone source."]
        TimeZoneSourceUnspecified,
        #[doc = "Use publisher's time zone setting."]
        Publisher,
        #[doc = "Use the user's time zone setting."]
        User,
    }
    impl DayPartTargetingTimeZoneType {
        pub fn as_str(self) -> &'static str {
            match self {
                DayPartTargetingTimeZoneType::TimeZoneSourceUnspecified => {
                    "TIME_ZONE_SOURCE_UNSPECIFIED"
                }
                DayPartTargetingTimeZoneType::Publisher => "PUBLISHER",
                DayPartTargetingTimeZoneType::User => "USER",
            }
        }
    }
    impl ::std::fmt::Display for DayPartTargetingTimeZoneType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DayPartTargetingTimeZoneType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DayPartTargetingTimeZoneType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TIME_ZONE_SOURCE_UNSPECIFIED" => {
                    DayPartTargetingTimeZoneType::TimeZoneSourceUnspecified
                }
                "PUBLISHER" => DayPartTargetingTimeZoneType::Publisher,
                "USER" => DayPartTargetingTimeZoneType::User,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DayPartTargeting {
        #[doc = "A list of day part targeting criterion."]
        #[serde(rename = "dayParts", default)]
        pub day_parts: Option<Vec<crate::schemas::DayPart>>,
        #[doc = "The timezone to use for interpreting the day part targeting."]
        #[serde(rename = "timeZoneType", default)]
        pub time_zone_type: Option<crate::schemas::DayPartTargetingTimeZoneType>,
    }
    impl ::field_selector::FieldSelector for DayPartTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DealCreativePreApprovalPolicy {
        #[doc = "A placeholder for an undefined creative pre-approval policy."]
        CreativePreApprovalPolicyUnspecified,
        #[doc = "The seller needs to approve each creative before it can serve."]
        SellerPreApprovalRequired,
        #[doc = "The seller does not need to approve each creative before it can serve."]
        SellerPreApprovalNotRequired,
    }
    impl DealCreativePreApprovalPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                DealCreativePreApprovalPolicy::CreativePreApprovalPolicyUnspecified => {
                    "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED"
                }
                DealCreativePreApprovalPolicy::SellerPreApprovalRequired => {
                    "SELLER_PRE_APPROVAL_REQUIRED"
                }
                DealCreativePreApprovalPolicy::SellerPreApprovalNotRequired => {
                    "SELLER_PRE_APPROVAL_NOT_REQUIRED"
                }
            }
        }
    }
    impl ::std::fmt::Display for DealCreativePreApprovalPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DealCreativePreApprovalPolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DealCreativePreApprovalPolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED" => {
                    DealCreativePreApprovalPolicy::CreativePreApprovalPolicyUnspecified
                }
                "SELLER_PRE_APPROVAL_REQUIRED" => {
                    DealCreativePreApprovalPolicy::SellerPreApprovalRequired
                }
                "SELLER_PRE_APPROVAL_NOT_REQUIRED" => {
                    DealCreativePreApprovalPolicy::SellerPreApprovalNotRequired
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DealCreativeSafeFrameCompatibility {
        #[doc = "A placeholder for an undefined creative safe-frame compatibility."]
        CreativeSafeFrameCompatibilityUnspecified,
        #[doc = "The creatives need to be compatible with the safe frame option."]
        Compatible,
        #[doc = "The creatives can be incompatible with the safe frame option."]
        Incompatible,
    }
    impl DealCreativeSafeFrameCompatibility {
        pub fn as_str(self) -> &'static str {
            match self {
                DealCreativeSafeFrameCompatibility::CreativeSafeFrameCompatibilityUnspecified => {
                    "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED"
                }
                DealCreativeSafeFrameCompatibility::Compatible => "COMPATIBLE",
                DealCreativeSafeFrameCompatibility::Incompatible => "INCOMPATIBLE",
            }
        }
    }
    impl ::std::fmt::Display for DealCreativeSafeFrameCompatibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DealCreativeSafeFrameCompatibility {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DealCreativeSafeFrameCompatibility {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED" => {
                    DealCreativeSafeFrameCompatibility::CreativeSafeFrameCompatibilityUnspecified
                }
                "COMPATIBLE" => DealCreativeSafeFrameCompatibility::Compatible,
                "INCOMPATIBLE" => DealCreativeSafeFrameCompatibility::Incompatible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DealProgrammaticCreativeSource {
        #[doc = "A placeholder for an undefined programmatic creative source."]
        ProgrammaticCreativeSourceUnspecified,
        #[doc = "The advertiser provides the creatives."]
        Advertiser,
        #[doc = "The publisher provides the creatives to be served."]
        Publisher,
    }
    impl DealProgrammaticCreativeSource {
        pub fn as_str(self) -> &'static str {
            match self {
                DealProgrammaticCreativeSource::ProgrammaticCreativeSourceUnspecified => {
                    "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED"
                }
                DealProgrammaticCreativeSource::Advertiser => "ADVERTISER",
                DealProgrammaticCreativeSource::Publisher => "PUBLISHER",
            }
        }
    }
    impl ::std::fmt::Display for DealProgrammaticCreativeSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DealProgrammaticCreativeSource {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DealProgrammaticCreativeSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED" => {
                    DealProgrammaticCreativeSource::ProgrammaticCreativeSourceUnspecified
                }
                "ADVERTISER" => DealProgrammaticCreativeSource::Advertiser,
                "PUBLISHER" => DealProgrammaticCreativeSource::Publisher,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DealSyndicationProduct {
        #[doc = "A placeholder for an undefined syndication product."]
        SyndicationProductUnspecified,
        #[doc = "This typically represents a web page."]
        Content,
        #[doc = "This represents a mobile property."]
        Mobile,
        #[doc = "This represents video ad formats."]
        Video,
        #[doc = "This represents ads shown within games."]
        Games,
    }
    impl DealSyndicationProduct {
        pub fn as_str(self) -> &'static str {
            match self {
                DealSyndicationProduct::SyndicationProductUnspecified => {
                    "SYNDICATION_PRODUCT_UNSPECIFIED"
                }
                DealSyndicationProduct::Content => "CONTENT",
                DealSyndicationProduct::Mobile => "MOBILE",
                DealSyndicationProduct::Video => "VIDEO",
                DealSyndicationProduct::Games => "GAMES",
            }
        }
    }
    impl ::std::fmt::Display for DealSyndicationProduct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DealSyndicationProduct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DealSyndicationProduct {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNDICATION_PRODUCT_UNSPECIFIED" => {
                    DealSyndicationProduct::SyndicationProductUnspecified
                }
                "CONTENT" => DealSyndicationProduct::Content,
                "MOBILE" => DealSyndicationProduct::Mobile,
                "VIDEO" => DealSyndicationProduct::Video,
                "GAMES" => DealSyndicationProduct::Games,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Deal {
        #[doc = "Proposed flight end time of the deal.\nThis will generally be stored in a granularity of a second.\nA value is not required for Private Auction deals or Preferred Deals."]
        #[serde(rename = "availableEndTime", default)]
        pub available_end_time: Option<String>,
        #[doc = "Optional proposed flight start time of the deal.\nThis will generally be stored in the granularity of one second since deal\nserving starts at seconds boundary. Any time specified with more\ngranularity (e.g., in milliseconds) will be truncated towards the start of\ntime in seconds."]
        #[serde(rename = "availableStartTime", default)]
        pub available_start_time: Option<String>,
        #[doc = "Buyer private data (hidden from seller)."]
        #[serde(rename = "buyerPrivateData", default)]
        pub buyer_private_data: Option<crate::schemas::PrivateData>,
        #[doc = "The product ID from which this deal was created.\n\nNote: This field may be set only when creating the resource. Modifying\nthis field while updating the resource will result in an error."]
        #[serde(rename = "createProductId", default)]
        pub create_product_id: Option<String>,
        #[doc = "Optional revision number of the product that the deal was created from.\nIf present on create, and the server `product_revision` has advanced sinced\nthe passed-in `create_product_revision`, an `ABORTED` error will be\nreturned.\n\nNote: This field may be set only when creating the resource. Modifying\nthis field while updating the resource will result in an error."]
        #[serde(rename = "createProductRevision", default)]
        #[serde(with = "crate::parsed_string")]
        pub create_product_revision: Option<i64>,
        #[doc = "Output only. The time of the deal creation."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Output only. Specifies the creative pre-approval policy."]
        #[serde(rename = "creativePreApprovalPolicy", default)]
        pub creative_pre_approval_policy: Option<crate::schemas::DealCreativePreApprovalPolicy>,
        #[doc = "Output only. Restricitions about the creatives associated with the deal\n(i.e., size) This is available for Programmatic Guaranteed/Preferred Deals\nin Ad Manager."]
        #[serde(rename = "creativeRestrictions", default)]
        pub creative_restrictions: Option<crate::schemas::CreativeRestrictions>,
        #[doc = "Output only. Specifies whether the creative is safeFrame compatible."]
        #[serde(rename = "creativeSafeFrameCompatibility", default)]
        pub creative_safe_frame_compatibility:
            Option<crate::schemas::DealCreativeSafeFrameCompatibility>,
        #[doc = "Output only. A unique deal ID for the deal (server-assigned)."]
        #[serde(rename = "dealId", default)]
        pub deal_id: Option<String>,
        #[doc = "Output only. Metadata about the serving status of this deal."]
        #[serde(rename = "dealServingMetadata", default)]
        pub deal_serving_metadata: Option<crate::schemas::DealServingMetadata>,
        #[doc = "The negotiable terms of the deal."]
        #[serde(rename = "dealTerms", default)]
        pub deal_terms: Option<crate::schemas::DealTerms>,
        #[doc = "The set of fields around delivery control that are interesting for a buyer\nto see but are non-negotiable. These are set by the publisher."]
        #[serde(rename = "deliveryControl", default)]
        pub delivery_control: Option<crate::schemas::DeliveryControl>,
        #[doc = "Description for the deal terms."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The name of the deal."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "Output only. The external deal ID assigned to this deal once the deal is\nfinalized. This is the deal ID that shows up in serving/reporting etc."]
        #[serde(rename = "externalDealId", default)]
        pub external_deal_id: Option<String>,
        #[doc = "Output only. True, if the buyside inventory setup is complete for this\ndeal."]
        #[serde(rename = "isSetupComplete", default)]
        pub is_setup_complete: Option<bool>,
        #[doc = "Output only. Specifies the creative source for programmatic deals.\nPUBLISHER means creative is provided by seller and ADVERTISER means\ncreative is provided by buyer."]
        #[serde(rename = "programmaticCreativeSource", default)]
        pub programmatic_creative_source: Option<crate::schemas::DealProgrammaticCreativeSource>,
        #[doc = "Output only. ID of the proposal that this deal is part of."]
        #[serde(rename = "proposalId", default)]
        pub proposal_id: Option<String>,
        #[doc = "Output only. Seller contact information for the deal."]
        #[serde(rename = "sellerContacts", default)]
        pub seller_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "The syndication product associated with the deal.\n\nNote: This field may be set only when creating the resource. Modifying\nthis field while updating the resource will result in an error."]
        #[serde(rename = "syndicationProduct", default)]
        pub syndication_product: Option<crate::schemas::DealSyndicationProduct>,
        #[doc = "Output only. Specifies the subset of inventory targeted by the deal."]
        #[serde(rename = "targeting", default)]
        pub targeting: Option<crate::schemas::MarketplaceTargeting>,
        #[doc = "The shared targeting visible to buyers and sellers. Each shared\ntargeting entity is AND'd together."]
        #[serde(rename = "targetingCriterion", default)]
        pub targeting_criterion: Option<Vec<crate::schemas::TargetingCriteria>>,
        #[doc = "Output only. The time when the deal was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
        #[doc = "The web property code for the seller copied over from the product."]
        #[serde(rename = "webPropertyCode", default)]
        pub web_property_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for Deal {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DealPauseStatusFirstPausedBy {
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl DealPauseStatusFirstPausedBy {
        pub fn as_str(self) -> &'static str {
            match self {
                DealPauseStatusFirstPausedBy::BuyerSellerRoleUnspecified => {
                    "BUYER_SELLER_ROLE_UNSPECIFIED"
                }
                DealPauseStatusFirstPausedBy::Buyer => "BUYER",
                DealPauseStatusFirstPausedBy::Seller => "SELLER",
            }
        }
    }
    impl ::std::fmt::Display for DealPauseStatusFirstPausedBy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DealPauseStatusFirstPausedBy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DealPauseStatusFirstPausedBy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUYER_SELLER_ROLE_UNSPECIFIED" => {
                    DealPauseStatusFirstPausedBy::BuyerSellerRoleUnspecified
                }
                "BUYER" => DealPauseStatusFirstPausedBy::Buyer,
                "SELLER" => DealPauseStatusFirstPausedBy::Seller,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DealPauseStatus {
        #[doc = "The buyer's reason for pausing, if the buyer paused the deal."]
        #[serde(rename = "buyerPauseReason", default)]
        pub buyer_pause_reason: Option<String>,
        #[doc = "The role of the person who first paused this deal."]
        #[serde(rename = "firstPausedBy", default)]
        pub first_paused_by: Option<crate::schemas::DealPauseStatusFirstPausedBy>,
        #[doc = "True, if the buyer has paused the deal unilaterally."]
        #[serde(rename = "hasBuyerPaused", default)]
        pub has_buyer_paused: Option<bool>,
        #[doc = "True, if the seller has paused the deal unilaterally."]
        #[serde(rename = "hasSellerPaused", default)]
        pub has_seller_paused: Option<bool>,
        #[doc = "The seller's reason for pausing, if the seller paused the deal."]
        #[serde(rename = "sellerPauseReason", default)]
        pub seller_pause_reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for DealPauseStatus {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DealServingMetadata {
        #[doc = "Output only. Tracks which parties (if any) have paused a deal."]
        #[serde(rename = "dealPauseStatus", default)]
        pub deal_pause_status: Option<crate::schemas::DealPauseStatus>,
    }
    impl ::field_selector::FieldSelector for DealServingMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DealTermsBrandingType {
        #[doc = "A placeholder for an undefined branding type."]
        BrandingTypeUnspecified,
        #[doc = "Full URL is included in bid requests."]
        Branded,
        #[doc = "A TopLevelDomain or masked URL is sent in bid requests\nrather than the full one."]
        SemiTransparent,
    }
    impl DealTermsBrandingType {
        pub fn as_str(self) -> &'static str {
            match self {
                DealTermsBrandingType::BrandingTypeUnspecified => "BRANDING_TYPE_UNSPECIFIED",
                DealTermsBrandingType::Branded => "BRANDED",
                DealTermsBrandingType::SemiTransparent => "SEMI_TRANSPARENT",
            }
        }
    }
    impl ::std::fmt::Display for DealTermsBrandingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DealTermsBrandingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DealTermsBrandingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BRANDING_TYPE_UNSPECIFIED" => DealTermsBrandingType::BrandingTypeUnspecified,
                "BRANDED" => DealTermsBrandingType::Branded,
                "SEMI_TRANSPARENT" => DealTermsBrandingType::SemiTransparent,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DealTerms {
        #[doc = "Visibility of the URL in bid requests. (default: BRANDED)"]
        #[serde(rename = "brandingType", default)]
        pub branding_type: Option<crate::schemas::DealTermsBrandingType>,
        #[doc = "Publisher provided description for the terms."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Non-binding estimate of the estimated gross spend for this deal.\nCan be set by buyer or seller."]
        #[serde(rename = "estimatedGrossSpend", default)]
        pub estimated_gross_spend: Option<crate::schemas::Price>,
        #[doc = "Non-binding estimate of the impressions served per day.\nCan be set by buyer or seller."]
        #[serde(rename = "estimatedImpressionsPerDay", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_impressions_per_day: Option<i64>,
        #[doc = "The terms for guaranteed fixed price deals."]
        #[serde(rename = "guaranteedFixedPriceTerms", default)]
        pub guaranteed_fixed_price_terms: Option<crate::schemas::GuaranteedFixedPriceTerms>,
        #[doc = "The terms for non-guaranteed auction deals."]
        #[serde(rename = "nonGuaranteedAuctionTerms", default)]
        pub non_guaranteed_auction_terms: Option<crate::schemas::NonGuaranteedAuctionTerms>,
        #[doc = "The terms for non-guaranteed fixed price deals."]
        #[serde(rename = "nonGuaranteedFixedPriceTerms", default)]
        pub non_guaranteed_fixed_price_terms: Option<crate::schemas::NonGuaranteedFixedPriceTerms>,
        #[doc = "The time zone name. For deals with Cost Per Day billing, defines the\ntime zone used to mark the boundaries of a day. It should be an\nIANA TZ name, such as \"America/Los_Angeles\". For more information,\nsee https://en.wikipedia.org/wiki/List_of_tz_database_time_zones."]
        #[serde(rename = "sellerTimeZone", default)]
        pub seller_time_zone: Option<String>,
    }
    impl ::field_selector::FieldSelector for DealTerms {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeliveryControlCreativeBlockingLevel {
        #[doc = "A placeholder for an undefined creative blocking level."]
        CreativeBlockingLevelUnspecified,
        #[doc = "Publisher blocking rules will be applied."]
        PublisherBlockingRules,
        #[doc = "The Ad Exchange policy blocking rules will be applied."]
        AdxPolicyBlockingOnly,
    }
    impl DeliveryControlCreativeBlockingLevel {
        pub fn as_str(self) -> &'static str {
            match self {
                DeliveryControlCreativeBlockingLevel::CreativeBlockingLevelUnspecified => {
                    "CREATIVE_BLOCKING_LEVEL_UNSPECIFIED"
                }
                DeliveryControlCreativeBlockingLevel::PublisherBlockingRules => {
                    "PUBLISHER_BLOCKING_RULES"
                }
                DeliveryControlCreativeBlockingLevel::AdxPolicyBlockingOnly => {
                    "ADX_POLICY_BLOCKING_ONLY"
                }
            }
        }
    }
    impl ::std::fmt::Display for DeliveryControlCreativeBlockingLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeliveryControlCreativeBlockingLevel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeliveryControlCreativeBlockingLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_BLOCKING_LEVEL_UNSPECIFIED" => {
                    DeliveryControlCreativeBlockingLevel::CreativeBlockingLevelUnspecified
                }
                "PUBLISHER_BLOCKING_RULES" => {
                    DeliveryControlCreativeBlockingLevel::PublisherBlockingRules
                }
                "ADX_POLICY_BLOCKING_ONLY" => {
                    DeliveryControlCreativeBlockingLevel::AdxPolicyBlockingOnly
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeliveryControlDeliveryRateType {
        #[doc = "A placeholder for an undefined delivery rate type."]
        DeliveryRateTypeUnspecified,
        #[doc = "Impressions are served uniformly over the life of the deal."]
        Evenly,
        #[doc = "Impressions are served front-loaded."]
        FrontLoaded,
        #[doc = "Impressions are served as fast as possible."]
        AsFastAsPossible,
    }
    impl DeliveryControlDeliveryRateType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeliveryControlDeliveryRateType::DeliveryRateTypeUnspecified => {
                    "DELIVERY_RATE_TYPE_UNSPECIFIED"
                }
                DeliveryControlDeliveryRateType::Evenly => "EVENLY",
                DeliveryControlDeliveryRateType::FrontLoaded => "FRONT_LOADED",
                DeliveryControlDeliveryRateType::AsFastAsPossible => "AS_FAST_AS_POSSIBLE",
            }
        }
    }
    impl ::std::fmt::Display for DeliveryControlDeliveryRateType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeliveryControlDeliveryRateType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeliveryControlDeliveryRateType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DELIVERY_RATE_TYPE_UNSPECIFIED" => {
                    DeliveryControlDeliveryRateType::DeliveryRateTypeUnspecified
                }
                "EVENLY" => DeliveryControlDeliveryRateType::Evenly,
                "FRONT_LOADED" => DeliveryControlDeliveryRateType::FrontLoaded,
                "AS_FAST_AS_POSSIBLE" => DeliveryControlDeliveryRateType::AsFastAsPossible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeliveryControl {
        #[doc = "Output only. Specified the creative blocking levels to be applied."]
        #[serde(rename = "creativeBlockingLevel", default)]
        pub creative_blocking_level: Option<crate::schemas::DeliveryControlCreativeBlockingLevel>,
        #[doc = "Output only. Specifies how the impression delivery will be paced."]
        #[serde(rename = "deliveryRateType", default)]
        pub delivery_rate_type: Option<crate::schemas::DeliveryControlDeliveryRateType>,
        #[doc = "Output only. Specifies any frequency caps."]
        #[serde(rename = "frequencyCaps", default)]
        pub frequency_caps: Option<Vec<crate::schemas::FrequencyCap>>,
    }
    impl ::field_selector::FieldSelector for DeliveryControl {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DisapprovalReason {
        #[doc = "The length of the image animation is longer than allowed."]
        LengthOfImageAnimation,
        #[doc = "The click through URL doesn't work properly."]
        BrokenUrl,
        #[doc = "Something is wrong with the creative itself."]
        MediaNotFunctional,
        #[doc = "The ad makes a fourth party call to an unapproved vendor."]
        InvalidFourthPartyCall,
        #[doc = "The ad targets consumers using remarketing lists and/or collects\ndata for subsequent use in retargeting, but does not correctly declare\nthat use."]
        IncorrectRemarketingDeclaration,
        #[doc = "Clicking on the ad leads to an error page."]
        LandingPageError,
        #[doc = "The ad size when rendered does not match the declaration."]
        AdSizeDoesNotMatchAdSlot,
        #[doc = "Ads with a white background require a border, which was missing."]
        NoBorder,
        #[doc = "The creative attempts to set cookies from a fourth party that is not\ncertified."]
        FourthPartyBrowserCookies,
        #[doc = "The creative sets an LSO object."]
        LsoObjects,
        #[doc = "The ad serves a blank."]
        BlankCreative,
        #[doc = "The ad uses rotation, but not all destination URLs were declared."]
        DestinationUrlsUndeclared,
        #[doc = "There is a problem with the way the click macro is used."]
        ProblemWithClickMacro,
        #[doc = "The ad technology declaration is not accurate."]
        IncorrectAdTechnologyDeclaration,
        #[doc = "The actual destination URL does not match the declared destination URL."]
        IncorrectDestinationUrlDeclaration,
        #[doc = "The declared expanding direction does not match the actual direction."]
        ExpandableIncorrectDirection,
        #[doc = "The ad does not expand in a supported direction."]
        ExpandableDirectionNotSupported,
        #[doc = "The ad uses an expandable vendor that is not supported."]
        ExpandableInvalidVendor,
        #[doc = "There was an issue with the expandable ad."]
        ExpandableFunctionality,
        #[doc = "The ad uses a video vendor that is not supported."]
        VideoInvalidVendor,
        #[doc = "The length of the video ad is not supported."]
        VideoUnsupportedLength,
        #[doc = "The format of the video ad is not supported."]
        VideoUnsupportedFormat,
        #[doc = "There was an issue with the video ad."]
        VideoFunctionality,
        #[doc = "The landing page does not conform to Ad Exchange policy."]
        LandingPageDisabled,
        #[doc = "The ad or the landing page may contain malware."]
        MalwareSuspected,
        #[doc = "The ad contains adult images or video content."]
        AdultImageOrVideo,
        #[doc = "The ad contains text that is unclear or inaccurate."]
        InaccurateAdText,
        #[doc = "The ad promotes counterfeit designer goods."]
        CounterfeitDesignerGoods,
        #[doc = "The ad causes a popup window to appear."]
        PopUp,
        #[doc = "The creative does not follow policies set for the RTB protocol."]
        InvalidRtbProtocolUsage,
        #[doc = "The ad contains a URL that uses a numeric IP address for the domain."]
        RawIpAddressInSnippet,
        #[doc = "The ad or landing page contains unacceptable content because it initiated\na software or executable download."]
        UnacceptableContentSoftware,
        #[doc = "The ad set an unauthorized cookie on a Google domain."]
        UnauthorizedCookieOnGoogleDomain,
        #[doc = "Flash content found when no flash was declared."]
        UndeclaredFlashObjects,
        #[doc = "SSL support declared but not working correctly."]
        InvalidSslDeclaration,
        #[doc = "Rich Media - Direct Download in Ad (ex. PDF download)."]
        DirectDownloadInAd,
        #[doc = "Maximum download size exceeded."]
        MaximumDownloadSizeExceeded,
        #[doc = "Bad Destination URL: Site Not Crawlable."]
        DestinationUrlSiteNotCrawlable,
        #[doc = "Bad URL: Legal disapproval."]
        BadUrlLegalDisapproval,
        #[doc = "Pharmaceuticals, Gambling, Alcohol not allowed and at least one was\ndetected."]
        PharmaGamblingAlcoholNotAllowed,
        #[doc = "Dynamic DNS at Destination URL."]
        DynamicDnsAtDestinationUrl,
        #[doc = "Poor Image / Video Quality."]
        PoorImageOrVideoQuality,
        #[doc = "For example, Image Trick to Click."]
        UnacceptableImageContent,
        #[doc = "Incorrect Image Layout."]
        IncorrectImageLayout,
        #[doc = "Irrelevant Image / Video."]
        IrrelevantImageOrVideo,
        #[doc = "Broken back button."]
        DestinationSiteDoesNotAllowGoingBack,
        #[doc = "Misleading/Inaccurate claims in ads."]
        MisleadingClaimsInAd,
        #[doc = "Restricted Products."]
        RestrictedProducts,
        #[doc = "Unacceptable content. For example, malware."]
        UnacceptableContent,
        #[doc = "The ad automatically redirects to the destination site without a click,\nor reports a click when none were made."]
        AutomatedAdClicking,
        #[doc = "The ad uses URL protocols that do not exist or are not allowed on AdX."]
        InvalidUrlProtocol,
        #[doc = "Restricted content (for example, alcohol) was found in the ad but not\ndeclared."]
        UndeclaredRestrictedContent,
        #[doc = "Violation of the remarketing list policy."]
        InvalidRemarketingListUsage,
        #[doc = "The destination site's robot.txt file prevents it from being crawled."]
        DestinationSiteNotCrawlableRobotsTxt,
        #[doc = "Click to download must link to an app."]
        ClickToDownloadNotAnApp,
        #[doc = "A review extension must be an accurate review."]
        InaccurateReviewExtension,
        #[doc = "Sexually explicit content."]
        SexuallyExplicitContent,
        #[doc = "The ad tries to gain an unfair traffic advantage."]
        GainingAnUnfairAdvantage,
        #[doc = "The ad tries to circumvent Google's advertising systems."]
        GamingTheGoogleNetwork,
        #[doc = "The ad promotes dangerous knives."]
        DangerousProductsKnives,
        #[doc = "The ad promotes explosives."]
        DangerousProductsExplosives,
        #[doc = "The ad promotes guns & parts."]
        DangerousProductsGuns,
        #[doc = "The ad promotes recreational drugs/services & related equipment."]
        DangerousProductsDrugs,
        #[doc = "The ad promotes tobacco products/services & related equipment."]
        DangerousProductsTobacco,
        #[doc = "The ad promotes weapons."]
        DangerousProductsWeapons,
        #[doc = "The ad is unclear or irrelevant to the destination site."]
        UnclearOrIrrelevantAd,
        #[doc = "The ad does not meet professional standards."]
        ProfessionalStandards,
        #[doc = "The promotion is unnecessarily difficult to navigate."]
        DysfunctionalPromotion,
        #[doc = "Violation of Google's policy for interest-based ads."]
        InvalidInterestBasedAd,
        #[doc = "Misuse of personal information."]
        MisuseOfPersonalInformation,
        #[doc = "Omission of relevant information."]
        OmissionOfRelevantInformation,
        #[doc = "Unavailable promotions."]
        UnavailablePromotions,
        #[doc = "Misleading or unrealistic promotions."]
        MisleadingPromotions,
        #[doc = "Offensive or inappropriate content."]
        InappropriateContent,
        #[doc = "Capitalizing on sensitive events."]
        SensitiveEvents,
        #[doc = "Shocking content."]
        ShockingContent,
        #[doc = "Products & Services that enable dishonest behavior."]
        EnablingDishonestBehavior,
        #[doc = "The ad does not meet technical requirements."]
        TechnicalRequirements,
        #[doc = "Restricted political content."]
        RestrictedPoliticalContent,
        #[doc = "Unsupported content."]
        UnsupportedContent,
        #[doc = "Invalid bidding method."]
        InvalidBiddingMethod,
        #[doc = "Video length exceeds limits."]
        VideoTooLong,
        #[doc = "Unacceptable content: Japanese healthcare."]
        ViolatesJapanesePharmacyLaw,
        #[doc = "Online pharmacy ID required."]
        UnaccreditedPetPharmacy,
        #[doc = "Unacceptable content: Abortion."]
        Abortion,
        #[doc = "Unacceptable content: Birth control."]
        Contraceptives,
        #[doc = "Restricted in China."]
        NeedCertificatesToAdvertiseInChina,
        #[doc = "Unacceptable content: Korean healthcare."]
        KcdspRegistration,
        #[doc = "Non-family safe or adult content."]
        NotFamilySafe,
        #[doc = "Clinical trial recruitment."]
        ClinicalTrialRecruitment,
        #[doc = "Maximum number of HTTP calls exceeded."]
        MaximumNumberOfHttpCallsExceeded,
        #[doc = "Maximum number of cookies exceeded."]
        MaximumNumberOfCookiesExceeded,
        #[doc = "Financial service ad does not adhere to specifications."]
        PersonalLoans,
        #[doc = "Flash content was found in an unsupported context."]
        UnsupportedFlashContent,
        #[doc = "Misuse by an Open Measurement SDK script."]
        MisuseByOmidScript,
        #[doc = "Use of an Open Measurement SDK vendor not on approved whitelist."]
        NonWhitelistedOmidVendor,
        #[doc = "Unacceptable landing page."]
        DestinationExperience,
        #[doc = "Unsupported language."]
        UnsupportedLanguage,
        #[doc = "Non-SSL compliant."]
        NonSslCompliant,
        #[doc = "Temporary pausing of creative."]
        TemporaryPause,
        #[doc = "Promotes services related to bail bonds."]
        BailBonds,
    }
    impl DisapprovalReason {
        pub fn as_str(self) -> &'static str {
            match self {
                DisapprovalReason::LengthOfImageAnimation => "LENGTH_OF_IMAGE_ANIMATION",
                DisapprovalReason::BrokenUrl => "BROKEN_URL",
                DisapprovalReason::MediaNotFunctional => "MEDIA_NOT_FUNCTIONAL",
                DisapprovalReason::InvalidFourthPartyCall => "INVALID_FOURTH_PARTY_CALL",
                DisapprovalReason::IncorrectRemarketingDeclaration => {
                    "INCORRECT_REMARKETING_DECLARATION"
                }
                DisapprovalReason::LandingPageError => "LANDING_PAGE_ERROR",
                DisapprovalReason::AdSizeDoesNotMatchAdSlot => "AD_SIZE_DOES_NOT_MATCH_AD_SLOT",
                DisapprovalReason::NoBorder => "NO_BORDER",
                DisapprovalReason::FourthPartyBrowserCookies => "FOURTH_PARTY_BROWSER_COOKIES",
                DisapprovalReason::LsoObjects => "LSO_OBJECTS",
                DisapprovalReason::BlankCreative => "BLANK_CREATIVE",
                DisapprovalReason::DestinationUrlsUndeclared => "DESTINATION_URLS_UNDECLARED",
                DisapprovalReason::ProblemWithClickMacro => "PROBLEM_WITH_CLICK_MACRO",
                DisapprovalReason::IncorrectAdTechnologyDeclaration => {
                    "INCORRECT_AD_TECHNOLOGY_DECLARATION"
                }
                DisapprovalReason::IncorrectDestinationUrlDeclaration => {
                    "INCORRECT_DESTINATION_URL_DECLARATION"
                }
                DisapprovalReason::ExpandableIncorrectDirection => "EXPANDABLE_INCORRECT_DIRECTION",
                DisapprovalReason::ExpandableDirectionNotSupported => {
                    "EXPANDABLE_DIRECTION_NOT_SUPPORTED"
                }
                DisapprovalReason::ExpandableInvalidVendor => "EXPANDABLE_INVALID_VENDOR",
                DisapprovalReason::ExpandableFunctionality => "EXPANDABLE_FUNCTIONALITY",
                DisapprovalReason::VideoInvalidVendor => "VIDEO_INVALID_VENDOR",
                DisapprovalReason::VideoUnsupportedLength => "VIDEO_UNSUPPORTED_LENGTH",
                DisapprovalReason::VideoUnsupportedFormat => "VIDEO_UNSUPPORTED_FORMAT",
                DisapprovalReason::VideoFunctionality => "VIDEO_FUNCTIONALITY",
                DisapprovalReason::LandingPageDisabled => "LANDING_PAGE_DISABLED",
                DisapprovalReason::MalwareSuspected => "MALWARE_SUSPECTED",
                DisapprovalReason::AdultImageOrVideo => "ADULT_IMAGE_OR_VIDEO",
                DisapprovalReason::InaccurateAdText => "INACCURATE_AD_TEXT",
                DisapprovalReason::CounterfeitDesignerGoods => "COUNTERFEIT_DESIGNER_GOODS",
                DisapprovalReason::PopUp => "POP_UP",
                DisapprovalReason::InvalidRtbProtocolUsage => "INVALID_RTB_PROTOCOL_USAGE",
                DisapprovalReason::RawIpAddressInSnippet => "RAW_IP_ADDRESS_IN_SNIPPET",
                DisapprovalReason::UnacceptableContentSoftware => "UNACCEPTABLE_CONTENT_SOFTWARE",
                DisapprovalReason::UnauthorizedCookieOnGoogleDomain => {
                    "UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN"
                }
                DisapprovalReason::UndeclaredFlashObjects => "UNDECLARED_FLASH_OBJECTS",
                DisapprovalReason::InvalidSslDeclaration => "INVALID_SSL_DECLARATION",
                DisapprovalReason::DirectDownloadInAd => "DIRECT_DOWNLOAD_IN_AD",
                DisapprovalReason::MaximumDownloadSizeExceeded => "MAXIMUM_DOWNLOAD_SIZE_EXCEEDED",
                DisapprovalReason::DestinationUrlSiteNotCrawlable => {
                    "DESTINATION_URL_SITE_NOT_CRAWLABLE"
                }
                DisapprovalReason::BadUrlLegalDisapproval => "BAD_URL_LEGAL_DISAPPROVAL",
                DisapprovalReason::PharmaGamblingAlcoholNotAllowed => {
                    "PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED"
                }
                DisapprovalReason::DynamicDnsAtDestinationUrl => "DYNAMIC_DNS_AT_DESTINATION_URL",
                DisapprovalReason::PoorImageOrVideoQuality => "POOR_IMAGE_OR_VIDEO_QUALITY",
                DisapprovalReason::UnacceptableImageContent => "UNACCEPTABLE_IMAGE_CONTENT",
                DisapprovalReason::IncorrectImageLayout => "INCORRECT_IMAGE_LAYOUT",
                DisapprovalReason::IrrelevantImageOrVideo => "IRRELEVANT_IMAGE_OR_VIDEO",
                DisapprovalReason::DestinationSiteDoesNotAllowGoingBack => {
                    "DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK"
                }
                DisapprovalReason::MisleadingClaimsInAd => "MISLEADING_CLAIMS_IN_AD",
                DisapprovalReason::RestrictedProducts => "RESTRICTED_PRODUCTS",
                DisapprovalReason::UnacceptableContent => "UNACCEPTABLE_CONTENT",
                DisapprovalReason::AutomatedAdClicking => "AUTOMATED_AD_CLICKING",
                DisapprovalReason::InvalidUrlProtocol => "INVALID_URL_PROTOCOL",
                DisapprovalReason::UndeclaredRestrictedContent => "UNDECLARED_RESTRICTED_CONTENT",
                DisapprovalReason::InvalidRemarketingListUsage => "INVALID_REMARKETING_LIST_USAGE",
                DisapprovalReason::DestinationSiteNotCrawlableRobotsTxt => {
                    "DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT"
                }
                DisapprovalReason::ClickToDownloadNotAnApp => "CLICK_TO_DOWNLOAD_NOT_AN_APP",
                DisapprovalReason::InaccurateReviewExtension => "INACCURATE_REVIEW_EXTENSION",
                DisapprovalReason::SexuallyExplicitContent => "SEXUALLY_EXPLICIT_CONTENT",
                DisapprovalReason::GainingAnUnfairAdvantage => "GAINING_AN_UNFAIR_ADVANTAGE",
                DisapprovalReason::GamingTheGoogleNetwork => "GAMING_THE_GOOGLE_NETWORK",
                DisapprovalReason::DangerousProductsKnives => "DANGEROUS_PRODUCTS_KNIVES",
                DisapprovalReason::DangerousProductsExplosives => "DANGEROUS_PRODUCTS_EXPLOSIVES",
                DisapprovalReason::DangerousProductsGuns => "DANGEROUS_PRODUCTS_GUNS",
                DisapprovalReason::DangerousProductsDrugs => "DANGEROUS_PRODUCTS_DRUGS",
                DisapprovalReason::DangerousProductsTobacco => "DANGEROUS_PRODUCTS_TOBACCO",
                DisapprovalReason::DangerousProductsWeapons => "DANGEROUS_PRODUCTS_WEAPONS",
                DisapprovalReason::UnclearOrIrrelevantAd => "UNCLEAR_OR_IRRELEVANT_AD",
                DisapprovalReason::ProfessionalStandards => "PROFESSIONAL_STANDARDS",
                DisapprovalReason::DysfunctionalPromotion => "DYSFUNCTIONAL_PROMOTION",
                DisapprovalReason::InvalidInterestBasedAd => "INVALID_INTEREST_BASED_AD",
                DisapprovalReason::MisuseOfPersonalInformation => "MISUSE_OF_PERSONAL_INFORMATION",
                DisapprovalReason::OmissionOfRelevantInformation => {
                    "OMISSION_OF_RELEVANT_INFORMATION"
                }
                DisapprovalReason::UnavailablePromotions => "UNAVAILABLE_PROMOTIONS",
                DisapprovalReason::MisleadingPromotions => "MISLEADING_PROMOTIONS",
                DisapprovalReason::InappropriateContent => "INAPPROPRIATE_CONTENT",
                DisapprovalReason::SensitiveEvents => "SENSITIVE_EVENTS",
                DisapprovalReason::ShockingContent => "SHOCKING_CONTENT",
                DisapprovalReason::EnablingDishonestBehavior => "ENABLING_DISHONEST_BEHAVIOR",
                DisapprovalReason::TechnicalRequirements => "TECHNICAL_REQUIREMENTS",
                DisapprovalReason::RestrictedPoliticalContent => "RESTRICTED_POLITICAL_CONTENT",
                DisapprovalReason::UnsupportedContent => "UNSUPPORTED_CONTENT",
                DisapprovalReason::InvalidBiddingMethod => "INVALID_BIDDING_METHOD",
                DisapprovalReason::VideoTooLong => "VIDEO_TOO_LONG",
                DisapprovalReason::ViolatesJapanesePharmacyLaw => "VIOLATES_JAPANESE_PHARMACY_LAW",
                DisapprovalReason::UnaccreditedPetPharmacy => "UNACCREDITED_PET_PHARMACY",
                DisapprovalReason::Abortion => "ABORTION",
                DisapprovalReason::Contraceptives => "CONTRACEPTIVES",
                DisapprovalReason::NeedCertificatesToAdvertiseInChina => {
                    "NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA"
                }
                DisapprovalReason::KcdspRegistration => "KCDSP_REGISTRATION",
                DisapprovalReason::NotFamilySafe => "NOT_FAMILY_SAFE",
                DisapprovalReason::ClinicalTrialRecruitment => "CLINICAL_TRIAL_RECRUITMENT",
                DisapprovalReason::MaximumNumberOfHttpCallsExceeded => {
                    "MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED"
                }
                DisapprovalReason::MaximumNumberOfCookiesExceeded => {
                    "MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED"
                }
                DisapprovalReason::PersonalLoans => "PERSONAL_LOANS",
                DisapprovalReason::UnsupportedFlashContent => "UNSUPPORTED_FLASH_CONTENT",
                DisapprovalReason::MisuseByOmidScript => "MISUSE_BY_OMID_SCRIPT",
                DisapprovalReason::NonWhitelistedOmidVendor => "NON_WHITELISTED_OMID_VENDOR",
                DisapprovalReason::DestinationExperience => "DESTINATION_EXPERIENCE",
                DisapprovalReason::UnsupportedLanguage => "UNSUPPORTED_LANGUAGE",
                DisapprovalReason::NonSslCompliant => "NON_SSL_COMPLIANT",
                DisapprovalReason::TemporaryPause => "TEMPORARY_PAUSE",
                DisapprovalReason::BailBonds => "BAIL_BONDS",
            }
        }
    }
    impl ::std::fmt::Display for DisapprovalReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DisapprovalReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DisapprovalReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LENGTH_OF_IMAGE_ANIMATION" => DisapprovalReason::LengthOfImageAnimation,
                "BROKEN_URL" => DisapprovalReason::BrokenUrl,
                "MEDIA_NOT_FUNCTIONAL" => DisapprovalReason::MediaNotFunctional,
                "INVALID_FOURTH_PARTY_CALL" => DisapprovalReason::InvalidFourthPartyCall,
                "INCORRECT_REMARKETING_DECLARATION" => {
                    DisapprovalReason::IncorrectRemarketingDeclaration
                }
                "LANDING_PAGE_ERROR" => DisapprovalReason::LandingPageError,
                "AD_SIZE_DOES_NOT_MATCH_AD_SLOT" => DisapprovalReason::AdSizeDoesNotMatchAdSlot,
                "NO_BORDER" => DisapprovalReason::NoBorder,
                "FOURTH_PARTY_BROWSER_COOKIES" => DisapprovalReason::FourthPartyBrowserCookies,
                "LSO_OBJECTS" => DisapprovalReason::LsoObjects,
                "BLANK_CREATIVE" => DisapprovalReason::BlankCreative,
                "DESTINATION_URLS_UNDECLARED" => DisapprovalReason::DestinationUrlsUndeclared,
                "PROBLEM_WITH_CLICK_MACRO" => DisapprovalReason::ProblemWithClickMacro,
                "INCORRECT_AD_TECHNOLOGY_DECLARATION" => {
                    DisapprovalReason::IncorrectAdTechnologyDeclaration
                }
                "INCORRECT_DESTINATION_URL_DECLARATION" => {
                    DisapprovalReason::IncorrectDestinationUrlDeclaration
                }
                "EXPANDABLE_INCORRECT_DIRECTION" => DisapprovalReason::ExpandableIncorrectDirection,
                "EXPANDABLE_DIRECTION_NOT_SUPPORTED" => {
                    DisapprovalReason::ExpandableDirectionNotSupported
                }
                "EXPANDABLE_INVALID_VENDOR" => DisapprovalReason::ExpandableInvalidVendor,
                "EXPANDABLE_FUNCTIONALITY" => DisapprovalReason::ExpandableFunctionality,
                "VIDEO_INVALID_VENDOR" => DisapprovalReason::VideoInvalidVendor,
                "VIDEO_UNSUPPORTED_LENGTH" => DisapprovalReason::VideoUnsupportedLength,
                "VIDEO_UNSUPPORTED_FORMAT" => DisapprovalReason::VideoUnsupportedFormat,
                "VIDEO_FUNCTIONALITY" => DisapprovalReason::VideoFunctionality,
                "LANDING_PAGE_DISABLED" => DisapprovalReason::LandingPageDisabled,
                "MALWARE_SUSPECTED" => DisapprovalReason::MalwareSuspected,
                "ADULT_IMAGE_OR_VIDEO" => DisapprovalReason::AdultImageOrVideo,
                "INACCURATE_AD_TEXT" => DisapprovalReason::InaccurateAdText,
                "COUNTERFEIT_DESIGNER_GOODS" => DisapprovalReason::CounterfeitDesignerGoods,
                "POP_UP" => DisapprovalReason::PopUp,
                "INVALID_RTB_PROTOCOL_USAGE" => DisapprovalReason::InvalidRtbProtocolUsage,
                "RAW_IP_ADDRESS_IN_SNIPPET" => DisapprovalReason::RawIpAddressInSnippet,
                "UNACCEPTABLE_CONTENT_SOFTWARE" => DisapprovalReason::UnacceptableContentSoftware,
                "UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN" => {
                    DisapprovalReason::UnauthorizedCookieOnGoogleDomain
                }
                "UNDECLARED_FLASH_OBJECTS" => DisapprovalReason::UndeclaredFlashObjects,
                "INVALID_SSL_DECLARATION" => DisapprovalReason::InvalidSslDeclaration,
                "DIRECT_DOWNLOAD_IN_AD" => DisapprovalReason::DirectDownloadInAd,
                "MAXIMUM_DOWNLOAD_SIZE_EXCEEDED" => DisapprovalReason::MaximumDownloadSizeExceeded,
                "DESTINATION_URL_SITE_NOT_CRAWLABLE" => {
                    DisapprovalReason::DestinationUrlSiteNotCrawlable
                }
                "BAD_URL_LEGAL_DISAPPROVAL" => DisapprovalReason::BadUrlLegalDisapproval,
                "PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED" => {
                    DisapprovalReason::PharmaGamblingAlcoholNotAllowed
                }
                "DYNAMIC_DNS_AT_DESTINATION_URL" => DisapprovalReason::DynamicDnsAtDestinationUrl,
                "POOR_IMAGE_OR_VIDEO_QUALITY" => DisapprovalReason::PoorImageOrVideoQuality,
                "UNACCEPTABLE_IMAGE_CONTENT" => DisapprovalReason::UnacceptableImageContent,
                "INCORRECT_IMAGE_LAYOUT" => DisapprovalReason::IncorrectImageLayout,
                "IRRELEVANT_IMAGE_OR_VIDEO" => DisapprovalReason::IrrelevantImageOrVideo,
                "DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK" => {
                    DisapprovalReason::DestinationSiteDoesNotAllowGoingBack
                }
                "MISLEADING_CLAIMS_IN_AD" => DisapprovalReason::MisleadingClaimsInAd,
                "RESTRICTED_PRODUCTS" => DisapprovalReason::RestrictedProducts,
                "UNACCEPTABLE_CONTENT" => DisapprovalReason::UnacceptableContent,
                "AUTOMATED_AD_CLICKING" => DisapprovalReason::AutomatedAdClicking,
                "INVALID_URL_PROTOCOL" => DisapprovalReason::InvalidUrlProtocol,
                "UNDECLARED_RESTRICTED_CONTENT" => DisapprovalReason::UndeclaredRestrictedContent,
                "INVALID_REMARKETING_LIST_USAGE" => DisapprovalReason::InvalidRemarketingListUsage,
                "DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT" => {
                    DisapprovalReason::DestinationSiteNotCrawlableRobotsTxt
                }
                "CLICK_TO_DOWNLOAD_NOT_AN_APP" => DisapprovalReason::ClickToDownloadNotAnApp,
                "INACCURATE_REVIEW_EXTENSION" => DisapprovalReason::InaccurateReviewExtension,
                "SEXUALLY_EXPLICIT_CONTENT" => DisapprovalReason::SexuallyExplicitContent,
                "GAINING_AN_UNFAIR_ADVANTAGE" => DisapprovalReason::GainingAnUnfairAdvantage,
                "GAMING_THE_GOOGLE_NETWORK" => DisapprovalReason::GamingTheGoogleNetwork,
                "DANGEROUS_PRODUCTS_KNIVES" => DisapprovalReason::DangerousProductsKnives,
                "DANGEROUS_PRODUCTS_EXPLOSIVES" => DisapprovalReason::DangerousProductsExplosives,
                "DANGEROUS_PRODUCTS_GUNS" => DisapprovalReason::DangerousProductsGuns,
                "DANGEROUS_PRODUCTS_DRUGS" => DisapprovalReason::DangerousProductsDrugs,
                "DANGEROUS_PRODUCTS_TOBACCO" => DisapprovalReason::DangerousProductsTobacco,
                "DANGEROUS_PRODUCTS_WEAPONS" => DisapprovalReason::DangerousProductsWeapons,
                "UNCLEAR_OR_IRRELEVANT_AD" => DisapprovalReason::UnclearOrIrrelevantAd,
                "PROFESSIONAL_STANDARDS" => DisapprovalReason::ProfessionalStandards,
                "DYSFUNCTIONAL_PROMOTION" => DisapprovalReason::DysfunctionalPromotion,
                "INVALID_INTEREST_BASED_AD" => DisapprovalReason::InvalidInterestBasedAd,
                "MISUSE_OF_PERSONAL_INFORMATION" => DisapprovalReason::MisuseOfPersonalInformation,
                "OMISSION_OF_RELEVANT_INFORMATION" => {
                    DisapprovalReason::OmissionOfRelevantInformation
                }
                "UNAVAILABLE_PROMOTIONS" => DisapprovalReason::UnavailablePromotions,
                "MISLEADING_PROMOTIONS" => DisapprovalReason::MisleadingPromotions,
                "INAPPROPRIATE_CONTENT" => DisapprovalReason::InappropriateContent,
                "SENSITIVE_EVENTS" => DisapprovalReason::SensitiveEvents,
                "SHOCKING_CONTENT" => DisapprovalReason::ShockingContent,
                "ENABLING_DISHONEST_BEHAVIOR" => DisapprovalReason::EnablingDishonestBehavior,
                "TECHNICAL_REQUIREMENTS" => DisapprovalReason::TechnicalRequirements,
                "RESTRICTED_POLITICAL_CONTENT" => DisapprovalReason::RestrictedPoliticalContent,
                "UNSUPPORTED_CONTENT" => DisapprovalReason::UnsupportedContent,
                "INVALID_BIDDING_METHOD" => DisapprovalReason::InvalidBiddingMethod,
                "VIDEO_TOO_LONG" => DisapprovalReason::VideoTooLong,
                "VIOLATES_JAPANESE_PHARMACY_LAW" => DisapprovalReason::ViolatesJapanesePharmacyLaw,
                "UNACCREDITED_PET_PHARMACY" => DisapprovalReason::UnaccreditedPetPharmacy,
                "ABORTION" => DisapprovalReason::Abortion,
                "CONTRACEPTIVES" => DisapprovalReason::Contraceptives,
                "NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA" => {
                    DisapprovalReason::NeedCertificatesToAdvertiseInChina
                }
                "KCDSP_REGISTRATION" => DisapprovalReason::KcdspRegistration,
                "NOT_FAMILY_SAFE" => DisapprovalReason::NotFamilySafe,
                "CLINICAL_TRIAL_RECRUITMENT" => DisapprovalReason::ClinicalTrialRecruitment,
                "MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED" => {
                    DisapprovalReason::MaximumNumberOfHttpCallsExceeded
                }
                "MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED" => {
                    DisapprovalReason::MaximumNumberOfCookiesExceeded
                }
                "PERSONAL_LOANS" => DisapprovalReason::PersonalLoans,
                "UNSUPPORTED_FLASH_CONTENT" => DisapprovalReason::UnsupportedFlashContent,
                "MISUSE_BY_OMID_SCRIPT" => DisapprovalReason::MisuseByOmidScript,
                "NON_WHITELISTED_OMID_VENDOR" => DisapprovalReason::NonWhitelistedOmidVendor,
                "DESTINATION_EXPERIENCE" => DisapprovalReason::DestinationExperience,
                "UNSUPPORTED_LANGUAGE" => DisapprovalReason::UnsupportedLanguage,
                "NON_SSL_COMPLIANT" => DisapprovalReason::NonSslCompliant,
                "TEMPORARY_PAUSE" => DisapprovalReason::TemporaryPause,
                "BAIL_BONDS" => DisapprovalReason::BailBonds,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Disapproval {
        #[doc = "Additional details about the reason for disapproval."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<String>>,
        #[doc = "The categorized reason for disapproval."]
        #[serde(rename = "reason", default)]
        pub reason: Option<crate::schemas::DisapprovalReason>,
    }
    impl ::field_selector::FieldSelector for Disapproval {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty;
    impl ::field_selector::FieldSelector for Empty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FilterSetBreakdownDimensionsItems {}
    impl FilterSetBreakdownDimensionsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for FilterSetBreakdownDimensionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterSetBreakdownDimensionsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterSetBreakdownDimensionsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FilterSetEnvironment {
        #[doc = "A placeholder for an undefined environment; indicates that no environment\nfilter will be applied."]
        EnvironmentUnspecified,
        #[doc = "The ad impression appears on the web."]
        Web,
        #[doc = "The ad impression appears in an app."]
        App,
    }
    impl FilterSetEnvironment {
        pub fn as_str(self) -> &'static str {
            match self {
                FilterSetEnvironment::EnvironmentUnspecified => "ENVIRONMENT_UNSPECIFIED",
                FilterSetEnvironment::Web => "WEB",
                FilterSetEnvironment::App => "APP",
            }
        }
    }
    impl ::std::fmt::Display for FilterSetEnvironment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterSetEnvironment {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterSetEnvironment {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENVIRONMENT_UNSPECIFIED" => FilterSetEnvironment::EnvironmentUnspecified,
                "WEB" => FilterSetEnvironment::Web,
                "APP" => FilterSetEnvironment::App,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FilterSetFormatsItems {}
    impl FilterSetFormatsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for FilterSetFormatsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterSetFormatsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterSetFormatsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FilterSetPlatformsItems {}
    impl FilterSetPlatformsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for FilterSetPlatformsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterSetPlatformsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterSetPlatformsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FilterSetTimeSeriesGranularity {
        #[doc = "A placeholder for an unspecified interval; no time series is applied.\nAll rows in response will contain data for the entire requested time range."]
        TimeSeriesGranularityUnspecified,
        #[doc = "Indicates that data will be broken down by the hour."]
        Hourly,
        #[doc = "Indicates that data will be broken down by the day."]
        Daily,
    }
    impl FilterSetTimeSeriesGranularity {
        pub fn as_str(self) -> &'static str {
            match self {
                FilterSetTimeSeriesGranularity::TimeSeriesGranularityUnspecified => {
                    "TIME_SERIES_GRANULARITY_UNSPECIFIED"
                }
                FilterSetTimeSeriesGranularity::Hourly => "HOURLY",
                FilterSetTimeSeriesGranularity::Daily => "DAILY",
            }
        }
    }
    impl ::std::fmt::Display for FilterSetTimeSeriesGranularity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterSetTimeSeriesGranularity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterSetTimeSeriesGranularity {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TIME_SERIES_GRANULARITY_UNSPECIFIED" => {
                    FilterSetTimeSeriesGranularity::TimeSeriesGranularityUnspecified
                }
                "HOURLY" => FilterSetTimeSeriesGranularity::Hourly,
                "DAILY" => FilterSetTimeSeriesGranularity::Daily,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FilterSet {
        #[doc = "An absolute date range, defined by a start date and an end date.\nInterpreted relative to Pacific time zone."]
        #[serde(rename = "absoluteDateRange", default)]
        pub absolute_date_range: Option<crate::schemas::AbsoluteDateRange>,
        #[doc = "The set of dimensions along which to break down the response; may be empty.\nIf multiple dimensions are requested, the breakdown is along the Cartesian\nproduct of the requested dimensions."]
        #[serde(rename = "breakdownDimensions", default)]
        pub breakdown_dimensions: Option<Vec<crate::schemas::FilterSetBreakdownDimensionsItems>>,
        #[doc = "The ID of the creative on which to filter; optional. This field may be set\nonly for a filter set that accesses account-level troubleshooting data,\ni.e., one whose name matches the `bidders/*/accounts/*/filterSets/*`\npattern."]
        #[serde(rename = "creativeId", default)]
        pub creative_id: Option<String>,
        #[doc = "The ID of the deal on which to filter; optional. This field may be set\nonly for a filter set that accesses account-level troubleshooting data,\ni.e., one whose name matches the `bidders/*/accounts/*/filterSets/*`\npattern."]
        #[serde(rename = "dealId", default)]
        #[serde(with = "crate::parsed_string")]
        pub deal_id: Option<i64>,
        #[doc = "The environment on which to filter; optional."]
        #[serde(rename = "environment", default)]
        pub environment: Option<crate::schemas::FilterSetEnvironment>,
        #[doc = "The list of formats on which to filter; may be empty. The filters\nrepresented by multiple formats are ORed together (i.e., if non-empty,\nresults must match any one of the formats)."]
        #[serde(rename = "formats", default)]
        pub formats: Option<Vec<crate::schemas::FilterSetFormatsItems>>,
        #[doc = "A user-defined name of the filter set. Filter set names must be unique\nglobally and match one of the patterns:\n\n- `bidders/*/filterSets/*` (for accessing bidder-level troubleshooting\ndata)\n- `bidders/*/accounts/*/filterSets/*` (for accessing account-level\ntroubleshooting data)\n\nThis field is required in create operations."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The list of platforms on which to filter; may be empty. The filters\nrepresented by multiple platforms are ORed together (i.e., if non-empty,\nresults must match any one of the platforms)."]
        #[serde(rename = "platforms", default)]
        pub platforms: Option<Vec<crate::schemas::FilterSetPlatformsItems>>,
        #[doc = "For Open Bidding partners only.\nThe list of publisher identifiers on which to filter; may be empty.\nThe filters represented by multiple publisher identifiers are ORed\ntogether."]
        #[serde(rename = "publisherIdentifiers", default)]
        pub publisher_identifiers: Option<Vec<String>>,
        #[doc = "An open-ended realtime time range, defined by the aggregation start\ntimestamp."]
        #[serde(rename = "realtimeTimeRange", default)]
        pub realtime_time_range: Option<crate::schemas::RealtimeTimeRange>,
        #[doc = "A relative date range, defined by an offset from today and a duration.\nInterpreted relative to Pacific time zone."]
        #[serde(rename = "relativeDateRange", default)]
        pub relative_date_range: Option<crate::schemas::RelativeDateRange>,
        #[doc = "For Authorized Buyers only.\nThe list of IDs of the seller (publisher) networks on which to filter;\nmay be empty. The filters represented by multiple seller network IDs are\nORed together (i.e., if non-empty, results must match any one of the\npublisher networks). See\n[seller-network-ids](https://developers.google.com/authorized-buyers/rtb/downloads/seller-network-ids)\nfile for the set of existing seller network IDs."]
        #[serde(rename = "sellerNetworkIds", default)]
        pub seller_network_ids: Option<Vec<i32>>,
        #[doc = "The granularity of time intervals if a time series breakdown is desired;\noptional."]
        #[serde(rename = "timeSeriesGranularity", default)]
        pub time_series_granularity: Option<crate::schemas::FilterSetTimeSeriesGranularity>,
    }
    impl ::field_selector::FieldSelector for FilterSet {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FilteredBidCreativeRow {
        #[doc = "The number of bids with the specified creative."]
        #[serde(rename = "bidCount", default)]
        pub bid_count: Option<crate::schemas::MetricValue>,
        #[doc = "The ID of the creative."]
        #[serde(rename = "creativeId", default)]
        pub creative_id: Option<String>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
    }
    impl ::field_selector::FieldSelector for FilteredBidCreativeRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FilteredBidDetailRow {
        #[doc = "The number of bids with the specified detail."]
        #[serde(rename = "bidCount", default)]
        pub bid_count: Option<crate::schemas::MetricValue>,
        #[doc = "The ID of the detail. The associated value can be looked up in the\ndictionary file corresponding to the DetailType in the response message."]
        #[serde(rename = "detailId", default)]
        pub detail_id: Option<i32>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
    }
    impl ::field_selector::FieldSelector for FilteredBidDetailRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FirstPartyMobileApplicationTargeting {
        #[doc = "A list of application IDs to be excluded."]
        #[serde(rename = "excludedAppIds", default)]
        pub excluded_app_ids: Option<Vec<String>>,
        #[doc = "A list of application IDs to be included."]
        #[serde(rename = "targetedAppIds", default)]
        pub targeted_app_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for FirstPartyMobileApplicationTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FrequencyCapTimeUnitType {
        #[doc = "A placeholder for an undefined time unit type. This just indicates the\nvariable with this value hasn't been initialized."]
        TimeUnitTypeUnspecified,
        #[doc = "Minute"]
        Minute,
        #[doc = "Hour"]
        Hour,
        #[doc = "Day"]
        Day,
        #[doc = "Week"]
        Week,
        #[doc = "Month"]
        Month,
        #[doc = "Lifetime"]
        Lifetime,
    }
    impl FrequencyCapTimeUnitType {
        pub fn as_str(self) -> &'static str {
            match self {
                FrequencyCapTimeUnitType::TimeUnitTypeUnspecified => "TIME_UNIT_TYPE_UNSPECIFIED",
                FrequencyCapTimeUnitType::Minute => "MINUTE",
                FrequencyCapTimeUnitType::Hour => "HOUR",
                FrequencyCapTimeUnitType::Day => "DAY",
                FrequencyCapTimeUnitType::Week => "WEEK",
                FrequencyCapTimeUnitType::Month => "MONTH",
                FrequencyCapTimeUnitType::Lifetime => "LIFETIME",
            }
        }
    }
    impl ::std::fmt::Display for FrequencyCapTimeUnitType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FrequencyCapTimeUnitType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FrequencyCapTimeUnitType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TIME_UNIT_TYPE_UNSPECIFIED" => FrequencyCapTimeUnitType::TimeUnitTypeUnspecified,
                "MINUTE" => FrequencyCapTimeUnitType::Minute,
                "HOUR" => FrequencyCapTimeUnitType::Hour,
                "DAY" => FrequencyCapTimeUnitType::Day,
                "WEEK" => FrequencyCapTimeUnitType::Week,
                "MONTH" => FrequencyCapTimeUnitType::Month,
                "LIFETIME" => FrequencyCapTimeUnitType::Lifetime,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FrequencyCap {
        #[doc = "The maximum number of impressions that can be served to a user within the\nspecified time period."]
        #[serde(rename = "maxImpressions", default)]
        pub max_impressions: Option<i32>,
        #[doc = "The amount of time, in the units specified by time_unit_type. Defines the\namount of time over which impressions per user are counted and capped."]
        #[serde(rename = "numTimeUnits", default)]
        pub num_time_units: Option<i32>,
        #[doc = "The time unit. Along with num_time_units defines the amount of time over\nwhich impressions per user are counted and capped."]
        #[serde(rename = "timeUnitType", default)]
        pub time_unit_type: Option<crate::schemas::FrequencyCapTimeUnitType>,
    }
    impl ::field_selector::FieldSelector for FrequencyCap {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GuaranteedFixedPriceTerms {
        #[doc = "Fixed price for the specified buyer."]
        #[serde(rename = "fixedPrices", default)]
        pub fixed_prices: Option<Vec<crate::schemas::PricePerBuyer>>,
        #[doc = "Guaranteed impressions as a percentage. This is the percentage\nof guaranteed looks that the buyer is guaranteeing to buy."]
        #[serde(rename = "guaranteedImpressions", default)]
        #[serde(with = "crate::parsed_string")]
        pub guaranteed_impressions: Option<i64>,
        #[doc = "Count of guaranteed looks. Required for deal, optional for product."]
        #[serde(rename = "guaranteedLooks", default)]
        #[serde(with = "crate::parsed_string")]
        pub guaranteed_looks: Option<i64>,
        #[doc = "Daily minimum looks for CPD deal types."]
        #[serde(rename = "minimumDailyLooks", default)]
        #[serde(with = "crate::parsed_string")]
        pub minimum_daily_looks: Option<i64>,
    }
    impl ::field_selector::FieldSelector for GuaranteedFixedPriceTerms {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct HtmlContent {
        #[doc = "The height of the HTML snippet in pixels."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The HTML snippet that displays the ad when inserted in the web page."]
        #[serde(rename = "snippet", default)]
        pub snippet: Option<String>,
        #[doc = "The width of the HTML snippet in pixels."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for HtmlContent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Image {
        #[doc = "Image height in pixels."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The URL of the image."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
        #[doc = "Image width in pixels."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Image {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImpressionMetricsRow {
        #[doc = "The number of impressions available to the buyer on Ad Exchange.\nIn some cases this value may be unavailable."]
        #[serde(rename = "availableImpressions", default)]
        pub available_impressions: Option<crate::schemas::MetricValue>,
        #[doc = "The number of impressions for which Ad Exchange sent the buyer a bid\nrequest."]
        #[serde(rename = "bidRequests", default)]
        pub bid_requests: Option<crate::schemas::MetricValue>,
        #[doc = "The number of impressions that match the buyer's inventory pretargeting."]
        #[serde(rename = "inventoryMatches", default)]
        pub inventory_matches: Option<crate::schemas::MetricValue>,
        #[doc = "The number of impressions for which Ad Exchange received a response from\nthe buyer that contained at least one applicable bid."]
        #[serde(rename = "responsesWithBids", default)]
        pub responses_with_bids: Option<crate::schemas::MetricValue>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
        #[doc = "The number of impressions for which the buyer successfully sent a response\nto Ad Exchange."]
        #[serde(rename = "successfulResponses", default)]
        pub successful_responses: Option<crate::schemas::MetricValue>,
    }
    impl ::field_selector::FieldSelector for ImpressionMetricsRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventorySizeTargeting {
        #[doc = "A list of inventory sizes to be excluded."]
        #[serde(rename = "excludedInventorySizes", default)]
        pub excluded_inventory_sizes: Option<Vec<crate::schemas::AdSize>>,
        #[doc = "A list of inventory sizes to be included."]
        #[serde(rename = "targetedInventorySizes", default)]
        pub targeted_inventory_sizes: Option<Vec<crate::schemas::AdSize>>,
    }
    impl ::field_selector::FieldSelector for InventorySizeTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListBidMetricsResponse {
        #[doc = "List of rows, each containing a set of bid metrics."]
        #[serde(rename = "bidMetricsRows", default)]
        pub bid_metrics_rows: Option<Vec<crate::schemas::BidMetricsRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListBidMetricsRequest.pageToken\nfield in the subsequent call to the bidMetrics.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListBidMetricsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListBidResponseErrorsResponse {
        #[doc = "List of rows, with counts of bid responses aggregated by callout status."]
        #[serde(rename = "calloutStatusRows", default)]
        pub callout_status_rows: Option<Vec<crate::schemas::CalloutStatusRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListBidResponseErrorsRequest.pageToken\nfield in the subsequent call to the bidResponseErrors.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListBidResponseErrorsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListBidResponsesWithoutBidsResponse {
        #[doc = "List of rows, with counts of bid responses without bids aggregated by\nstatus."]
        #[serde(rename = "bidResponseWithoutBidsStatusRows", default)]
        pub bid_response_without_bids_status_rows:
            Option<Vec<crate::schemas::BidResponseWithoutBidsStatusRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListBidResponsesWithoutBidsRequest.pageToken\nfield in the subsequent call to the bidResponsesWithoutBids.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListBidResponsesWithoutBidsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListClientUserInvitationsResponse {
        #[doc = "The returned list of client users."]
        #[serde(rename = "invitations", default)]
        pub invitations: Option<Vec<crate::schemas::ClientUserInvitation>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListClientUserInvitationsRequest.pageToken\nfield in the subsequent call to the\nclients.invitations.list\nmethod to retrieve the next\npage of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListClientUserInvitationsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListClientUsersResponse {
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListClientUsersRequest.pageToken\nfield in the subsequent call to the\nclients.invitations.list\nmethod to retrieve the next\npage of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The returned list of client users."]
        #[serde(rename = "users", default)]
        pub users: Option<Vec<crate::schemas::ClientUser>>,
    }
    impl ::field_selector::FieldSelector for ListClientUsersResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListClientsResponse {
        #[doc = "The returned list of clients."]
        #[serde(rename = "clients", default)]
        pub clients: Option<Vec<crate::schemas::Client>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListClientsRequest.pageToken\nfield in the subsequent call to the\naccounts.clients.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListClientsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListCreativeStatusBreakdownByCreativeResponse {
        #[doc = "List of rows, with counts of bids with a given creative status aggregated\nby creative."]
        #[serde(rename = "filteredBidCreativeRows", default)]
        pub filtered_bid_creative_rows: Option<Vec<crate::schemas::FilteredBidCreativeRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListCreativeStatusBreakdownByCreativeRequest.pageToken\nfield in the subsequent call to the filteredBids.creatives.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListCreativeStatusBreakdownByCreativeResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListCreativeStatusBreakdownByDetailResponseDetailType {
        #[doc = "A placeholder for an undefined status.\nThis value will never be returned in responses."]
        DetailTypeUnspecified,
        #[doc = "Indicates that the detail ID refers to a creative attribute; see\n[publisher-excludable-creative-attributes](https://developers.google.com/authorized-buyers/rtb/downloads/publisher-excludable-creative-attributes)."]
        CreativeAttribute,
        #[doc = "Indicates that the detail ID refers to a vendor; see\n[vendors](https://developers.google.com/authorized-buyers/rtb/downloads/vendors)."]
        Vendor,
        #[doc = "Indicates that the detail ID refers to a sensitive category; see\n[ad-sensitive-categories](https://developers.google.com/authorized-buyers/rtb/downloads/ad-sensitive-categories)."]
        SensitiveCategory,
        #[doc = "Indicates that the detail ID refers to a product category; see\n[ad-product-categories](https://developers.google.com/authorized-buyers/rtb/downloads/ad-product-categories)."]
        ProductCategory,
        #[doc = "Indicates that the detail ID refers to a disapproval reason; see\nDisapprovalReason enum in\n[snippet-status-report-proto](https://developers.google.com/authorized-buyers/rtb/downloads/snippet-status-report-proto)."]
        DisapprovalReason,
    }
    impl ListCreativeStatusBreakdownByDetailResponseDetailType {
        pub fn as_str(self) -> &'static str {
            match self {
                ListCreativeStatusBreakdownByDetailResponseDetailType::DetailTypeUnspecified => {
                    "DETAIL_TYPE_UNSPECIFIED"
                }
                ListCreativeStatusBreakdownByDetailResponseDetailType::CreativeAttribute => {
                    "CREATIVE_ATTRIBUTE"
                }
                ListCreativeStatusBreakdownByDetailResponseDetailType::Vendor => "VENDOR",
                ListCreativeStatusBreakdownByDetailResponseDetailType::SensitiveCategory => {
                    "SENSITIVE_CATEGORY"
                }
                ListCreativeStatusBreakdownByDetailResponseDetailType::ProductCategory => {
                    "PRODUCT_CATEGORY"
                }
                ListCreativeStatusBreakdownByDetailResponseDetailType::DisapprovalReason => {
                    "DISAPPROVAL_REASON"
                }
            }
        }
    }
    impl ::std::fmt::Display for ListCreativeStatusBreakdownByDetailResponseDetailType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListCreativeStatusBreakdownByDetailResponseDetailType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListCreativeStatusBreakdownByDetailResponseDetailType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DETAIL_TYPE_UNSPECIFIED" => {
                    ListCreativeStatusBreakdownByDetailResponseDetailType::DetailTypeUnspecified
                }
                "CREATIVE_ATTRIBUTE" => {
                    ListCreativeStatusBreakdownByDetailResponseDetailType::CreativeAttribute
                }
                "VENDOR" => ListCreativeStatusBreakdownByDetailResponseDetailType::Vendor,
                "SENSITIVE_CATEGORY" => {
                    ListCreativeStatusBreakdownByDetailResponseDetailType::SensitiveCategory
                }
                "PRODUCT_CATEGORY" => {
                    ListCreativeStatusBreakdownByDetailResponseDetailType::ProductCategory
                }
                "DISAPPROVAL_REASON" => {
                    ListCreativeStatusBreakdownByDetailResponseDetailType::DisapprovalReason
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListCreativeStatusBreakdownByDetailResponse {
        #[doc = "The type of detail that the detail IDs represent."]
        #[serde(rename = "detailType", default)]
        pub detail_type:
            Option<crate::schemas::ListCreativeStatusBreakdownByDetailResponseDetailType>,
        #[doc = "List of rows, with counts of bids with a given creative status aggregated\nby detail."]
        #[serde(rename = "filteredBidDetailRows", default)]
        pub filtered_bid_detail_rows: Option<Vec<crate::schemas::FilteredBidDetailRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListCreativeStatusBreakdownByDetailRequest.pageToken\nfield in the subsequent call to the filteredBids.details.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListCreativeStatusBreakdownByDetailResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListCreativesResponse {
        #[doc = "The list of creatives."]
        #[serde(rename = "creatives", default)]
        pub creatives: Option<Vec<crate::schemas::Creative>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListCreativesRequest.page_token\nfield in the subsequent call to `ListCreatives` method to retrieve the next\npage of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListCreativesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListDealAssociationsResponse {
        #[doc = "The list of associations."]
        #[serde(rename = "associations", default)]
        pub associations: Option<Vec<crate::schemas::CreativeDealAssociation>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListDealAssociationsRequest.page_token\nfield in the subsequent call to 'ListDealAssociation' method to retrieve\nthe next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListDealAssociationsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListFilterSetsResponse {
        #[doc = "The filter sets belonging to the buyer."]
        #[serde(rename = "filterSets", default)]
        pub filter_sets: Option<Vec<crate::schemas::FilterSet>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListFilterSetsRequest.pageToken\nfield in the subsequent call to the\naccounts.filterSets.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListFilterSetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListFilteredBidRequestsResponse {
        #[doc = "List of rows, with counts of filtered bid requests aggregated by callout\nstatus."]
        #[serde(rename = "calloutStatusRows", default)]
        pub callout_status_rows: Option<Vec<crate::schemas::CalloutStatusRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListFilteredBidRequestsRequest.pageToken\nfield in the subsequent call to the filteredBidRequests.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListFilteredBidRequestsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListFilteredBidsResponse {
        #[doc = "List of rows, with counts of filtered bids aggregated by filtering reason\n(i.e. creative status)."]
        #[serde(rename = "creativeStatusRows", default)]
        pub creative_status_rows: Option<Vec<crate::schemas::CreativeStatusRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListFilteredBidsRequest.pageToken\nfield in the subsequent call to the filteredBids.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListFilteredBidsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListImpressionMetricsResponse {
        #[doc = "List of rows, each containing a set of impression metrics."]
        #[serde(rename = "impressionMetricsRows", default)]
        pub impression_metrics_rows: Option<Vec<crate::schemas::ImpressionMetricsRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListImpressionMetricsRequest.pageToken\nfield in the subsequent call to the impressionMetrics.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListImpressionMetricsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListLosingBidsResponse {
        #[doc = "List of rows, with counts of losing bids aggregated by loss reason (i.e.\ncreative status)."]
        #[serde(rename = "creativeStatusRows", default)]
        pub creative_status_rows: Option<Vec<crate::schemas::CreativeStatusRow>>,
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListLosingBidsRequest.pageToken\nfield in the subsequent call to the losingBids.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListLosingBidsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListNonBillableWinningBidsResponse {
        #[doc = "A token to retrieve the next page of results.\nPass this value in the\nListNonBillableWinningBidsRequest.pageToken\nfield in the subsequent call to the nonBillableWinningBids.list\nmethod to retrieve the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "List of rows, with counts of bids not billed aggregated by reason."]
        #[serde(rename = "nonBillableWinningBidStatusRows", default)]
        pub non_billable_winning_bid_status_rows:
            Option<Vec<crate::schemas::NonBillableWinningBidStatusRow>>,
    }
    impl ::field_selector::FieldSelector for ListNonBillableWinningBidsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListProductsResponse {
        #[doc = "List pagination support."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of matching products at their head revision number."]
        #[serde(rename = "products", default)]
        pub products: Option<Vec<crate::schemas::Product>>,
    }
    impl ::field_selector::FieldSelector for ListProductsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListProposalsResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of proposals."]
        #[serde(rename = "proposals", default)]
        pub proposals: Option<Vec<crate::schemas::Proposal>>,
    }
    impl ::field_selector::FieldSelector for ListProposalsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListPublisherProfilesResponse {
        #[doc = "List pagination support"]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of matching publisher profiles."]
        #[serde(rename = "publisherProfiles", default)]
        pub publisher_profiles: Option<Vec<crate::schemas::PublisherProfile>>,
    }
    impl ::field_selector::FieldSelector for ListPublisherProfilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LocationContext {
        #[doc = "IDs representing the geo location for this context.\nPlease refer to the\n[geo-table.csv](https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv)\nfile for different geo criteria IDs."]
        #[serde(rename = "geoCriteriaIds", default)]
        pub geo_criteria_ids: Option<Vec<i32>>,
    }
    impl ::field_selector::FieldSelector for LocationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MarketplaceTargeting {
        #[doc = "Geo criteria IDs to be included/excluded."]
        #[serde(rename = "geoTargeting", default)]
        pub geo_targeting: Option<crate::schemas::CriteriaTargeting>,
        #[doc = "Inventory sizes to be included/excluded."]
        #[serde(rename = "inventorySizeTargeting", default)]
        pub inventory_size_targeting: Option<crate::schemas::InventorySizeTargeting>,
        #[doc = "Placement targeting information, e.g., URL, mobile applications."]
        #[serde(rename = "placementTargeting", default)]
        pub placement_targeting: Option<crate::schemas::PlacementTargeting>,
        #[doc = "Technology targeting information, e.g., operating system, device category."]
        #[serde(rename = "technologyTargeting", default)]
        pub technology_targeting: Option<crate::schemas::TechnologyTargeting>,
        #[doc = "Video targeting information."]
        #[serde(rename = "videoTargeting", default)]
        pub video_targeting: Option<crate::schemas::VideoTargeting>,
    }
    impl ::field_selector::FieldSelector for MarketplaceTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MetricValue {
        #[doc = "The expected value of the metric."]
        #[serde(rename = "value", default)]
        #[serde(with = "crate::parsed_string")]
        pub value: Option<i64>,
        #[doc = "The variance (i.e. square of the standard deviation) of the metric value.\nIf value is exact, variance is 0.\nCan be used to calculate margin of error as a percentage of value, using\nthe following formula, where Z is the standard constant that depends on the\ndesired size of the confidence interval (e.g. for 90% confidence interval,\nuse Z = 1.645):\n\n  marginOfError = 100 * Z * sqrt(variance) / value"]
        #[serde(rename = "variance", default)]
        #[serde(with = "crate::parsed_string")]
        pub variance: Option<i64>,
    }
    impl ::field_selector::FieldSelector for MetricValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MobileApplicationTargeting {
        #[doc = "Publisher owned apps to be targeted or excluded by the publisher to\ndisplay the ads in."]
        #[serde(rename = "firstPartyTargeting", default)]
        pub first_party_targeting: Option<crate::schemas::FirstPartyMobileApplicationTargeting>,
    }
    impl ::field_selector::FieldSelector for MobileApplicationTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Money {
        #[doc = "The 3-letter currency code defined in ISO 4217."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999 inclusive.\nIf `units` is positive, `nanos` must be positive or zero.\nIf `units` is zero, `nanos` can be positive, zero, or negative.\nIf `units` is negative, `nanos` must be negative or zero.\nFor example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(rename = "nanos", default)]
        pub nanos: Option<i32>,
        #[doc = "The whole units of the amount.\nFor example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        #[serde(rename = "units", default)]
        #[serde(with = "crate::parsed_string")]
        pub units: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Money {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NativeContent {
        #[doc = "The name of the advertiser or sponsor, to be displayed in the ad creative."]
        #[serde(rename = "advertiserName", default)]
        pub advertiser_name: Option<String>,
        #[doc = "The app icon, for app download ads."]
        #[serde(rename = "appIcon", default)]
        pub app_icon: Option<crate::schemas::Image>,
        #[doc = "A long description of the ad."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "A label for the button that the user is supposed to click."]
        #[serde(rename = "callToAction", default)]
        pub call_to_action: Option<String>,
        #[doc = "The URL that the browser/SDK will load when the user clicks the ad."]
        #[serde(rename = "clickLinkUrl", default)]
        pub click_link_url: Option<String>,
        #[doc = "The URL to use for click tracking."]
        #[serde(rename = "clickTrackingUrl", default)]
        pub click_tracking_url: Option<String>,
        #[doc = "A short title for the ad."]
        #[serde(rename = "headline", default)]
        pub headline: Option<String>,
        #[doc = "A large image."]
        #[serde(rename = "image", default)]
        pub image: Option<crate::schemas::Image>,
        #[doc = "A smaller image, for the advertiser's logo."]
        #[serde(rename = "logo", default)]
        pub logo: Option<crate::schemas::Image>,
        #[doc = "The price of the promoted app including currency info."]
        #[serde(rename = "priceDisplayText", default)]
        pub price_display_text: Option<String>,
        #[doc = "The app rating in the app store. Must be in the range [0-5]."]
        #[serde(rename = "starRating", default)]
        pub star_rating: Option<f64>,
        #[doc = "The URL to the app store to purchase/download the promoted app."]
        #[serde(rename = "storeUrl", default)]
        pub store_url: Option<String>,
        #[doc = "The URL to fetch a native video ad."]
        #[serde(rename = "videoUrl", default)]
        pub video_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for NativeContent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NonBillableWinningBidStatusRowStatus {
        #[doc = "A placeholder for an undefined status.\nThis value will never be returned in responses."]
        StatusUnspecified,
        #[doc = "The buyer was not billed because the ad was not rendered by the\npublisher."]
        AdNotRendered,
        #[doc = "The buyer was not billed because the impression won by the bid was\ndetermined to be invalid."]
        InvalidImpression,
    }
    impl NonBillableWinningBidStatusRowStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                NonBillableWinningBidStatusRowStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                NonBillableWinningBidStatusRowStatus::AdNotRendered => "AD_NOT_RENDERED",
                NonBillableWinningBidStatusRowStatus::InvalidImpression => "INVALID_IMPRESSION",
            }
        }
    }
    impl ::std::fmt::Display for NonBillableWinningBidStatusRowStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonBillableWinningBidStatusRowStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonBillableWinningBidStatusRowStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNSPECIFIED" => NonBillableWinningBidStatusRowStatus::StatusUnspecified,
                "AD_NOT_RENDERED" => NonBillableWinningBidStatusRowStatus::AdNotRendered,
                "INVALID_IMPRESSION" => NonBillableWinningBidStatusRowStatus::InvalidImpression,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonBillableWinningBidStatusRow {
        #[doc = "The number of bids with the specified status."]
        #[serde(rename = "bidCount", default)]
        pub bid_count: Option<crate::schemas::MetricValue>,
        #[doc = "The values of all dimensions associated with metric values in this row."]
        #[serde(rename = "rowDimensions", default)]
        pub row_dimensions: Option<crate::schemas::RowDimensions>,
        #[doc = "The status specifying why the winning bids were not billed."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::NonBillableWinningBidStatusRowStatus>,
    }
    impl ::field_selector::FieldSelector for NonBillableWinningBidStatusRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonGuaranteedAuctionTerms {
        #[doc = "True if open auction buyers are allowed to compete with invited buyers\nin this private auction."]
        #[serde(rename = "autoOptimizePrivateAuction", default)]
        pub auto_optimize_private_auction: Option<bool>,
        #[doc = "Reserve price for the specified buyer."]
        #[serde(rename = "reservePricesPerBuyer", default)]
        pub reserve_prices_per_buyer: Option<Vec<crate::schemas::PricePerBuyer>>,
    }
    impl ::field_selector::FieldSelector for NonGuaranteedAuctionTerms {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NonGuaranteedFixedPriceTerms {
        #[doc = "Fixed price for the specified buyer."]
        #[serde(rename = "fixedPrices", default)]
        pub fixed_prices: Option<Vec<crate::schemas::PricePerBuyer>>,
    }
    impl ::field_selector::FieldSelector for NonGuaranteedFixedPriceTerms {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NoteCreatorRole {
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl NoteCreatorRole {
        pub fn as_str(self) -> &'static str {
            match self {
                NoteCreatorRole::BuyerSellerRoleUnspecified => "BUYER_SELLER_ROLE_UNSPECIFIED",
                NoteCreatorRole::Buyer => "BUYER",
                NoteCreatorRole::Seller => "SELLER",
            }
        }
    }
    impl ::std::fmt::Display for NoteCreatorRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NoteCreatorRole {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NoteCreatorRole {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUYER_SELLER_ROLE_UNSPECIFIED" => NoteCreatorRole::BuyerSellerRoleUnspecified,
                "BUYER" => NoteCreatorRole::Buyer,
                "SELLER" => NoteCreatorRole::Seller,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Note {
        #[doc = "Output only. The timestamp for when this note was created."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Output only. The role of the person (buyer/seller) creating the note."]
        #[serde(rename = "creatorRole", default)]
        pub creator_role: Option<crate::schemas::NoteCreatorRole>,
        #[doc = "The actual note to attach.\n(max-length: 1024 unicode code units)\n\nNote: This field may be set only when creating the resource. Modifying\nthis field while updating the resource will result in an error."]
        #[serde(rename = "note", default)]
        pub note: Option<String>,
        #[doc = "Output only. The unique ID for the note."]
        #[serde(rename = "noteId", default)]
        pub note_id: Option<String>,
        #[doc = "Output only. The revision number of the proposal when the note is created."]
        #[serde(rename = "proposalRevision", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Note {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OperatingSystemTargeting {
        #[doc = "IDs of operating systems to be included/excluded."]
        #[serde(rename = "operatingSystemCriteria", default)]
        pub operating_system_criteria: Option<crate::schemas::CriteriaTargeting>,
        #[doc = "IDs of operating system versions to be included/excluded."]
        #[serde(rename = "operatingSystemVersionCriteria", default)]
        pub operating_system_version_criteria: Option<crate::schemas::CriteriaTargeting>,
    }
    impl ::field_selector::FieldSelector for OperatingSystemTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PauseProposalRequest {
        #[doc = "The reason why the proposal is being paused.\nThis human readable message will be displayed in the seller's UI.\n(Max length: 1000 unicode code units.)"]
        #[serde(rename = "reason", default)]
        pub reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for PauseProposalRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PlacementTargeting {
        #[doc = "Mobile application targeting information in a deal.\nThis doesn't apply to Auction Packages."]
        #[serde(rename = "mobileApplicationTargeting", default)]
        pub mobile_application_targeting: Option<crate::schemas::MobileApplicationTargeting>,
        #[doc = "URLs to be included/excluded."]
        #[serde(rename = "urlTargeting", default)]
        pub url_targeting: Option<crate::schemas::UrlTargeting>,
    }
    impl ::field_selector::FieldSelector for PlacementTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlatformContextPlatformsItems {}
    impl PlatformContextPlatformsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for PlatformContextPlatformsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlatformContextPlatformsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlatformContextPlatformsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PlatformContext {
        #[doc = "The platforms this restriction applies to."]
        #[serde(rename = "platforms", default)]
        pub platforms: Option<Vec<crate::schemas::PlatformContextPlatformsItems>>,
    }
    impl ::field_selector::FieldSelector for PlatformContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PricePricingType {
        #[doc = "A placeholder for an undefined pricing type. If the pricing type is\nunpsecified, `COST_PER_MILLE` will be used instead."]
        PricingTypeUnspecified,
        #[doc = "Cost per thousand impressions."]
        CostPerMille,
        #[doc = "Cost per day"]
        CostPerDay,
    }
    impl PricePricingType {
        pub fn as_str(self) -> &'static str {
            match self {
                PricePricingType::PricingTypeUnspecified => "PRICING_TYPE_UNSPECIFIED",
                PricePricingType::CostPerMille => "COST_PER_MILLE",
                PricePricingType::CostPerDay => "COST_PER_DAY",
            }
        }
    }
    impl ::std::fmt::Display for PricePricingType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PricePricingType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PricePricingType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRICING_TYPE_UNSPECIFIED" => PricePricingType::PricingTypeUnspecified,
                "COST_PER_MILLE" => PricePricingType::CostPerMille,
                "COST_PER_DAY" => PricePricingType::CostPerDay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Price {
        #[doc = "The actual price with currency specified."]
        #[serde(rename = "amount", default)]
        pub amount: Option<crate::schemas::Money>,
        #[doc = "The pricing type for the deal/product. (default: CPM)"]
        #[serde(rename = "pricingType", default)]
        pub pricing_type: Option<crate::schemas::PricePricingType>,
    }
    impl ::field_selector::FieldSelector for Price {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PricePerBuyer {
        #[doc = "The list of advertisers for this price when associated with this buyer.\nIf empty, all advertisers with this buyer pay this price."]
        #[serde(rename = "advertiserIds", default)]
        pub advertiser_ids: Option<Vec<String>>,
        #[doc = "The buyer who will pay this price. If unset, all buyers can pay this price\n(if the\nadvertisers match, and there's no more specific rule matching the buyer)."]
        #[serde(rename = "buyer", default)]
        pub buyer: Option<crate::schemas::Buyer>,
        #[doc = "The specified price."]
        #[serde(rename = "price", default)]
        pub price: Option<crate::schemas::Price>,
    }
    impl ::field_selector::FieldSelector for PricePerBuyer {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PrivateData {
        #[doc = "A buyer or seller specified reference ID. This can be queried in the list\noperations (max-length: 1024 unicode code units)."]
        #[serde(rename = "referenceId", default)]
        pub reference_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for PrivateData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProductSyndicationProduct {
        #[doc = "A placeholder for an undefined syndication product."]
        SyndicationProductUnspecified,
        #[doc = "This typically represents a web page."]
        Content,
        #[doc = "This represents a mobile property."]
        Mobile,
        #[doc = "This represents video ad formats."]
        Video,
        #[doc = "This represents ads shown within games."]
        Games,
    }
    impl ProductSyndicationProduct {
        pub fn as_str(self) -> &'static str {
            match self {
                ProductSyndicationProduct::SyndicationProductUnspecified => {
                    "SYNDICATION_PRODUCT_UNSPECIFIED"
                }
                ProductSyndicationProduct::Content => "CONTENT",
                ProductSyndicationProduct::Mobile => "MOBILE",
                ProductSyndicationProduct::Video => "VIDEO",
                ProductSyndicationProduct::Games => "GAMES",
            }
        }
    }
    impl ::std::fmt::Display for ProductSyndicationProduct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProductSyndicationProduct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProductSyndicationProduct {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNDICATION_PRODUCT_UNSPECIFIED" => {
                    ProductSyndicationProduct::SyndicationProductUnspecified
                }
                "CONTENT" => ProductSyndicationProduct::Content,
                "MOBILE" => ProductSyndicationProduct::Mobile,
                "VIDEO" => ProductSyndicationProduct::Video,
                "GAMES" => ProductSyndicationProduct::Games,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Product {
        #[doc = "The proposed end time for the deal. The field will be truncated to the\norder of seconds during serving."]
        #[serde(rename = "availableEndTime", default)]
        pub available_end_time: Option<String>,
        #[doc = "Inventory availability dates. The start time will be truncated to seconds\nduring serving. Thus, a field specified as 3:23:34.456 (HH:mm:ss.SSS) will\nbe truncated to 3:23:34 when serving."]
        #[serde(rename = "availableStartTime", default)]
        pub available_start_time: Option<String>,
        #[doc = "Creation time."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Optional contact information for the creator of this product."]
        #[serde(rename = "creatorContacts", default)]
        pub creator_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "The display name for this product as set by the seller."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "If the creator has already signed off on the product, then the buyer can\nfinalize the deal by accepting the product as is. When copying to a\nproposal, if any of the terms are changed, then auto_finalize is\nautomatically set to false."]
        #[serde(rename = "hasCreatorSignedOff", default)]
        pub has_creator_signed_off: Option<bool>,
        #[doc = "The unique ID for the product."]
        #[serde(rename = "productId", default)]
        pub product_id: Option<String>,
        #[doc = "The revision number of the product (auto-assigned by Marketplace)."]
        #[serde(rename = "productRevision", default)]
        #[serde(with = "crate::parsed_string")]
        pub product_revision: Option<i64>,
        #[doc = "An ID which can be used by the Publisher Profile API to get more\ninformation about the seller that created this product."]
        #[serde(rename = "publisherProfileId", default)]
        pub publisher_profile_id: Option<String>,
        #[doc = "Information about the seller that created this product."]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
        #[doc = "The syndication product associated with the deal."]
        #[serde(rename = "syndicationProduct", default)]
        pub syndication_product: Option<crate::schemas::ProductSyndicationProduct>,
        #[doc = "Targeting that is shared between the buyer and the seller. Each targeting\ncriterion has a specified key and for each key there is a list of inclusion\nvalue or exclusion values."]
        #[serde(rename = "targetingCriterion", default)]
        pub targeting_criterion: Option<Vec<crate::schemas::TargetingCriteria>>,
        #[doc = "The negotiable terms of the deal."]
        #[serde(rename = "terms", default)]
        pub terms: Option<crate::schemas::DealTerms>,
        #[doc = "Time of last update."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
        #[doc = "The web-property code for the seller. This needs to be copied as is when\nadding a new deal to a proposal."]
        #[serde(rename = "webPropertyCode", default)]
        pub web_property_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProposalLastUpdaterOrCommentorRole {
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl ProposalLastUpdaterOrCommentorRole {
        pub fn as_str(self) -> &'static str {
            match self {
                ProposalLastUpdaterOrCommentorRole::BuyerSellerRoleUnspecified => {
                    "BUYER_SELLER_ROLE_UNSPECIFIED"
                }
                ProposalLastUpdaterOrCommentorRole::Buyer => "BUYER",
                ProposalLastUpdaterOrCommentorRole::Seller => "SELLER",
            }
        }
    }
    impl ::std::fmt::Display for ProposalLastUpdaterOrCommentorRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProposalLastUpdaterOrCommentorRole {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProposalLastUpdaterOrCommentorRole {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUYER_SELLER_ROLE_UNSPECIFIED" => {
                    ProposalLastUpdaterOrCommentorRole::BuyerSellerRoleUnspecified
                }
                "BUYER" => ProposalLastUpdaterOrCommentorRole::Buyer,
                "SELLER" => ProposalLastUpdaterOrCommentorRole::Seller,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProposalOriginatorRole {
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl ProposalOriginatorRole {
        pub fn as_str(self) -> &'static str {
            match self {
                ProposalOriginatorRole::BuyerSellerRoleUnspecified => {
                    "BUYER_SELLER_ROLE_UNSPECIFIED"
                }
                ProposalOriginatorRole::Buyer => "BUYER",
                ProposalOriginatorRole::Seller => "SELLER",
            }
        }
    }
    impl ::std::fmt::Display for ProposalOriginatorRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProposalOriginatorRole {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProposalOriginatorRole {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUYER_SELLER_ROLE_UNSPECIFIED" => {
                    ProposalOriginatorRole::BuyerSellerRoleUnspecified
                }
                "BUYER" => ProposalOriginatorRole::Buyer,
                "SELLER" => ProposalOriginatorRole::Seller,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProposalProposalState {
        #[doc = "A placeholder for an undefined proposal state."]
        ProposalStateUnspecified,
        #[doc = "The proposal is under negotiation or renegotiation."]
        Proposed,
        #[doc = "The proposal has been accepted by the buyer."]
        BuyerAccepted,
        #[doc = "The proposal has been accepted by the seller."]
        SellerAccepted,
        #[doc = "The negotiations on the proposal were canceled and the proposal was never\nfinalized."]
        Canceled,
        #[doc = "The proposal is finalized. During renegotiation, the proposal may\nnot be in this state."]
        Finalized,
    }
    impl ProposalProposalState {
        pub fn as_str(self) -> &'static str {
            match self {
                ProposalProposalState::ProposalStateUnspecified => "PROPOSAL_STATE_UNSPECIFIED",
                ProposalProposalState::Proposed => "PROPOSED",
                ProposalProposalState::BuyerAccepted => "BUYER_ACCEPTED",
                ProposalProposalState::SellerAccepted => "SELLER_ACCEPTED",
                ProposalProposalState::Canceled => "CANCELED",
                ProposalProposalState::Finalized => "FINALIZED",
            }
        }
    }
    impl ::std::fmt::Display for ProposalProposalState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProposalProposalState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProposalProposalState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PROPOSAL_STATE_UNSPECIFIED" => ProposalProposalState::ProposalStateUnspecified,
                "PROPOSED" => ProposalProposalState::Proposed,
                "BUYER_ACCEPTED" => ProposalProposalState::BuyerAccepted,
                "SELLER_ACCEPTED" => ProposalProposalState::SellerAccepted,
                "CANCELED" => ProposalProposalState::Canceled,
                "FINALIZED" => ProposalProposalState::Finalized,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Proposal {
        #[doc = "Output only. Reference to the buyer that will get billed for this proposal."]
        #[serde(rename = "billedBuyer", default)]
        pub billed_buyer: Option<crate::schemas::Buyer>,
        #[doc = "Reference to the buyer on the proposal.\n\nNote: This field may be set only when creating the resource. Modifying\nthis field while updating the resource will result in an error."]
        #[serde(rename = "buyer", default)]
        pub buyer: Option<crate::schemas::Buyer>,
        #[doc = "Contact information for the buyer."]
        #[serde(rename = "buyerContacts", default)]
        pub buyer_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "Private data for buyer. (hidden from seller)."]
        #[serde(rename = "buyerPrivateData", default)]
        pub buyer_private_data: Option<crate::schemas::PrivateData>,
        #[doc = "The deals associated with this proposal. For Private Auction proposals\n(whose deals have NonGuaranteedAuctionTerms), there will only be one deal."]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::Deal>>,
        #[doc = "The name for the proposal."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "Output only. True if the proposal is being renegotiated."]
        #[serde(rename = "isRenegotiating", default)]
        pub is_renegotiating: Option<bool>,
        #[doc = "Output only. True, if the buyside inventory setup is complete for this\nproposal."]
        #[serde(rename = "isSetupComplete", default)]
        pub is_setup_complete: Option<bool>,
        #[doc = "Output only. The role of the last user that either updated the proposal or\nleft a comment."]
        #[serde(rename = "lastUpdaterOrCommentorRole", default)]
        pub last_updater_or_commentor_role:
            Option<crate::schemas::ProposalLastUpdaterOrCommentorRole>,
        #[doc = "Output only. The notes associated with this proposal."]
        #[serde(rename = "notes", default)]
        pub notes: Option<Vec<crate::schemas::Note>>,
        #[doc = "Output only. Indicates whether the buyer/seller created the proposal."]
        #[serde(rename = "originatorRole", default)]
        pub originator_role: Option<crate::schemas::ProposalOriginatorRole>,
        #[doc = "Output only. Private auction ID if this proposal is a private auction\nproposal."]
        #[serde(rename = "privateAuctionId", default)]
        pub private_auction_id: Option<String>,
        #[doc = "Output only. The unique ID of the proposal."]
        #[serde(rename = "proposalId", default)]
        pub proposal_id: Option<String>,
        #[doc = "Output only. The revision number for the proposal.\nEach update to the proposal or the deal causes the proposal revision number\nto auto-increment. The buyer keeps track of the last revision number they\nknow of and pass it in when making an update. If the head revision number\non the server has since incremented, then an ABORTED error is returned\nduring the update operation to let the buyer know that a subsequent update\nwas made."]
        #[serde(rename = "proposalRevision", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision: Option<i64>,
        #[doc = "Output only. The current state of the proposal."]
        #[serde(rename = "proposalState", default)]
        pub proposal_state: Option<crate::schemas::ProposalProposalState>,
        #[doc = "Reference to the seller on the proposal.\n\nNote: This field may be set only when creating the resource. Modifying\nthis field while updating the resource will result in an error."]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
        #[doc = "Output only. Contact information for the seller."]
        #[serde(rename = "sellerContacts", default)]
        pub seller_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "Output only. The time when the proposal was last revised."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for Proposal {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PublisherProfile {
        #[doc = "Description on the publisher's audience."]
        #[serde(rename = "audienceDescription", default)]
        pub audience_description: Option<String>,
        #[doc = "Statement explaining what's unique about publisher's business, and why\nbuyers should partner with the publisher."]
        #[serde(rename = "buyerPitchStatement", default)]
        pub buyer_pitch_statement: Option<String>,
        #[doc = "Contact information for direct reservation deals. This is free text entered\nby the publisher and may include information like names, phone numbers and\nemail addresses."]
        #[serde(rename = "directDealsContact", default)]
        pub direct_deals_contact: Option<String>,
        #[doc = "Name of the publisher profile."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The list of domains represented in this publisher profile. Empty if this is\na parent profile. These are top private domains, meaning that these will\nnot contain a string like \"photos.google.co.uk/123\", but will instead\ncontain \"google.co.uk\"."]
        #[serde(rename = "domains", default)]
        pub domains: Option<Vec<String>>,
        #[doc = "URL to publisher's Google+ page."]
        #[serde(rename = "googlePlusUrl", default)]
        pub google_plus_url: Option<String>,
        #[doc = "A Google public URL to the logo for this publisher profile. The logo is\nstored as a PNG, JPG, or GIF image."]
        #[serde(rename = "logoUrl", default)]
        pub logo_url: Option<String>,
        #[doc = "URL to additional marketing and sales materials."]
        #[serde(rename = "mediaKitUrl", default)]
        pub media_kit_url: Option<String>,
        #[doc = "Overview of the publisher."]
        #[serde(rename = "overview", default)]
        pub overview: Option<String>,
        #[doc = "Contact information for programmatic deals. This is free text entered by\nthe publisher and may include information like names, phone numbers and\nemail addresses."]
        #[serde(rename = "programmaticDealsContact", default)]
        pub programmatic_deals_contact: Option<String>,
        #[doc = "Unique ID for publisher profile."]
        #[serde(rename = "publisherProfileId", default)]
        pub publisher_profile_id: Option<String>,
        #[doc = "URL to a publisher rate card."]
        #[serde(rename = "rateCardInfoUrl", default)]
        pub rate_card_info_url: Option<String>,
        #[doc = "URL to a sample content page."]
        #[serde(rename = "samplePageUrl", default)]
        pub sample_page_url: Option<String>,
        #[doc = "Seller of the publisher profile."]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
        #[doc = "Up to three key metrics and rankings. Max 100 characters each.\nFor example \"#1 Mobile News Site for 20 Straight Months\"."]
        #[serde(rename = "topHeadlines", default)]
        pub top_headlines: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for PublisherProfile {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RealtimeTimeRange {
        #[doc = "The start timestamp of the real-time RTB metrics aggregation."]
        #[serde(rename = "startTimestamp", default)]
        pub start_timestamp: Option<String>,
    }
    impl ::field_selector::FieldSelector for RealtimeTimeRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RelativeDateRange {
        #[doc = "The number of days in the requested date range, e.g., for a range spanning\ntoday: 1. For a range spanning the last 7 days: 7."]
        #[serde(rename = "durationDays", default)]
        pub duration_days: Option<i32>,
        #[doc = "The end date of the filter set, specified as the number of days before\ntoday, e.g., for a range where the last date is today: 0."]
        #[serde(rename = "offsetDays", default)]
        pub offset_days: Option<i32>,
    }
    impl ::field_selector::FieldSelector for RelativeDateRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RemoveDealAssociationRequest {
        #[doc = "The association between a creative and a deal that should be removed."]
        #[serde(rename = "association", default)]
        pub association: Option<crate::schemas::CreativeDealAssociation>,
    }
    impl ::field_selector::FieldSelector for RemoveDealAssociationRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResumeProposalRequest;
    impl ::field_selector::FieldSelector for ResumeProposalRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RowDimensions {
        #[doc = "The publisher identifier for this row, if a breakdown by\n[BreakdownDimension.PUBLISHER_IDENTIFIER](https://developers.google.com/authorized-buyers/apis/reference/rest/v2beta1/bidders.accounts.filterSets#FilterSet.BreakdownDimension)\nwas requested."]
        #[serde(rename = "publisherIdentifier", default)]
        pub publisher_identifier: Option<String>,
        #[doc = "The time interval that this row represents."]
        #[serde(rename = "timeInterval", default)]
        pub time_interval: Option<crate::schemas::TimeInterval>,
    }
    impl ::field_selector::FieldSelector for RowDimensions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SecurityContextSecuritiesItems {}
    impl SecurityContextSecuritiesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for SecurityContextSecuritiesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SecurityContextSecuritiesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SecurityContextSecuritiesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SecurityContext {
        #[doc = "The security types in this context."]
        #[serde(rename = "securities", default)]
        pub securities: Option<Vec<crate::schemas::SecurityContextSecuritiesItems>>,
    }
    impl ::field_selector::FieldSelector for SecurityContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Seller {
        #[doc = "The unique ID for the seller. The seller fills in this field.\nThe seller account ID is then available to buyer in the product."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "Optional sub-account ID for the seller."]
        #[serde(rename = "subAccountId", default)]
        pub sub_account_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Seller {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServingContextAll {
        #[doc = "A simple context."]
        SimpleContext,
    }
    impl ServingContextAll {
        pub fn as_str(self) -> &'static str {
            match self {
                ServingContextAll::SimpleContext => "SIMPLE_CONTEXT",
            }
        }
    }
    impl ::std::fmt::Display for ServingContextAll {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServingContextAll {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServingContextAll {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SIMPLE_CONTEXT" => ServingContextAll::SimpleContext,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServingContext {
        #[doc = "Matches all contexts."]
        #[serde(rename = "all", default)]
        pub all: Option<crate::schemas::ServingContextAll>,
        #[doc = "Matches impressions for a particular app type."]
        #[serde(rename = "appType", default)]
        pub app_type: Option<crate::schemas::AppContext>,
        #[doc = "Matches impressions for a particular auction type."]
        #[serde(rename = "auctionType", default)]
        pub auction_type: Option<crate::schemas::AuctionContext>,
        #[doc = "Matches impressions coming from users *or* publishers in a specific\nlocation."]
        #[serde(rename = "location", default)]
        pub location: Option<crate::schemas::LocationContext>,
        #[doc = "Matches impressions coming from a particular platform."]
        #[serde(rename = "platform", default)]
        pub platform: Option<crate::schemas::PlatformContext>,
        #[doc = "Matches impressions for a particular security type."]
        #[serde(rename = "securityType", default)]
        pub security_type: Option<crate::schemas::SecurityContext>,
    }
    impl ::field_selector::FieldSelector for ServingContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServingRestrictionStatus {
        #[doc = "The status is not known."]
        StatusUnspecified,
        #[doc = "The ad was disapproved in this context."]
        Disapproval,
        #[doc = "The ad is pending review in this context."]
        PendingReview,
    }
    impl ServingRestrictionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ServingRestrictionStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                ServingRestrictionStatus::Disapproval => "DISAPPROVAL",
                ServingRestrictionStatus::PendingReview => "PENDING_REVIEW",
            }
        }
    }
    impl ::std::fmt::Display for ServingRestrictionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServingRestrictionStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServingRestrictionStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNSPECIFIED" => ServingRestrictionStatus::StatusUnspecified,
                "DISAPPROVAL" => ServingRestrictionStatus::Disapproval,
                "PENDING_REVIEW" => ServingRestrictionStatus::PendingReview,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServingRestriction {
        #[doc = "The contexts for the restriction."]
        #[serde(rename = "contexts", default)]
        pub contexts: Option<Vec<crate::schemas::ServingContext>>,
        #[doc = "Disapproval bound to this restriction.\nOnly present if status=DISAPPROVED.\nCan be used to filter the response of the\ncreatives.list\nmethod."]
        #[serde(rename = "disapproval", default)]
        pub disapproval: Option<crate::schemas::Disapproval>,
        #[doc = "Any disapprovals bound to this restriction.\nOnly present if status=DISAPPROVED.\nCan be used to filter the response of the\ncreatives.list\nmethod.\nDeprecated; please use\ndisapproval\nfield instead."]
        #[serde(rename = "disapprovalReasons", default)]
        pub disapproval_reasons: Option<Vec<crate::schemas::Disapproval>>,
        #[doc = "The status of the creative in this context (for example, it has been\nexplicitly disapproved or is pending review)."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::ServingRestrictionStatus>,
    }
    impl ::field_selector::FieldSelector for ServingRestriction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Size {
        #[doc = "The height of the creative."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The width of the creative"]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Size {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StopWatchingCreativeRequest;
    impl ::field_selector::FieldSelector for StopWatchingCreativeRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TargetingCriteria {
        #[doc = "The list of values to exclude from targeting. Each value is AND'd\ntogether."]
        #[serde(rename = "exclusions", default)]
        pub exclusions: Option<Vec<crate::schemas::TargetingValue>>,
        #[doc = "The list of value to include as part of the targeting. Each value is OR'd\ntogether."]
        #[serde(rename = "inclusions", default)]
        pub inclusions: Option<Vec<crate::schemas::TargetingValue>>,
        #[doc = "The key representing the shared targeting criterion.\nTargeting criteria defined by Google ad servers will begin with GOOG_.\nThird parties may define their own keys.\nA list of permissible keys along with the acceptable values will be\nprovided as part of the external documentation."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
    }
    impl ::field_selector::FieldSelector for TargetingCriteria {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TargetingValue {
        #[doc = "The creative size value to include/exclude.\nFilled in when key = GOOG_CREATIVE_SIZE"]
        #[serde(rename = "creativeSizeValue", default)]
        pub creative_size_value: Option<crate::schemas::CreativeSize>,
        #[doc = "The daypart targeting to include / exclude.\nFilled in when the key is GOOG_DAYPART_TARGETING.\nThe definition of this targeting is derived from the structure\nused by Ad Manager."]
        #[serde(rename = "dayPartTargetingValue", default)]
        pub day_part_targeting_value: Option<crate::schemas::DayPartTargeting>,
        #[doc = "The long value to include/exclude."]
        #[serde(rename = "longValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub long_value: Option<i64>,
        #[doc = "The string value to include/exclude."]
        #[serde(rename = "stringValue", default)]
        pub string_value: Option<String>,
    }
    impl ::field_selector::FieldSelector for TargetingValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TechnologyTargeting {
        #[doc = "IDs of device capabilities to be included/excluded."]
        #[serde(rename = "deviceCapabilityTargeting", default)]
        pub device_capability_targeting: Option<crate::schemas::CriteriaTargeting>,
        #[doc = "IDs of device categories to be included/excluded."]
        #[serde(rename = "deviceCategoryTargeting", default)]
        pub device_category_targeting: Option<crate::schemas::CriteriaTargeting>,
        #[doc = "Operating system related targeting information."]
        #[serde(rename = "operatingSystemTargeting", default)]
        pub operating_system_targeting: Option<crate::schemas::OperatingSystemTargeting>,
    }
    impl ::field_selector::FieldSelector for TechnologyTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeInterval {
        #[doc = "The timestamp marking the end of the range (exclusive) for which data is\nincluded."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "The timestamp marking the start of the range (inclusive) for which data is\nincluded."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for TimeInterval {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeOfDay {
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose\nto allow the value \"24:00:00\" for scenarios like business closing time."]
        #[serde(rename = "hours", default)]
        pub hours: Option<i32>,
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        #[serde(rename = "minutes", default)]
        pub minutes: Option<i32>,
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        #[serde(rename = "nanos", default)]
        pub nanos: Option<i32>,
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may\nallow the value 60 if it allows leap-seconds."]
        #[serde(rename = "seconds", default)]
        pub seconds: Option<i32>,
    }
    impl ::field_selector::FieldSelector for TimeOfDay {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UrlTargeting {
        #[doc = "A list of URLs to be excluded."]
        #[serde(rename = "excludedUrls", default)]
        pub excluded_urls: Option<Vec<String>>,
        #[doc = "A list of URLs to be included."]
        #[serde(rename = "targetedUrls", default)]
        pub targeted_urls: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for UrlTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VideoContent {
        #[doc = "The URL to fetch a video ad."]
        #[serde(rename = "videoUrl", default)]
        pub video_url: Option<String>,
        #[doc = "The contents of a VAST document for a video ad.\nThis document should conform to the VAST 2.0 or 3.0 standard."]
        #[serde(rename = "videoVastXml", default)]
        pub video_vast_xml: Option<String>,
    }
    impl ::field_selector::FieldSelector for VideoContent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VideoTargetingExcludedPositionTypesItems {}
    impl VideoTargetingExcludedPositionTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for VideoTargetingExcludedPositionTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VideoTargetingExcludedPositionTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VideoTargetingExcludedPositionTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VideoTargetingTargetedPositionTypesItems {}
    impl VideoTargetingTargetedPositionTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for VideoTargetingTargetedPositionTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VideoTargetingTargetedPositionTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VideoTargetingTargetedPositionTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VideoTargeting {
        #[doc = "A list of video positions to be excluded.\nPosition types can either be included or excluded (XOR)."]
        #[serde(rename = "excludedPositionTypes", default)]
        pub excluded_position_types:
            Option<Vec<crate::schemas::VideoTargetingExcludedPositionTypesItems>>,
        #[doc = "A list of video positions to be included.\nWhen the included list is present, the excluded list must be empty.\nWhen the excluded list is present, the included list must be empty."]
        #[serde(rename = "targetedPositionTypes", default)]
        pub targeted_position_types:
            Option<Vec<crate::schemas::VideoTargetingTargetedPositionTypesItems>>,
    }
    impl ::field_selector::FieldSelector for VideoTargeting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WatchCreativeRequest {
        #[doc = "The Pub/Sub topic to publish notifications to.\nThis topic must already exist and must give permission to\nad-exchange-buyside-reports@google.com to write to the topic.\nThis should be the full resource name in\n\"projects/{project_id}/topics/{topic_id}\" format."]
        #[serde(rename = "topic", default)]
        pub topic: Option<String>,
    }
    impl ::field_selector::FieldSelector for WatchCreativeRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the accounts resource"]
    pub fn accounts(&self) -> crate::accounts::AccountsActions<A> {
        crate::accounts::AccountsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the bidders resource"]
    pub fn bidders(&self) -> crate::bidders::BiddersActions<A> {
        crate::bidders::BiddersActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod accounts {
    pub mod params {}
    pub struct AccountsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AccountsActions<'a, A> {
        #[doc = "Actions that can be performed on the clients resource"]
        pub fn clients(&self) -> clients::ClientsActions<A> {
            clients::ClientsActions
        }
        #[doc = "Actions that can be performed on the creatives resource"]
        pub fn creatives(&self) -> creatives::CreativesActions<A> {
            creatives::CreativesActions
        }
        #[doc = "Actions that can be performed on the finalized_proposals resource"]
        pub fn finalized_proposals(&self) -> finalized_proposals::FinalizedProposalsActions<A> {
            finalized_proposals::FinalizedProposalsActions
        }
        #[doc = "Actions that can be performed on the products resource"]
        pub fn products(&self) -> products::ProductsActions<A> {
            products::ProductsActions
        }
        #[doc = "Actions that can be performed on the proposals resource"]
        pub fn proposals(&self) -> proposals::ProposalsActions<A> {
            proposals::ProposalsActions
        }
        #[doc = "Actions that can be performed on the publisher_profiles resource"]
        pub fn publisher_profiles(&self) -> publisher_profiles::PublisherProfilesActions<A> {
            publisher_profiles::PublisherProfilesActions
        }
    }
    pub mod clients {
        pub mod params {}
        pub struct ClientsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ClientsActions<'a, A> {
            #[doc = "Creates a new client buyer."]
            pub fn create(
                &self,
                request: crate::schemas::Client,
                account_id: i64,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id,
                }
            }
            #[doc = "Gets a client buyer with a given client account ID."]
            pub fn get(&self, account_id: i64, client_account_id: i64) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id,
                    client_account_id,
                }
            }
            #[doc = "Lists all the clients for the current sponsor buyer."]
            pub fn list(&self, account_id: i64) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id,
                    page_size: None,
                    page_token: None,
                    partner_client_id: None,
                }
            }
            #[doc = "Updates an existing client buyer."]
            pub fn update(
                &self,
                request: crate::schemas::Client,
                account_id: i64,
                client_account_id: i64,
            ) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id,
                    client_account_id,
                }
            }
            #[doc = "Actions that can be performed on the invitations resource"]
            pub fn invitations(&self) -> invitations::InvitationsActions<A> {
                invitations::InvitationsActions
            }
            #[doc = "Actions that can be performed on the users resource"]
            pub fn users(&self) -> users::UsersActions<A> {
                users::UsersActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Client,
            account_id: i64,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Client, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                {
                    let str_value = self.account_id.to_string();
                    output.push_str(&str_value);
                }
                output.push_str("/clients");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: i64,
            client_account_id: i64,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Client, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                {
                    let str_value = self.account_id.to_string();
                    output.push_str(&str_value);
                }
                output.push_str("/clients/");
                {
                    let str_value = self.client_account_id.to_string();
                    output.push_str(&str_value);
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: i64,
            page_size: Option<i32>,
            page_token: Option<String>,
            partner_client_id: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Requested page size. The server may return fewer clients than requested.\nIf unspecified, the server will pick an appropriate default."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListClientsResponse.nextPageToken\nreturned from the previous call to the\naccounts.clients.list\nmethod."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Optional unique identifier (from the standpoint of an Ad Exchange sponsor\nbuyer partner) of the client to return.\nIf specified, at most one client will be returned in the response."]
            pub fn partner_client_id(&mut self, value: impl Into<String>) -> &mut Self {
                self.partner_client_id = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_clients<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "clients")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListClientsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                {
                    let str_value = self.account_id.to_string();
                    output.push_str(&str_value);
                }
                output.push_str("/clients");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("partnerClientId", &self.partner_client_id)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Client,
            account_id: i64,
            client_account_id: i64,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Client, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                {
                    let str_value = self.account_id.to_string();
                    output.push_str(&str_value);
                }
                output.push_str("/clients/");
                {
                    let str_value = self.client_account_id.to_string();
                    output.push_str(&str_value);
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        pub mod invitations {
            pub mod params {}
            pub struct InvitationsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> InvitationsActions<'a, A> {
                #[doc = "Creates and sends out an email invitation to access\nan Ad Exchange client buyer account."]
                pub fn create(
                    &self,
                    request: crate::schemas::ClientUserInvitation,
                    account_id: i64,
                    client_account_id: i64,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id,
                        client_account_id,
                    }
                }
                #[doc = "Retrieves an existing client user invitation."]
                pub fn get(
                    &self,
                    account_id: i64,
                    client_account_id: i64,
                    invitation_id: i64,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id,
                        client_account_id,
                        invitation_id,
                    }
                }
                #[doc = "Lists all the client users invitations for a client\nwith a given account ID."]
                pub fn list(
                    &self,
                    account_id: i64,
                    client_account_id: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id,
                        client_account_id: client_account_id.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::ClientUserInvitation,
                account_id: i64,
                client_account_id: i64,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ClientUserInvitation, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    {
                        let str_value = self.account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/clients/");
                    {
                        let str_value = self.client_account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/invitations");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: i64,
                client_account_id: i64,
                invitation_id: i64,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ClientUserInvitation, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    {
                        let str_value = self.account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/clients/");
                    {
                        let str_value = self.client_account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/invitations/");
                    {
                        let str_value = self.invitation_id.to_string();
                        output.push_str(&str_value);
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: i64,
                client_account_id: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. Server may return fewer clients than requested.\nIf unspecified, server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListClientUserInvitationsResponse.nextPageToken\nreturned from the previous call to the\nclients.invitations.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_invitations<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "invitations")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListClientUserInvitationsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    {
                        let str_value = self.account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/clients/");
                    output.push_str(&self.client_account_id);
                    output.push_str("/invitations");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod users {
            pub mod params {}
            pub struct UsersActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> UsersActions<'a, A> {
                #[doc = "Retrieves an existing client user."]
                pub fn get(
                    &self,
                    account_id: i64,
                    client_account_id: i64,
                    user_id: i64,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id,
                        client_account_id,
                        user_id,
                    }
                }
                #[doc = "Lists all the known client users for a specified\nsponsor buyer account ID."]
                pub fn list(
                    &self,
                    account_id: i64,
                    client_account_id: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id,
                        client_account_id: client_account_id.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates an existing client user.\nOnly the user status can be changed on update."]
                pub fn update(
                    &self,
                    request: crate::schemas::ClientUser,
                    account_id: i64,
                    client_account_id: i64,
                    user_id: i64,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id,
                        client_account_id,
                        user_id,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: i64,
                client_account_id: i64,
                user_id: i64,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ClientUser, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    {
                        let str_value = self.account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/clients/");
                    {
                        let str_value = self.client_account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/users/");
                    {
                        let str_value = self.user_id.to_string();
                        output.push_str(&str_value);
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: i64,
                client_account_id: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer clients than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListClientUsersResponse.nextPageToken\nreturned from the previous call to the\naccounts.clients.users.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_users<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "users")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListClientUsersResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    {
                        let str_value = self.account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/clients/");
                    output.push_str(&self.client_account_id);
                    output.push_str("/users");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::ClientUser,
                account_id: i64,
                client_account_id: i64,
                user_id: i64,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ClientUser, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    {
                        let str_value = self.account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/clients/");
                    {
                        let str_value = self.client_account_id.to_string();
                        output.push_str(&str_value);
                    }
                    output.push_str("/users/");
                    {
                        let str_value = self.user_id.to_string();
                        output.push_str(&str_value);
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
        }
    }
    pub mod creatives {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum CreateDuplicateIdMode {}
            impl CreateDuplicateIdMode {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for CreateDuplicateIdMode {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for CreateDuplicateIdMode {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for CreateDuplicateIdMode {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
        }
        pub struct CreativesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CreativesActions<'a, A> {
            #[doc = "Creates a creative."]
            pub fn create(
                &self,
                request: crate::schemas::Creative,
                account_id: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    duplicate_id_mode: None,
                }
            }
            #[doc = "Gets a creative."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                creative_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    creative_id: creative_id.into(),
                }
            }
            #[doc = "Lists creatives."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    page_size: None,
                    page_token: None,
                    query: None,
                }
            }
            #[doc = "Stops watching a creative. Will stop push notifications being sent to the\ntopics when the creative changes status."]
            pub fn stop_watching(
                &self,
                request: crate::schemas::StopWatchingCreativeRequest,
                account_id: impl Into<String>,
                creative_id: impl Into<String>,
            ) -> StopWatchingRequestBuilder<A> {
                StopWatchingRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    creative_id: creative_id.into(),
                }
            }
            #[doc = "Updates a creative."]
            pub fn update(
                &self,
                request: crate::schemas::Creative,
                account_id: impl Into<String>,
                creative_id: impl Into<String>,
            ) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    creative_id: creative_id.into(),
                }
            }
            #[doc = "Watches a creative. Will result in push notifications being sent to the\ntopic when the creative changes status."]
            pub fn watch(
                &self,
                request: crate::schemas::WatchCreativeRequest,
                account_id: impl Into<String>,
                creative_id: impl Into<String>,
            ) -> WatchRequestBuilder<A> {
                WatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    creative_id: creative_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the deal_associations resource"]
            pub fn deal_associations(&self) -> deal_associations::DealAssociationsActions<A> {
                deal_associations::DealAssociationsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Creative,
            account_id: String,
            duplicate_id_mode: Option<crate::creatives::params::CreateDuplicateIdMode>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "Indicates if multiple creatives can share an ID or not. Default is\nNO_DUPLICATES (one ID per creative)."]
            pub fn duplicate_id_mode(
                &mut self,
                value: crate::creatives::params::CreateDuplicateIdMode,
            ) -> &mut Self {
                self.duplicate_id_mode = Some(value);
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Creative, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/creatives");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("duplicateIdMode", &self.duplicate_id_mode)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            creative_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Creative, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/creatives/");
                output.push_str(&self.creative_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            page_size: Option<i32>,
            page_token: Option<String>,
            query: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Requested page size. The server may return fewer creatives than requested\n(due to timeout constraint) even if more are available via another call.\nIf unspecified, server will pick an appropriate default.\nAcceptable values are 1 to 1000, inclusive."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListCreativesResponse.next_page_token\nreturned from the previous call to 'ListCreatives' method."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "An optional query string to filter creatives. If no filter is specified,\nall active creatives will be returned.\n<p>Supported queries are:\n<ul>\n<li>accountId=<i>account_id_string</i>\n<li>creativeId=<i>creative_id_string</i>\n<li>dealsStatus: {approved, conditionally_approved, disapproved,\n                   not_checked}\n<li>openAuctionStatus: {approved, conditionally_approved, disapproved,\n                          not_checked}\n<li>attribute: {a numeric attribute from the list of attributes}\n<li>disapprovalReason: {a reason from\nDisapprovalReason}\n</ul>\nExample: 'accountId=12345 AND (dealsStatus:disapproved AND\ndisapprovalReason:unacceptable_content) OR attribute:47'"]
            pub fn query(&mut self, value: impl Into<String>) -> &mut Self {
                self.query = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_creatives<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "creatives")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListCreativesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/creatives");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("query", &self.query)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct StopWatchingRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::StopWatchingCreativeRequest,
            account_id: String,
            creative_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> StopWatchingRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/creatives/");
                output.push_str(&self.creative_id);
                output.push_str(":stopWatching");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Creative,
            account_id: String,
            creative_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Creative, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/creatives/");
                output.push_str(&self.creative_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct WatchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::WatchCreativeRequest,
            account_id: String,
            creative_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> WatchRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/creatives/");
                output.push_str(&self.creative_id);
                output.push_str(":watch");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        pub mod deal_associations {
            pub mod params {}
            pub struct DealAssociationsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> DealAssociationsActions<'a, A> {
                #[doc = "Associate an existing deal with a creative."]
                pub fn add(
                    &self,
                    request: crate::schemas::AddDealAssociationRequest,
                    account_id: impl Into<String>,
                    creative_id: impl Into<String>,
                ) -> AddRequestBuilder<A> {
                    AddRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id: account_id.into(),
                        creative_id: creative_id.into(),
                    }
                }
                #[doc = "List all creative-deal associations."]
                pub fn list(
                    &self,
                    account_id: impl Into<String>,
                    creative_id: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id: account_id.into(),
                        creative_id: creative_id.into(),
                        page_size: None,
                        page_token: None,
                        query: None,
                    }
                }
                #[doc = "Remove the association between a deal and a creative."]
                pub fn remove(
                    &self,
                    request: crate::schemas::RemoveDealAssociationRequest,
                    account_id: impl Into<String>,
                    creative_id: impl Into<String>,
                ) -> RemoveRequestBuilder<A> {
                    RemoveRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        account_id: account_id.into(),
                        creative_id: creative_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct AddRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::AddDealAssociationRequest,
                account_id: String,
                creative_id: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> AddRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/creatives/");
                    output.push_str(&self.creative_id);
                    output.push_str("/dealAssociations:add");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: String,
                creative_id: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                query: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. Server may return fewer associations than requested.\nIf unspecified, server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListDealAssociationsResponse.next_page_token\nreturned from the previous call to 'ListDealAssociations' method."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "An optional query string to filter deal associations. If no filter is\nspecified, all associations will be returned.\nSupported queries are:\n<ul>\n<li>accountId=<i>account_id_string</i>\n<li>creativeId=<i>creative_id_string</i>\n<li>dealsId=<i>deals_id_string</i>\n<li>dealsStatus:{approved, conditionally_approved, disapproved,\n                  not_checked}\n<li>openAuctionStatus:{approved, conditionally_approved, disapproved,\n                         not_checked}\n</ul>\nExample: 'dealsId=12345 AND dealsStatus:disapproved'"]
                pub fn query(&mut self, value: impl Into<String>) -> &mut Self {
                    self.query = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_associations<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "associations")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListDealAssociationsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/creatives/");
                    output.push_str(&self.creative_id);
                    output.push_str("/dealAssociations");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("query", &self.query)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct RemoveRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::RemoveDealAssociationRequest,
                account_id: String,
                creative_id: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> RemoveRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/creatives/");
                    output.push_str(&self.creative_id);
                    output.push_str("/dealAssociations:remove");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
        }
    }
    pub mod finalized_proposals {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListFilterSyntax {}
            impl ListFilterSyntax {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for ListFilterSyntax {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListFilterSyntax {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListFilterSyntax {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
        }
        pub struct FinalizedProposalsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> FinalizedProposalsActions<'a, A> {
            #[doc = "List finalized proposals, regardless if a proposal is being renegotiated.\nA filter expression (PQL query) may be specified to filter the results.\nThe notes will not be returned."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    filter: None,
                    filter_syntax: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            filter: Option<String>,
            filter_syntax: Option<crate::finalized_proposals::params::ListFilterSyntax>,
            page_size: Option<i32>,
            page_token: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "An optional PQL filter query used to query for proposals.\n\nNested repeated fields, such as proposal.deals.targetingCriterion,\ncannot be filtered."]
            pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Syntax the filter is written in. Current implementation defaults to PQL\nbut in the future it will be LIST_FILTER."]
            pub fn filter_syntax(
                &mut self,
                value: crate::finalized_proposals::params::ListFilterSyntax,
            ) -> &mut Self {
                self.filter_syntax = Some(value);
                self
            }
            #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The page token as returned from ListProposalsResponse."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_proposals<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "proposals")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListProposalsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/finalizedProposals");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("filterSyntax", &self.filter_syntax)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
    }
    pub mod products {
        pub mod params {}
        pub struct ProductsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProductsActions<'a, A> {
            #[doc = "Gets the requested product by ID."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                product_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    product_id: product_id.into(),
                }
            }
            #[doc = "List all products visible to the buyer (optionally filtered by the\nspecified PQL query)."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    filter: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            product_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Product, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/products/");
                output.push_str(&self.product_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            filter: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "An optional PQL query used to query for products. See\nhttps://developers.google.com/ad-manager/docs/pqlreference\nfor documentation about PQL and examples.\n\nNested repeated fields, such as product.targetingCriterion.inclusions,\ncannot be filtered."]
            pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The page token as returned from ListProductsResponse."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_products<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "products")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListProductsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/products");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
    }
    pub mod proposals {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListFilterSyntax {}
            impl ListFilterSyntax {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for ListFilterSyntax {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListFilterSyntax {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListFilterSyntax {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
        }
        pub struct ProposalsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProposalsActions<'a, A> {
            #[doc = "Mark the proposal as accepted at the given revision number. If the number\ndoes not match the server's revision number an `ABORTED` error message will\nbe returned. This call updates the proposal_state from `PROPOSED` to\n`BUYER_ACCEPTED`, or from `SELLER_ACCEPTED` to `FINALIZED`."]
            pub fn accept(
                &self,
                request: crate::schemas::AcceptProposalRequest,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> AcceptRequestBuilder<A> {
                AcceptRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Create a new note and attach it to the proposal. The note is assigned\na unique ID by the server.\nThe proposal revision number will not increase when associated with a\nnew note."]
            pub fn add_note(
                &self,
                request: crate::schemas::AddNoteRequest,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> AddNoteRequestBuilder<A> {
                AddNoteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Cancel an ongoing negotiation on a proposal. This does not cancel or end\nserving for the deals if the proposal has been finalized, but only cancels\na negotiation unilaterally."]
            pub fn cancel_negotiation(
                &self,
                request: crate::schemas::CancelNegotiationRequest,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> CancelNegotiationRequestBuilder<A> {
                CancelNegotiationRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Update the given proposal to indicate that setup has been completed.\nThis method is called by the buyer when the line items have been created\non their end for a finalized proposal and all the required creatives\nhave been uploaded using the creatives API. This call updates the\n`is_setup_completed` bit on the proposal and also notifies the seller.\nThe server will advance the revision number of the most recent proposal."]
            pub fn complete_setup(
                &self,
                request: crate::schemas::CompleteSetupRequest,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> CompleteSetupRequestBuilder<A> {
                CompleteSetupRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Create the given proposal. Each created proposal and any deals it contains\nare assigned a unique ID by the server."]
            pub fn create(
                &self,
                request: crate::schemas::Proposal,
                account_id: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                }
            }
            #[doc = "Gets a proposal given its ID. The proposal is returned at its head\nrevision."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "List proposals. A filter expression (PQL query) may be specified to\nfilter the results. To retrieve all finalized proposals, regardless if a\nproposal is being renegotiated, see the FinalizedProposals resource.\nNote that Bidder/ChildSeat relationships differ from the usual behavior.\nA Bidder account can only see its child seats' proposals by specifying\nthe ChildSeat's accountId in the request path."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    filter: None,
                    filter_syntax: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Update the given proposal to pause serving.\nThis method will set the\n`DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to true for all\ndeals in the proposal.\n\nIt is a no-op to pause an already-paused proposal.\nIt is an error to call PauseProposal for a proposal that is not\nfinalized or renegotiating."]
            pub fn pause(
                &self,
                request: crate::schemas::PauseProposalRequest,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> PauseRequestBuilder<A> {
                PauseRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Update the given proposal to resume serving.\nThis method will set the\n`DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to false for all\ndeals in the proposal.\n\nNote that if the `has_seller_paused` bit is also set, serving will not\nresume until the seller also resumes.\n\nIt is a no-op to resume an already-running proposal.\nIt is an error to call ResumeProposal for a proposal that is not\nfinalized or renegotiating."]
            pub fn resume(
                &self,
                request: crate::schemas::ResumeProposalRequest,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> ResumeRequestBuilder<A> {
                ResumeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Update the given proposal at the client known revision number. If the\nserver revision has advanced since the passed-in\n`proposal.proposal_revision`, an `ABORTED` error message will be returned.\nOnly the buyer-modifiable fields of the proposal will be updated.\n\nNote that the deals in the proposal will be updated to match the passed-in\ncopy.\nIf a passed-in deal does not have a `deal_id`, the server will assign a new\nunique ID and create the deal.\nIf passed-in deal has a `deal_id`, it will be updated to match the\npassed-in copy.\nAny existing deals not present in the passed-in proposal will be deleted.\nIt is an error to pass in a deal with a `deal_id` not present at head."]
            pub fn update(
                &self,
                request: crate::schemas::Proposal,
                account_id: impl Into<String>,
                proposal_id: impl Into<String>,
            ) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    proposal_id: proposal_id.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AcceptRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::AcceptProposalRequest,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> AcceptRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output.push_str(":accept");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct AddNoteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::AddNoteRequest,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> AddNoteRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Note, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output.push_str(":addNote");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct CancelNegotiationRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::CancelNegotiationRequest,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> CancelNegotiationRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output.push_str(":cancelNegotiation");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct CompleteSetupRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::CompleteSetupRequest,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> CompleteSetupRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output.push_str(":completeSetup");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Proposal,
            account_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            filter: Option<String>,
            filter_syntax: Option<crate::proposals::params::ListFilterSyntax>,
            page_size: Option<i32>,
            page_token: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "An optional PQL filter query used to query for proposals.\n\nNested repeated fields, such as proposal.deals.targetingCriterion,\ncannot be filtered."]
            pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Syntax the filter is written in. Current implementation defaults to PQL\nbut in the future it will be LIST_FILTER."]
            pub fn filter_syntax(
                &mut self,
                value: crate::proposals::params::ListFilterSyntax,
            ) -> &mut Self {
                self.filter_syntax = Some(value);
                self
            }
            #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The page token as returned from ListProposalsResponse."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_proposals<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "proposals")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListProposalsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("filterSyntax", &self.filter_syntax)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct PauseRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::PauseProposalRequest,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> PauseRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output.push_str(":pause");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ResumeRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::ResumeProposalRequest,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ResumeRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output.push_str(":resume");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Proposal,
            account_id: String,
            proposal_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/proposals/");
                output.push_str(&self.proposal_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod publisher_profiles {
        pub mod params {}
        pub struct PublisherProfilesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PublisherProfilesActions<'a, A> {
            #[doc = "Gets the requested publisher profile by id."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                publisher_profile_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    publisher_profile_id: publisher_profile_id.into(),
                }
            }
            #[doc = "List all publisher profiles visible to the buyer"]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    account_id: account_id.into(),
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            publisher_profile_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::PublisherProfile, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/publisherProfiles/");
                output.push_str(&self.publisher_profile_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            page_size: Option<i32>,
            page_token: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Specify the number of results to include per page."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The page token as return from ListPublisherProfilesResponse."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_publisher_profiles<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "publisherProfiles")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListPublisherProfilesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/accounts/");
                output.push_str(&self.account_id);
                output.push_str("/publisherProfiles");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
    }
}
pub mod bidders {
    pub mod params {}
    pub struct BiddersActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> BiddersActions<'a, A> {
        #[doc = "Actions that can be performed on the accounts resource"]
        pub fn accounts(&self) -> accounts::AccountsActions<A> {
            accounts::AccountsActions
        }
        #[doc = "Actions that can be performed on the filter_sets resource"]
        pub fn filter_sets(&self) -> filter_sets::FilterSetsActions<A> {
            filter_sets::FilterSetsActions
        }
    }
    pub mod accounts {
        pub mod params {}
        pub struct AccountsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AccountsActions<'a, A> {
            #[doc = "Actions that can be performed on the filter_sets resource"]
            pub fn filter_sets(&self) -> filter_sets::FilterSetsActions<A> {
                filter_sets::FilterSetsActions
            }
        }
        pub mod filter_sets {
            pub mod params {}
            pub struct FilterSetsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> FilterSetsActions<'a, A> {
                #[doc = "Creates the specified filter set for the account with the given account ID."]
                pub fn create(
                    &self,
                    request: crate::schemas::FilterSet,
                    owner_name: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        owner_name: owner_name.into(),
                        is_transient: None,
                    }
                }
                #[doc = "Deletes the requested filter set from the account with the given account\nID."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Retrieves the requested filter set for the account with the given account\nID."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Lists all filter sets for the account with the given account ID."]
                pub fn list(&self, owner_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        owner_name: owner_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the bid_metrics resource"]
                pub fn bid_metrics(&self) -> bid_metrics::BidMetricsActions<A> {
                    bid_metrics::BidMetricsActions
                }
                #[doc = "Actions that can be performed on the bid_response_errors resource"]
                pub fn bid_response_errors(
                    &self,
                ) -> bid_response_errors::BidResponseErrorsActions<A> {
                    bid_response_errors::BidResponseErrorsActions
                }
                #[doc = "Actions that can be performed on the bid_responses_without_bids resource"]
                pub fn bid_responses_without_bids(
                    &self,
                ) -> bid_responses_without_bids::BidResponsesWithoutBidsActions<A> {
                    bid_responses_without_bids::BidResponsesWithoutBidsActions
                }
                #[doc = "Actions that can be performed on the filtered_bid_requests resource"]
                pub fn filtered_bid_requests(
                    &self,
                ) -> filtered_bid_requests::FilteredBidRequestsActions<A> {
                    filtered_bid_requests::FilteredBidRequestsActions
                }
                #[doc = "Actions that can be performed on the filtered_bids resource"]
                pub fn filtered_bids(&self) -> filtered_bids::FilteredBidsActions<A> {
                    filtered_bids::FilteredBidsActions
                }
                #[doc = "Actions that can be performed on the impression_metrics resource"]
                pub fn impression_metrics(
                    &self,
                ) -> impression_metrics::ImpressionMetricsActions<A> {
                    impression_metrics::ImpressionMetricsActions
                }
                #[doc = "Actions that can be performed on the losing_bids resource"]
                pub fn losing_bids(&self) -> losing_bids::LosingBidsActions<A> {
                    losing_bids::LosingBidsActions
                }
                #[doc = "Actions that can be performed on the non_billable_winning_bids resource"]
                pub fn non_billable_winning_bids(
                    &self,
                ) -> non_billable_winning_bids::NonBillableWinningBidsActions<A> {
                    non_billable_winning_bids::NonBillableWinningBidsActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::FilterSet,
                owner_name: String,
                is_transient: Option<bool>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "Whether the filter set is transient, or should be persisted indefinitely.\nBy default, filter sets are not transient.\nIf transient, it will be available for at least 1 hour after creation."]
                pub fn is_transient(&mut self, value: bool) -> &mut Self {
                    self.is_transient = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::FilterSet, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.owner_name);
                    output.push_str("/filterSets");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("isTransient", &self.is_transient)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::FilterSet, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                owner_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListFilterSetsResponse.nextPageToken\nreturned from the previous call to the\naccounts.filterSets.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_filter_sets<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "filterSets")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListFilterSetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.owner_name);
                    output.push_str("/filterSets");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
            pub mod bid_metrics {
                pub mod params {}
                pub struct BidMetricsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> BidMetricsActions<'a, A> {
                    #[doc = "Lists all metrics that are measured in terms of number of bids."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListBidMetricsResponse.nextPageToken\nreturned from the previous call to the bidMetrics.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_bid_metrics_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "bidMetricsRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListBidMetricsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/bidMetrics");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod bid_response_errors {
                pub mod params {}
                pub struct BidResponseErrorsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> BidResponseErrorsActions<'a, A> {
                    #[doc = "List all errors that occurred in bid responses, with the number of bid\nresponses affected for each reason."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListBidResponseErrorsResponse.nextPageToken\nreturned from the previous call to the bidResponseErrors.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_callout_status_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "calloutStatusRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListBidResponseErrorsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/bidResponseErrors");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod bid_responses_without_bids {
                pub mod params {}
                pub struct BidResponsesWithoutBidsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> BidResponsesWithoutBidsActions<'a, A> {
                    #[doc = "List all reasons for which bid responses were considered to have no\napplicable bids, with the number of bid responses affected for each reason."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListBidResponsesWithoutBidsResponse.nextPageToken\nreturned from the previous call to the bidResponsesWithoutBids.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_bid_response_without_bids_status_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "bidResponseWithoutBidsStatusRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListBidResponsesWithoutBidsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/bidResponsesWithoutBids");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod filtered_bid_requests {
                pub mod params {}
                pub struct FilteredBidRequestsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FilteredBidRequestsActions<'a, A> {
                    #[doc = "List all reasons that caused a bid request not to be sent for an\nimpression, with the number of bid requests not sent for each reason."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListFilteredBidRequestsResponse.nextPageToken\nreturned from the previous call to the filteredBidRequests.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_callout_status_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "calloutStatusRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListFilteredBidRequestsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/filteredBidRequests");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod filtered_bids {
                pub mod params {}
                pub struct FilteredBidsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FilteredBidsActions<'a, A> {
                    #[doc = "List all reasons for which bids were filtered, with the number of bids\nfiltered for each reason."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the creatives resource"]
                    pub fn creatives(&self) -> creatives::CreativesActions<A> {
                        creatives::CreativesActions
                    }
                    #[doc = "Actions that can be performed on the details resource"]
                    pub fn details(&self) -> details::DetailsActions<A> {
                        details::DetailsActions
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListFilteredBidsResponse.nextPageToken\nreturned from the previous call to the filteredBids.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_creative_status_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "creativeStatusRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListFilteredBidsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/filteredBids");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
                pub mod creatives {
                    pub mod params {}
                    pub struct CreativesActions<'a, A> {
                        pub(super) reqwest: &'a reqwest::Client,
                        pub(super) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> CreativesActions<'a, A> {
                        #[doc = "List all creatives associated with a specific reason for which bids were\nfiltered, with the number of bids filtered for each creative."]
                        pub fn list(
                            &self,
                            filter_set_name: impl Into<String>,
                            creative_status_id: i32,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                filter_set_name: filter_set_name.into(),
                                creative_status_id,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        filter_set_name: String,
                        creative_status_id: i32,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                        pub fn page_size(&mut self, value: i32) -> &mut Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListCreativeStatusBreakdownByCreativeResponse.nextPageToken\nreturned from the previous call to the filteredBids.creatives.list\nmethod."]
                        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn iter_filtered_bid_creative_rows<T>(
                            &'a mut self,
                        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                        {
                            struct ItemIter<'a, M, T> {
                                method: &'a mut M,
                                finished: bool,
                                items_iter: Option<::std::vec::IntoIter<T>>,
                            }
                            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                            where
                                M: crate::IterableMethod,
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                type Item = Result<T, Box<dyn ::std::error::Error>>;
                                fn next(
                                    &mut self,
                                ) -> Option<Result<T, Box<dyn ::std::error::Error>>>
                                {
                                    use ::field_selector::FieldSelector;
                                    #[derive(:: serde :: Deserialize, FieldSelector)]
                                    struct Resp<T>
                                    where
                                        T: FieldSelector,
                                    {
                                        #[serde(rename = "filteredBidCreativeRows")]
                                        items: Option<Vec<T>>,
                                        #[serde(rename = "nextPageToken")]
                                        next_page_token: Option<String>,
                                    }
                                    loop {
                                        if let Some(iter) = self.items_iter.as_mut() {
                                            match iter.next() {
                                                Some(v) => return Some(Ok(v)),
                                                None => {}
                                            }
                                        }
                                        if self.finished {
                                            return None;
                                        }
                                        let resp: Resp<T> = match self.method.execute() {
                                            Ok(r) => r,
                                            Err(err) => return Some(Err(err)),
                                        };
                                        if let Some(next_page_token) = resp.next_page_token {
                                            self.method.set_page_token(next_page_token);
                                        } else {
                                            self.finished = true;
                                        }
                                        self.items_iter = resp.items.map(|i| i.into_iter());
                                    }
                                }
                            }
                            ItemIter {
                                method: self,
                                finished: false,
                                items_iter: None,
                            }
                        }
                        pub fn iter<T>(
                            &'a mut self,
                        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                        {
                            crate::PageIter {
                                method: self,
                                finished: false,
                                _phantom: ::std::default::Default::default(),
                            }
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<
                            crate::schemas::ListCreativeStatusBreakdownByCreativeResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                            output.push_str("v2beta1/");
                            output.push_str(&self.filter_set_name);
                            output.push_str("/filteredBids/");
                            {
                                let str_value = self.creative_status_id.to_string();
                                output.push_str(&str_value);
                            }
                            output.push_str("/creatives");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/adexchange.buyer",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                    }
                }
                pub mod details {
                    pub mod params {}
                    pub struct DetailsActions<'a, A> {
                        pub(super) reqwest: &'a reqwest::Client,
                        pub(super) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> DetailsActions<'a, A> {
                        #[doc = "List all details associated with a specific reason for which bids were\nfiltered, with the number of bids filtered for each detail."]
                        pub fn list(
                            &self,
                            filter_set_name: impl Into<String>,
                            creative_status_id: i32,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                filter_set_name: filter_set_name.into(),
                                creative_status_id,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        filter_set_name: String,
                        creative_status_id: i32,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                        pub fn page_size(&mut self, value: i32) -> &mut Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListCreativeStatusBreakdownByDetailResponse.nextPageToken\nreturned from the previous call to the filteredBids.details.list\nmethod."]
                        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn iter_filtered_bid_detail_rows<T>(
                            &'a mut self,
                        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                        {
                            struct ItemIter<'a, M, T> {
                                method: &'a mut M,
                                finished: bool,
                                items_iter: Option<::std::vec::IntoIter<T>>,
                            }
                            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                            where
                                M: crate::IterableMethod,
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                type Item = Result<T, Box<dyn ::std::error::Error>>;
                                fn next(
                                    &mut self,
                                ) -> Option<Result<T, Box<dyn ::std::error::Error>>>
                                {
                                    use ::field_selector::FieldSelector;
                                    #[derive(:: serde :: Deserialize, FieldSelector)]
                                    struct Resp<T>
                                    where
                                        T: FieldSelector,
                                    {
                                        #[serde(rename = "filteredBidDetailRows")]
                                        items: Option<Vec<T>>,
                                        #[serde(rename = "nextPageToken")]
                                        next_page_token: Option<String>,
                                    }
                                    loop {
                                        if let Some(iter) = self.items_iter.as_mut() {
                                            match iter.next() {
                                                Some(v) => return Some(Ok(v)),
                                                None => {}
                                            }
                                        }
                                        if self.finished {
                                            return None;
                                        }
                                        let resp: Resp<T> = match self.method.execute() {
                                            Ok(r) => r,
                                            Err(err) => return Some(Err(err)),
                                        };
                                        if let Some(next_page_token) = resp.next_page_token {
                                            self.method.set_page_token(next_page_token);
                                        } else {
                                            self.finished = true;
                                        }
                                        self.items_iter = resp.items.map(|i| i.into_iter());
                                    }
                                }
                            }
                            ItemIter {
                                method: self,
                                finished: false,
                                items_iter: None,
                            }
                        }
                        pub fn iter<T>(
                            &'a mut self,
                        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                        {
                            crate::PageIter {
                                method: self,
                                finished: false,
                                _phantom: ::std::default::Default::default(),
                            }
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<
                            crate::schemas::ListCreativeStatusBreakdownByDetailResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                            output.push_str("v2beta1/");
                            output.push_str(&self.filter_set_name);
                            output.push_str("/filteredBids/");
                            {
                                let str_value = self.creative_status_id.to_string();
                                output.push_str(&str_value);
                            }
                            output.push_str("/details");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/adexchange.buyer",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                    }
                }
            }
            pub mod impression_metrics {
                pub mod params {}
                pub struct ImpressionMetricsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> ImpressionMetricsActions<'a, A> {
                    #[doc = "Lists all metrics that are measured in terms of number of impressions."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListImpressionMetricsResponse.nextPageToken\nreturned from the previous call to the impressionMetrics.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_impression_metrics_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "impressionMetricsRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListImpressionMetricsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/impressionMetrics");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod losing_bids {
                pub mod params {}
                pub struct LosingBidsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> LosingBidsActions<'a, A> {
                    #[doc = "List all reasons for which bids lost in the auction, with the number of\nbids that lost for each reason."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListLosingBidsResponse.nextPageToken\nreturned from the previous call to the losingBids.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_creative_status_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "creativeStatusRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListLosingBidsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/losingBids");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod non_billable_winning_bids {
                pub mod params {}
                pub struct NonBillableWinningBidsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> NonBillableWinningBidsActions<'a, A> {
                    #[doc = "List all reasons for which winning bids were not billable, with the number\nof bids not billed for each reason."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListNonBillableWinningBidsResponse.nextPageToken\nreturned from the previous call to the nonBillableWinningBids.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_non_billable_winning_bid_status_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "nonBillableWinningBidStatusRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListNonBillableWinningBidsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/nonBillableWinningBids");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
        }
    }
    pub mod filter_sets {
        pub mod params {}
        pub struct FilterSetsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> FilterSetsActions<'a, A> {
            #[doc = "Creates the specified filter set for the account with the given account ID."]
            pub fn create(
                &self,
                request: crate::schemas::FilterSet,
                owner_name: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    owner_name: owner_name.into(),
                    is_transient: None,
                }
            }
            #[doc = "Deletes the requested filter set from the account with the given account\nID."]
            pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Retrieves the requested filter set for the account with the given account\nID."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Lists all filter sets for the account with the given account ID."]
            pub fn list(&self, owner_name: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    owner_name: owner_name.into(),
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Actions that can be performed on the bid_metrics resource"]
            pub fn bid_metrics(&self) -> bid_metrics::BidMetricsActions<A> {
                bid_metrics::BidMetricsActions
            }
            #[doc = "Actions that can be performed on the bid_response_errors resource"]
            pub fn bid_response_errors(&self) -> bid_response_errors::BidResponseErrorsActions<A> {
                bid_response_errors::BidResponseErrorsActions
            }
            #[doc = "Actions that can be performed on the bid_responses_without_bids resource"]
            pub fn bid_responses_without_bids(
                &self,
            ) -> bid_responses_without_bids::BidResponsesWithoutBidsActions<A> {
                bid_responses_without_bids::BidResponsesWithoutBidsActions
            }
            #[doc = "Actions that can be performed on the filtered_bid_requests resource"]
            pub fn filtered_bid_requests(
                &self,
            ) -> filtered_bid_requests::FilteredBidRequestsActions<A> {
                filtered_bid_requests::FilteredBidRequestsActions
            }
            #[doc = "Actions that can be performed on the filtered_bids resource"]
            pub fn filtered_bids(&self) -> filtered_bids::FilteredBidsActions<A> {
                filtered_bids::FilteredBidsActions
            }
            #[doc = "Actions that can be performed on the impression_metrics resource"]
            pub fn impression_metrics(&self) -> impression_metrics::ImpressionMetricsActions<A> {
                impression_metrics::ImpressionMetricsActions
            }
            #[doc = "Actions that can be performed on the losing_bids resource"]
            pub fn losing_bids(&self) -> losing_bids::LosingBidsActions<A> {
                losing_bids::LosingBidsActions
            }
            #[doc = "Actions that can be performed on the non_billable_winning_bids resource"]
            pub fn non_billable_winning_bids(
                &self,
            ) -> non_billable_winning_bids::NonBillableWinningBidsActions<A> {
                non_billable_winning_bids::NonBillableWinningBidsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::FilterSet,
            owner_name: String,
            is_transient: Option<bool>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "Whether the filter set is transient, or should be persisted indefinitely.\nBy default, filter sets are not transient.\nIf transient, it will be available for at least 1 hour after creation."]
            pub fn is_transient(&mut self, value: bool) -> &mut Self {
                self.is_transient = Some(value);
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FilterSet, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/");
                output.push_str(&self.owner_name);
                output.push_str("/filterSets");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("isTransient", &self.is_transient)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::FilterSet, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            owner_name: String,
            page_size: Option<i32>,
            page_token: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListFilterSetsResponse.nextPageToken\nreturned from the previous call to the\naccounts.filterSets.list\nmethod."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_filter_sets<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "filterSets")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListFilterSetsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                output.push_str("v2beta1/");
                output.push_str(&self.owner_name);
                output.push_str("/filterSets");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        pub mod bid_metrics {
            pub mod params {}
            pub struct BidMetricsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> BidMetricsActions<'a, A> {
                #[doc = "Lists all metrics that are measured in terms of number of bids."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListBidMetricsResponse.nextPageToken\nreturned from the previous call to the bidMetrics.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_bid_metrics_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "bidMetricsRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListBidMetricsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/bidMetrics");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod bid_response_errors {
            pub mod params {}
            pub struct BidResponseErrorsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> BidResponseErrorsActions<'a, A> {
                #[doc = "List all errors that occurred in bid responses, with the number of bid\nresponses affected for each reason."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListBidResponseErrorsResponse.nextPageToken\nreturned from the previous call to the bidResponseErrors.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_callout_status_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "calloutStatusRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListBidResponseErrorsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/bidResponseErrors");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod bid_responses_without_bids {
            pub mod params {}
            pub struct BidResponsesWithoutBidsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> BidResponsesWithoutBidsActions<'a, A> {
                #[doc = "List all reasons for which bid responses were considered to have no\napplicable bids, with the number of bid responses affected for each reason."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListBidResponsesWithoutBidsResponse.nextPageToken\nreturned from the previous call to the bidResponsesWithoutBids.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_bid_response_without_bids_status_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "bidResponseWithoutBidsStatusRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListBidResponsesWithoutBidsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/bidResponsesWithoutBids");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod filtered_bid_requests {
            pub mod params {}
            pub struct FilteredBidRequestsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> FilteredBidRequestsActions<'a, A> {
                #[doc = "List all reasons that caused a bid request not to be sent for an\nimpression, with the number of bid requests not sent for each reason."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListFilteredBidRequestsResponse.nextPageToken\nreturned from the previous call to the filteredBidRequests.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_callout_status_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "calloutStatusRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListFilteredBidRequestsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/filteredBidRequests");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod filtered_bids {
            pub mod params {}
            pub struct FilteredBidsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> FilteredBidsActions<'a, A> {
                #[doc = "List all reasons for which bids were filtered, with the number of bids\nfiltered for each reason."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the creatives resource"]
                pub fn creatives(&self) -> creatives::CreativesActions<A> {
                    creatives::CreativesActions
                }
                #[doc = "Actions that can be performed on the details resource"]
                pub fn details(&self) -> details::DetailsActions<A> {
                    details::DetailsActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListFilteredBidsResponse.nextPageToken\nreturned from the previous call to the filteredBids.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_creative_status_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "creativeStatusRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListFilteredBidsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/filteredBids");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
            pub mod creatives {
                pub mod params {}
                pub struct CreativesActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreativesActions<'a, A> {
                    #[doc = "List all creatives associated with a specific reason for which bids were\nfiltered, with the number of bids filtered for each creative."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                        creative_status_id: i32,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            creative_status_id,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    creative_status_id: i32,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListCreativeStatusBreakdownByCreativeResponse.nextPageToken\nreturned from the previous call to the filteredBids.creatives.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_filtered_bid_creative_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "filteredBidCreativeRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListCreativeStatusBreakdownByCreativeResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/filteredBids/");
                        {
                            let str_value = self.creative_status_id.to_string();
                            output.push_str(&str_value);
                        }
                        output.push_str("/creatives");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod details {
                pub mod params {}
                pub struct DetailsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> DetailsActions<'a, A> {
                    #[doc = "List all details associated with a specific reason for which bids were\nfiltered, with the number of bids filtered for each detail."]
                    pub fn list(
                        &self,
                        filter_set_name: impl Into<String>,
                        creative_status_id: i32,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            filter_set_name: filter_set_name.into(),
                            creative_status_id,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    filter_set_name: String,
                    creative_status_id: i32,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListCreativeStatusBreakdownByDetailResponse.nextPageToken\nreturned from the previous call to the filteredBids.details.list\nmethod."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_filtered_bid_detail_rows<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "filteredBidDetailRows")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListCreativeStatusBreakdownByDetailResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                        output.push_str("v2beta1/");
                        output.push_str(&self.filter_set_name);
                        output.push_str("/filteredBids/");
                        {
                            let str_value = self.creative_status_id.to_string();
                            output.push_str(&str_value);
                        }
                        output.push_str("/details");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/adexchange.buyer",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
        }
        pub mod impression_metrics {
            pub mod params {}
            pub struct ImpressionMetricsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ImpressionMetricsActions<'a, A> {
                #[doc = "Lists all metrics that are measured in terms of number of impressions."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListImpressionMetricsResponse.nextPageToken\nreturned from the previous call to the impressionMetrics.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_impression_metrics_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "impressionMetricsRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListImpressionMetricsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/impressionMetrics");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod losing_bids {
            pub mod params {}
            pub struct LosingBidsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> LosingBidsActions<'a, A> {
                #[doc = "List all reasons for which bids lost in the auction, with the number of\nbids that lost for each reason."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListLosingBidsResponse.nextPageToken\nreturned from the previous call to the losingBids.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_creative_status_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "creativeStatusRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListLosingBidsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/losingBids");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod non_billable_winning_bids {
            pub mod params {}
            pub struct NonBillableWinningBidsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> NonBillableWinningBidsActions<'a, A> {
                #[doc = "List all reasons for which winning bids were not billable, with the number\nof bids not billed for each reason."]
                pub fn list(&self, filter_set_name: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        filter_set_name: filter_set_name.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                filter_set_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Requested page size. The server may return fewer results than requested.\nIf unspecified, the server will pick an appropriate default."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return.\nTypically, this is the value of\nListNonBillableWinningBidsResponse.nextPageToken\nreturned from the previous call to the nonBillableWinningBids.list\nmethod."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_non_billable_winning_bid_status_rows<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "nonBillableWinningBidStatusRows")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::ListNonBillableWinningBidsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adexchangebuyer.googleapis.com/".to_owned();
                    output.push_str("v2beta1/");
                    output.push_str(&self.filter_set_name);
                    output.push_str("/nonBillableWinningBids");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/adexchange.buyer",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
    }
}
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}