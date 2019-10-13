use playfab_api::traits::PlayFabConfig;

#[derive(Debug)]
pub struct PlayFab;
impl PlayFabConfig for PlayFab {
    const TITLE_ID: &'static str = "5C86";
}
