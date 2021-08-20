pub struct ExampleOutsideFeature {
    pub(in crate::example) service: ExampleOutsideService,
}
pub struct ExampleOutsideService {
    pub outline_service_url: &'static str,
}
