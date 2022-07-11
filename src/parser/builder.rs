///This trait used to implement
///
///This trait is used to describe the process of creating a request / response
pub trait Builder {
    fn build(self) -> String;
}