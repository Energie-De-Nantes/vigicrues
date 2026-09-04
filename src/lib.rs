pub type Result<T = ()> = std::result::Result<T, reqwest::Error>;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Scale {
    #[default]
    #[serde(rename = "H")]
    Level,
    #[serde(rename = "Q")]
    Flow,
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Flow => "Q",
            Self::Level => "H",
        };

        f.write_str(s)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    #[serde(rename = "VersionFlux")]
    pub version: String,
    pub serie: Serie,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Serie {
    #[serde(rename = "CdStationHydro")]
    pub code_station: String,
    #[serde(rename = "LbStationHydro")]
    pub label_station: String,
    pub link: String,
    #[serde(rename = "GrdSerie")]
    pub scale: Scale,
    #[serde(rename = "ObssHydro")]
    pub mesures: Vec<Mesure>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Mesure {
    #[serde(rename = "DtObsHydro")]
    pub time: jiff::civil::DateTime,
    #[serde(rename = "ResObsHydro")]
    pub mesure: f32,
}

pub async fn level(station: &str) -> Result<Data> {
    fetch(station, Scale::Level).await
}

pub async fn flow(station: &str) -> Result<Data> {
    fetch(station, Scale::Flow).await
}

async fn fetch(station: &str, scale: Scale) -> Result<Data> {
    let url = format!(
        "https://www.vigicrues.gouv.fr/services/observations.json/index.php?CdStationHydro={station}&GrdSerie={scale}&FormatDate=iso"
    );

    reqwest::get(&url).await?.json().await
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Station {
    #[serde(rename = "VersionFlux")]
    pub version: String,
    #[serde(rename = "CdStationHydro")]
    pub code: String,
    #[serde(rename = "LbStationHydro")]
    pub label: String,
    #[serde(rename = "LbCoursEau")]
    pub cours_eau: String,
    #[serde(rename = "CdStationHydroAncienRef")]
    pub old_code: Option<String>,
    #[serde(rename = "CoordStationHydro")]
    pub coord: Coord,
    #[serde(rename = "CdCommune")]
    pub code_commune: String,
    #[serde(rename = "Evenement")]
    pub events: Vec<()>,
    #[serde(rename = "VigilanceCrues")]
    pub vigilance_crues: VigilanceCrues,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Coord {
    #[serde(rename = "CoordXStationHydro")]
    pub x: String,
    #[serde(rename = "CoordYStationHydro")]
    pub y: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct VigilanceCrues {
    #[serde(rename = "StationPrevision")]
    pub previsions: bool,
    #[serde(rename = "Photo", deserialize_with = "de_bool")]
    pub photo: bool,
    #[serde(rename = "ZoomInitial")]
    pub zoom_initial: String,
    #[serde(rename = "PereBoitEntVigiCru")]
    pub entity: Entity,
    #[serde(rename = "CruesHistoriques")]
    pub flood_historic: Vec<()>,
    #[serde(rename = "StationsBassin")]
    pub stations: Vec<StationInfo>,
    #[serde(rename = "FluxDonnees")]
    pub data: DataInfo,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Entity {
    #[serde(rename = "CdEntVigiCru")]
    pub code: String,
    #[serde(rename = "Link")]
    pub link: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct StationInfo {
    #[serde(rename = "CdStationHydro")]
    pub code_station: String,
    #[serde(rename = "LbStationHydro")]
    pub label_station: String,
    #[serde(rename = "LbCoursEau")]
    pub cours_eau: String,
    #[serde(rename = "idspc")]
    pub idspc: String,
    #[serde(rename = "Link")]
    pub link: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DataInfo {
    pub observations: Observations,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Observations {
    #[serde(rename = "Hauteurs")]
    pub level: String,
    #[serde(rename = "Debits")]
    pub flow: String,
}

fn de_bool<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let b: String = serde::Deserialize::deserialize(deserializer)?;

    Ok(b == "t")
}

pub async fn station(code: &str) -> Result<Station> {
    let url = format!(
        "https://www.vigicrues.gouv.fr/services/station.json/index.php?CdStationHydro={code}"
    );

    reqwest::get(&url).await?.json().await
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn level() {
        assert!(dbg!(crate::level("M730242011").await).is_ok());
    }

    #[tokio::test]
    async fn flow() {
        assert!(dbg!(crate::flow("M730242011").await).is_ok());
    }

    #[tokio::test]
    async fn station() {
        assert!(dbg!(crate::station("M730242011").await).is_ok());
    }
}
