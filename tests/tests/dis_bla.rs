use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_tools_gen::dis::Args;
use spv::BLA;
use std::io::stdout;

#[test]
fn test_bla() -> anyhow::Result<()> {
    Args {
        path: BLA.spv(),
        ..Default::default()
    }
    .run::<CoreInstSet>(&mut stdout())
}
