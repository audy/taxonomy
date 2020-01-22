//! Code related to handling of taxonomic ranks
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{Result, TaxonomyError};

/// A taxonomic rank. For example, a species or phylum.
///
/// We use this instead of a String/&str to allow stricter type-checking
/// by forcing all taxonomic ranks to fall within the below categories
/// (this includes all current NCBI ranks and a few others, mostly ones
/// specific to zoology and botany).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaxRank {
    Domain,
    Subdomain,
    Realm,
    Subrealm,
    Hyperkingdom,
    Superkingdom,
    Kingdom,
    Subkingdom,
    Infrakingdom,
    Parvkingdom,
    Superphylum,
    Phylum,
    Subphylum,
    Infraphylum,
    Microphylum,
    Superclass,
    Class,
    Subclass,
    Infraclass,
    Parvclass,
    Superdivision,
    Division,
    Subdivision,
    Infradivision,
    Superlegion,
    Legion,
    Sublegion,
    Infralegion,
    Supercohort,
    Cohort,
    Subcohort,
    Infracohort,
    Superorder,
    Gigaorder,
    Magnorder,
    Grandorder,
    Mirorder,
    SeriesFish,
    Order,
    Nanorder,
    Hypoorder,
    Suborder,
    Infraorder,
    Parvorder,
    Section,
    Subsection,
    Gigafamily,
    Megafamily,
    Grandfamily,
    Hyperfamily,
    Superfamily,
    Epifamily,
    SeriesLepidoptera,
    GroupLepidoptera,
    Family,
    Subfamily,
    Infrafamily,
    Supertribe,
    Tribe,
    Subtribe,
    Infratribe,
    Genus,
    Subgenus,
    SeriesBotany,
    SubseriesBotany,
    SpeciesGroup,
    SpeciesSubgroup,
    Species,
    Subspecies,
    Varietas,
    Subvarietas,
    Forma,
    Subforma,
    Cultivar,
    Breed,
    Strain,
    Individual,
    // TODO: Unspecified prevents an auto-impl of Ord because it has no defined
    // place in the ordering (like a NaN) so we should manually derive out a
    // PartialOrd impl for TaxRank
    Unspecified,
    // there may be additional ranks added in the future so we don't want
    // downstream users to count on exhaustively matching this list
    #[doc(hidden)]
    __Nonexhaustive,
}

impl TaxRank {
    /// Coverts a TaxRank into a one of the rank strings NCBI uses.
    /// Note that this doesn't handle ranks that are not used by the NCBI taxonomy.
    pub fn to_ncbi_rank(self) -> &'static str {
        match self {
            TaxRank::Superkingdom => "superkingdom",
            TaxRank::Kingdom => "kingdom",
            TaxRank::Subkingdom => "subkingdom",
            TaxRank::Superphylum => "superphylum",
            TaxRank::Phylum => "phylum",
            TaxRank::Subphylum => "subphylum",
            TaxRank::Superclass => "superclass",
            TaxRank::Class => "class",
            TaxRank::Subclass => "subclass",
            TaxRank::Infraclass => "infraclass",
            TaxRank::Cohort => "cohort",
            TaxRank::Superorder => "superorder",
            TaxRank::Order => "order",
            TaxRank::Suborder => "suborder",
            TaxRank::Infraorder => "infraorder",
            TaxRank::Parvorder => "parvorder",
            TaxRank::Superfamily => "superfamily",
            TaxRank::Family => "family",
            TaxRank::Subfamily => "subfamily",
            TaxRank::Tribe => "tribe",
            TaxRank::Subtribe => "subtribe",
            TaxRank::Genus => "genus",
            TaxRank::Subgenus => "subgenus",
            TaxRank::SpeciesGroup => "species group",
            TaxRank::SpeciesSubgroup => "species subgroup",
            TaxRank::Species => "species",
            TaxRank::Subspecies => "subspecies",
            TaxRank::Varietas => "varietas",
            TaxRank::Forma => "forma",
            TaxRank::Unspecified => "no rank",
            // TODO: not sure if we want to manually coerce everything like this?
            _ => "no rank",
        }
    }
}

impl FromStr for TaxRank {
    type Err = TaxonomyError;

    fn from_str(s: &str) -> Result<Self> {
        // many of these synonyms (and the ranks themselves) were pulled from:
        // https://en.wikipedia.org/wiki/Taxonomic_rank
        match s.trim().to_lowercase().as_ref() {
            "domain" | "regio" => Ok(TaxRank::Domain),
            "subdomain" => Ok(TaxRank::Subdomain),
            "superkingdom" => Ok(TaxRank::Superkingdom),
            "kingdom" | "regnum" => Ok(TaxRank::Kingdom),
            "subkingdom" | "subregnum" => Ok(TaxRank::Subkingdom),
            "superphylum" | "superphyla" => Ok(TaxRank::Superphylum),
            "phylum" | "phyla" | "divisio" => Ok(TaxRank::Phylum),
            "subphylum" | "subphyla" | "subdivisio" => Ok(TaxRank::Subphylum),
            "superclass" => Ok(TaxRank::Superclass),
            "class" | "classis" => Ok(TaxRank::Class),
            "subclass" | "subclassis" => Ok(TaxRank::Subclass),
            "infraclass" => Ok(TaxRank::Infraclass),
            "cohort" => Ok(TaxRank::Cohort),
            "superorder" => Ok(TaxRank::Superorder),
            "order" | "ordo" => Ok(TaxRank::Order),
            "suborder" | "subordo" => Ok(TaxRank::Suborder),
            "infraorder" => Ok(TaxRank::Infraorder),
            "parvorder" => Ok(TaxRank::Parvorder),
            "section" | "sectio" => Ok(TaxRank::Section),
            "subsection" => Ok(TaxRank::Subsection),
            "superfamily" => Ok(TaxRank::Superfamily),
            "family" | "familia" => Ok(TaxRank::Family),
            "subfamily" => Ok(TaxRank::Subfamily),
            "tribe" | "subtribus" => Ok(TaxRank::Tribe),
            "subtribe" => Ok(TaxRank::Subtribe),
            "genus" | "genera" => Ok(TaxRank::Genus),
            "subgenus" => Ok(TaxRank::Subgenus),
            "species group" => Ok(TaxRank::SpeciesGroup),
            "species subgroup" => Ok(TaxRank::SpeciesSubgroup),
            "species" => Ok(TaxRank::Species),
            "subspecies" => Ok(TaxRank::Subspecies),
            "variety" | "varietas" => Ok(TaxRank::Varietas),
            "form" | "forma" => Ok(TaxRank::Forma),
            "subform" | "subforma" => Ok(TaxRank::Subforma),
            "strain" => Ok(TaxRank::Strain),
            "no rank" => Ok(TaxRank::Unspecified),
            _ => Err(TaxonomyError::UnrecognizedRank {
                rank: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::Result;

    use super::TaxRank;
    use super::TaxRank::*;

    static RANKS: &[super::TaxRank] = &[
        Domain,
        Subdomain,
        Realm,
        Subrealm,
        Hyperkingdom,
        Superkingdom,
        Kingdom,
        Subkingdom,
        Infrakingdom,
        Parvkingdom,
        Superphylum,
        Phylum,
        Subphylum,
        Infraphylum,
        Microphylum,
        Superclass,
        Class,
        Subclass,
        Infraclass,
        Parvclass,
        Superdivision,
        Division,
        Subdivision,
        Infradivision,
        Superlegion,
        Legion,
        Sublegion,
        Infralegion,
        Supercohort,
        Cohort,
        Subcohort,
        Infracohort,
        Superorder,
        Gigaorder,
        Magnorder,
        Grandorder,
        Mirorder,
        SeriesFish,
        Order,
        Nanorder,
        Hypoorder,
        Suborder,
        Infraorder,
        Parvorder,
        Section,
        Subsection,
        Gigafamily,
        Megafamily,
        Grandfamily,
        Hyperfamily,
        Superfamily,
        Epifamily,
        SeriesLepidoptera,
        GroupLepidoptera,
        Family,
        Subfamily,
        Infrafamily,
        Supertribe,
        Tribe,
        Subtribe,
        Infratribe,
        Genus,
        Subgenus,
        SeriesBotany,
        SubseriesBotany,
        SpeciesGroup,
        SpeciesSubgroup,
        Species,
        Subspecies,
        Varietas,
        Subvarietas,
        Forma,
        Subforma,
        Cultivar,
        Breed,
        Strain,
        Individual,
        Unspecified,
    ];

    #[test]
    fn test_ranks() {
        for rank in RANKS.iter() {
            let _ = rank.to_ncbi_rank();
        }
    }

    #[test]
    fn test_str_to_rank() -> Result<()> {
        for rank in RANKS.iter() {
            let _ = TaxRank::from_str(rank.to_ncbi_rank())?;
        }
        assert!(TaxRank::from_str("fake_data").is_err());
        Ok(())
    }
}
