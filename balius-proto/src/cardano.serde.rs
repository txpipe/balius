// @generated
impl serde::Serialize for AddressPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exact_address.is_empty() {
            len += 1;
        }
        if !self.payment_part.is_empty() {
            len += 1;
        }
        if !self.delegation_part.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.AddressPattern", len)?;
        if !self.exact_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("exactAddress", pbjson::private::base64::encode(&self.exact_address).as_str())?;
        }
        if !self.payment_part.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("paymentPart", pbjson::private::base64::encode(&self.payment_part).as_str())?;
        }
        if !self.delegation_part.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("delegationPart", pbjson::private::base64::encode(&self.delegation_part).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exact_address",
            "exactAddress",
            "payment_part",
            "paymentPart",
            "delegation_part",
            "delegationPart",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExactAddress,
            PaymentPart,
            DelegationPart,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exactAddress" | "exact_address" => Ok(GeneratedField::ExactAddress),
                            "paymentPart" | "payment_part" => Ok(GeneratedField::PaymentPart),
                            "delegationPart" | "delegation_part" => Ok(GeneratedField::DelegationPart),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.AddressPattern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddressPattern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exact_address__ = None;
                let mut payment_part__ = None;
                let mut delegation_part__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExactAddress => {
                            if exact_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactAddress"));
                            }
                            exact_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PaymentPart => {
                            if payment_part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentPart"));
                            }
                            payment_part__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DelegationPart => {
                            if delegation_part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delegationPart"));
                            }
                            delegation_part__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AddressPattern {
                    exact_address: exact_address__.unwrap_or_default(),
                    payment_part: payment_part__.unwrap_or_default(),
                    delegation_part: delegation_part__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.AddressPattern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Anchor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.content_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Anchor", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.content_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("contentHash", pbjson::private::base64::encode(&self.content_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Anchor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "content_hash",
            "contentHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            ContentHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "contentHash" | "content_hash" => Ok(GeneratedField::ContentHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Anchor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Anchor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Anchor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut content_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContentHash => {
                            if content_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentHash"));
                            }
                            content_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Anchor {
                    url: url__.unwrap_or_default(),
                    content_hash: content_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Anchor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Asset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.output_coin != 0 {
            len += 1;
        }
        if self.mint_coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Asset", len)?;
        if !self.name.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("name", pbjson::private::base64::encode(&self.name).as_str())?;
        }
        if self.output_coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("outputCoin", ToString::to_string(&self.output_coin).as_str())?;
        }
        if self.mint_coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("mintCoin", ToString::to_string(&self.mint_coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Asset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "output_coin",
            "outputCoin",
            "mint_coin",
            "mintCoin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            OutputCoin,
            MintCoin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "outputCoin" | "output_coin" => Ok(GeneratedField::OutputCoin),
                            "mintCoin" | "mint_coin" => Ok(GeneratedField::MintCoin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Asset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Asset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Asset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut output_coin__ = None;
                let mut mint_coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OutputCoin => {
                            if output_coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputCoin"));
                            }
                            output_coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MintCoin => {
                            if mint_coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintCoin"));
                            }
                            mint_coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Asset {
                    name: name__.unwrap_or_default(),
                    output_coin: output_coin__.unwrap_or_default(),
                    mint_coin: mint_coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Asset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policy_id.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.AssetPattern", len)?;
        if !self.policy_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("policyId", pbjson::private::base64::encode(&self.policy_id).as_str())?;
        }
        if !self.asset_name.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("assetName", pbjson::private::base64::encode(&self.asset_name).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_id",
            "policyId",
            "asset_name",
            "assetName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyId,
            AssetName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "policyId" | "policy_id" => Ok(GeneratedField::PolicyId),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.AssetPattern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetPattern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_id__ = None;
                let mut asset_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PolicyId => {
                            if policy_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyId"));
                            }
                            policy_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AssetPattern {
                    policy_id: policy_id__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.AssetPattern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthCommitteeHotCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.committee_cold_credential.is_some() {
            len += 1;
        }
        if self.committee_hot_credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.AuthCommitteeHotCert", len)?;
        if let Some(v) = self.committee_cold_credential.as_ref() {
            struct_ser.serialize_field("committeeColdCredential", v)?;
        }
        if let Some(v) = self.committee_hot_credential.as_ref() {
            struct_ser.serialize_field("committeeHotCredential", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthCommitteeHotCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "committee_cold_credential",
            "committeeColdCredential",
            "committee_hot_credential",
            "committeeHotCredential",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommitteeColdCredential,
            CommitteeHotCredential,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "committeeColdCredential" | "committee_cold_credential" => Ok(GeneratedField::CommitteeColdCredential),
                            "committeeHotCredential" | "committee_hot_credential" => Ok(GeneratedField::CommitteeHotCredential),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthCommitteeHotCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.AuthCommitteeHotCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthCommitteeHotCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut committee_cold_credential__ = None;
                let mut committee_hot_credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CommitteeColdCredential => {
                            if committee_cold_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeColdCredential"));
                            }
                            committee_cold_credential__ = map_.next_value()?;
                        }
                        GeneratedField::CommitteeHotCredential => {
                            if committee_hot_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeHotCredential"));
                            }
                            committee_hot_credential__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AuthCommitteeHotCert {
                    committee_cold_credential: committee_cold_credential__,
                    committee_hot_credential: committee_hot_credential__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.AuthCommitteeHotCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuxData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.scripts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.AuxData", len)?;
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.scripts.is_empty() {
            struct_ser.serialize_field("scripts", &self.scripts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuxData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "scripts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Scripts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "scripts" => Ok(GeneratedField::Scripts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuxData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.AuxData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuxData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut scripts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scripts => {
                            if scripts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scripts"));
                            }
                            scripts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuxData {
                    metadata: metadata__.unwrap_or_default(),
                    scripts: scripts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.AuxData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigInt {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.big_int.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.BigInt", len)?;
        if let Some(v) = self.big_int.as_ref() {
            match v {
                big_int::BigInt::Int(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("int", ToString::to_string(&v).as_str())?;
                }
                big_int::BigInt::BigUInt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("bigUInt", pbjson::private::base64::encode(&v).as_str())?;
                }
                big_int::BigInt::BigNInt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("bigNInt", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigInt {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "int",
            "big_u_int",
            "bigUInt",
            "big_n_int",
            "bigNInt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Int,
            BigUInt,
            BigNInt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "int" => Ok(GeneratedField::Int),
                            "bigUInt" | "big_u_int" => Ok(GeneratedField::BigUInt),
                            "bigNInt" | "big_n_int" => Ok(GeneratedField::BigNInt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigInt;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.BigInt")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigInt, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut big_int__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Int => {
                            if big_int__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int"));
                            }
                            big_int__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| big_int::BigInt::Int(x.0));
                        }
                        GeneratedField::BigUInt => {
                            if big_int__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigUInt"));
                            }
                            big_int__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| big_int::BigInt::BigUInt(x.0));
                        }
                        GeneratedField::BigNInt => {
                            if big_int__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigNInt"));
                            }
                            big_int__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| big_int::BigInt::BigNInt(x.0));
                        }
                    }
                }
                Ok(BigInt {
                    big_int: big_int__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.BigInt", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Block {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        if self.body.is_some() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Block", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Block {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "body",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Body,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            "body" => Ok(GeneratedField::Body),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Block")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Block, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut body__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Block {
                    header: header__,
                    body: body__,
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Block", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockBody {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.BlockBody", len)?;
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockBody {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tx,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tx" => Ok(GeneratedField::Tx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockBody;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.BlockBody")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockBody, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BlockBody {
                    tx: tx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.BlockBody", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slot != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.BlockHeader", len)?;
        if self.slot != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("slot", ToString::to_string(&self.slot).as_str())?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slot",
            "hash",
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Slot,
            Hash,
            Height,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "slot" => Ok(GeneratedField::Slot),
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.BlockHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slot__ = None;
                let mut hash__ = None;
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Slot => {
                            if slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slot"));
                            }
                            slot__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BlockHeader {
                    slot: slot__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.BlockHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockVersionData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.script_version != 0 {
            len += 1;
        }
        if !self.slot_duration.is_empty() {
            len += 1;
        }
        if !self.max_block_size.is_empty() {
            len += 1;
        }
        if !self.max_header_size.is_empty() {
            len += 1;
        }
        if !self.max_tx_size.is_empty() {
            len += 1;
        }
        if !self.max_proposal_size.is_empty() {
            len += 1;
        }
        if !self.mpc_thd.is_empty() {
            len += 1;
        }
        if !self.heavy_del_thd.is_empty() {
            len += 1;
        }
        if !self.update_vote_thd.is_empty() {
            len += 1;
        }
        if !self.update_proposal_thd.is_empty() {
            len += 1;
        }
        if !self.update_implicit.is_empty() {
            len += 1;
        }
        if self.softfork_rule.is_some() {
            len += 1;
        }
        if self.tx_fee_policy.is_some() {
            len += 1;
        }
        if !self.unlock_stake_epoch.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.BlockVersionData", len)?;
        if self.script_version != 0 {
            struct_ser.serialize_field("scriptVersion", &self.script_version)?;
        }
        if !self.slot_duration.is_empty() {
            struct_ser.serialize_field("slotDuration", &self.slot_duration)?;
        }
        if !self.max_block_size.is_empty() {
            struct_ser.serialize_field("maxBlockSize", &self.max_block_size)?;
        }
        if !self.max_header_size.is_empty() {
            struct_ser.serialize_field("maxHeaderSize", &self.max_header_size)?;
        }
        if !self.max_tx_size.is_empty() {
            struct_ser.serialize_field("maxTxSize", &self.max_tx_size)?;
        }
        if !self.max_proposal_size.is_empty() {
            struct_ser.serialize_field("maxProposalSize", &self.max_proposal_size)?;
        }
        if !self.mpc_thd.is_empty() {
            struct_ser.serialize_field("mpcThd", &self.mpc_thd)?;
        }
        if !self.heavy_del_thd.is_empty() {
            struct_ser.serialize_field("heavyDelThd", &self.heavy_del_thd)?;
        }
        if !self.update_vote_thd.is_empty() {
            struct_ser.serialize_field("updateVoteThd", &self.update_vote_thd)?;
        }
        if !self.update_proposal_thd.is_empty() {
            struct_ser.serialize_field("updateProposalThd", &self.update_proposal_thd)?;
        }
        if !self.update_implicit.is_empty() {
            struct_ser.serialize_field("updateImplicit", &self.update_implicit)?;
        }
        if let Some(v) = self.softfork_rule.as_ref() {
            struct_ser.serialize_field("softforkRule", v)?;
        }
        if let Some(v) = self.tx_fee_policy.as_ref() {
            struct_ser.serialize_field("txFeePolicy", v)?;
        }
        if !self.unlock_stake_epoch.is_empty() {
            struct_ser.serialize_field("unlockStakeEpoch", &self.unlock_stake_epoch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockVersionData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "script_version",
            "scriptVersion",
            "slot_duration",
            "slotDuration",
            "max_block_size",
            "maxBlockSize",
            "max_header_size",
            "maxHeaderSize",
            "max_tx_size",
            "maxTxSize",
            "max_proposal_size",
            "maxProposalSize",
            "mpc_thd",
            "mpcThd",
            "heavy_del_thd",
            "heavyDelThd",
            "update_vote_thd",
            "updateVoteThd",
            "update_proposal_thd",
            "updateProposalThd",
            "update_implicit",
            "updateImplicit",
            "softfork_rule",
            "softforkRule",
            "tx_fee_policy",
            "txFeePolicy",
            "unlock_stake_epoch",
            "unlockStakeEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScriptVersion,
            SlotDuration,
            MaxBlockSize,
            MaxHeaderSize,
            MaxTxSize,
            MaxProposalSize,
            MpcThd,
            HeavyDelThd,
            UpdateVoteThd,
            UpdateProposalThd,
            UpdateImplicit,
            SoftforkRule,
            TxFeePolicy,
            UnlockStakeEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "scriptVersion" | "script_version" => Ok(GeneratedField::ScriptVersion),
                            "slotDuration" | "slot_duration" => Ok(GeneratedField::SlotDuration),
                            "maxBlockSize" | "max_block_size" => Ok(GeneratedField::MaxBlockSize),
                            "maxHeaderSize" | "max_header_size" => Ok(GeneratedField::MaxHeaderSize),
                            "maxTxSize" | "max_tx_size" => Ok(GeneratedField::MaxTxSize),
                            "maxProposalSize" | "max_proposal_size" => Ok(GeneratedField::MaxProposalSize),
                            "mpcThd" | "mpc_thd" => Ok(GeneratedField::MpcThd),
                            "heavyDelThd" | "heavy_del_thd" => Ok(GeneratedField::HeavyDelThd),
                            "updateVoteThd" | "update_vote_thd" => Ok(GeneratedField::UpdateVoteThd),
                            "updateProposalThd" | "update_proposal_thd" => Ok(GeneratedField::UpdateProposalThd),
                            "updateImplicit" | "update_implicit" => Ok(GeneratedField::UpdateImplicit),
                            "softforkRule" | "softfork_rule" => Ok(GeneratedField::SoftforkRule),
                            "txFeePolicy" | "tx_fee_policy" => Ok(GeneratedField::TxFeePolicy),
                            "unlockStakeEpoch" | "unlock_stake_epoch" => Ok(GeneratedField::UnlockStakeEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockVersionData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.BlockVersionData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockVersionData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut script_version__ = None;
                let mut slot_duration__ = None;
                let mut max_block_size__ = None;
                let mut max_header_size__ = None;
                let mut max_tx_size__ = None;
                let mut max_proposal_size__ = None;
                let mut mpc_thd__ = None;
                let mut heavy_del_thd__ = None;
                let mut update_vote_thd__ = None;
                let mut update_proposal_thd__ = None;
                let mut update_implicit__ = None;
                let mut softfork_rule__ = None;
                let mut tx_fee_policy__ = None;
                let mut unlock_stake_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScriptVersion => {
                            if script_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptVersion"));
                            }
                            script_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SlotDuration => {
                            if slot_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotDuration"));
                            }
                            slot_duration__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxBlockSize => {
                            if max_block_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBlockSize"));
                            }
                            max_block_size__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxHeaderSize => {
                            if max_header_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxHeaderSize"));
                            }
                            max_header_size__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxTxSize => {
                            if max_tx_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTxSize"));
                            }
                            max_tx_size__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxProposalSize => {
                            if max_proposal_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxProposalSize"));
                            }
                            max_proposal_size__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MpcThd => {
                            if mpc_thd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mpcThd"));
                            }
                            mpc_thd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeavyDelThd => {
                            if heavy_del_thd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heavyDelThd"));
                            }
                            heavy_del_thd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateVoteThd => {
                            if update_vote_thd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateVoteThd"));
                            }
                            update_vote_thd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateProposalThd => {
                            if update_proposal_thd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateProposalThd"));
                            }
                            update_proposal_thd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateImplicit => {
                            if update_implicit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateImplicit"));
                            }
                            update_implicit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SoftforkRule => {
                            if softfork_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("softforkRule"));
                            }
                            softfork_rule__ = map_.next_value()?;
                        }
                        GeneratedField::TxFeePolicy => {
                            if tx_fee_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txFeePolicy"));
                            }
                            tx_fee_policy__ = map_.next_value()?;
                        }
                        GeneratedField::UnlockStakeEpoch => {
                            if unlock_stake_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unlockStakeEpoch"));
                            }
                            unlock_stake_epoch__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BlockVersionData {
                    script_version: script_version__.unwrap_or_default(),
                    slot_duration: slot_duration__.unwrap_or_default(),
                    max_block_size: max_block_size__.unwrap_or_default(),
                    max_header_size: max_header_size__.unwrap_or_default(),
                    max_tx_size: max_tx_size__.unwrap_or_default(),
                    max_proposal_size: max_proposal_size__.unwrap_or_default(),
                    mpc_thd: mpc_thd__.unwrap_or_default(),
                    heavy_del_thd: heavy_del_thd__.unwrap_or_default(),
                    update_vote_thd: update_vote_thd__.unwrap_or_default(),
                    update_proposal_thd: update_proposal_thd__.unwrap_or_default(),
                    update_implicit: update_implicit__.unwrap_or_default(),
                    softfork_rule: softfork_rule__,
                    tx_fee_policy: tx_fee_policy__,
                    unlock_stake_epoch: unlock_stake_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.BlockVersionData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Certificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.redeemer.is_some() {
            len += 1;
        }
        if self.certificate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Certificate", len)?;
        if let Some(v) = self.redeemer.as_ref() {
            struct_ser.serialize_field("redeemer", v)?;
        }
        if let Some(v) = self.certificate.as_ref() {
            match v {
                certificate::Certificate::StakeRegistration(v) => {
                    struct_ser.serialize_field("stakeRegistration", v)?;
                }
                certificate::Certificate::StakeDeregistration(v) => {
                    struct_ser.serialize_field("stakeDeregistration", v)?;
                }
                certificate::Certificate::StakeDelegation(v) => {
                    struct_ser.serialize_field("stakeDelegation", v)?;
                }
                certificate::Certificate::PoolRegistration(v) => {
                    struct_ser.serialize_field("poolRegistration", v)?;
                }
                certificate::Certificate::PoolRetirement(v) => {
                    struct_ser.serialize_field("poolRetirement", v)?;
                }
                certificate::Certificate::GenesisKeyDelegation(v) => {
                    struct_ser.serialize_field("genesisKeyDelegation", v)?;
                }
                certificate::Certificate::MirCert(v) => {
                    struct_ser.serialize_field("mirCert", v)?;
                }
                certificate::Certificate::RegCert(v) => {
                    struct_ser.serialize_field("regCert", v)?;
                }
                certificate::Certificate::UnregCert(v) => {
                    struct_ser.serialize_field("unregCert", v)?;
                }
                certificate::Certificate::VoteDelegCert(v) => {
                    struct_ser.serialize_field("voteDelegCert", v)?;
                }
                certificate::Certificate::StakeVoteDelegCert(v) => {
                    struct_ser.serialize_field("stakeVoteDelegCert", v)?;
                }
                certificate::Certificate::StakeRegDelegCert(v) => {
                    struct_ser.serialize_field("stakeRegDelegCert", v)?;
                }
                certificate::Certificate::VoteRegDelegCert(v) => {
                    struct_ser.serialize_field("voteRegDelegCert", v)?;
                }
                certificate::Certificate::StakeVoteRegDelegCert(v) => {
                    struct_ser.serialize_field("stakeVoteRegDelegCert", v)?;
                }
                certificate::Certificate::AuthCommitteeHotCert(v) => {
                    struct_ser.serialize_field("authCommitteeHotCert", v)?;
                }
                certificate::Certificate::ResignCommitteeColdCert(v) => {
                    struct_ser.serialize_field("resignCommitteeColdCert", v)?;
                }
                certificate::Certificate::RegDrepCert(v) => {
                    struct_ser.serialize_field("regDrepCert", v)?;
                }
                certificate::Certificate::UnregDrepCert(v) => {
                    struct_ser.serialize_field("unregDrepCert", v)?;
                }
                certificate::Certificate::UpdateDrepCert(v) => {
                    struct_ser.serialize_field("updateDrepCert", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Certificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "redeemer",
            "stake_registration",
            "stakeRegistration",
            "stake_deregistration",
            "stakeDeregistration",
            "stake_delegation",
            "stakeDelegation",
            "pool_registration",
            "poolRegistration",
            "pool_retirement",
            "poolRetirement",
            "genesis_key_delegation",
            "genesisKeyDelegation",
            "mir_cert",
            "mirCert",
            "reg_cert",
            "regCert",
            "unreg_cert",
            "unregCert",
            "vote_deleg_cert",
            "voteDelegCert",
            "stake_vote_deleg_cert",
            "stakeVoteDelegCert",
            "stake_reg_deleg_cert",
            "stakeRegDelegCert",
            "vote_reg_deleg_cert",
            "voteRegDelegCert",
            "stake_vote_reg_deleg_cert",
            "stakeVoteRegDelegCert",
            "auth_committee_hot_cert",
            "authCommitteeHotCert",
            "resign_committee_cold_cert",
            "resignCommitteeColdCert",
            "reg_drep_cert",
            "regDrepCert",
            "unreg_drep_cert",
            "unregDrepCert",
            "update_drep_cert",
            "updateDrepCert",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Redeemer,
            StakeRegistration,
            StakeDeregistration,
            StakeDelegation,
            PoolRegistration,
            PoolRetirement,
            GenesisKeyDelegation,
            MirCert,
            RegCert,
            UnregCert,
            VoteDelegCert,
            StakeVoteDelegCert,
            StakeRegDelegCert,
            VoteRegDelegCert,
            StakeVoteRegDelegCert,
            AuthCommitteeHotCert,
            ResignCommitteeColdCert,
            RegDrepCert,
            UnregDrepCert,
            UpdateDrepCert,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "redeemer" => Ok(GeneratedField::Redeemer),
                            "stakeRegistration" | "stake_registration" => Ok(GeneratedField::StakeRegistration),
                            "stakeDeregistration" | "stake_deregistration" => Ok(GeneratedField::StakeDeregistration),
                            "stakeDelegation" | "stake_delegation" => Ok(GeneratedField::StakeDelegation),
                            "poolRegistration" | "pool_registration" => Ok(GeneratedField::PoolRegistration),
                            "poolRetirement" | "pool_retirement" => Ok(GeneratedField::PoolRetirement),
                            "genesisKeyDelegation" | "genesis_key_delegation" => Ok(GeneratedField::GenesisKeyDelegation),
                            "mirCert" | "mir_cert" => Ok(GeneratedField::MirCert),
                            "regCert" | "reg_cert" => Ok(GeneratedField::RegCert),
                            "unregCert" | "unreg_cert" => Ok(GeneratedField::UnregCert),
                            "voteDelegCert" | "vote_deleg_cert" => Ok(GeneratedField::VoteDelegCert),
                            "stakeVoteDelegCert" | "stake_vote_deleg_cert" => Ok(GeneratedField::StakeVoteDelegCert),
                            "stakeRegDelegCert" | "stake_reg_deleg_cert" => Ok(GeneratedField::StakeRegDelegCert),
                            "voteRegDelegCert" | "vote_reg_deleg_cert" => Ok(GeneratedField::VoteRegDelegCert),
                            "stakeVoteRegDelegCert" | "stake_vote_reg_deleg_cert" => Ok(GeneratedField::StakeVoteRegDelegCert),
                            "authCommitteeHotCert" | "auth_committee_hot_cert" => Ok(GeneratedField::AuthCommitteeHotCert),
                            "resignCommitteeColdCert" | "resign_committee_cold_cert" => Ok(GeneratedField::ResignCommitteeColdCert),
                            "regDrepCert" | "reg_drep_cert" => Ok(GeneratedField::RegDrepCert),
                            "unregDrepCert" | "unreg_drep_cert" => Ok(GeneratedField::UnregDrepCert),
                            "updateDrepCert" | "update_drep_cert" => Ok(GeneratedField::UpdateDrepCert),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Certificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Certificate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Certificate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut redeemer__ = None;
                let mut certificate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Redeemer => {
                            if redeemer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemer"));
                            }
                            redeemer__ = map_.next_value()?;
                        }
                        GeneratedField::StakeRegistration => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeRegistration"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::StakeRegistration)
;
                        }
                        GeneratedField::StakeDeregistration => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeDeregistration"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::StakeDeregistration)
;
                        }
                        GeneratedField::StakeDelegation => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeDelegation"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::StakeDelegation)
;
                        }
                        GeneratedField::PoolRegistration => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolRegistration"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::PoolRegistration)
;
                        }
                        GeneratedField::PoolRetirement => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolRetirement"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::PoolRetirement)
;
                        }
                        GeneratedField::GenesisKeyDelegation => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesisKeyDelegation"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::GenesisKeyDelegation)
;
                        }
                        GeneratedField::MirCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mirCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::MirCert)
;
                        }
                        GeneratedField::RegCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::RegCert)
;
                        }
                        GeneratedField::UnregCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unregCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::UnregCert)
;
                        }
                        GeneratedField::VoteDelegCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voteDelegCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::VoteDelegCert)
;
                        }
                        GeneratedField::StakeVoteDelegCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeVoteDelegCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::StakeVoteDelegCert)
;
                        }
                        GeneratedField::StakeRegDelegCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeRegDelegCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::StakeRegDelegCert)
;
                        }
                        GeneratedField::VoteRegDelegCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voteRegDelegCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::VoteRegDelegCert)
;
                        }
                        GeneratedField::StakeVoteRegDelegCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeVoteRegDelegCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::StakeVoteRegDelegCert)
;
                        }
                        GeneratedField::AuthCommitteeHotCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authCommitteeHotCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::AuthCommitteeHotCert)
;
                        }
                        GeneratedField::ResignCommitteeColdCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resignCommitteeColdCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::ResignCommitteeColdCert)
;
                        }
                        GeneratedField::RegDrepCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regDrepCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::RegDrepCert)
;
                        }
                        GeneratedField::UnregDrepCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unregDrepCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::UnregDrepCert)
;
                        }
                        GeneratedField::UpdateDrepCert => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDrepCert"));
                            }
                            certificate__ = map_.next_value::<::std::option::Option<_>>()?.map(certificate::Certificate::UpdateDrepCert)
;
                        }
                    }
                }
                Ok(Certificate {
                    redeemer: redeemer__,
                    certificate: certificate__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Certificate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Collateral {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collateral.is_empty() {
            len += 1;
        }
        if self.collateral_return.is_some() {
            len += 1;
        }
        if self.total_collateral != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Collateral", len)?;
        if !self.collateral.is_empty() {
            struct_ser.serialize_field("collateral", &self.collateral)?;
        }
        if let Some(v) = self.collateral_return.as_ref() {
            struct_ser.serialize_field("collateralReturn", v)?;
        }
        if self.total_collateral != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("totalCollateral", ToString::to_string(&self.total_collateral).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Collateral {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collateral",
            "collateral_return",
            "collateralReturn",
            "total_collateral",
            "totalCollateral",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Collateral,
            CollateralReturn,
            TotalCollateral,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "collateral" => Ok(GeneratedField::Collateral),
                            "collateralReturn" | "collateral_return" => Ok(GeneratedField::CollateralReturn),
                            "totalCollateral" | "total_collateral" => Ok(GeneratedField::TotalCollateral),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Collateral;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Collateral")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Collateral, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collateral__ = None;
                let mut collateral_return__ = None;
                let mut total_collateral__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Collateral => {
                            if collateral__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateral"));
                            }
                            collateral__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollateralReturn => {
                            if collateral_return__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralReturn"));
                            }
                            collateral_return__ = map_.next_value()?;
                        }
                        GeneratedField::TotalCollateral => {
                            if total_collateral__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCollateral"));
                            }
                            total_collateral__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Collateral {
                    collateral: collateral__.unwrap_or_default(),
                    collateral_return: collateral_return__,
                    total_collateral: total_collateral__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Collateral", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Committee {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        if self.threshold.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Committee", len)?;
        if !self.members.is_empty() {
            let v: std::collections::HashMap<_, _> = self.members.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("members", &v)?;
        }
        if let Some(v) = self.threshold.as_ref() {
            struct_ser.serialize_field("threshold", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Committee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "threshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            Threshold,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            "threshold" => Ok(GeneratedField::Threshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Committee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Committee")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Committee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<u64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Committee {
                    members: members__.unwrap_or_default(),
                    threshold: threshold__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Committee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Constitution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.anchor.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Constitution", len)?;
        if let Some(v) = self.anchor.as_ref() {
            struct_ser.serialize_field("anchor", v)?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Constitution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "anchor",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Anchor,
            Hash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "anchor" => Ok(GeneratedField::Anchor),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Constitution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Constitution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Constitution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut anchor__ = None;
                let mut hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Anchor => {
                            if anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            anchor__ = map_.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Constitution {
                    anchor: anchor__,
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Constitution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Constr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tag != 0 {
            len += 1;
        }
        if self.any_constructor != 0 {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Constr", len)?;
        if self.tag != 0 {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if self.any_constructor != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("anyConstructor", ToString::to_string(&self.any_constructor).as_str())?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Constr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
            "any_constructor",
            "anyConstructor",
            "fields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
            AnyConstructor,
            Fields,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tag" => Ok(GeneratedField::Tag),
                            "anyConstructor" | "any_constructor" => Ok(GeneratedField::AnyConstructor),
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Constr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Constr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Constr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                let mut any_constructor__ = None;
                let mut fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AnyConstructor => {
                            if any_constructor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anyConstructor"));
                            }
                            any_constructor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Constr {
                    tag: tag__.unwrap_or_default(),
                    any_constructor: any_constructor__.unwrap_or_default(),
                    fields: fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Constr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CostModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.CostModel", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CostModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CostModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.CostModel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CostModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(CostModel {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.CostModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CostModelMap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plutus_v1.is_some() {
            len += 1;
        }
        if self.plutus_v2.is_some() {
            len += 1;
        }
        if self.plutus_v3.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.CostModelMap", len)?;
        if let Some(v) = self.plutus_v1.as_ref() {
            struct_ser.serialize_field("plutusV1", v)?;
        }
        if let Some(v) = self.plutus_v2.as_ref() {
            struct_ser.serialize_field("plutusV2", v)?;
        }
        if let Some(v) = self.plutus_v3.as_ref() {
            struct_ser.serialize_field("plutusV3", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CostModelMap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plutus_v1",
            "plutusV1",
            "plutus_v2",
            "plutusV2",
            "plutus_v3",
            "plutusV3",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlutusV1,
            PlutusV2,
            PlutusV3,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "plutusV1" | "plutus_v1" => Ok(GeneratedField::PlutusV1),
                            "plutusV2" | "plutus_v2" => Ok(GeneratedField::PlutusV2),
                            "plutusV3" | "plutus_v3" => Ok(GeneratedField::PlutusV3),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CostModelMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.CostModelMap")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CostModelMap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plutus_v1__ = None;
                let mut plutus_v2__ = None;
                let mut plutus_v3__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlutusV1 => {
                            if plutus_v1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV1"));
                            }
                            plutus_v1__ = map_.next_value()?;
                        }
                        GeneratedField::PlutusV2 => {
                            if plutus_v2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV2"));
                            }
                            plutus_v2__ = map_.next_value()?;
                        }
                        GeneratedField::PlutusV3 => {
                            if plutus_v3__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV3"));
                            }
                            plutus_v3__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CostModelMap {
                    plutus_v1: plutus_v1__,
                    plutus_v2: plutus_v2__,
                    plutus_v3: plutus_v3__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.CostModelMap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CostModels {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plutus_v1.is_some() {
            len += 1;
        }
        if self.plutus_v2.is_some() {
            len += 1;
        }
        if self.plutus_v3.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.CostModels", len)?;
        if let Some(v) = self.plutus_v1.as_ref() {
            struct_ser.serialize_field("plutusV1", v)?;
        }
        if let Some(v) = self.plutus_v2.as_ref() {
            struct_ser.serialize_field("plutusV2", v)?;
        }
        if let Some(v) = self.plutus_v3.as_ref() {
            struct_ser.serialize_field("plutusV3", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CostModels {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plutus_v1",
            "plutusV1",
            "plutus_v2",
            "plutusV2",
            "plutus_v3",
            "plutusV3",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlutusV1,
            PlutusV2,
            PlutusV3,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "plutusV1" | "plutus_v1" => Ok(GeneratedField::PlutusV1),
                            "plutusV2" | "plutus_v2" => Ok(GeneratedField::PlutusV2),
                            "plutusV3" | "plutus_v3" => Ok(GeneratedField::PlutusV3),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CostModels;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.CostModels")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CostModels, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plutus_v1__ = None;
                let mut plutus_v2__ = None;
                let mut plutus_v3__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlutusV1 => {
                            if plutus_v1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV1"));
                            }
                            plutus_v1__ = map_.next_value()?;
                        }
                        GeneratedField::PlutusV2 => {
                            if plutus_v2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV2"));
                            }
                            plutus_v2__ = map_.next_value()?;
                        }
                        GeneratedField::PlutusV3 => {
                            if plutus_v3__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV3"));
                            }
                            plutus_v3__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CostModels {
                    plutus_v1: plutus_v1__,
                    plutus_v2: plutus_v2__,
                    plutus_v3: plutus_v3__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.CostModels", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DRep {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.drep.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.DRep", len)?;
        if let Some(v) = self.drep.as_ref() {
            match v {
                d_rep::Drep::AddrKeyHash(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("addrKeyHash", pbjson::private::base64::encode(&v).as_str())?;
                }
                d_rep::Drep::ScriptHash(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("scriptHash", pbjson::private::base64::encode(&v).as_str())?;
                }
                d_rep::Drep::Abstain(v) => {
                    struct_ser.serialize_field("abstain", v)?;
                }
                d_rep::Drep::NoConfidence(v) => {
                    struct_ser.serialize_field("noConfidence", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DRep {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "addr_key_hash",
            "addrKeyHash",
            "script_hash",
            "scriptHash",
            "abstain",
            "no_confidence",
            "noConfidence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddrKeyHash,
            ScriptHash,
            Abstain,
            NoConfidence,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "addrKeyHash" | "addr_key_hash" => Ok(GeneratedField::AddrKeyHash),
                            "scriptHash" | "script_hash" => Ok(GeneratedField::ScriptHash),
                            "abstain" => Ok(GeneratedField::Abstain),
                            "noConfidence" | "no_confidence" => Ok(GeneratedField::NoConfidence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DRep;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.DRep")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DRep, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drep__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AddrKeyHash => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addrKeyHash"));
                            }
                            drep__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| d_rep::Drep::AddrKeyHash(x.0));
                        }
                        GeneratedField::ScriptHash => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptHash"));
                            }
                            drep__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| d_rep::Drep::ScriptHash(x.0));
                        }
                        GeneratedField::Abstain => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abstain"));
                            }
                            drep__ = map_.next_value::<::std::option::Option<_>>()?.map(d_rep::Drep::Abstain);
                        }
                        GeneratedField::NoConfidence => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noConfidence"));
                            }
                            drep__ = map_.next_value::<::std::option::Option<_>>()?.map(d_rep::Drep::NoConfidence);
                        }
                    }
                }
                Ok(DRep {
                    drep: drep__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.DRep", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DRepVotingThresholds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.motion_no_confidence.is_some() {
            len += 1;
        }
        if self.committee_normal.is_some() {
            len += 1;
        }
        if self.committee_no_confidence.is_some() {
            len += 1;
        }
        if self.update_to_constitution.is_some() {
            len += 1;
        }
        if self.hard_fork_initiation.is_some() {
            len += 1;
        }
        if self.pp_network_group.is_some() {
            len += 1;
        }
        if self.pp_economic_group.is_some() {
            len += 1;
        }
        if self.pp_technical_group.is_some() {
            len += 1;
        }
        if self.pp_gov_group.is_some() {
            len += 1;
        }
        if self.treasury_withdrawal.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.DRepVotingThresholds", len)?;
        if let Some(v) = self.motion_no_confidence.as_ref() {
            struct_ser.serialize_field("motionNoConfidence", v)?;
        }
        if let Some(v) = self.committee_normal.as_ref() {
            struct_ser.serialize_field("committeeNormal", v)?;
        }
        if let Some(v) = self.committee_no_confidence.as_ref() {
            struct_ser.serialize_field("committeeNoConfidence", v)?;
        }
        if let Some(v) = self.update_to_constitution.as_ref() {
            struct_ser.serialize_field("updateToConstitution", v)?;
        }
        if let Some(v) = self.hard_fork_initiation.as_ref() {
            struct_ser.serialize_field("hardForkInitiation", v)?;
        }
        if let Some(v) = self.pp_network_group.as_ref() {
            struct_ser.serialize_field("ppNetworkGroup", v)?;
        }
        if let Some(v) = self.pp_economic_group.as_ref() {
            struct_ser.serialize_field("ppEconomicGroup", v)?;
        }
        if let Some(v) = self.pp_technical_group.as_ref() {
            struct_ser.serialize_field("ppTechnicalGroup", v)?;
        }
        if let Some(v) = self.pp_gov_group.as_ref() {
            struct_ser.serialize_field("ppGovGroup", v)?;
        }
        if let Some(v) = self.treasury_withdrawal.as_ref() {
            struct_ser.serialize_field("treasuryWithdrawal", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DRepVotingThresholds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "motion_no_confidence",
            "motionNoConfidence",
            "committee_normal",
            "committeeNormal",
            "committee_no_confidence",
            "committeeNoConfidence",
            "update_to_constitution",
            "updateToConstitution",
            "hard_fork_initiation",
            "hardForkInitiation",
            "pp_network_group",
            "ppNetworkGroup",
            "pp_economic_group",
            "ppEconomicGroup",
            "pp_technical_group",
            "ppTechnicalGroup",
            "pp_gov_group",
            "ppGovGroup",
            "treasury_withdrawal",
            "treasuryWithdrawal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MotionNoConfidence,
            CommitteeNormal,
            CommitteeNoConfidence,
            UpdateToConstitution,
            HardForkInitiation,
            PpNetworkGroup,
            PpEconomicGroup,
            PpTechnicalGroup,
            PpGovGroup,
            TreasuryWithdrawal,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "motionNoConfidence" | "motion_no_confidence" => Ok(GeneratedField::MotionNoConfidence),
                            "committeeNormal" | "committee_normal" => Ok(GeneratedField::CommitteeNormal),
                            "committeeNoConfidence" | "committee_no_confidence" => Ok(GeneratedField::CommitteeNoConfidence),
                            "updateToConstitution" | "update_to_constitution" => Ok(GeneratedField::UpdateToConstitution),
                            "hardForkInitiation" | "hard_fork_initiation" => Ok(GeneratedField::HardForkInitiation),
                            "ppNetworkGroup" | "pp_network_group" => Ok(GeneratedField::PpNetworkGroup),
                            "ppEconomicGroup" | "pp_economic_group" => Ok(GeneratedField::PpEconomicGroup),
                            "ppTechnicalGroup" | "pp_technical_group" => Ok(GeneratedField::PpTechnicalGroup),
                            "ppGovGroup" | "pp_gov_group" => Ok(GeneratedField::PpGovGroup),
                            "treasuryWithdrawal" | "treasury_withdrawal" => Ok(GeneratedField::TreasuryWithdrawal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DRepVotingThresholds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.DRepVotingThresholds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DRepVotingThresholds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut motion_no_confidence__ = None;
                let mut committee_normal__ = None;
                let mut committee_no_confidence__ = None;
                let mut update_to_constitution__ = None;
                let mut hard_fork_initiation__ = None;
                let mut pp_network_group__ = None;
                let mut pp_economic_group__ = None;
                let mut pp_technical_group__ = None;
                let mut pp_gov_group__ = None;
                let mut treasury_withdrawal__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MotionNoConfidence => {
                            if motion_no_confidence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("motionNoConfidence"));
                            }
                            motion_no_confidence__ = map_.next_value()?;
                        }
                        GeneratedField::CommitteeNormal => {
                            if committee_normal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeNormal"));
                            }
                            committee_normal__ = map_.next_value()?;
                        }
                        GeneratedField::CommitteeNoConfidence => {
                            if committee_no_confidence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeNoConfidence"));
                            }
                            committee_no_confidence__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateToConstitution => {
                            if update_to_constitution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateToConstitution"));
                            }
                            update_to_constitution__ = map_.next_value()?;
                        }
                        GeneratedField::HardForkInitiation => {
                            if hard_fork_initiation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hardForkInitiation"));
                            }
                            hard_fork_initiation__ = map_.next_value()?;
                        }
                        GeneratedField::PpNetworkGroup => {
                            if pp_network_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ppNetworkGroup"));
                            }
                            pp_network_group__ = map_.next_value()?;
                        }
                        GeneratedField::PpEconomicGroup => {
                            if pp_economic_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ppEconomicGroup"));
                            }
                            pp_economic_group__ = map_.next_value()?;
                        }
                        GeneratedField::PpTechnicalGroup => {
                            if pp_technical_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ppTechnicalGroup"));
                            }
                            pp_technical_group__ = map_.next_value()?;
                        }
                        GeneratedField::PpGovGroup => {
                            if pp_gov_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ppGovGroup"));
                            }
                            pp_gov_group__ = map_.next_value()?;
                        }
                        GeneratedField::TreasuryWithdrawal => {
                            if treasury_withdrawal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treasuryWithdrawal"));
                            }
                            treasury_withdrawal__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DRepVotingThresholds {
                    motion_no_confidence: motion_no_confidence__,
                    committee_normal: committee_normal__,
                    committee_no_confidence: committee_no_confidence__,
                    update_to_constitution: update_to_constitution__,
                    hard_fork_initiation: hard_fork_initiation__,
                    pp_network_group: pp_network_group__,
                    pp_economic_group: pp_economic_group__,
                    pp_technical_group: pp_technical_group__,
                    pp_gov_group: pp_gov_group__,
                    treasury_withdrawal: treasury_withdrawal__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.DRepVotingThresholds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Datum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        if !self.original_cbor.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Datum", len)?;
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
        }
        if !self.original_cbor.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("originalCbor", pbjson::private::base64::encode(&self.original_cbor).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Datum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "payload",
            "original_cbor",
            "originalCbor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Payload,
            OriginalCbor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "payload" => Ok(GeneratedField::Payload),
                            "originalCbor" | "original_cbor" => Ok(GeneratedField::OriginalCbor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Datum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Datum")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Datum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut payload__ = None;
                let mut original_cbor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
                        }
                        GeneratedField::OriginalCbor => {
                            if original_cbor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalCbor"));
                            }
                            original_cbor__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Datum {
                    hash: hash__.unwrap_or_default(),
                    payload: payload__,
                    original_cbor: original_cbor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Datum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EraBoundary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.time != 0 {
            len += 1;
        }
        if self.slot != 0 {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.EraBoundary", len)?;
        if self.time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("time", ToString::to_string(&self.time).as_str())?;
        }
        if self.slot != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("slot", ToString::to_string(&self.slot).as_str())?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EraBoundary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "time",
            "slot",
            "epoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Time,
            Slot,
            Epoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "time" => Ok(GeneratedField::Time),
                            "slot" => Ok(GeneratedField::Slot),
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EraBoundary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.EraBoundary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EraBoundary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut time__ = None;
                let mut slot__ = None;
                let mut epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Slot => {
                            if slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slot"));
                            }
                            slot__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EraBoundary {
                    time: time__.unwrap_or_default(),
                    slot: slot__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.EraBoundary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EraSummaries {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.summaries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.EraSummaries", len)?;
        if !self.summaries.is_empty() {
            struct_ser.serialize_field("summaries", &self.summaries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EraSummaries {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "summaries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Summaries,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "summaries" => Ok(GeneratedField::Summaries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EraSummaries;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.EraSummaries")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EraSummaries, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut summaries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Summaries => {
                            if summaries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summaries"));
                            }
                            summaries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EraSummaries {
                    summaries: summaries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.EraSummaries", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EraSummary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        if self.protocol_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.EraSummary", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        if let Some(v) = self.protocol_params.as_ref() {
            struct_ser.serialize_field("protocolParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EraSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "start",
            "end",
            "protocol_params",
            "protocolParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Start,
            End,
            ProtocolParams,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "protocolParams" | "protocol_params" => Ok(GeneratedField::ProtocolParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EraSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.EraSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EraSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut start__ = None;
                let mut end__ = None;
                let mut protocol_params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = map_.next_value()?;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = map_.next_value()?;
                        }
                        GeneratedField::ProtocolParams => {
                            if protocol_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolParams"));
                            }
                            protocol_params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EraSummary {
                    name: name__.unwrap_or_default(),
                    start: start__,
                    end: end__,
                    protocol_params: protocol_params__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.EraSummary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvalError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.EvalError", len)?;
        if !self.msg.is_empty() {
            struct_ser.serialize_field("msg", &self.msg)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvalError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Msg,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvalError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.EvalError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvalError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvalError {
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.EvalError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvalTrace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.EvalTrace", len)?;
        if !self.msg.is_empty() {
            struct_ser.serialize_field("msg", &self.msg)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvalTrace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Msg,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvalTrace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.EvalTrace")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvalTrace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvalTrace {
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.EvalTrace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExPrices {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.steps.is_some() {
            len += 1;
        }
        if self.memory.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ExPrices", len)?;
        if let Some(v) = self.steps.as_ref() {
            struct_ser.serialize_field("steps", v)?;
        }
        if let Some(v) = self.memory.as_ref() {
            struct_ser.serialize_field("memory", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExPrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "steps",
            "memory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Steps,
            Memory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "steps" => Ok(GeneratedField::Steps),
                            "memory" => Ok(GeneratedField::Memory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExPrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ExPrices")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExPrices, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut steps__ = None;
                let mut memory__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = map_.next_value()?;
                        }
                        GeneratedField::Memory => {
                            if memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            memory__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExPrices {
                    steps: steps__,
                    memory: memory__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ExPrices", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExUnits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.steps != 0 {
            len += 1;
        }
        if self.memory != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ExUnits", len)?;
        if self.steps != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("steps", ToString::to_string(&self.steps).as_str())?;
        }
        if self.memory != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("memory", ToString::to_string(&self.memory).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExUnits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "steps",
            "memory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Steps,
            Memory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "steps" => Ok(GeneratedField::Steps),
                            "memory" => Ok(GeneratedField::Memory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExUnits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ExUnits")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExUnits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut steps__ = None;
                let mut memory__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Memory => {
                            if memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            memory__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExUnits {
                    steps: steps__.unwrap_or_default(),
                    memory: memory__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ExUnits", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtraEntropy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ExtraEntropy", len)?;
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtraEntropy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tag" => Ok(GeneratedField::Tag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtraEntropy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ExtraEntropy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtraEntropy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExtraEntropy {
                    tag: tag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ExtraEntropy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenDelegs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.delegate.is_empty() {
            len += 1;
        }
        if !self.vrf.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.GenDelegs", len)?;
        if !self.delegate.is_empty() {
            struct_ser.serialize_field("delegate", &self.delegate)?;
        }
        if !self.vrf.is_empty() {
            struct_ser.serialize_field("vrf", &self.vrf)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenDelegs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegate",
            "vrf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Delegate,
            Vrf,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "delegate" => Ok(GeneratedField::Delegate),
                            "vrf" => Ok(GeneratedField::Vrf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenDelegs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.GenDelegs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenDelegs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delegate__ = None;
                let mut vrf__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Delegate => {
                            if delegate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delegate"));
                            }
                            delegate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vrf => {
                            if vrf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vrf"));
                            }
                            vrf__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenDelegs {
                    delegate: delegate__.unwrap_or_default(),
                    vrf: vrf__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.GenDelegs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Genesis {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.avvm_distr.is_empty() {
            len += 1;
        }
        if self.block_version_data.is_some() {
            len += 1;
        }
        if !self.fts_seed.is_empty() {
            len += 1;
        }
        if self.protocol_consts.is_some() {
            len += 1;
        }
        if self.start_time != 0 {
            len += 1;
        }
        if !self.boot_stakeholders.is_empty() {
            len += 1;
        }
        if !self.heavy_delegation.is_empty() {
            len += 1;
        }
        if !self.non_avvm_balances.is_empty() {
            len += 1;
        }
        if !self.vss_certs.is_empty() {
            len += 1;
        }
        if self.active_slots_coeff.is_some() {
            len += 1;
        }
        if self.epoch_length != 0 {
            len += 1;
        }
        if !self.gen_delegs.is_empty() {
            len += 1;
        }
        if !self.initial_funds.is_empty() {
            len += 1;
        }
        if self.max_kes_evolutions != 0 {
            len += 1;
        }
        if self.max_lovelace_supply != 0 {
            len += 1;
        }
        if !self.network_id.is_empty() {
            len += 1;
        }
        if self.network_magic != 0 {
            len += 1;
        }
        if self.protocol_params.is_some() {
            len += 1;
        }
        if self.security_param != 0 {
            len += 1;
        }
        if self.slot_length != 0 {
            len += 1;
        }
        if self.slots_per_kes_period != 0 {
            len += 1;
        }
        if !self.system_start.is_empty() {
            len += 1;
        }
        if self.update_quorum != 0 {
            len += 1;
        }
        if self.lovelace_per_utxo_word != 0 {
            len += 1;
        }
        if self.execution_prices.is_some() {
            len += 1;
        }
        if self.max_tx_ex_units.is_some() {
            len += 1;
        }
        if self.max_block_ex_units.is_some() {
            len += 1;
        }
        if self.max_value_size != 0 {
            len += 1;
        }
        if self.collateral_percentage != 0 {
            len += 1;
        }
        if self.max_collateral_inputs != 0 {
            len += 1;
        }
        if self.cost_models.is_some() {
            len += 1;
        }
        if self.committee.is_some() {
            len += 1;
        }
        if self.constitution.is_some() {
            len += 1;
        }
        if self.committee_min_size != 0 {
            len += 1;
        }
        if self.committee_max_term_length != 0 {
            len += 1;
        }
        if self.gov_action_lifetime != 0 {
            len += 1;
        }
        if self.gov_action_deposit != 0 {
            len += 1;
        }
        if self.drep_deposit != 0 {
            len += 1;
        }
        if self.drep_activity != 0 {
            len += 1;
        }
        if self.min_fee_ref_script_cost_per_byte.is_some() {
            len += 1;
        }
        if self.drep_voting_thresholds.is_some() {
            len += 1;
        }
        if self.pool_voting_thresholds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Genesis", len)?;
        if !self.avvm_distr.is_empty() {
            struct_ser.serialize_field("avvmDistr", &self.avvm_distr)?;
        }
        if let Some(v) = self.block_version_data.as_ref() {
            struct_ser.serialize_field("blockVersionData", v)?;
        }
        if !self.fts_seed.is_empty() {
            struct_ser.serialize_field("ftsSeed", &self.fts_seed)?;
        }
        if let Some(v) = self.protocol_consts.as_ref() {
            struct_ser.serialize_field("protocolConsts", v)?;
        }
        if self.start_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("startTime", ToString::to_string(&self.start_time).as_str())?;
        }
        if !self.boot_stakeholders.is_empty() {
            let v: std::collections::HashMap<_, _> = self.boot_stakeholders.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("bootStakeholders", &v)?;
        }
        if !self.heavy_delegation.is_empty() {
            struct_ser.serialize_field("heavyDelegation", &self.heavy_delegation)?;
        }
        if !self.non_avvm_balances.is_empty() {
            struct_ser.serialize_field("nonAvvmBalances", &self.non_avvm_balances)?;
        }
        if !self.vss_certs.is_empty() {
            struct_ser.serialize_field("vssCerts", &self.vss_certs)?;
        }
        if let Some(v) = self.active_slots_coeff.as_ref() {
            struct_ser.serialize_field("activeSlotsCoeff", v)?;
        }
        if self.epoch_length != 0 {
            struct_ser.serialize_field("epochLength", &self.epoch_length)?;
        }
        if !self.gen_delegs.is_empty() {
            struct_ser.serialize_field("genDelegs", &self.gen_delegs)?;
        }
        if !self.initial_funds.is_empty() {
            let v: std::collections::HashMap<_, _> = self.initial_funds.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("initialFunds", &v)?;
        }
        if self.max_kes_evolutions != 0 {
            struct_ser.serialize_field("maxKesEvolutions", &self.max_kes_evolutions)?;
        }
        if self.max_lovelace_supply != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxLovelaceSupply", ToString::to_string(&self.max_lovelace_supply).as_str())?;
        }
        if !self.network_id.is_empty() {
            struct_ser.serialize_field("networkId", &self.network_id)?;
        }
        if self.network_magic != 0 {
            struct_ser.serialize_field("networkMagic", &self.network_magic)?;
        }
        if let Some(v) = self.protocol_params.as_ref() {
            struct_ser.serialize_field("protocolParams", v)?;
        }
        if self.security_param != 0 {
            struct_ser.serialize_field("securityParam", &self.security_param)?;
        }
        if self.slot_length != 0 {
            struct_ser.serialize_field("slotLength", &self.slot_length)?;
        }
        if self.slots_per_kes_period != 0 {
            struct_ser.serialize_field("slotsPerKesPeriod", &self.slots_per_kes_period)?;
        }
        if !self.system_start.is_empty() {
            struct_ser.serialize_field("systemStart", &self.system_start)?;
        }
        if self.update_quorum != 0 {
            struct_ser.serialize_field("updateQuorum", &self.update_quorum)?;
        }
        if self.lovelace_per_utxo_word != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("lovelacePerUtxoWord", ToString::to_string(&self.lovelace_per_utxo_word).as_str())?;
        }
        if let Some(v) = self.execution_prices.as_ref() {
            struct_ser.serialize_field("executionPrices", v)?;
        }
        if let Some(v) = self.max_tx_ex_units.as_ref() {
            struct_ser.serialize_field("maxTxExUnits", v)?;
        }
        if let Some(v) = self.max_block_ex_units.as_ref() {
            struct_ser.serialize_field("maxBlockExUnits", v)?;
        }
        if self.max_value_size != 0 {
            struct_ser.serialize_field("maxValueSize", &self.max_value_size)?;
        }
        if self.collateral_percentage != 0 {
            struct_ser.serialize_field("collateralPercentage", &self.collateral_percentage)?;
        }
        if self.max_collateral_inputs != 0 {
            struct_ser.serialize_field("maxCollateralInputs", &self.max_collateral_inputs)?;
        }
        if let Some(v) = self.cost_models.as_ref() {
            struct_ser.serialize_field("costModels", v)?;
        }
        if let Some(v) = self.committee.as_ref() {
            struct_ser.serialize_field("committee", v)?;
        }
        if let Some(v) = self.constitution.as_ref() {
            struct_ser.serialize_field("constitution", v)?;
        }
        if self.committee_min_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("committeeMinSize", ToString::to_string(&self.committee_min_size).as_str())?;
        }
        if self.committee_max_term_length != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("committeeMaxTermLength", ToString::to_string(&self.committee_max_term_length).as_str())?;
        }
        if self.gov_action_lifetime != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("govActionLifetime", ToString::to_string(&self.gov_action_lifetime).as_str())?;
        }
        if self.gov_action_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("govActionDeposit", ToString::to_string(&self.gov_action_deposit).as_str())?;
        }
        if self.drep_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("drepDeposit", ToString::to_string(&self.drep_deposit).as_str())?;
        }
        if self.drep_activity != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("drepActivity", ToString::to_string(&self.drep_activity).as_str())?;
        }
        if let Some(v) = self.min_fee_ref_script_cost_per_byte.as_ref() {
            struct_ser.serialize_field("minFeeRefScriptCostPerByte", v)?;
        }
        if let Some(v) = self.drep_voting_thresholds.as_ref() {
            struct_ser.serialize_field("drepVotingThresholds", v)?;
        }
        if let Some(v) = self.pool_voting_thresholds.as_ref() {
            struct_ser.serialize_field("poolVotingThresholds", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Genesis {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "avvm_distr",
            "avvmDistr",
            "block_version_data",
            "blockVersionData",
            "fts_seed",
            "ftsSeed",
            "protocol_consts",
            "protocolConsts",
            "start_time",
            "startTime",
            "boot_stakeholders",
            "bootStakeholders",
            "heavy_delegation",
            "heavyDelegation",
            "non_avvm_balances",
            "nonAvvmBalances",
            "vss_certs",
            "vssCerts",
            "active_slots_coeff",
            "activeSlotsCoeff",
            "epoch_length",
            "epochLength",
            "gen_delegs",
            "genDelegs",
            "initial_funds",
            "initialFunds",
            "max_kes_evolutions",
            "maxKesEvolutions",
            "max_lovelace_supply",
            "maxLovelaceSupply",
            "network_id",
            "networkId",
            "network_magic",
            "networkMagic",
            "protocol_params",
            "protocolParams",
            "security_param",
            "securityParam",
            "slot_length",
            "slotLength",
            "slots_per_kes_period",
            "slotsPerKesPeriod",
            "system_start",
            "systemStart",
            "update_quorum",
            "updateQuorum",
            "lovelace_per_utxo_word",
            "lovelacePerUtxoWord",
            "execution_prices",
            "executionPrices",
            "max_tx_ex_units",
            "maxTxExUnits",
            "max_block_ex_units",
            "maxBlockExUnits",
            "max_value_size",
            "maxValueSize",
            "collateral_percentage",
            "collateralPercentage",
            "max_collateral_inputs",
            "maxCollateralInputs",
            "cost_models",
            "costModels",
            "committee",
            "constitution",
            "committee_min_size",
            "committeeMinSize",
            "committee_max_term_length",
            "committeeMaxTermLength",
            "gov_action_lifetime",
            "govActionLifetime",
            "gov_action_deposit",
            "govActionDeposit",
            "drep_deposit",
            "drepDeposit",
            "drep_activity",
            "drepActivity",
            "min_fee_ref_script_cost_per_byte",
            "minFeeRefScriptCostPerByte",
            "drep_voting_thresholds",
            "drepVotingThresholds",
            "pool_voting_thresholds",
            "poolVotingThresholds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AvvmDistr,
            BlockVersionData,
            FtsSeed,
            ProtocolConsts,
            StartTime,
            BootStakeholders,
            HeavyDelegation,
            NonAvvmBalances,
            VssCerts,
            ActiveSlotsCoeff,
            EpochLength,
            GenDelegs,
            InitialFunds,
            MaxKesEvolutions,
            MaxLovelaceSupply,
            NetworkId,
            NetworkMagic,
            ProtocolParams,
            SecurityParam,
            SlotLength,
            SlotsPerKesPeriod,
            SystemStart,
            UpdateQuorum,
            LovelacePerUtxoWord,
            ExecutionPrices,
            MaxTxExUnits,
            MaxBlockExUnits,
            MaxValueSize,
            CollateralPercentage,
            MaxCollateralInputs,
            CostModels,
            Committee,
            Constitution,
            CommitteeMinSize,
            CommitteeMaxTermLength,
            GovActionLifetime,
            GovActionDeposit,
            DrepDeposit,
            DrepActivity,
            MinFeeRefScriptCostPerByte,
            DrepVotingThresholds,
            PoolVotingThresholds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "avvmDistr" | "avvm_distr" => Ok(GeneratedField::AvvmDistr),
                            "blockVersionData" | "block_version_data" => Ok(GeneratedField::BlockVersionData),
                            "ftsSeed" | "fts_seed" => Ok(GeneratedField::FtsSeed),
                            "protocolConsts" | "protocol_consts" => Ok(GeneratedField::ProtocolConsts),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "bootStakeholders" | "boot_stakeholders" => Ok(GeneratedField::BootStakeholders),
                            "heavyDelegation" | "heavy_delegation" => Ok(GeneratedField::HeavyDelegation),
                            "nonAvvmBalances" | "non_avvm_balances" => Ok(GeneratedField::NonAvvmBalances),
                            "vssCerts" | "vss_certs" => Ok(GeneratedField::VssCerts),
                            "activeSlotsCoeff" | "active_slots_coeff" => Ok(GeneratedField::ActiveSlotsCoeff),
                            "epochLength" | "epoch_length" => Ok(GeneratedField::EpochLength),
                            "genDelegs" | "gen_delegs" => Ok(GeneratedField::GenDelegs),
                            "initialFunds" | "initial_funds" => Ok(GeneratedField::InitialFunds),
                            "maxKesEvolutions" | "max_kes_evolutions" => Ok(GeneratedField::MaxKesEvolutions),
                            "maxLovelaceSupply" | "max_lovelace_supply" => Ok(GeneratedField::MaxLovelaceSupply),
                            "networkId" | "network_id" => Ok(GeneratedField::NetworkId),
                            "networkMagic" | "network_magic" => Ok(GeneratedField::NetworkMagic),
                            "protocolParams" | "protocol_params" => Ok(GeneratedField::ProtocolParams),
                            "securityParam" | "security_param" => Ok(GeneratedField::SecurityParam),
                            "slotLength" | "slot_length" => Ok(GeneratedField::SlotLength),
                            "slotsPerKesPeriod" | "slots_per_kes_period" => Ok(GeneratedField::SlotsPerKesPeriod),
                            "systemStart" | "system_start" => Ok(GeneratedField::SystemStart),
                            "updateQuorum" | "update_quorum" => Ok(GeneratedField::UpdateQuorum),
                            "lovelacePerUtxoWord" | "lovelace_per_utxo_word" => Ok(GeneratedField::LovelacePerUtxoWord),
                            "executionPrices" | "execution_prices" => Ok(GeneratedField::ExecutionPrices),
                            "maxTxExUnits" | "max_tx_ex_units" => Ok(GeneratedField::MaxTxExUnits),
                            "maxBlockExUnits" | "max_block_ex_units" => Ok(GeneratedField::MaxBlockExUnits),
                            "maxValueSize" | "max_value_size" => Ok(GeneratedField::MaxValueSize),
                            "collateralPercentage" | "collateral_percentage" => Ok(GeneratedField::CollateralPercentage),
                            "maxCollateralInputs" | "max_collateral_inputs" => Ok(GeneratedField::MaxCollateralInputs),
                            "costModels" | "cost_models" => Ok(GeneratedField::CostModels),
                            "committee" => Ok(GeneratedField::Committee),
                            "constitution" => Ok(GeneratedField::Constitution),
                            "committeeMinSize" | "committee_min_size" => Ok(GeneratedField::CommitteeMinSize),
                            "committeeMaxTermLength" | "committee_max_term_length" => Ok(GeneratedField::CommitteeMaxTermLength),
                            "govActionLifetime" | "gov_action_lifetime" => Ok(GeneratedField::GovActionLifetime),
                            "govActionDeposit" | "gov_action_deposit" => Ok(GeneratedField::GovActionDeposit),
                            "drepDeposit" | "drep_deposit" => Ok(GeneratedField::DrepDeposit),
                            "drepActivity" | "drep_activity" => Ok(GeneratedField::DrepActivity),
                            "minFeeRefScriptCostPerByte" | "min_fee_ref_script_cost_per_byte" => Ok(GeneratedField::MinFeeRefScriptCostPerByte),
                            "drepVotingThresholds" | "drep_voting_thresholds" => Ok(GeneratedField::DrepVotingThresholds),
                            "poolVotingThresholds" | "pool_voting_thresholds" => Ok(GeneratedField::PoolVotingThresholds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Genesis;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Genesis")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Genesis, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut avvm_distr__ = None;
                let mut block_version_data__ = None;
                let mut fts_seed__ = None;
                let mut protocol_consts__ = None;
                let mut start_time__ = None;
                let mut boot_stakeholders__ = None;
                let mut heavy_delegation__ = None;
                let mut non_avvm_balances__ = None;
                let mut vss_certs__ = None;
                let mut active_slots_coeff__ = None;
                let mut epoch_length__ = None;
                let mut gen_delegs__ = None;
                let mut initial_funds__ = None;
                let mut max_kes_evolutions__ = None;
                let mut max_lovelace_supply__ = None;
                let mut network_id__ = None;
                let mut network_magic__ = None;
                let mut protocol_params__ = None;
                let mut security_param__ = None;
                let mut slot_length__ = None;
                let mut slots_per_kes_period__ = None;
                let mut system_start__ = None;
                let mut update_quorum__ = None;
                let mut lovelace_per_utxo_word__ = None;
                let mut execution_prices__ = None;
                let mut max_tx_ex_units__ = None;
                let mut max_block_ex_units__ = None;
                let mut max_value_size__ = None;
                let mut collateral_percentage__ = None;
                let mut max_collateral_inputs__ = None;
                let mut cost_models__ = None;
                let mut committee__ = None;
                let mut constitution__ = None;
                let mut committee_min_size__ = None;
                let mut committee_max_term_length__ = None;
                let mut gov_action_lifetime__ = None;
                let mut gov_action_deposit__ = None;
                let mut drep_deposit__ = None;
                let mut drep_activity__ = None;
                let mut min_fee_ref_script_cost_per_byte__ = None;
                let mut drep_voting_thresholds__ = None;
                let mut pool_voting_thresholds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AvvmDistr => {
                            if avvm_distr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avvmDistr"));
                            }
                            avvm_distr__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::BlockVersionData => {
                            if block_version_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockVersionData"));
                            }
                            block_version_data__ = map_.next_value()?;
                        }
                        GeneratedField::FtsSeed => {
                            if fts_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ftsSeed"));
                            }
                            fts_seed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtocolConsts => {
                            if protocol_consts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolConsts"));
                            }
                            protocol_consts__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BootStakeholders => {
                            if boot_stakeholders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootStakeholders"));
                            }
                            boot_stakeholders__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<u64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::HeavyDelegation => {
                            if heavy_delegation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heavyDelegation"));
                            }
                            heavy_delegation__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::NonAvvmBalances => {
                            if non_avvm_balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonAvvmBalances"));
                            }
                            non_avvm_balances__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::VssCerts => {
                            if vss_certs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vssCerts"));
                            }
                            vss_certs__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ActiveSlotsCoeff => {
                            if active_slots_coeff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeSlotsCoeff"));
                            }
                            active_slots_coeff__ = map_.next_value()?;
                        }
                        GeneratedField::EpochLength => {
                            if epoch_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochLength"));
                            }
                            epoch_length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GenDelegs => {
                            if gen_delegs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genDelegs"));
                            }
                            gen_delegs__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::InitialFunds => {
                            if initial_funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialFunds"));
                            }
                            initial_funds__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<u64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::MaxKesEvolutions => {
                            if max_kes_evolutions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxKesEvolutions"));
                            }
                            max_kes_evolutions__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxLovelaceSupply => {
                            if max_lovelace_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLovelaceSupply"));
                            }
                            max_lovelace_supply__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NetworkId => {
                            if network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkId"));
                            }
                            network_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NetworkMagic => {
                            if network_magic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkMagic"));
                            }
                            network_magic__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtocolParams => {
                            if protocol_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolParams"));
                            }
                            protocol_params__ = map_.next_value()?;
                        }
                        GeneratedField::SecurityParam => {
                            if security_param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("securityParam"));
                            }
                            security_param__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SlotLength => {
                            if slot_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotLength"));
                            }
                            slot_length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SlotsPerKesPeriod => {
                            if slots_per_kes_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotsPerKesPeriod"));
                            }
                            slots_per_kes_period__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SystemStart => {
                            if system_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemStart"));
                            }
                            system_start__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateQuorum => {
                            if update_quorum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateQuorum"));
                            }
                            update_quorum__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LovelacePerUtxoWord => {
                            if lovelace_per_utxo_word__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lovelacePerUtxoWord"));
                            }
                            lovelace_per_utxo_word__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExecutionPrices => {
                            if execution_prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionPrices"));
                            }
                            execution_prices__ = map_.next_value()?;
                        }
                        GeneratedField::MaxTxExUnits => {
                            if max_tx_ex_units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTxExUnits"));
                            }
                            max_tx_ex_units__ = map_.next_value()?;
                        }
                        GeneratedField::MaxBlockExUnits => {
                            if max_block_ex_units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBlockExUnits"));
                            }
                            max_block_ex_units__ = map_.next_value()?;
                        }
                        GeneratedField::MaxValueSize => {
                            if max_value_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxValueSize"));
                            }
                            max_value_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CollateralPercentage => {
                            if collateral_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralPercentage"));
                            }
                            collateral_percentage__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxCollateralInputs => {
                            if max_collateral_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCollateralInputs"));
                            }
                            max_collateral_inputs__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CostModels => {
                            if cost_models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("costModels"));
                            }
                            cost_models__ = map_.next_value()?;
                        }
                        GeneratedField::Committee => {
                            if committee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committee"));
                            }
                            committee__ = map_.next_value()?;
                        }
                        GeneratedField::Constitution => {
                            if constitution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constitution"));
                            }
                            constitution__ = map_.next_value()?;
                        }
                        GeneratedField::CommitteeMinSize => {
                            if committee_min_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeMinSize"));
                            }
                            committee_min_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommitteeMaxTermLength => {
                            if committee_max_term_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeMaxTermLength"));
                            }
                            committee_max_term_length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GovActionLifetime => {
                            if gov_action_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionLifetime"));
                            }
                            gov_action_lifetime__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GovActionDeposit => {
                            if gov_action_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionDeposit"));
                            }
                            gov_action_deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DrepDeposit => {
                            if drep_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepDeposit"));
                            }
                            drep_deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DrepActivity => {
                            if drep_activity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepActivity"));
                            }
                            drep_activity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinFeeRefScriptCostPerByte => {
                            if min_fee_ref_script_cost_per_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minFeeRefScriptCostPerByte"));
                            }
                            min_fee_ref_script_cost_per_byte__ = map_.next_value()?;
                        }
                        GeneratedField::DrepVotingThresholds => {
                            if drep_voting_thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepVotingThresholds"));
                            }
                            drep_voting_thresholds__ = map_.next_value()?;
                        }
                        GeneratedField::PoolVotingThresholds => {
                            if pool_voting_thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolVotingThresholds"));
                            }
                            pool_voting_thresholds__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Genesis {
                    avvm_distr: avvm_distr__.unwrap_or_default(),
                    block_version_data: block_version_data__,
                    fts_seed: fts_seed__.unwrap_or_default(),
                    protocol_consts: protocol_consts__,
                    start_time: start_time__.unwrap_or_default(),
                    boot_stakeholders: boot_stakeholders__.unwrap_or_default(),
                    heavy_delegation: heavy_delegation__.unwrap_or_default(),
                    non_avvm_balances: non_avvm_balances__.unwrap_or_default(),
                    vss_certs: vss_certs__.unwrap_or_default(),
                    active_slots_coeff: active_slots_coeff__,
                    epoch_length: epoch_length__.unwrap_or_default(),
                    gen_delegs: gen_delegs__.unwrap_or_default(),
                    initial_funds: initial_funds__.unwrap_or_default(),
                    max_kes_evolutions: max_kes_evolutions__.unwrap_or_default(),
                    max_lovelace_supply: max_lovelace_supply__.unwrap_or_default(),
                    network_id: network_id__.unwrap_or_default(),
                    network_magic: network_magic__.unwrap_or_default(),
                    protocol_params: protocol_params__,
                    security_param: security_param__.unwrap_or_default(),
                    slot_length: slot_length__.unwrap_or_default(),
                    slots_per_kes_period: slots_per_kes_period__.unwrap_or_default(),
                    system_start: system_start__.unwrap_or_default(),
                    update_quorum: update_quorum__.unwrap_or_default(),
                    lovelace_per_utxo_word: lovelace_per_utxo_word__.unwrap_or_default(),
                    execution_prices: execution_prices__,
                    max_tx_ex_units: max_tx_ex_units__,
                    max_block_ex_units: max_block_ex_units__,
                    max_value_size: max_value_size__.unwrap_or_default(),
                    collateral_percentage: collateral_percentage__.unwrap_or_default(),
                    max_collateral_inputs: max_collateral_inputs__.unwrap_or_default(),
                    cost_models: cost_models__,
                    committee: committee__,
                    constitution: constitution__,
                    committee_min_size: committee_min_size__.unwrap_or_default(),
                    committee_max_term_length: committee_max_term_length__.unwrap_or_default(),
                    gov_action_lifetime: gov_action_lifetime__.unwrap_or_default(),
                    gov_action_deposit: gov_action_deposit__.unwrap_or_default(),
                    drep_deposit: drep_deposit__.unwrap_or_default(),
                    drep_activity: drep_activity__.unwrap_or_default(),
                    min_fee_ref_script_cost_per_byte: min_fee_ref_script_cost_per_byte__,
                    drep_voting_thresholds: drep_voting_thresholds__,
                    pool_voting_thresholds: pool_voting_thresholds__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Genesis", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenesisKeyDelegationCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.genesis_hash.is_empty() {
            len += 1;
        }
        if !self.genesis_delegate_hash.is_empty() {
            len += 1;
        }
        if !self.vrf_keyhash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.GenesisKeyDelegationCert", len)?;
        if !self.genesis_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("genesisHash", pbjson::private::base64::encode(&self.genesis_hash).as_str())?;
        }
        if !self.genesis_delegate_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("genesisDelegateHash", pbjson::private::base64::encode(&self.genesis_delegate_hash).as_str())?;
        }
        if !self.vrf_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("vrfKeyhash", pbjson::private::base64::encode(&self.vrf_keyhash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisKeyDelegationCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "genesis_hash",
            "genesisHash",
            "genesis_delegate_hash",
            "genesisDelegateHash",
            "vrf_keyhash",
            "vrfKeyhash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GenesisHash,
            GenesisDelegateHash,
            VrfKeyhash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "genesisHash" | "genesis_hash" => Ok(GeneratedField::GenesisHash),
                            "genesisDelegateHash" | "genesis_delegate_hash" => Ok(GeneratedField::GenesisDelegateHash),
                            "vrfKeyhash" | "vrf_keyhash" => Ok(GeneratedField::VrfKeyhash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisKeyDelegationCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.GenesisKeyDelegationCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisKeyDelegationCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut genesis_hash__ = None;
                let mut genesis_delegate_hash__ = None;
                let mut vrf_keyhash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GenesisHash => {
                            if genesis_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesisHash"));
                            }
                            genesis_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GenesisDelegateHash => {
                            if genesis_delegate_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesisDelegateHash"));
                            }
                            genesis_delegate_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VrfKeyhash => {
                            if vrf_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vrfKeyhash"));
                            }
                            vrf_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GenesisKeyDelegationCert {
                    genesis_hash: genesis_hash__.unwrap_or_default(),
                    genesis_delegate_hash: genesis_delegate_hash__.unwrap_or_default(),
                    vrf_keyhash: vrf_keyhash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.GenesisKeyDelegationCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GovernanceAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.governance_action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.GovernanceAction", len)?;
        if let Some(v) = self.governance_action.as_ref() {
            match v {
                governance_action::GovernanceAction::ParameterChangeAction(v) => {
                    struct_ser.serialize_field("parameterChangeAction", v)?;
                }
                governance_action::GovernanceAction::HardForkInitiationAction(v) => {
                    struct_ser.serialize_field("hardForkInitiationAction", v)?;
                }
                governance_action::GovernanceAction::TreasuryWithdrawalsAction(v) => {
                    struct_ser.serialize_field("treasuryWithdrawalsAction", v)?;
                }
                governance_action::GovernanceAction::NoConfidenceAction(v) => {
                    struct_ser.serialize_field("noConfidenceAction", v)?;
                }
                governance_action::GovernanceAction::UpdateCommitteeAction(v) => {
                    struct_ser.serialize_field("updateCommitteeAction", v)?;
                }
                governance_action::GovernanceAction::NewConstitutionAction(v) => {
                    struct_ser.serialize_field("newConstitutionAction", v)?;
                }
                governance_action::GovernanceAction::InfoAction(v) => {
                    struct_ser.serialize_field("infoAction", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GovernanceAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameter_change_action",
            "parameterChangeAction",
            "hard_fork_initiation_action",
            "hardForkInitiationAction",
            "treasury_withdrawals_action",
            "treasuryWithdrawalsAction",
            "no_confidence_action",
            "noConfidenceAction",
            "update_committee_action",
            "updateCommitteeAction",
            "new_constitution_action",
            "newConstitutionAction",
            "info_action",
            "infoAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParameterChangeAction,
            HardForkInitiationAction,
            TreasuryWithdrawalsAction,
            NoConfidenceAction,
            UpdateCommitteeAction,
            NewConstitutionAction,
            InfoAction,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parameterChangeAction" | "parameter_change_action" => Ok(GeneratedField::ParameterChangeAction),
                            "hardForkInitiationAction" | "hard_fork_initiation_action" => Ok(GeneratedField::HardForkInitiationAction),
                            "treasuryWithdrawalsAction" | "treasury_withdrawals_action" => Ok(GeneratedField::TreasuryWithdrawalsAction),
                            "noConfidenceAction" | "no_confidence_action" => Ok(GeneratedField::NoConfidenceAction),
                            "updateCommitteeAction" | "update_committee_action" => Ok(GeneratedField::UpdateCommitteeAction),
                            "newConstitutionAction" | "new_constitution_action" => Ok(GeneratedField::NewConstitutionAction),
                            "infoAction" | "info_action" => Ok(GeneratedField::InfoAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GovernanceAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.GovernanceAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GovernanceAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut governance_action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParameterChangeAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameterChangeAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<_>>()?.map(governance_action::GovernanceAction::ParameterChangeAction)
;
                        }
                        GeneratedField::HardForkInitiationAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hardForkInitiationAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<_>>()?.map(governance_action::GovernanceAction::HardForkInitiationAction)
;
                        }
                        GeneratedField::TreasuryWithdrawalsAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treasuryWithdrawalsAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<_>>()?.map(governance_action::GovernanceAction::TreasuryWithdrawalsAction)
;
                        }
                        GeneratedField::NoConfidenceAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noConfidenceAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<_>>()?.map(governance_action::GovernanceAction::NoConfidenceAction)
;
                        }
                        GeneratedField::UpdateCommitteeAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateCommitteeAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<_>>()?.map(governance_action::GovernanceAction::UpdateCommitteeAction)
;
                        }
                        GeneratedField::NewConstitutionAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newConstitutionAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<_>>()?.map(governance_action::GovernanceAction::NewConstitutionAction)
;
                        }
                        GeneratedField::InfoAction => {
                            if governance_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("infoAction"));
                            }
                            governance_action__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| governance_action::GovernanceAction::InfoAction(x.0));
                        }
                    }
                }
                Ok(GovernanceAction {
                    governance_action: governance_action__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.GovernanceAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GovernanceActionId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transaction_id.is_empty() {
            len += 1;
        }
        if self.governance_action_index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.GovernanceActionId", len)?;
        if !self.transaction_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("transactionId", pbjson::private::base64::encode(&self.transaction_id).as_str())?;
        }
        if self.governance_action_index != 0 {
            struct_ser.serialize_field("governanceActionIndex", &self.governance_action_index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GovernanceActionId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_id",
            "transactionId",
            "governance_action_index",
            "governanceActionIndex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionId,
            GovernanceActionIndex,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transactionId" | "transaction_id" => Ok(GeneratedField::TransactionId),
                            "governanceActionIndex" | "governance_action_index" => Ok(GeneratedField::GovernanceActionIndex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GovernanceActionId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.GovernanceActionId")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GovernanceActionId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_id__ = None;
                let mut governance_action_index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransactionId => {
                            if transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionId"));
                            }
                            transaction_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GovernanceActionIndex => {
                            if governance_action_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("governanceActionIndex"));
                            }
                            governance_action_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GovernanceActionId {
                    transaction_id: transaction_id__.unwrap_or_default(),
                    governance_action_index: governance_action_index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.GovernanceActionId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GovernanceActionProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deposit != 0 {
            len += 1;
        }
        if !self.reward_account.is_empty() {
            len += 1;
        }
        if self.gov_action.is_some() {
            len += 1;
        }
        if self.anchor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.GovernanceActionProposal", len)?;
        if self.deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deposit", ToString::to_string(&self.deposit).as_str())?;
        }
        if !self.reward_account.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("rewardAccount", pbjson::private::base64::encode(&self.reward_account).as_str())?;
        }
        if let Some(v) = self.gov_action.as_ref() {
            struct_ser.serialize_field("govAction", v)?;
        }
        if let Some(v) = self.anchor.as_ref() {
            struct_ser.serialize_field("anchor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GovernanceActionProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit",
            "reward_account",
            "rewardAccount",
            "gov_action",
            "govAction",
            "anchor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposit,
            RewardAccount,
            GovAction,
            Anchor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deposit" => Ok(GeneratedField::Deposit),
                            "rewardAccount" | "reward_account" => Ok(GeneratedField::RewardAccount),
                            "govAction" | "gov_action" => Ok(GeneratedField::GovAction),
                            "anchor" => Ok(GeneratedField::Anchor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GovernanceActionProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.GovernanceActionProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GovernanceActionProposal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposit__ = None;
                let mut reward_account__ = None;
                let mut gov_action__ = None;
                let mut anchor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RewardAccount => {
                            if reward_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAccount"));
                            }
                            reward_account__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GovAction => {
                            if gov_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govAction"));
                            }
                            gov_action__ = map_.next_value()?;
                        }
                        GeneratedField::Anchor => {
                            if anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            anchor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GovernanceActionProposal {
                    deposit: deposit__.unwrap_or_default(),
                    reward_account: reward_account__.unwrap_or_default(),
                    gov_action: gov_action__,
                    anchor: anchor__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.GovernanceActionProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HardForkInitiationAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gov_action_id.is_some() {
            len += 1;
        }
        if self.protocol_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.HardForkInitiationAction", len)?;
        if let Some(v) = self.gov_action_id.as_ref() {
            struct_ser.serialize_field("govActionId", v)?;
        }
        if let Some(v) = self.protocol_version.as_ref() {
            struct_ser.serialize_field("protocolVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HardForkInitiationAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gov_action_id",
            "govActionId",
            "protocol_version",
            "protocolVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GovActionId,
            ProtocolVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "govActionId" | "gov_action_id" => Ok(GeneratedField::GovActionId),
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HardForkInitiationAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.HardForkInitiationAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HardForkInitiationAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gov_action_id__ = None;
                let mut protocol_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GovActionId => {
                            if gov_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionId"));
                            }
                            gov_action_id__ = map_.next_value()?;
                        }
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HardForkInitiationAction {
                    gov_action_id: gov_action_id__,
                    protocol_version: protocol_version__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.HardForkInitiationAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeavyDelegation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cert.is_empty() {
            len += 1;
        }
        if !self.delegate_pk.is_empty() {
            len += 1;
        }
        if !self.issuer_pk.is_empty() {
            len += 1;
        }
        if self.omega != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.HeavyDelegation", len)?;
        if !self.cert.is_empty() {
            struct_ser.serialize_field("cert", &self.cert)?;
        }
        if !self.delegate_pk.is_empty() {
            struct_ser.serialize_field("delegatePk", &self.delegate_pk)?;
        }
        if !self.issuer_pk.is_empty() {
            struct_ser.serialize_field("issuerPk", &self.issuer_pk)?;
        }
        if self.omega != 0 {
            struct_ser.serialize_field("omega", &self.omega)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeavyDelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cert",
            "delegate_pk",
            "delegatePk",
            "issuer_pk",
            "issuerPk",
            "omega",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cert,
            DelegatePk,
            IssuerPk,
            Omega,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cert" => Ok(GeneratedField::Cert),
                            "delegatePk" | "delegate_pk" => Ok(GeneratedField::DelegatePk),
                            "issuerPk" | "issuer_pk" => Ok(GeneratedField::IssuerPk),
                            "omega" => Ok(GeneratedField::Omega),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeavyDelegation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.HeavyDelegation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeavyDelegation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cert__ = None;
                let mut delegate_pk__ = None;
                let mut issuer_pk__ = None;
                let mut omega__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cert => {
                            if cert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cert"));
                            }
                            cert__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DelegatePk => {
                            if delegate_pk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delegatePk"));
                            }
                            delegate_pk__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IssuerPk => {
                            if issuer_pk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuerPk"));
                            }
                            issuer_pk__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Omega => {
                            if omega__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omega"));
                            }
                            omega__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HeavyDelegation {
                    cert: cert__.unwrap_or_default(),
                    delegate_pk: delegate_pk__.unwrap_or_default(),
                    issuer_pk: issuer_pk__.unwrap_or_default(),
                    omega: omega__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.HeavyDelegation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.label != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Metadata", len)?;
        if self.label != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("label", ToString::to_string(&self.label).as_str())?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "label",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Label,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "label" => Ok(GeneratedField::Label),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Metadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Metadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut label__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Metadata {
                    label: label__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Metadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metadatum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadatum.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Metadatum", len)?;
        if let Some(v) = self.metadatum.as_ref() {
            match v {
                metadatum::Metadatum::Int(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("int", ToString::to_string(&v).as_str())?;
                }
                metadatum::Metadatum::Bytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("bytes", pbjson::private::base64::encode(&v).as_str())?;
                }
                metadatum::Metadatum::Text(v) => {
                    struct_ser.serialize_field("text", v)?;
                }
                metadatum::Metadatum::Array(v) => {
                    struct_ser.serialize_field("array", v)?;
                }
                metadatum::Metadatum::Map(v) => {
                    struct_ser.serialize_field("map", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metadatum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "int",
            "bytes",
            "text",
            "array",
            "map",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Int,
            Bytes,
            Text,
            Array,
            Map,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "int" => Ok(GeneratedField::Int),
                            "bytes" => Ok(GeneratedField::Bytes),
                            "text" => Ok(GeneratedField::Text),
                            "array" => Ok(GeneratedField::Array),
                            "map" => Ok(GeneratedField::Map),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metadatum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Metadatum")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Metadatum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadatum__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Int => {
                            if metadatum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int"));
                            }
                            metadatum__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| metadatum::Metadatum::Int(x.0));
                        }
                        GeneratedField::Bytes => {
                            if metadatum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            metadatum__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| metadatum::Metadatum::Bytes(x.0));
                        }
                        GeneratedField::Text => {
                            if metadatum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            metadatum__ = map_.next_value::<::std::option::Option<_>>()?.map(metadatum::Metadatum::Text);
                        }
                        GeneratedField::Array => {
                            if metadatum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("array"));
                            }
                            metadatum__ = map_.next_value::<::std::option::Option<_>>()?.map(metadatum::Metadatum::Array)
;
                        }
                        GeneratedField::Map => {
                            if metadatum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            metadatum__ = map_.next_value::<::std::option::Option<_>>()?.map(metadatum::Metadatum::Map)
;
                        }
                    }
                }
                Ok(Metadatum {
                    metadatum: metadatum__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Metadatum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetadatumArray {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.MetadatumArray", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetadatumArray {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetadatumArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.MetadatumArray")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetadatumArray, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetadatumArray {
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.MetadatumArray", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetadatumMap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.MetadatumMap", len)?;
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetadatumMap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pairs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetadatumMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.MetadatumMap")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetadatumMap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pairs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetadatumMap {
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.MetadatumMap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetadatumPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.MetadatumPair", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetadatumPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetadatumPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.MetadatumPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetadatumPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MetadatumPair {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.MetadatumPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MirCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.from != 0 {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if self.other_pot != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.MirCert", len)?;
        if self.from != 0 {
            let v = MirSource::try_from(self.from)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.from)))?;
            struct_ser.serialize_field("from", &v)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if self.other_pot != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("otherPot", ToString::to_string(&self.other_pot).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MirCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "to",
            "other_pot",
            "otherPot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
            OtherPot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            "otherPot" | "other_pot" => Ok(GeneratedField::OtherPot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.MirCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MirCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                let mut other_pot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value::<MirSource>()? as i32);
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtherPot => {
                            if other_pot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherPot"));
                            }
                            other_pot__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MirCert {
                    from: from__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    other_pot: other_pot__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.MirCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MirSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MIR_SOURCE_UNSPECIFIED",
            Self::Reserves => "MIR_SOURCE_RESERVES",
            Self::Treasury => "MIR_SOURCE_TREASURY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MirSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MIR_SOURCE_UNSPECIFIED",
            "MIR_SOURCE_RESERVES",
            "MIR_SOURCE_TREASURY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MIR_SOURCE_UNSPECIFIED" => Ok(MirSource::Unspecified),
                    "MIR_SOURCE_RESERVES" => Ok(MirSource::Reserves),
                    "MIR_SOURCE_TREASURY" => Ok(MirSource::Treasury),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MirTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if self.delta_coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.MirTarget", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if self.delta_coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("deltaCoin", ToString::to_string(&self.delta_coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MirTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "delta_coin",
            "deltaCoin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            DeltaCoin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "deltaCoin" | "delta_coin" => Ok(GeneratedField::DeltaCoin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.MirTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MirTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut delta_coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::DeltaCoin => {
                            if delta_coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deltaCoin"));
                            }
                            delta_coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MirTarget {
                    stake_credential: stake_credential__,
                    delta_coin: delta_coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.MirTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Multiasset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policy_id.is_empty() {
            len += 1;
        }
        if !self.assets.is_empty() {
            len += 1;
        }
        if self.redeemer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Multiasset", len)?;
        if !self.policy_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("policyId", pbjson::private::base64::encode(&self.policy_id).as_str())?;
        }
        if !self.assets.is_empty() {
            struct_ser.serialize_field("assets", &self.assets)?;
        }
        if let Some(v) = self.redeemer.as_ref() {
            struct_ser.serialize_field("redeemer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Multiasset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_id",
            "policyId",
            "assets",
            "redeemer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyId,
            Assets,
            Redeemer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "policyId" | "policy_id" => Ok(GeneratedField::PolicyId),
                            "assets" => Ok(GeneratedField::Assets),
                            "redeemer" => Ok(GeneratedField::Redeemer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Multiasset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Multiasset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Multiasset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_id__ = None;
                let mut assets__ = None;
                let mut redeemer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PolicyId => {
                            if policy_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyId"));
                            }
                            policy_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Redeemer => {
                            if redeemer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemer"));
                            }
                            redeemer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Multiasset {
                    policy_id: policy_id__.unwrap_or_default(),
                    assets: assets__.unwrap_or_default(),
                    redeemer: redeemer__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Multiasset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NativeScript {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.native_script.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.NativeScript", len)?;
        if let Some(v) = self.native_script.as_ref() {
            match v {
                native_script::NativeScript::ScriptPubkey(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("scriptPubkey", pbjson::private::base64::encode(&v).as_str())?;
                }
                native_script::NativeScript::ScriptAll(v) => {
                    struct_ser.serialize_field("scriptAll", v)?;
                }
                native_script::NativeScript::ScriptAny(v) => {
                    struct_ser.serialize_field("scriptAny", v)?;
                }
                native_script::NativeScript::ScriptNOfK(v) => {
                    struct_ser.serialize_field("scriptNOfK", v)?;
                }
                native_script::NativeScript::InvalidBefore(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("invalidBefore", ToString::to_string(&v).as_str())?;
                }
                native_script::NativeScript::InvalidHereafter(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("invalidHereafter", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NativeScript {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "script_pubkey",
            "scriptPubkey",
            "script_all",
            "scriptAll",
            "script_any",
            "scriptAny",
            "script_n_of_k",
            "scriptNOfK",
            "invalid_before",
            "invalidBefore",
            "invalid_hereafter",
            "invalidHereafter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScriptPubkey,
            ScriptAll,
            ScriptAny,
            ScriptNOfK,
            InvalidBefore,
            InvalidHereafter,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "scriptPubkey" | "script_pubkey" => Ok(GeneratedField::ScriptPubkey),
                            "scriptAll" | "script_all" => Ok(GeneratedField::ScriptAll),
                            "scriptAny" | "script_any" => Ok(GeneratedField::ScriptAny),
                            "scriptNOfK" | "script_n_of_k" => Ok(GeneratedField::ScriptNOfK),
                            "invalidBefore" | "invalid_before" => Ok(GeneratedField::InvalidBefore),
                            "invalidHereafter" | "invalid_hereafter" => Ok(GeneratedField::InvalidHereafter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NativeScript;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.NativeScript")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NativeScript, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut native_script__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScriptPubkey => {
                            if native_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptPubkey"));
                            }
                            native_script__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| native_script::NativeScript::ScriptPubkey(x.0));
                        }
                        GeneratedField::ScriptAll => {
                            if native_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptAll"));
                            }
                            native_script__ = map_.next_value::<::std::option::Option<_>>()?.map(native_script::NativeScript::ScriptAll)
;
                        }
                        GeneratedField::ScriptAny => {
                            if native_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptAny"));
                            }
                            native_script__ = map_.next_value::<::std::option::Option<_>>()?.map(native_script::NativeScript::ScriptAny)
;
                        }
                        GeneratedField::ScriptNOfK => {
                            if native_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptNOfK"));
                            }
                            native_script__ = map_.next_value::<::std::option::Option<_>>()?.map(native_script::NativeScript::ScriptNOfK)
;
                        }
                        GeneratedField::InvalidBefore => {
                            if native_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invalidBefore"));
                            }
                            native_script__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| native_script::NativeScript::InvalidBefore(x.0));
                        }
                        GeneratedField::InvalidHereafter => {
                            if native_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invalidHereafter"));
                            }
                            native_script__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| native_script::NativeScript::InvalidHereafter(x.0));
                        }
                    }
                }
                Ok(NativeScript {
                    native_script: native_script__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.NativeScript", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NativeScriptList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.NativeScriptList", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NativeScriptList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NativeScriptList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.NativeScriptList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NativeScriptList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NativeScriptList {
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.NativeScriptList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NewCommitteeCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.committee_cold_credential.is_some() {
            len += 1;
        }
        if self.expires_epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.NewCommitteeCredentials", len)?;
        if let Some(v) = self.committee_cold_credential.as_ref() {
            struct_ser.serialize_field("committeeColdCredential", v)?;
        }
        if self.expires_epoch != 0 {
            struct_ser.serialize_field("expiresEpoch", &self.expires_epoch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NewCommitteeCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "committee_cold_credential",
            "committeeColdCredential",
            "expires_epoch",
            "expiresEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommitteeColdCredential,
            ExpiresEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "committeeColdCredential" | "committee_cold_credential" => Ok(GeneratedField::CommitteeColdCredential),
                            "expiresEpoch" | "expires_epoch" => Ok(GeneratedField::ExpiresEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NewCommitteeCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.NewCommitteeCredentials")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NewCommitteeCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut committee_cold_credential__ = None;
                let mut expires_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CommitteeColdCredential => {
                            if committee_cold_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeColdCredential"));
                            }
                            committee_cold_credential__ = map_.next_value()?;
                        }
                        GeneratedField::ExpiresEpoch => {
                            if expires_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresEpoch"));
                            }
                            expires_epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(NewCommitteeCredentials {
                    committee_cold_credential: committee_cold_credential__,
                    expires_epoch: expires_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.NewCommitteeCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NewConstitutionAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gov_action_id.is_some() {
            len += 1;
        }
        if self.constitution.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.NewConstitutionAction", len)?;
        if let Some(v) = self.gov_action_id.as_ref() {
            struct_ser.serialize_field("govActionId", v)?;
        }
        if let Some(v) = self.constitution.as_ref() {
            struct_ser.serialize_field("constitution", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NewConstitutionAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gov_action_id",
            "govActionId",
            "constitution",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GovActionId,
            Constitution,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "govActionId" | "gov_action_id" => Ok(GeneratedField::GovActionId),
                            "constitution" => Ok(GeneratedField::Constitution),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NewConstitutionAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.NewConstitutionAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NewConstitutionAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gov_action_id__ = None;
                let mut constitution__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GovActionId => {
                            if gov_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionId"));
                            }
                            gov_action_id__ = map_.next_value()?;
                        }
                        GeneratedField::Constitution => {
                            if constitution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constitution"));
                            }
                            constitution__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NewConstitutionAction {
                    gov_action_id: gov_action_id__,
                    constitution: constitution__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.NewConstitutionAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NoConfidenceAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gov_action_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.NoConfidenceAction", len)?;
        if let Some(v) = self.gov_action_id.as_ref() {
            struct_ser.serialize_field("govActionId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NoConfidenceAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gov_action_id",
            "govActionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GovActionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "govActionId" | "gov_action_id" => Ok(GeneratedField::GovActionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NoConfidenceAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.NoConfidenceAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NoConfidenceAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gov_action_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GovActionId => {
                            if gov_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionId"));
                            }
                            gov_action_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NoConfidenceAction {
                    gov_action_id: gov_action_id__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.NoConfidenceAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.coins_per_utxo_byte != 0 {
            len += 1;
        }
        if self.max_tx_size != 0 {
            len += 1;
        }
        if self.min_fee_coefficient != 0 {
            len += 1;
        }
        if self.min_fee_constant != 0 {
            len += 1;
        }
        if self.max_block_body_size != 0 {
            len += 1;
        }
        if self.max_block_header_size != 0 {
            len += 1;
        }
        if self.stake_key_deposit != 0 {
            len += 1;
        }
        if self.pool_deposit != 0 {
            len += 1;
        }
        if self.pool_retirement_epoch_bound != 0 {
            len += 1;
        }
        if self.desired_number_of_pools != 0 {
            len += 1;
        }
        if self.pool_influence.is_some() {
            len += 1;
        }
        if self.monetary_expansion.is_some() {
            len += 1;
        }
        if self.treasury_expansion.is_some() {
            len += 1;
        }
        if self.min_pool_cost != 0 {
            len += 1;
        }
        if self.protocol_version.is_some() {
            len += 1;
        }
        if self.max_value_size != 0 {
            len += 1;
        }
        if self.collateral_percentage != 0 {
            len += 1;
        }
        if self.max_collateral_inputs != 0 {
            len += 1;
        }
        if self.cost_models.is_some() {
            len += 1;
        }
        if self.prices.is_some() {
            len += 1;
        }
        if self.max_execution_units_per_transaction.is_some() {
            len += 1;
        }
        if self.max_execution_units_per_block.is_some() {
            len += 1;
        }
        if self.min_fee_script_ref_cost_per_byte.is_some() {
            len += 1;
        }
        if self.pool_voting_thresholds.is_some() {
            len += 1;
        }
        if self.drep_voting_thresholds.is_some() {
            len += 1;
        }
        if self.min_committee_size != 0 {
            len += 1;
        }
        if self.committee_term_limit != 0 {
            len += 1;
        }
        if self.governance_action_validity_period != 0 {
            len += 1;
        }
        if self.governance_action_deposit != 0 {
            len += 1;
        }
        if self.drep_deposit != 0 {
            len += 1;
        }
        if self.drep_inactivity_period != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PParams", len)?;
        if self.coins_per_utxo_byte != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coinsPerUtxoByte", ToString::to_string(&self.coins_per_utxo_byte).as_str())?;
        }
        if self.max_tx_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxTxSize", ToString::to_string(&self.max_tx_size).as_str())?;
        }
        if self.min_fee_coefficient != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minFeeCoefficient", ToString::to_string(&self.min_fee_coefficient).as_str())?;
        }
        if self.min_fee_constant != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minFeeConstant", ToString::to_string(&self.min_fee_constant).as_str())?;
        }
        if self.max_block_body_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxBlockBodySize", ToString::to_string(&self.max_block_body_size).as_str())?;
        }
        if self.max_block_header_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxBlockHeaderSize", ToString::to_string(&self.max_block_header_size).as_str())?;
        }
        if self.stake_key_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("stakeKeyDeposit", ToString::to_string(&self.stake_key_deposit).as_str())?;
        }
        if self.pool_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolDeposit", ToString::to_string(&self.pool_deposit).as_str())?;
        }
        if self.pool_retirement_epoch_bound != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolRetirementEpochBound", ToString::to_string(&self.pool_retirement_epoch_bound).as_str())?;
        }
        if self.desired_number_of_pools != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("desiredNumberOfPools", ToString::to_string(&self.desired_number_of_pools).as_str())?;
        }
        if let Some(v) = self.pool_influence.as_ref() {
            struct_ser.serialize_field("poolInfluence", v)?;
        }
        if let Some(v) = self.monetary_expansion.as_ref() {
            struct_ser.serialize_field("monetaryExpansion", v)?;
        }
        if let Some(v) = self.treasury_expansion.as_ref() {
            struct_ser.serialize_field("treasuryExpansion", v)?;
        }
        if self.min_pool_cost != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minPoolCost", ToString::to_string(&self.min_pool_cost).as_str())?;
        }
        if let Some(v) = self.protocol_version.as_ref() {
            struct_ser.serialize_field("protocolVersion", v)?;
        }
        if self.max_value_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxValueSize", ToString::to_string(&self.max_value_size).as_str())?;
        }
        if self.collateral_percentage != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("collateralPercentage", ToString::to_string(&self.collateral_percentage).as_str())?;
        }
        if self.max_collateral_inputs != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxCollateralInputs", ToString::to_string(&self.max_collateral_inputs).as_str())?;
        }
        if let Some(v) = self.cost_models.as_ref() {
            struct_ser.serialize_field("costModels", v)?;
        }
        if let Some(v) = self.prices.as_ref() {
            struct_ser.serialize_field("prices", v)?;
        }
        if let Some(v) = self.max_execution_units_per_transaction.as_ref() {
            struct_ser.serialize_field("maxExecutionUnitsPerTransaction", v)?;
        }
        if let Some(v) = self.max_execution_units_per_block.as_ref() {
            struct_ser.serialize_field("maxExecutionUnitsPerBlock", v)?;
        }
        if let Some(v) = self.min_fee_script_ref_cost_per_byte.as_ref() {
            struct_ser.serialize_field("minFeeScriptRefCostPerByte", v)?;
        }
        if let Some(v) = self.pool_voting_thresholds.as_ref() {
            struct_ser.serialize_field("poolVotingThresholds", v)?;
        }
        if let Some(v) = self.drep_voting_thresholds.as_ref() {
            struct_ser.serialize_field("drepVotingThresholds", v)?;
        }
        if self.min_committee_size != 0 {
            struct_ser.serialize_field("minCommitteeSize", &self.min_committee_size)?;
        }
        if self.committee_term_limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("committeeTermLimit", ToString::to_string(&self.committee_term_limit).as_str())?;
        }
        if self.governance_action_validity_period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("governanceActionValidityPeriod", ToString::to_string(&self.governance_action_validity_period).as_str())?;
        }
        if self.governance_action_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("governanceActionDeposit", ToString::to_string(&self.governance_action_deposit).as_str())?;
        }
        if self.drep_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("drepDeposit", ToString::to_string(&self.drep_deposit).as_str())?;
        }
        if self.drep_inactivity_period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("drepInactivityPeriod", ToString::to_string(&self.drep_inactivity_period).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "coins_per_utxo_byte",
            "coinsPerUtxoByte",
            "max_tx_size",
            "maxTxSize",
            "min_fee_coefficient",
            "minFeeCoefficient",
            "min_fee_constant",
            "minFeeConstant",
            "max_block_body_size",
            "maxBlockBodySize",
            "max_block_header_size",
            "maxBlockHeaderSize",
            "stake_key_deposit",
            "stakeKeyDeposit",
            "pool_deposit",
            "poolDeposit",
            "pool_retirement_epoch_bound",
            "poolRetirementEpochBound",
            "desired_number_of_pools",
            "desiredNumberOfPools",
            "pool_influence",
            "poolInfluence",
            "monetary_expansion",
            "monetaryExpansion",
            "treasury_expansion",
            "treasuryExpansion",
            "min_pool_cost",
            "minPoolCost",
            "protocol_version",
            "protocolVersion",
            "max_value_size",
            "maxValueSize",
            "collateral_percentage",
            "collateralPercentage",
            "max_collateral_inputs",
            "maxCollateralInputs",
            "cost_models",
            "costModels",
            "prices",
            "max_execution_units_per_transaction",
            "maxExecutionUnitsPerTransaction",
            "max_execution_units_per_block",
            "maxExecutionUnitsPerBlock",
            "min_fee_script_ref_cost_per_byte",
            "minFeeScriptRefCostPerByte",
            "pool_voting_thresholds",
            "poolVotingThresholds",
            "drep_voting_thresholds",
            "drepVotingThresholds",
            "min_committee_size",
            "minCommitteeSize",
            "committee_term_limit",
            "committeeTermLimit",
            "governance_action_validity_period",
            "governanceActionValidityPeriod",
            "governance_action_deposit",
            "governanceActionDeposit",
            "drep_deposit",
            "drepDeposit",
            "drep_inactivity_period",
            "drepInactivityPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CoinsPerUtxoByte,
            MaxTxSize,
            MinFeeCoefficient,
            MinFeeConstant,
            MaxBlockBodySize,
            MaxBlockHeaderSize,
            StakeKeyDeposit,
            PoolDeposit,
            PoolRetirementEpochBound,
            DesiredNumberOfPools,
            PoolInfluence,
            MonetaryExpansion,
            TreasuryExpansion,
            MinPoolCost,
            ProtocolVersion,
            MaxValueSize,
            CollateralPercentage,
            MaxCollateralInputs,
            CostModels,
            Prices,
            MaxExecutionUnitsPerTransaction,
            MaxExecutionUnitsPerBlock,
            MinFeeScriptRefCostPerByte,
            PoolVotingThresholds,
            DrepVotingThresholds,
            MinCommitteeSize,
            CommitteeTermLimit,
            GovernanceActionValidityPeriod,
            GovernanceActionDeposit,
            DrepDeposit,
            DrepInactivityPeriod,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "coinsPerUtxoByte" | "coins_per_utxo_byte" => Ok(GeneratedField::CoinsPerUtxoByte),
                            "maxTxSize" | "max_tx_size" => Ok(GeneratedField::MaxTxSize),
                            "minFeeCoefficient" | "min_fee_coefficient" => Ok(GeneratedField::MinFeeCoefficient),
                            "minFeeConstant" | "min_fee_constant" => Ok(GeneratedField::MinFeeConstant),
                            "maxBlockBodySize" | "max_block_body_size" => Ok(GeneratedField::MaxBlockBodySize),
                            "maxBlockHeaderSize" | "max_block_header_size" => Ok(GeneratedField::MaxBlockHeaderSize),
                            "stakeKeyDeposit" | "stake_key_deposit" => Ok(GeneratedField::StakeKeyDeposit),
                            "poolDeposit" | "pool_deposit" => Ok(GeneratedField::PoolDeposit),
                            "poolRetirementEpochBound" | "pool_retirement_epoch_bound" => Ok(GeneratedField::PoolRetirementEpochBound),
                            "desiredNumberOfPools" | "desired_number_of_pools" => Ok(GeneratedField::DesiredNumberOfPools),
                            "poolInfluence" | "pool_influence" => Ok(GeneratedField::PoolInfluence),
                            "monetaryExpansion" | "monetary_expansion" => Ok(GeneratedField::MonetaryExpansion),
                            "treasuryExpansion" | "treasury_expansion" => Ok(GeneratedField::TreasuryExpansion),
                            "minPoolCost" | "min_pool_cost" => Ok(GeneratedField::MinPoolCost),
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            "maxValueSize" | "max_value_size" => Ok(GeneratedField::MaxValueSize),
                            "collateralPercentage" | "collateral_percentage" => Ok(GeneratedField::CollateralPercentage),
                            "maxCollateralInputs" | "max_collateral_inputs" => Ok(GeneratedField::MaxCollateralInputs),
                            "costModels" | "cost_models" => Ok(GeneratedField::CostModels),
                            "prices" => Ok(GeneratedField::Prices),
                            "maxExecutionUnitsPerTransaction" | "max_execution_units_per_transaction" => Ok(GeneratedField::MaxExecutionUnitsPerTransaction),
                            "maxExecutionUnitsPerBlock" | "max_execution_units_per_block" => Ok(GeneratedField::MaxExecutionUnitsPerBlock),
                            "minFeeScriptRefCostPerByte" | "min_fee_script_ref_cost_per_byte" => Ok(GeneratedField::MinFeeScriptRefCostPerByte),
                            "poolVotingThresholds" | "pool_voting_thresholds" => Ok(GeneratedField::PoolVotingThresholds),
                            "drepVotingThresholds" | "drep_voting_thresholds" => Ok(GeneratedField::DrepVotingThresholds),
                            "minCommitteeSize" | "min_committee_size" => Ok(GeneratedField::MinCommitteeSize),
                            "committeeTermLimit" | "committee_term_limit" => Ok(GeneratedField::CommitteeTermLimit),
                            "governanceActionValidityPeriod" | "governance_action_validity_period" => Ok(GeneratedField::GovernanceActionValidityPeriod),
                            "governanceActionDeposit" | "governance_action_deposit" => Ok(GeneratedField::GovernanceActionDeposit),
                            "drepDeposit" | "drep_deposit" => Ok(GeneratedField::DrepDeposit),
                            "drepInactivityPeriod" | "drep_inactivity_period" => Ok(GeneratedField::DrepInactivityPeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut coins_per_utxo_byte__ = None;
                let mut max_tx_size__ = None;
                let mut min_fee_coefficient__ = None;
                let mut min_fee_constant__ = None;
                let mut max_block_body_size__ = None;
                let mut max_block_header_size__ = None;
                let mut stake_key_deposit__ = None;
                let mut pool_deposit__ = None;
                let mut pool_retirement_epoch_bound__ = None;
                let mut desired_number_of_pools__ = None;
                let mut pool_influence__ = None;
                let mut monetary_expansion__ = None;
                let mut treasury_expansion__ = None;
                let mut min_pool_cost__ = None;
                let mut protocol_version__ = None;
                let mut max_value_size__ = None;
                let mut collateral_percentage__ = None;
                let mut max_collateral_inputs__ = None;
                let mut cost_models__ = None;
                let mut prices__ = None;
                let mut max_execution_units_per_transaction__ = None;
                let mut max_execution_units_per_block__ = None;
                let mut min_fee_script_ref_cost_per_byte__ = None;
                let mut pool_voting_thresholds__ = None;
                let mut drep_voting_thresholds__ = None;
                let mut min_committee_size__ = None;
                let mut committee_term_limit__ = None;
                let mut governance_action_validity_period__ = None;
                let mut governance_action_deposit__ = None;
                let mut drep_deposit__ = None;
                let mut drep_inactivity_period__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CoinsPerUtxoByte => {
                            if coins_per_utxo_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coinsPerUtxoByte"));
                            }
                            coins_per_utxo_byte__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxTxSize => {
                            if max_tx_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTxSize"));
                            }
                            max_tx_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinFeeCoefficient => {
                            if min_fee_coefficient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minFeeCoefficient"));
                            }
                            min_fee_coefficient__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinFeeConstant => {
                            if min_fee_constant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minFeeConstant"));
                            }
                            min_fee_constant__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxBlockBodySize => {
                            if max_block_body_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBlockBodySize"));
                            }
                            max_block_body_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxBlockHeaderSize => {
                            if max_block_header_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBlockHeaderSize"));
                            }
                            max_block_header_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StakeKeyDeposit => {
                            if stake_key_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeKeyDeposit"));
                            }
                            stake_key_deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PoolDeposit => {
                            if pool_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolDeposit"));
                            }
                            pool_deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PoolRetirementEpochBound => {
                            if pool_retirement_epoch_bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolRetirementEpochBound"));
                            }
                            pool_retirement_epoch_bound__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DesiredNumberOfPools => {
                            if desired_number_of_pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("desiredNumberOfPools"));
                            }
                            desired_number_of_pools__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PoolInfluence => {
                            if pool_influence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolInfluence"));
                            }
                            pool_influence__ = map_.next_value()?;
                        }
                        GeneratedField::MonetaryExpansion => {
                            if monetary_expansion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monetaryExpansion"));
                            }
                            monetary_expansion__ = map_.next_value()?;
                        }
                        GeneratedField::TreasuryExpansion => {
                            if treasury_expansion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treasuryExpansion"));
                            }
                            treasury_expansion__ = map_.next_value()?;
                        }
                        GeneratedField::MinPoolCost => {
                            if min_pool_cost__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPoolCost"));
                            }
                            min_pool_cost__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = map_.next_value()?;
                        }
                        GeneratedField::MaxValueSize => {
                            if max_value_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxValueSize"));
                            }
                            max_value_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CollateralPercentage => {
                            if collateral_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralPercentage"));
                            }
                            collateral_percentage__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxCollateralInputs => {
                            if max_collateral_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCollateralInputs"));
                            }
                            max_collateral_inputs__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CostModels => {
                            if cost_models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("costModels"));
                            }
                            cost_models__ = map_.next_value()?;
                        }
                        GeneratedField::Prices => {
                            if prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prices"));
                            }
                            prices__ = map_.next_value()?;
                        }
                        GeneratedField::MaxExecutionUnitsPerTransaction => {
                            if max_execution_units_per_transaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxExecutionUnitsPerTransaction"));
                            }
                            max_execution_units_per_transaction__ = map_.next_value()?;
                        }
                        GeneratedField::MaxExecutionUnitsPerBlock => {
                            if max_execution_units_per_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxExecutionUnitsPerBlock"));
                            }
                            max_execution_units_per_block__ = map_.next_value()?;
                        }
                        GeneratedField::MinFeeScriptRefCostPerByte => {
                            if min_fee_script_ref_cost_per_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minFeeScriptRefCostPerByte"));
                            }
                            min_fee_script_ref_cost_per_byte__ = map_.next_value()?;
                        }
                        GeneratedField::PoolVotingThresholds => {
                            if pool_voting_thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolVotingThresholds"));
                            }
                            pool_voting_thresholds__ = map_.next_value()?;
                        }
                        GeneratedField::DrepVotingThresholds => {
                            if drep_voting_thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepVotingThresholds"));
                            }
                            drep_voting_thresholds__ = map_.next_value()?;
                        }
                        GeneratedField::MinCommitteeSize => {
                            if min_committee_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCommitteeSize"));
                            }
                            min_committee_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommitteeTermLimit => {
                            if committee_term_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeTermLimit"));
                            }
                            committee_term_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GovernanceActionValidityPeriod => {
                            if governance_action_validity_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("governanceActionValidityPeriod"));
                            }
                            governance_action_validity_period__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GovernanceActionDeposit => {
                            if governance_action_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("governanceActionDeposit"));
                            }
                            governance_action_deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DrepDeposit => {
                            if drep_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepDeposit"));
                            }
                            drep_deposit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DrepInactivityPeriod => {
                            if drep_inactivity_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepInactivityPeriod"));
                            }
                            drep_inactivity_period__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PParams {
                    coins_per_utxo_byte: coins_per_utxo_byte__.unwrap_or_default(),
                    max_tx_size: max_tx_size__.unwrap_or_default(),
                    min_fee_coefficient: min_fee_coefficient__.unwrap_or_default(),
                    min_fee_constant: min_fee_constant__.unwrap_or_default(),
                    max_block_body_size: max_block_body_size__.unwrap_or_default(),
                    max_block_header_size: max_block_header_size__.unwrap_or_default(),
                    stake_key_deposit: stake_key_deposit__.unwrap_or_default(),
                    pool_deposit: pool_deposit__.unwrap_or_default(),
                    pool_retirement_epoch_bound: pool_retirement_epoch_bound__.unwrap_or_default(),
                    desired_number_of_pools: desired_number_of_pools__.unwrap_or_default(),
                    pool_influence: pool_influence__,
                    monetary_expansion: monetary_expansion__,
                    treasury_expansion: treasury_expansion__,
                    min_pool_cost: min_pool_cost__.unwrap_or_default(),
                    protocol_version: protocol_version__,
                    max_value_size: max_value_size__.unwrap_or_default(),
                    collateral_percentage: collateral_percentage__.unwrap_or_default(),
                    max_collateral_inputs: max_collateral_inputs__.unwrap_or_default(),
                    cost_models: cost_models__,
                    prices: prices__,
                    max_execution_units_per_transaction: max_execution_units_per_transaction__,
                    max_execution_units_per_block: max_execution_units_per_block__,
                    min_fee_script_ref_cost_per_byte: min_fee_script_ref_cost_per_byte__,
                    pool_voting_thresholds: pool_voting_thresholds__,
                    drep_voting_thresholds: drep_voting_thresholds__,
                    min_committee_size: min_committee_size__.unwrap_or_default(),
                    committee_term_limit: committee_term_limit__.unwrap_or_default(),
                    governance_action_validity_period: governance_action_validity_period__.unwrap_or_default(),
                    governance_action_deposit: governance_action_deposit__.unwrap_or_default(),
                    drep_deposit: drep_deposit__.unwrap_or_default(),
                    drep_inactivity_period: drep_inactivity_period__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParameterChangeAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gov_action_id.is_some() {
            len += 1;
        }
        if self.protocol_param_update.is_some() {
            len += 1;
        }
        if !self.policy_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ParameterChangeAction", len)?;
        if let Some(v) = self.gov_action_id.as_ref() {
            struct_ser.serialize_field("govActionId", v)?;
        }
        if let Some(v) = self.protocol_param_update.as_ref() {
            struct_ser.serialize_field("protocolParamUpdate", v)?;
        }
        if !self.policy_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("policyHash", pbjson::private::base64::encode(&self.policy_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParameterChangeAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gov_action_id",
            "govActionId",
            "protocol_param_update",
            "protocolParamUpdate",
            "policy_hash",
            "policyHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GovActionId,
            ProtocolParamUpdate,
            PolicyHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "govActionId" | "gov_action_id" => Ok(GeneratedField::GovActionId),
                            "protocolParamUpdate" | "protocol_param_update" => Ok(GeneratedField::ProtocolParamUpdate),
                            "policyHash" | "policy_hash" => Ok(GeneratedField::PolicyHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParameterChangeAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ParameterChangeAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParameterChangeAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gov_action_id__ = None;
                let mut protocol_param_update__ = None;
                let mut policy_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GovActionId => {
                            if gov_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionId"));
                            }
                            gov_action_id__ = map_.next_value()?;
                        }
                        GeneratedField::ProtocolParamUpdate => {
                            if protocol_param_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolParamUpdate"));
                            }
                            protocol_param_update__ = map_.next_value()?;
                        }
                        GeneratedField::PolicyHash => {
                            if policy_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyHash"));
                            }
                            policy_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ParameterChangeAction {
                    gov_action_id: gov_action_id__,
                    protocol_param_update: protocol_param_update__,
                    policy_hash: policy_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ParameterChangeAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlutusData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plutus_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PlutusData", len)?;
        if let Some(v) = self.plutus_data.as_ref() {
            match v {
                plutus_data::PlutusData::Constr(v) => {
                    struct_ser.serialize_field("constr", v)?;
                }
                plutus_data::PlutusData::Map(v) => {
                    struct_ser.serialize_field("map", v)?;
                }
                plutus_data::PlutusData::BigInt(v) => {
                    struct_ser.serialize_field("bigInt", v)?;
                }
                plutus_data::PlutusData::BoundedBytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("boundedBytes", pbjson::private::base64::encode(&v).as_str())?;
                }
                plutus_data::PlutusData::Array(v) => {
                    struct_ser.serialize_field("array", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlutusData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "constr",
            "map",
            "big_int",
            "bigInt",
            "bounded_bytes",
            "boundedBytes",
            "array",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Constr,
            Map,
            BigInt,
            BoundedBytes,
            Array,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "constr" => Ok(GeneratedField::Constr),
                            "map" => Ok(GeneratedField::Map),
                            "bigInt" | "big_int" => Ok(GeneratedField::BigInt),
                            "boundedBytes" | "bounded_bytes" => Ok(GeneratedField::BoundedBytes),
                            "array" => Ok(GeneratedField::Array),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlutusData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PlutusData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlutusData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plutus_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Constr => {
                            if plutus_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constr"));
                            }
                            plutus_data__ = map_.next_value::<::std::option::Option<_>>()?.map(plutus_data::PlutusData::Constr)
;
                        }
                        GeneratedField::Map => {
                            if plutus_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            plutus_data__ = map_.next_value::<::std::option::Option<_>>()?.map(plutus_data::PlutusData::Map)
;
                        }
                        GeneratedField::BigInt => {
                            if plutus_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigInt"));
                            }
                            plutus_data__ = map_.next_value::<::std::option::Option<_>>()?.map(plutus_data::PlutusData::BigInt)
;
                        }
                        GeneratedField::BoundedBytes => {
                            if plutus_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundedBytes"));
                            }
                            plutus_data__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| plutus_data::PlutusData::BoundedBytes(x.0));
                        }
                        GeneratedField::Array => {
                            if plutus_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("array"));
                            }
                            plutus_data__ = map_.next_value::<::std::option::Option<_>>()?.map(plutus_data::PlutusData::Array)
;
                        }
                    }
                }
                Ok(PlutusData {
                    plutus_data: plutus_data__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PlutusData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlutusDataArray {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PlutusDataArray", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlutusDataArray {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlutusDataArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PlutusDataArray")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlutusDataArray, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PlutusDataArray {
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PlutusDataArray", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlutusDataMap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PlutusDataMap", len)?;
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlutusDataMap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pairs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlutusDataMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PlutusDataMap")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlutusDataMap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pairs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PlutusDataMap {
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PlutusDataMap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlutusDataPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PlutusDataPair", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlutusDataPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlutusDataPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PlutusDataPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlutusDataPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PlutusDataPair {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PlutusDataPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PoolMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PoolMetadata", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PoolMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            Hash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PoolMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PoolMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PoolMetadata {
                    url: url__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PoolMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PoolRegistrationCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operator.is_empty() {
            len += 1;
        }
        if !self.vrf_keyhash.is_empty() {
            len += 1;
        }
        if self.pledge != 0 {
            len += 1;
        }
        if self.cost != 0 {
            len += 1;
        }
        if self.margin.is_some() {
            len += 1;
        }
        if !self.reward_account.is_empty() {
            len += 1;
        }
        if !self.pool_owners.is_empty() {
            len += 1;
        }
        if !self.relays.is_empty() {
            len += 1;
        }
        if self.pool_metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PoolRegistrationCert", len)?;
        if !self.operator.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("operator", pbjson::private::base64::encode(&self.operator).as_str())?;
        }
        if !self.vrf_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("vrfKeyhash", pbjson::private::base64::encode(&self.vrf_keyhash).as_str())?;
        }
        if self.pledge != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("pledge", ToString::to_string(&self.pledge).as_str())?;
        }
        if self.cost != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("cost", ToString::to_string(&self.cost).as_str())?;
        }
        if let Some(v) = self.margin.as_ref() {
            struct_ser.serialize_field("margin", v)?;
        }
        if !self.reward_account.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("rewardAccount", pbjson::private::base64::encode(&self.reward_account).as_str())?;
        }
        if !self.pool_owners.is_empty() {
            struct_ser.serialize_field("poolOwners", &self.pool_owners.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.relays.is_empty() {
            struct_ser.serialize_field("relays", &self.relays)?;
        }
        if let Some(v) = self.pool_metadata.as_ref() {
            struct_ser.serialize_field("poolMetadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PoolRegistrationCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operator",
            "vrf_keyhash",
            "vrfKeyhash",
            "pledge",
            "cost",
            "margin",
            "reward_account",
            "rewardAccount",
            "pool_owners",
            "poolOwners",
            "relays",
            "pool_metadata",
            "poolMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operator,
            VrfKeyhash,
            Pledge,
            Cost,
            Margin,
            RewardAccount,
            PoolOwners,
            Relays,
            PoolMetadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "operator" => Ok(GeneratedField::Operator),
                            "vrfKeyhash" | "vrf_keyhash" => Ok(GeneratedField::VrfKeyhash),
                            "pledge" => Ok(GeneratedField::Pledge),
                            "cost" => Ok(GeneratedField::Cost),
                            "margin" => Ok(GeneratedField::Margin),
                            "rewardAccount" | "reward_account" => Ok(GeneratedField::RewardAccount),
                            "poolOwners" | "pool_owners" => Ok(GeneratedField::PoolOwners),
                            "relays" => Ok(GeneratedField::Relays),
                            "poolMetadata" | "pool_metadata" => Ok(GeneratedField::PoolMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolRegistrationCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PoolRegistrationCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PoolRegistrationCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operator__ = None;
                let mut vrf_keyhash__ = None;
                let mut pledge__ = None;
                let mut cost__ = None;
                let mut margin__ = None;
                let mut reward_account__ = None;
                let mut pool_owners__ = None;
                let mut relays__ = None;
                let mut pool_metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VrfKeyhash => {
                            if vrf_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vrfKeyhash"));
                            }
                            vrf_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pledge => {
                            if pledge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pledge"));
                            }
                            pledge__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cost => {
                            if cost__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            cost__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Margin => {
                            if margin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("margin"));
                            }
                            margin__ = map_.next_value()?;
                        }
                        GeneratedField::RewardAccount => {
                            if reward_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAccount"));
                            }
                            reward_account__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PoolOwners => {
                            if pool_owners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolOwners"));
                            }
                            pool_owners__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Relays => {
                            if relays__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relays"));
                            }
                            relays__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolMetadata => {
                            if pool_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolMetadata"));
                            }
                            pool_metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PoolRegistrationCert {
                    operator: operator__.unwrap_or_default(),
                    vrf_keyhash: vrf_keyhash__.unwrap_or_default(),
                    pledge: pledge__.unwrap_or_default(),
                    cost: cost__.unwrap_or_default(),
                    margin: margin__,
                    reward_account: reward_account__.unwrap_or_default(),
                    pool_owners: pool_owners__.unwrap_or_default(),
                    relays: relays__.unwrap_or_default(),
                    pool_metadata: pool_metadata__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PoolRegistrationCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PoolRetirementCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pool_keyhash.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PoolRetirementCert", len)?;
        if !self.pool_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolKeyhash", pbjson::private::base64::encode(&self.pool_keyhash).as_str())?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PoolRetirementCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pool_keyhash",
            "poolKeyhash",
            "epoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolKeyhash,
            Epoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "poolKeyhash" | "pool_keyhash" => Ok(GeneratedField::PoolKeyhash),
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolRetirementCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PoolRetirementCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PoolRetirementCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pool_keyhash__ = None;
                let mut epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PoolKeyhash => {
                            if pool_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolKeyhash"));
                            }
                            pool_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PoolRetirementCert {
                    pool_keyhash: pool_keyhash__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PoolRetirementCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PoolVotingThresholds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.motion_no_confidence.is_some() {
            len += 1;
        }
        if self.committee_normal.is_some() {
            len += 1;
        }
        if self.committee_no_confidence.is_some() {
            len += 1;
        }
        if self.hard_fork_initiation.is_some() {
            len += 1;
        }
        if self.pp_security_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.PoolVotingThresholds", len)?;
        if let Some(v) = self.motion_no_confidence.as_ref() {
            struct_ser.serialize_field("motionNoConfidence", v)?;
        }
        if let Some(v) = self.committee_normal.as_ref() {
            struct_ser.serialize_field("committeeNormal", v)?;
        }
        if let Some(v) = self.committee_no_confidence.as_ref() {
            struct_ser.serialize_field("committeeNoConfidence", v)?;
        }
        if let Some(v) = self.hard_fork_initiation.as_ref() {
            struct_ser.serialize_field("hardForkInitiation", v)?;
        }
        if let Some(v) = self.pp_security_group.as_ref() {
            struct_ser.serialize_field("ppSecurityGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PoolVotingThresholds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "motion_no_confidence",
            "motionNoConfidence",
            "committee_normal",
            "committeeNormal",
            "committee_no_confidence",
            "committeeNoConfidence",
            "hard_fork_initiation",
            "hardForkInitiation",
            "pp_security_group",
            "ppSecurityGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MotionNoConfidence,
            CommitteeNormal,
            CommitteeNoConfidence,
            HardForkInitiation,
            PpSecurityGroup,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "motionNoConfidence" | "motion_no_confidence" => Ok(GeneratedField::MotionNoConfidence),
                            "committeeNormal" | "committee_normal" => Ok(GeneratedField::CommitteeNormal),
                            "committeeNoConfidence" | "committee_no_confidence" => Ok(GeneratedField::CommitteeNoConfidence),
                            "hardForkInitiation" | "hard_fork_initiation" => Ok(GeneratedField::HardForkInitiation),
                            "ppSecurityGroup" | "pp_security_group" => Ok(GeneratedField::PpSecurityGroup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolVotingThresholds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.PoolVotingThresholds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PoolVotingThresholds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut motion_no_confidence__ = None;
                let mut committee_normal__ = None;
                let mut committee_no_confidence__ = None;
                let mut hard_fork_initiation__ = None;
                let mut pp_security_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MotionNoConfidence => {
                            if motion_no_confidence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("motionNoConfidence"));
                            }
                            motion_no_confidence__ = map_.next_value()?;
                        }
                        GeneratedField::CommitteeNormal => {
                            if committee_normal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeNormal"));
                            }
                            committee_normal__ = map_.next_value()?;
                        }
                        GeneratedField::CommitteeNoConfidence => {
                            if committee_no_confidence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeNoConfidence"));
                            }
                            committee_no_confidence__ = map_.next_value()?;
                        }
                        GeneratedField::HardForkInitiation => {
                            if hard_fork_initiation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hardForkInitiation"));
                            }
                            hard_fork_initiation__ = map_.next_value()?;
                        }
                        GeneratedField::PpSecurityGroup => {
                            if pp_security_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ppSecurityGroup"));
                            }
                            pp_security_group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PoolVotingThresholds {
                    motion_no_confidence: motion_no_confidence__,
                    committee_normal: committee_normal__,
                    committee_no_confidence: committee_no_confidence__,
                    hard_fork_initiation: hard_fork_initiation__,
                    pp_security_group: pp_security_group__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.PoolVotingThresholds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProtocolConsts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.k != 0 {
            len += 1;
        }
        if self.protocol_magic != 0 {
            len += 1;
        }
        if self.vss_max_ttl != 0 {
            len += 1;
        }
        if self.vss_min_ttl != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ProtocolConsts", len)?;
        if self.k != 0 {
            struct_ser.serialize_field("k", &self.k)?;
        }
        if self.protocol_magic != 0 {
            struct_ser.serialize_field("protocolMagic", &self.protocol_magic)?;
        }
        if self.vss_max_ttl != 0 {
            struct_ser.serialize_field("vssMaxTtl", &self.vss_max_ttl)?;
        }
        if self.vss_min_ttl != 0 {
            struct_ser.serialize_field("vssMinTtl", &self.vss_min_ttl)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtocolConsts {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "k",
            "protocol_magic",
            "protocolMagic",
            "vss_max_ttl",
            "vssMaxTtl",
            "vss_min_ttl",
            "vssMinTtl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            K,
            ProtocolMagic,
            VssMaxTtl,
            VssMinTtl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "k" => Ok(GeneratedField::K),
                            "protocolMagic" | "protocol_magic" => Ok(GeneratedField::ProtocolMagic),
                            "vssMaxTtl" | "vss_max_ttl" => Ok(GeneratedField::VssMaxTtl),
                            "vssMinTtl" | "vss_min_ttl" => Ok(GeneratedField::VssMinTtl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtocolConsts;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ProtocolConsts")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtocolConsts, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut k__ = None;
                let mut protocol_magic__ = None;
                let mut vss_max_ttl__ = None;
                let mut vss_min_ttl__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::K => {
                            if k__.is_some() {
                                return Err(serde::de::Error::duplicate_field("k"));
                            }
                            k__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtocolMagic => {
                            if protocol_magic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolMagic"));
                            }
                            protocol_magic__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VssMaxTtl => {
                            if vss_max_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vssMaxTtl"));
                            }
                            vss_max_ttl__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VssMinTtl => {
                            if vss_min_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vssMinTtl"));
                            }
                            vss_min_ttl__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProtocolConsts {
                    k: k__.unwrap_or_default(),
                    protocol_magic: protocol_magic__.unwrap_or_default(),
                    vss_max_ttl: vss_max_ttl__.unwrap_or_default(),
                    vss_min_ttl: vss_min_ttl__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ProtocolConsts", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProtocolVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major != 0 {
            len += 1;
        }
        if self.minor != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ProtocolVersion", len)?;
        if self.major != 0 {
            struct_ser.serialize_field("major", &self.major)?;
        }
        if self.minor != 0 {
            struct_ser.serialize_field("minor", &self.minor)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtocolVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major",
            "minor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Major,
            Minor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "major" => Ok(GeneratedField::Major),
                            "minor" => Ok(GeneratedField::Minor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtocolVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ProtocolVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtocolVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major__ = None;
                let mut minor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Major => {
                            if major__.is_some() {
                                return Err(serde::de::Error::duplicate_field("major"));
                            }
                            major__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minor => {
                            if minor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minor"));
                            }
                            minor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProtocolVersion {
                    major: major__.unwrap_or_default(),
                    minor: minor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ProtocolVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RationalNumber {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.numerator != 0 {
            len += 1;
        }
        if self.denominator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.RationalNumber", len)?;
        if self.numerator != 0 {
            struct_ser.serialize_field("numerator", &self.numerator)?;
        }
        if self.denominator != 0 {
            struct_ser.serialize_field("denominator", &self.denominator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RationalNumber {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "numerator",
            "denominator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Numerator,
            Denominator,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "numerator" => Ok(GeneratedField::Numerator),
                            "denominator" => Ok(GeneratedField::Denominator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RationalNumber;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.RationalNumber")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RationalNumber, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut numerator__ = None;
                let mut denominator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Numerator => {
                            if numerator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numerator"));
                            }
                            numerator__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Denominator => {
                            if denominator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denominator"));
                            }
                            denominator__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RationalNumber {
                    numerator: numerator__.unwrap_or_default(),
                    denominator: denominator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.RationalNumber", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Redeemer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.purpose != 0 {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.ex_units.is_some() {
            len += 1;
        }
        if !self.original_cbor.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Redeemer", len)?;
        if self.purpose != 0 {
            let v = RedeemerPurpose::try_from(self.purpose)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.purpose)))?;
            struct_ser.serialize_field("purpose", &v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if let Some(v) = self.ex_units.as_ref() {
            struct_ser.serialize_field("exUnits", v)?;
        }
        if !self.original_cbor.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("originalCbor", pbjson::private::base64::encode(&self.original_cbor).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Redeemer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "purpose",
            "payload",
            "index",
            "ex_units",
            "exUnits",
            "original_cbor",
            "originalCbor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Purpose,
            Payload,
            Index,
            ExUnits,
            OriginalCbor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "purpose" => Ok(GeneratedField::Purpose),
                            "payload" => Ok(GeneratedField::Payload),
                            "index" => Ok(GeneratedField::Index),
                            "exUnits" | "ex_units" => Ok(GeneratedField::ExUnits),
                            "originalCbor" | "original_cbor" => Ok(GeneratedField::OriginalCbor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Redeemer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Redeemer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Redeemer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut purpose__ = None;
                let mut payload__ = None;
                let mut index__ = None;
                let mut ex_units__ = None;
                let mut original_cbor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Purpose => {
                            if purpose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            purpose__ = Some(map_.next_value::<RedeemerPurpose>()? as i32);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExUnits => {
                            if ex_units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exUnits"));
                            }
                            ex_units__ = map_.next_value()?;
                        }
                        GeneratedField::OriginalCbor => {
                            if original_cbor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalCbor"));
                            }
                            original_cbor__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Redeemer {
                    purpose: purpose__.unwrap_or_default(),
                    payload: payload__,
                    index: index__.unwrap_or_default(),
                    ex_units: ex_units__,
                    original_cbor: original_cbor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Redeemer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedeemerPurpose {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REDEEMER_PURPOSE_UNSPECIFIED",
            Self::Spend => "REDEEMER_PURPOSE_SPEND",
            Self::Mint => "REDEEMER_PURPOSE_MINT",
            Self::Cert => "REDEEMER_PURPOSE_CERT",
            Self::Reward => "REDEEMER_PURPOSE_REWARD",
            Self::Vote => "REDEEMER_PURPOSE_VOTE",
            Self::Propose => "REDEEMER_PURPOSE_PROPOSE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RedeemerPurpose {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REDEEMER_PURPOSE_UNSPECIFIED",
            "REDEEMER_PURPOSE_SPEND",
            "REDEEMER_PURPOSE_MINT",
            "REDEEMER_PURPOSE_CERT",
            "REDEEMER_PURPOSE_REWARD",
            "REDEEMER_PURPOSE_VOTE",
            "REDEEMER_PURPOSE_PROPOSE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedeemerPurpose;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REDEEMER_PURPOSE_UNSPECIFIED" => Ok(RedeemerPurpose::Unspecified),
                    "REDEEMER_PURPOSE_SPEND" => Ok(RedeemerPurpose::Spend),
                    "REDEEMER_PURPOSE_MINT" => Ok(RedeemerPurpose::Mint),
                    "REDEEMER_PURPOSE_CERT" => Ok(RedeemerPurpose::Cert),
                    "REDEEMER_PURPOSE_REWARD" => Ok(RedeemerPurpose::Reward),
                    "REDEEMER_PURPOSE_VOTE" => Ok(RedeemerPurpose::Vote),
                    "REDEEMER_PURPOSE_PROPOSE" => Ok(RedeemerPurpose::Propose),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.RegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.RegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RegCert {
                    stake_credential: stake_credential__,
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.RegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegDRepCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.drep_credential.is_some() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        if self.anchor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.RegDRepCert", len)?;
        if let Some(v) = self.drep_credential.as_ref() {
            struct_ser.serialize_field("drepCredential", v)?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        if let Some(v) = self.anchor.as_ref() {
            struct_ser.serialize_field("anchor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegDRepCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drep_credential",
            "drepCredential",
            "coin",
            "anchor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DrepCredential,
            Coin,
            Anchor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "drepCredential" | "drep_credential" => Ok(GeneratedField::DrepCredential),
                            "coin" => Ok(GeneratedField::Coin),
                            "anchor" => Ok(GeneratedField::Anchor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegDRepCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.RegDRepCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegDRepCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drep_credential__ = None;
                let mut coin__ = None;
                let mut anchor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DrepCredential => {
                            if drep_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepCredential"));
                            }
                            drep_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Anchor => {
                            if anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            anchor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegDRepCert {
                    drep_credential: drep_credential__,
                    coin: coin__.unwrap_or_default(),
                    anchor: anchor__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.RegDRepCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ip_v4.is_empty() {
            len += 1;
        }
        if !self.ip_v6.is_empty() {
            len += 1;
        }
        if !self.dns_name.is_empty() {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Relay", len)?;
        if !self.ip_v4.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ipV4", pbjson::private::base64::encode(&self.ip_v4).as_str())?;
        }
        if !self.ip_v6.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ipV6", pbjson::private::base64::encode(&self.ip_v6).as_str())?;
        }
        if !self.dns_name.is_empty() {
            struct_ser.serialize_field("dnsName", &self.dns_name)?;
        }
        if self.port != 0 {
            struct_ser.serialize_field("port", &self.port)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relay {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ip_v4",
            "ipV4",
            "ip_v6",
            "ipV6",
            "dns_name",
            "dnsName",
            "port",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpV4,
            IpV6,
            DnsName,
            Port,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ipV4" | "ip_v4" => Ok(GeneratedField::IpV4),
                            "ipV6" | "ip_v6" => Ok(GeneratedField::IpV6),
                            "dnsName" | "dns_name" => Ok(GeneratedField::DnsName),
                            "port" => Ok(GeneratedField::Port),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Relay")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Relay, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip_v4__ = None;
                let mut ip_v6__ = None;
                let mut dns_name__ = None;
                let mut port__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IpV4 => {
                            if ip_v4__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipV4"));
                            }
                            ip_v4__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IpV6 => {
                            if ip_v6__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipV6"));
                            }
                            ip_v6__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DnsName => {
                            if dns_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsName"));
                            }
                            dns_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Relay {
                    ip_v4: ip_v4__.unwrap_or_default(),
                    ip_v6: ip_v6__.unwrap_or_default(),
                    dns_name: dns_name__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Relay", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResignCommitteeColdCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.committee_cold_credential.is_some() {
            len += 1;
        }
        if self.anchor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ResignCommitteeColdCert", len)?;
        if let Some(v) = self.committee_cold_credential.as_ref() {
            struct_ser.serialize_field("committeeColdCredential", v)?;
        }
        if let Some(v) = self.anchor.as_ref() {
            struct_ser.serialize_field("anchor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResignCommitteeColdCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "committee_cold_credential",
            "committeeColdCredential",
            "anchor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommitteeColdCredential,
            Anchor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "committeeColdCredential" | "committee_cold_credential" => Ok(GeneratedField::CommitteeColdCredential),
                            "anchor" => Ok(GeneratedField::Anchor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResignCommitteeColdCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ResignCommitteeColdCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResignCommitteeColdCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut committee_cold_credential__ = None;
                let mut anchor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CommitteeColdCredential => {
                            if committee_cold_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("committeeColdCredential"));
                            }
                            committee_cold_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Anchor => {
                            if anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            anchor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResignCommitteeColdCert {
                    committee_cold_credential: committee_cold_credential__,
                    anchor: anchor__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ResignCommitteeColdCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Script {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.script.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Script", len)?;
        if let Some(v) = self.script.as_ref() {
            match v {
                script::Script::Native(v) => {
                    struct_ser.serialize_field("native", v)?;
                }
                script::Script::PlutusV1(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("plutusV1", pbjson::private::base64::encode(&v).as_str())?;
                }
                script::Script::PlutusV2(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("plutusV2", pbjson::private::base64::encode(&v).as_str())?;
                }
                script::Script::PlutusV3(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("plutusV3", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Script {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "native",
            "plutus_v1",
            "plutusV1",
            "plutus_v2",
            "plutusV2",
            "plutus_v3",
            "plutusV3",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Native,
            PlutusV1,
            PlutusV2,
            PlutusV3,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "native" => Ok(GeneratedField::Native),
                            "plutusV1" | "plutus_v1" => Ok(GeneratedField::PlutusV1),
                            "plutusV2" | "plutus_v2" => Ok(GeneratedField::PlutusV2),
                            "plutusV3" | "plutus_v3" => Ok(GeneratedField::PlutusV3),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Script;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Script")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Script, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut script__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Native => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("native"));
                            }
                            script__ = map_.next_value::<::std::option::Option<_>>()?.map(script::Script::Native)
;
                        }
                        GeneratedField::PlutusV1 => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV1"));
                            }
                            script__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| script::Script::PlutusV1(x.0));
                        }
                        GeneratedField::PlutusV2 => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV2"));
                            }
                            script__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| script::Script::PlutusV2(x.0));
                        }
                        GeneratedField::PlutusV3 => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusV3"));
                            }
                            script__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| script::Script::PlutusV3(x.0));
                        }
                    }
                }
                Ok(Script {
                    script: script__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Script", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScriptNOfK {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.k != 0 {
            len += 1;
        }
        if !self.scripts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.ScriptNOfK", len)?;
        if self.k != 0 {
            struct_ser.serialize_field("k", &self.k)?;
        }
        if !self.scripts.is_empty() {
            struct_ser.serialize_field("scripts", &self.scripts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScriptNOfK {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "k",
            "scripts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            K,
            Scripts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "k" => Ok(GeneratedField::K),
                            "scripts" => Ok(GeneratedField::Scripts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScriptNOfK;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.ScriptNOfK")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScriptNOfK, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut k__ = None;
                let mut scripts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::K => {
                            if k__.is_some() {
                                return Err(serde::de::Error::duplicate_field("k"));
                            }
                            k__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scripts => {
                            if scripts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scripts"));
                            }
                            scripts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ScriptNOfK {
                    k: k__.unwrap_or_default(),
                    scripts: scripts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.ScriptNOfK", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SoftforkRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.init_thd.is_empty() {
            len += 1;
        }
        if !self.min_thd.is_empty() {
            len += 1;
        }
        if !self.thd_decrement.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.SoftforkRule", len)?;
        if !self.init_thd.is_empty() {
            struct_ser.serialize_field("initThd", &self.init_thd)?;
        }
        if !self.min_thd.is_empty() {
            struct_ser.serialize_field("minThd", &self.min_thd)?;
        }
        if !self.thd_decrement.is_empty() {
            struct_ser.serialize_field("thdDecrement", &self.thd_decrement)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SoftforkRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "init_thd",
            "initThd",
            "min_thd",
            "minThd",
            "thd_decrement",
            "thdDecrement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InitThd,
            MinThd,
            ThdDecrement,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "initThd" | "init_thd" => Ok(GeneratedField::InitThd),
                            "minThd" | "min_thd" => Ok(GeneratedField::MinThd),
                            "thdDecrement" | "thd_decrement" => Ok(GeneratedField::ThdDecrement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SoftforkRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.SoftforkRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SoftforkRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut init_thd__ = None;
                let mut min_thd__ = None;
                let mut thd_decrement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InitThd => {
                            if init_thd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initThd"));
                            }
                            init_thd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinThd => {
                            if min_thd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minThd"));
                            }
                            min_thd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ThdDecrement => {
                            if thd_decrement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thdDecrement"));
                            }
                            thd_decrement__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SoftforkRule {
                    init_thd: init_thd__.unwrap_or_default(),
                    min_thd: min_thd__.unwrap_or_default(),
                    thd_decrement: thd_decrement__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.SoftforkRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StakeCredential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.StakeCredential", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            match v {
                stake_credential::StakeCredential::AddrKeyHash(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("addrKeyHash", pbjson::private::base64::encode(&v).as_str())?;
                }
                stake_credential::StakeCredential::ScriptHash(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("scriptHash", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StakeCredential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "addr_key_hash",
            "addrKeyHash",
            "script_hash",
            "scriptHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddrKeyHash,
            ScriptHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "addrKeyHash" | "addr_key_hash" => Ok(GeneratedField::AddrKeyHash),
                            "scriptHash" | "script_hash" => Ok(GeneratedField::ScriptHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StakeCredential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.StakeCredential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StakeCredential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AddrKeyHash => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addrKeyHash"));
                            }
                            stake_credential__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| stake_credential::StakeCredential::AddrKeyHash(x.0));
                        }
                        GeneratedField::ScriptHash => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptHash"));
                            }
                            stake_credential__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| stake_credential::StakeCredential::ScriptHash(x.0));
                        }
                    }
                }
                Ok(StakeCredential {
                    stake_credential: stake_credential__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.StakeCredential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StakeDelegationCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if !self.pool_keyhash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.StakeDelegationCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if !self.pool_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolKeyhash", pbjson::private::base64::encode(&self.pool_keyhash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StakeDelegationCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "pool_keyhash",
            "poolKeyhash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            PoolKeyhash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "poolKeyhash" | "pool_keyhash" => Ok(GeneratedField::PoolKeyhash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StakeDelegationCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.StakeDelegationCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StakeDelegationCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut pool_keyhash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::PoolKeyhash => {
                            if pool_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolKeyhash"));
                            }
                            pool_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StakeDelegationCert {
                    stake_credential: stake_credential__,
                    pool_keyhash: pool_keyhash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.StakeDelegationCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StakeRegDelegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if !self.pool_keyhash.is_empty() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.StakeRegDelegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if !self.pool_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolKeyhash", pbjson::private::base64::encode(&self.pool_keyhash).as_str())?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StakeRegDelegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "pool_keyhash",
            "poolKeyhash",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            PoolKeyhash,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "poolKeyhash" | "pool_keyhash" => Ok(GeneratedField::PoolKeyhash),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StakeRegDelegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.StakeRegDelegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StakeRegDelegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut pool_keyhash__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::PoolKeyhash => {
                            if pool_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolKeyhash"));
                            }
                            pool_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StakeRegDelegCert {
                    stake_credential: stake_credential__,
                    pool_keyhash: pool_keyhash__.unwrap_or_default(),
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.StakeRegDelegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StakeVoteDelegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if !self.pool_keyhash.is_empty() {
            len += 1;
        }
        if self.drep.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.StakeVoteDelegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if !self.pool_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolKeyhash", pbjson::private::base64::encode(&self.pool_keyhash).as_str())?;
        }
        if let Some(v) = self.drep.as_ref() {
            struct_ser.serialize_field("drep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StakeVoteDelegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "pool_keyhash",
            "poolKeyhash",
            "drep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            PoolKeyhash,
            Drep,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "poolKeyhash" | "pool_keyhash" => Ok(GeneratedField::PoolKeyhash),
                            "drep" => Ok(GeneratedField::Drep),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StakeVoteDelegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.StakeVoteDelegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StakeVoteDelegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut pool_keyhash__ = None;
                let mut drep__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::PoolKeyhash => {
                            if pool_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolKeyhash"));
                            }
                            pool_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Drep => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drep"));
                            }
                            drep__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StakeVoteDelegCert {
                    stake_credential: stake_credential__,
                    pool_keyhash: pool_keyhash__.unwrap_or_default(),
                    drep: drep__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.StakeVoteDelegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StakeVoteRegDelegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if !self.pool_keyhash.is_empty() {
            len += 1;
        }
        if self.drep.is_some() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.StakeVoteRegDelegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if !self.pool_keyhash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("poolKeyhash", pbjson::private::base64::encode(&self.pool_keyhash).as_str())?;
        }
        if let Some(v) = self.drep.as_ref() {
            struct_ser.serialize_field("drep", v)?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StakeVoteRegDelegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "pool_keyhash",
            "poolKeyhash",
            "drep",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            PoolKeyhash,
            Drep,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "poolKeyhash" | "pool_keyhash" => Ok(GeneratedField::PoolKeyhash),
                            "drep" => Ok(GeneratedField::Drep),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StakeVoteRegDelegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.StakeVoteRegDelegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StakeVoteRegDelegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut pool_keyhash__ = None;
                let mut drep__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::PoolKeyhash => {
                            if pool_keyhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolKeyhash"));
                            }
                            pool_keyhash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Drep => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drep"));
                            }
                            drep__ = map_.next_value()?;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StakeVoteRegDelegCert {
                    stake_credential: stake_credential__,
                    pool_keyhash: pool_keyhash__.unwrap_or_default(),
                    drep: drep__,
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.StakeVoteRegDelegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TreasuryWithdrawalsAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.withdrawals.is_empty() {
            len += 1;
        }
        if !self.policy_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TreasuryWithdrawalsAction", len)?;
        if !self.withdrawals.is_empty() {
            struct_ser.serialize_field("withdrawals", &self.withdrawals)?;
        }
        if !self.policy_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("policyHash", pbjson::private::base64::encode(&self.policy_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryWithdrawalsAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "withdrawals",
            "policy_hash",
            "policyHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Withdrawals,
            PolicyHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "withdrawals" => Ok(GeneratedField::Withdrawals),
                            "policyHash" | "policy_hash" => Ok(GeneratedField::PolicyHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TreasuryWithdrawalsAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TreasuryWithdrawalsAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TreasuryWithdrawalsAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut withdrawals__ = None;
                let mut policy_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Withdrawals => {
                            if withdrawals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawals"));
                            }
                            withdrawals__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PolicyHash => {
                            if policy_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyHash"));
                            }
                            policy_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TreasuryWithdrawalsAction {
                    withdrawals: withdrawals__.unwrap_or_default(),
                    policy_hash: policy_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TreasuryWithdrawalsAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inputs.is_empty() {
            len += 1;
        }
        if !self.outputs.is_empty() {
            len += 1;
        }
        if !self.certificates.is_empty() {
            len += 1;
        }
        if !self.withdrawals.is_empty() {
            len += 1;
        }
        if !self.mint.is_empty() {
            len += 1;
        }
        if !self.reference_inputs.is_empty() {
            len += 1;
        }
        if self.witnesses.is_some() {
            len += 1;
        }
        if self.collateral.is_some() {
            len += 1;
        }
        if self.fee != 0 {
            len += 1;
        }
        if self.validity.is_some() {
            len += 1;
        }
        if self.successful {
            len += 1;
        }
        if self.auxiliary.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.proposals.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Tx", len)?;
        if !self.inputs.is_empty() {
            struct_ser.serialize_field("inputs", &self.inputs)?;
        }
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs)?;
        }
        if !self.certificates.is_empty() {
            struct_ser.serialize_field("certificates", &self.certificates)?;
        }
        if !self.withdrawals.is_empty() {
            struct_ser.serialize_field("withdrawals", &self.withdrawals)?;
        }
        if !self.mint.is_empty() {
            struct_ser.serialize_field("mint", &self.mint)?;
        }
        if !self.reference_inputs.is_empty() {
            struct_ser.serialize_field("referenceInputs", &self.reference_inputs)?;
        }
        if let Some(v) = self.witnesses.as_ref() {
            struct_ser.serialize_field("witnesses", v)?;
        }
        if let Some(v) = self.collateral.as_ref() {
            struct_ser.serialize_field("collateral", v)?;
        }
        if self.fee != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("fee", ToString::to_string(&self.fee).as_str())?;
        }
        if let Some(v) = self.validity.as_ref() {
            struct_ser.serialize_field("validity", v)?;
        }
        if self.successful {
            struct_ser.serialize_field("successful", &self.successful)?;
        }
        if let Some(v) = self.auxiliary.as_ref() {
            struct_ser.serialize_field("auxiliary", v)?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if !self.proposals.is_empty() {
            struct_ser.serialize_field("proposals", &self.proposals)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inputs",
            "outputs",
            "certificates",
            "withdrawals",
            "mint",
            "reference_inputs",
            "referenceInputs",
            "witnesses",
            "collateral",
            "fee",
            "validity",
            "successful",
            "auxiliary",
            "hash",
            "proposals",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Inputs,
            Outputs,
            Certificates,
            Withdrawals,
            Mint,
            ReferenceInputs,
            Witnesses,
            Collateral,
            Fee,
            Validity,
            Successful,
            Auxiliary,
            Hash,
            Proposals,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "inputs" => Ok(GeneratedField::Inputs),
                            "outputs" => Ok(GeneratedField::Outputs),
                            "certificates" => Ok(GeneratedField::Certificates),
                            "withdrawals" => Ok(GeneratedField::Withdrawals),
                            "mint" => Ok(GeneratedField::Mint),
                            "referenceInputs" | "reference_inputs" => Ok(GeneratedField::ReferenceInputs),
                            "witnesses" => Ok(GeneratedField::Witnesses),
                            "collateral" => Ok(GeneratedField::Collateral),
                            "fee" => Ok(GeneratedField::Fee),
                            "validity" => Ok(GeneratedField::Validity),
                            "successful" => Ok(GeneratedField::Successful),
                            "auxiliary" => Ok(GeneratedField::Auxiliary),
                            "hash" => Ok(GeneratedField::Hash),
                            "proposals" => Ok(GeneratedField::Proposals),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Tx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inputs__ = None;
                let mut outputs__ = None;
                let mut certificates__ = None;
                let mut withdrawals__ = None;
                let mut mint__ = None;
                let mut reference_inputs__ = None;
                let mut witnesses__ = None;
                let mut collateral__ = None;
                let mut fee__ = None;
                let mut validity__ = None;
                let mut successful__ = None;
                let mut auxiliary__ = None;
                let mut hash__ = None;
                let mut proposals__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Inputs => {
                            if inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            inputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Certificates => {
                            if certificates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificates"));
                            }
                            certificates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Withdrawals => {
                            if withdrawals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawals"));
                            }
                            withdrawals__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mint => {
                            if mint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mint"));
                            }
                            mint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceInputs => {
                            if reference_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceInputs"));
                            }
                            reference_inputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Witnesses => {
                            if witnesses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("witnesses"));
                            }
                            witnesses__ = map_.next_value()?;
                        }
                        GeneratedField::Collateral => {
                            if collateral__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateral"));
                            }
                            collateral__ = map_.next_value()?;
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Validity => {
                            if validity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validity"));
                            }
                            validity__ = map_.next_value()?;
                        }
                        GeneratedField::Successful => {
                            if successful__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            successful__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Auxiliary => {
                            if auxiliary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auxiliary"));
                            }
                            auxiliary__ = map_.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proposals => {
                            if proposals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposals"));
                            }
                            proposals__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Tx {
                    inputs: inputs__.unwrap_or_default(),
                    outputs: outputs__.unwrap_or_default(),
                    certificates: certificates__.unwrap_or_default(),
                    withdrawals: withdrawals__.unwrap_or_default(),
                    mint: mint__.unwrap_or_default(),
                    reference_inputs: reference_inputs__.unwrap_or_default(),
                    witnesses: witnesses__,
                    collateral: collateral__,
                    fee: fee__.unwrap_or_default(),
                    validity: validity__,
                    successful: successful__.unwrap_or_default(),
                    auxiliary: auxiliary__,
                    hash: hash__.unwrap_or_default(),
                    proposals: proposals__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Tx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxEval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee != 0 {
            len += 1;
        }
        if self.ex_units.is_some() {
            len += 1;
        }
        if !self.errors.is_empty() {
            len += 1;
        }
        if !self.traces.is_empty() {
            len += 1;
        }
        if !self.redeemers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxEval", len)?;
        if self.fee != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("fee", ToString::to_string(&self.fee).as_str())?;
        }
        if let Some(v) = self.ex_units.as_ref() {
            struct_ser.serialize_field("exUnits", v)?;
        }
        if !self.errors.is_empty() {
            struct_ser.serialize_field("errors", &self.errors)?;
        }
        if !self.traces.is_empty() {
            struct_ser.serialize_field("traces", &self.traces)?;
        }
        if !self.redeemers.is_empty() {
            struct_ser.serialize_field("redeemers", &self.redeemers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxEval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee",
            "ex_units",
            "exUnits",
            "errors",
            "traces",
            "redeemers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fee,
            ExUnits,
            Errors,
            Traces,
            Redeemers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fee" => Ok(GeneratedField::Fee),
                            "exUnits" | "ex_units" => Ok(GeneratedField::ExUnits),
                            "errors" => Ok(GeneratedField::Errors),
                            "traces" => Ok(GeneratedField::Traces),
                            "redeemers" => Ok(GeneratedField::Redeemers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxEval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxEval")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxEval, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                let mut ex_units__ = None;
                let mut errors__ = None;
                let mut traces__ = None;
                let mut redeemers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExUnits => {
                            if ex_units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exUnits"));
                            }
                            ex_units__ = map_.next_value()?;
                        }
                        GeneratedField::Errors => {
                            if errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            errors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Traces => {
                            if traces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traces"));
                            }
                            traces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Redeemers => {
                            if redeemers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemers"));
                            }
                            redeemers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TxEval {
                    fee: fee__.unwrap_or_default(),
                    ex_units: ex_units__,
                    errors: errors__.unwrap_or_default(),
                    traces: traces__.unwrap_or_default(),
                    redeemers: redeemers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxEval", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxFeePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.multiplier.is_empty() {
            len += 1;
        }
        if !self.summand.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxFeePolicy", len)?;
        if !self.multiplier.is_empty() {
            struct_ser.serialize_field("multiplier", &self.multiplier)?;
        }
        if !self.summand.is_empty() {
            struct_ser.serialize_field("summand", &self.summand)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxFeePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "multiplier",
            "summand",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Multiplier,
            Summand,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "multiplier" => Ok(GeneratedField::Multiplier),
                            "summand" => Ok(GeneratedField::Summand),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxFeePolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxFeePolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxFeePolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut multiplier__ = None;
                let mut summand__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Multiplier => {
                            if multiplier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiplier"));
                            }
                            multiplier__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Summand => {
                            if summand__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summand"));
                            }
                            summand__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TxFeePolicy {
                    multiplier: multiplier__.unwrap_or_default(),
                    summand: summand__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxFeePolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        if self.output_index != 0 {
            len += 1;
        }
        if self.as_output.is_some() {
            len += 1;
        }
        if self.redeemer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxInput", len)?;
        if !self.tx_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("txHash", pbjson::private::base64::encode(&self.tx_hash).as_str())?;
        }
        if self.output_index != 0 {
            struct_ser.serialize_field("outputIndex", &self.output_index)?;
        }
        if let Some(v) = self.as_output.as_ref() {
            struct_ser.serialize_field("asOutput", v)?;
        }
        if let Some(v) = self.redeemer.as_ref() {
            struct_ser.serialize_field("redeemer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx_hash",
            "txHash",
            "output_index",
            "outputIndex",
            "as_output",
            "asOutput",
            "redeemer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxHash,
            OutputIndex,
            AsOutput,
            Redeemer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            "outputIndex" | "output_index" => Ok(GeneratedField::OutputIndex),
                            "asOutput" | "as_output" => Ok(GeneratedField::AsOutput),
                            "redeemer" => Ok(GeneratedField::Redeemer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx_hash__ = None;
                let mut output_index__ = None;
                let mut as_output__ = None;
                let mut redeemer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OutputIndex => {
                            if output_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputIndex"));
                            }
                            output_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AsOutput => {
                            if as_output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asOutput"));
                            }
                            as_output__ = map_.next_value()?;
                        }
                        GeneratedField::Redeemer => {
                            if redeemer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemer"));
                            }
                            redeemer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxInput {
                    tx_hash: tx_hash__.unwrap_or_default(),
                    output_index: output_index__.unwrap_or_default(),
                    as_output: as_output__,
                    redeemer: redeemer__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxOutput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        if !self.assets.is_empty() {
            len += 1;
        }
        if self.datum.is_some() {
            len += 1;
        }
        if self.script.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxOutput", len)?;
        if !self.address.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        if !self.assets.is_empty() {
            struct_ser.serialize_field("assets", &self.assets)?;
        }
        if let Some(v) = self.datum.as_ref() {
            struct_ser.serialize_field("datum", v)?;
        }
        if let Some(v) = self.script.as_ref() {
            struct_ser.serialize_field("script", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "coin",
            "assets",
            "datum",
            "script",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Coin,
            Assets,
            Datum,
            Script,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "coin" => Ok(GeneratedField::Coin),
                            "assets" => Ok(GeneratedField::Assets),
                            "datum" => Ok(GeneratedField::Datum),
                            "script" => Ok(GeneratedField::Script),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxOutput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut coin__ = None;
                let mut assets__ = None;
                let mut datum__ = None;
                let mut script__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Datum => {
                            if datum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datum"));
                            }
                            datum__ = map_.next_value()?;
                        }
                        GeneratedField::Script => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            script__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxOutput {
                    address: address__.unwrap_or_default(),
                    coin: coin__.unwrap_or_default(),
                    assets: assets__.unwrap_or_default(),
                    datum: datum__,
                    script: script__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxOutput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxOutputPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.asset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxOutputPattern", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.asset.as_ref() {
            struct_ser.serialize_field("asset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxOutputPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "asset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Asset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "asset" => Ok(GeneratedField::Asset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxOutputPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxOutputPattern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxOutputPattern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut asset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::Asset => {
                            if asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asset"));
                            }
                            asset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxOutputPattern {
                    address: address__,
                    asset: asset__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxOutputPattern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consumes.is_some() {
            len += 1;
        }
        if self.produces.is_some() {
            len += 1;
        }
        if self.has_address.is_some() {
            len += 1;
        }
        if self.moves_asset.is_some() {
            len += 1;
        }
        if self.mints_asset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxPattern", len)?;
        if let Some(v) = self.consumes.as_ref() {
            struct_ser.serialize_field("consumes", v)?;
        }
        if let Some(v) = self.produces.as_ref() {
            struct_ser.serialize_field("produces", v)?;
        }
        if let Some(v) = self.has_address.as_ref() {
            struct_ser.serialize_field("hasAddress", v)?;
        }
        if let Some(v) = self.moves_asset.as_ref() {
            struct_ser.serialize_field("movesAsset", v)?;
        }
        if let Some(v) = self.mints_asset.as_ref() {
            struct_ser.serialize_field("mintsAsset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consumes",
            "produces",
            "has_address",
            "hasAddress",
            "moves_asset",
            "movesAsset",
            "mints_asset",
            "mintsAsset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consumes,
            Produces,
            HasAddress,
            MovesAsset,
            MintsAsset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "consumes" => Ok(GeneratedField::Consumes),
                            "produces" => Ok(GeneratedField::Produces),
                            "hasAddress" | "has_address" => Ok(GeneratedField::HasAddress),
                            "movesAsset" | "moves_asset" => Ok(GeneratedField::MovesAsset),
                            "mintsAsset" | "mints_asset" => Ok(GeneratedField::MintsAsset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxPattern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxPattern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consumes__ = None;
                let mut produces__ = None;
                let mut has_address__ = None;
                let mut moves_asset__ = None;
                let mut mints_asset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Consumes => {
                            if consumes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumes"));
                            }
                            consumes__ = map_.next_value()?;
                        }
                        GeneratedField::Produces => {
                            if produces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("produces"));
                            }
                            produces__ = map_.next_value()?;
                        }
                        GeneratedField::HasAddress => {
                            if has_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasAddress"));
                            }
                            has_address__ = map_.next_value()?;
                        }
                        GeneratedField::MovesAsset => {
                            if moves_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("movesAsset"));
                            }
                            moves_asset__ = map_.next_value()?;
                        }
                        GeneratedField::MintsAsset => {
                            if mints_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintsAsset"));
                            }
                            mints_asset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxPattern {
                    consumes: consumes__,
                    produces: produces__,
                    has_address: has_address__,
                    moves_asset: moves_asset__,
                    mints_asset: mints_asset__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxPattern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxValidity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.ttl != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.TxValidity", len)?;
        if self.start != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("start", ToString::to_string(&self.start).as_str())?;
        }
        if self.ttl != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ttl", ToString::to_string(&self.ttl).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxValidity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "ttl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            Ttl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "ttl" => Ok(GeneratedField::Ttl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxValidity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.TxValidity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxValidity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut ttl__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Ttl => {
                            if ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            ttl__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TxValidity {
                    start: start__.unwrap_or_default(),
                    ttl: ttl__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.TxValidity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnRegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.UnRegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnRegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnRegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.UnRegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnRegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UnRegCert {
                    stake_credential: stake_credential__,
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.UnRegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnRegDRepCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.drep_credential.is_some() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.UnRegDRepCert", len)?;
        if let Some(v) = self.drep_credential.as_ref() {
            struct_ser.serialize_field("drepCredential", v)?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnRegDRepCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drep_credential",
            "drepCredential",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DrepCredential,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "drepCredential" | "drep_credential" => Ok(GeneratedField::DrepCredential),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnRegDRepCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.UnRegDRepCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnRegDRepCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drep_credential__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DrepCredential => {
                            if drep_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepCredential"));
                            }
                            drep_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UnRegDRepCert {
                    drep_credential: drep_credential__,
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.UnRegDRepCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCommitteeAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gov_action_id.is_some() {
            len += 1;
        }
        if !self.remove_committee_credentials.is_empty() {
            len += 1;
        }
        if !self.new_committee_credentials.is_empty() {
            len += 1;
        }
        if self.new_committee_threshold.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.UpdateCommitteeAction", len)?;
        if let Some(v) = self.gov_action_id.as_ref() {
            struct_ser.serialize_field("govActionId", v)?;
        }
        if !self.remove_committee_credentials.is_empty() {
            struct_ser.serialize_field("removeCommitteeCredentials", &self.remove_committee_credentials)?;
        }
        if !self.new_committee_credentials.is_empty() {
            struct_ser.serialize_field("newCommitteeCredentials", &self.new_committee_credentials)?;
        }
        if let Some(v) = self.new_committee_threshold.as_ref() {
            struct_ser.serialize_field("newCommitteeThreshold", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCommitteeAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gov_action_id",
            "govActionId",
            "remove_committee_credentials",
            "removeCommitteeCredentials",
            "new_committee_credentials",
            "newCommitteeCredentials",
            "new_committee_threshold",
            "newCommitteeThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GovActionId,
            RemoveCommitteeCredentials,
            NewCommitteeCredentials,
            NewCommitteeThreshold,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "govActionId" | "gov_action_id" => Ok(GeneratedField::GovActionId),
                            "removeCommitteeCredentials" | "remove_committee_credentials" => Ok(GeneratedField::RemoveCommitteeCredentials),
                            "newCommitteeCredentials" | "new_committee_credentials" => Ok(GeneratedField::NewCommitteeCredentials),
                            "newCommitteeThreshold" | "new_committee_threshold" => Ok(GeneratedField::NewCommitteeThreshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCommitteeAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.UpdateCommitteeAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCommitteeAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gov_action_id__ = None;
                let mut remove_committee_credentials__ = None;
                let mut new_committee_credentials__ = None;
                let mut new_committee_threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GovActionId => {
                            if gov_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("govActionId"));
                            }
                            gov_action_id__ = map_.next_value()?;
                        }
                        GeneratedField::RemoveCommitteeCredentials => {
                            if remove_committee_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeCommitteeCredentials"));
                            }
                            remove_committee_credentials__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewCommitteeCredentials => {
                            if new_committee_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newCommitteeCredentials"));
                            }
                            new_committee_credentials__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewCommitteeThreshold => {
                            if new_committee_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newCommitteeThreshold"));
                            }
                            new_committee_threshold__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCommitteeAction {
                    gov_action_id: gov_action_id__,
                    remove_committee_credentials: remove_committee_credentials__.unwrap_or_default(),
                    new_committee_credentials: new_committee_credentials__.unwrap_or_default(),
                    new_committee_threshold: new_committee_threshold__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.UpdateCommitteeAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDRepCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.drep_credential.is_some() {
            len += 1;
        }
        if self.anchor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.UpdateDRepCert", len)?;
        if let Some(v) = self.drep_credential.as_ref() {
            struct_ser.serialize_field("drepCredential", v)?;
        }
        if let Some(v) = self.anchor.as_ref() {
            struct_ser.serialize_field("anchor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDRepCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drep_credential",
            "drepCredential",
            "anchor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DrepCredential,
            Anchor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "drepCredential" | "drep_credential" => Ok(GeneratedField::DrepCredential),
                            "anchor" => Ok(GeneratedField::Anchor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDRepCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.UpdateDRepCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDRepCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drep_credential__ = None;
                let mut anchor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DrepCredential => {
                            if drep_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drepCredential"));
                            }
                            drep_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Anchor => {
                            if anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            anchor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateDRepCert {
                    drep_credential: drep_credential__,
                    anchor: anchor__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.UpdateDRepCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VKeyWitness {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vkey.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.VKeyWitness", len)?;
        if !self.vkey.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("vkey", pbjson::private::base64::encode(&self.vkey).as_str())?;
        }
        if !self.signature.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VKeyWitness {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vkey",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vkey,
            Signature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "vkey" => Ok(GeneratedField::Vkey),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VKeyWitness;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.VKeyWitness")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VKeyWitness, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vkey__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vkey => {
                            if vkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vkey"));
                            }
                            vkey__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(VKeyWitness {
                    vkey: vkey__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.VKeyWitness", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VoteDelegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if self.drep.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.VoteDelegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if let Some(v) = self.drep.as_ref() {
            struct_ser.serialize_field("drep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VoteDelegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "drep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            Drep,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "drep" => Ok(GeneratedField::Drep),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VoteDelegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.VoteDelegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VoteDelegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut drep__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Drep => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drep"));
                            }
                            drep__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VoteDelegCert {
                    stake_credential: stake_credential__,
                    drep: drep__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.VoteDelegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VoteRegDelegCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stake_credential.is_some() {
            len += 1;
        }
        if self.drep.is_some() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.VoteRegDelegCert", len)?;
        if let Some(v) = self.stake_credential.as_ref() {
            struct_ser.serialize_field("stakeCredential", v)?;
        }
        if let Some(v) = self.drep.as_ref() {
            struct_ser.serialize_field("drep", v)?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VoteRegDelegCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stake_credential",
            "stakeCredential",
            "drep",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StakeCredential,
            Drep,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stakeCredential" | "stake_credential" => Ok(GeneratedField::StakeCredential),
                            "drep" => Ok(GeneratedField::Drep),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VoteRegDelegCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.VoteRegDelegCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VoteRegDelegCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stake_credential__ = None;
                let mut drep__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StakeCredential => {
                            if stake_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakeCredential"));
                            }
                            stake_credential__ = map_.next_value()?;
                        }
                        GeneratedField::Drep => {
                            if drep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drep"));
                            }
                            drep__ = map_.next_value()?;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(VoteRegDelegCert {
                    stake_credential: stake_credential__,
                    drep: drep__,
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.VoteRegDelegCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VotingThresholds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.thresholds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.VotingThresholds", len)?;
        if !self.thresholds.is_empty() {
            struct_ser.serialize_field("thresholds", &self.thresholds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VotingThresholds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "thresholds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Thresholds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "thresholds" => Ok(GeneratedField::Thresholds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VotingThresholds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.VotingThresholds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VotingThresholds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut thresholds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Thresholds => {
                            if thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thresholds"));
                            }
                            thresholds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VotingThresholds {
                    thresholds: thresholds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.VotingThresholds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VssCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expiry_epoch != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        if !self.signing_key.is_empty() {
            len += 1;
        }
        if !self.vss_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.VssCert", len)?;
        if self.expiry_epoch != 0 {
            struct_ser.serialize_field("expiryEpoch", &self.expiry_epoch)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        if !self.signing_key.is_empty() {
            struct_ser.serialize_field("signingKey", &self.signing_key)?;
        }
        if !self.vss_key.is_empty() {
            struct_ser.serialize_field("vssKey", &self.vss_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VssCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expiry_epoch",
            "expiryEpoch",
            "signature",
            "signing_key",
            "signingKey",
            "vss_key",
            "vssKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpiryEpoch,
            Signature,
            SigningKey,
            VssKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "expiryEpoch" | "expiry_epoch" => Ok(GeneratedField::ExpiryEpoch),
                            "signature" => Ok(GeneratedField::Signature),
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            "vssKey" | "vss_key" => Ok(GeneratedField::VssKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VssCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.VssCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VssCert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expiry_epoch__ = None;
                let mut signature__ = None;
                let mut signing_key__ = None;
                let mut vss_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExpiryEpoch => {
                            if expiry_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryEpoch"));
                            }
                            expiry_epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VssKey => {
                            if vss_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vssKey"));
                            }
                            vss_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VssCert {
                    expiry_epoch: expiry_epoch__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                    signing_key: signing_key__.unwrap_or_default(),
                    vss_key: vss_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.VssCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Withdrawal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reward_account.is_empty() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        if self.redeemer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.Withdrawal", len)?;
        if !self.reward_account.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("rewardAccount", pbjson::private::base64::encode(&self.reward_account).as_str())?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        if let Some(v) = self.redeemer.as_ref() {
            struct_ser.serialize_field("redeemer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Withdrawal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reward_account",
            "rewardAccount",
            "coin",
            "redeemer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RewardAccount,
            Coin,
            Redeemer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rewardAccount" | "reward_account" => Ok(GeneratedField::RewardAccount),
                            "coin" => Ok(GeneratedField::Coin),
                            "redeemer" => Ok(GeneratedField::Redeemer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Withdrawal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.Withdrawal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Withdrawal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reward_account__ = None;
                let mut coin__ = None;
                let mut redeemer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RewardAccount => {
                            if reward_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAccount"));
                            }
                            reward_account__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Redeemer => {
                            if redeemer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemer"));
                            }
                            redeemer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Withdrawal {
                    reward_account: reward_account__.unwrap_or_default(),
                    coin: coin__.unwrap_or_default(),
                    redeemer: redeemer__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.Withdrawal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WithdrawalAmount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reward_account.is_empty() {
            len += 1;
        }
        if self.coin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.WithdrawalAmount", len)?;
        if !self.reward_account.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("rewardAccount", pbjson::private::base64::encode(&self.reward_account).as_str())?;
        }
        if self.coin != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coin", ToString::to_string(&self.coin).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WithdrawalAmount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reward_account",
            "rewardAccount",
            "coin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RewardAccount,
            Coin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rewardAccount" | "reward_account" => Ok(GeneratedField::RewardAccount),
                            "coin" => Ok(GeneratedField::Coin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WithdrawalAmount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.WithdrawalAmount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WithdrawalAmount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reward_account__ = None;
                let mut coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RewardAccount => {
                            if reward_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAccount"));
                            }
                            reward_account__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Coin => {
                            if coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coin"));
                            }
                            coin__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WithdrawalAmount {
                    reward_account: reward_account__.unwrap_or_default(),
                    coin: coin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.WithdrawalAmount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WitnessSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vkeywitness.is_empty() {
            len += 1;
        }
        if !self.script.is_empty() {
            len += 1;
        }
        if !self.plutus_datums.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.cardano.WitnessSet", len)?;
        if !self.vkeywitness.is_empty() {
            struct_ser.serialize_field("vkeywitness", &self.vkeywitness)?;
        }
        if !self.script.is_empty() {
            struct_ser.serialize_field("script", &self.script)?;
        }
        if !self.plutus_datums.is_empty() {
            struct_ser.serialize_field("plutusDatums", &self.plutus_datums)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WitnessSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vkeywitness",
            "script",
            "plutus_datums",
            "plutusDatums",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vkeywitness,
            Script,
            PlutusDatums,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "vkeywitness" => Ok(GeneratedField::Vkeywitness),
                            "script" => Ok(GeneratedField::Script),
                            "plutusDatums" | "plutus_datums" => Ok(GeneratedField::PlutusDatums),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WitnessSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.cardano.WitnessSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WitnessSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vkeywitness__ = None;
                let mut script__ = None;
                let mut plutus_datums__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vkeywitness => {
                            if vkeywitness__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vkeywitness"));
                            }
                            vkeywitness__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Script => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlutusDatums => {
                            if plutus_datums__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plutusDatums"));
                            }
                            plutus_datums__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WitnessSet {
                    vkeywitness: vkeywitness__.unwrap_or_default(),
                    script: script__.unwrap_or_default(),
                    plutus_datums: plutus_datums__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.cardano.WitnessSet", FIELDS, GeneratedVisitor)
    }
}
