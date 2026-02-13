pub struct CutsceneSlide {
    pub image_name: String,
    pub text: String,
    pub time: f32,
}

pub struct CutsceneState {
    pub slides: Vec<CutsceneSlide>,
}
