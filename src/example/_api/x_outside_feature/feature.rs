pub struct ExampleOutsideFeature {
    pub(in crate::example) service: ExampleOutsideService,
}
pub struct ExampleOutsideService {
    pub service_url: &'static str,
}
