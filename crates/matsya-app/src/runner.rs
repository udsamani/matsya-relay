use matsya_common::MatsyaRelayResult;

#[allow(dead_code)]
pub trait Runner {
    fn run(&self) -> MatsyaRelayResult<()>;
}
