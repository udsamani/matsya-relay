use matsya_common::MatsyaRelayResult;

use crate::runner::App;

#[allow(dead_code)]
pub trait Plugin {
    fn register_plugin(app: &mut impl App) -> MatsyaRelayResult<()>;
}
