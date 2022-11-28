use std::time::{Duration, Instant};
use std::{path::PathBuf, sync::Arc};
use swc_common::{sync::Lrc};
use swc_common::{SourceMap};
use swc::{Compiler as Compiler, BoolOrDataConfig};
fn main() {
    let cwd = std::env::current_dir().unwrap();
    let path = PathBuf::from("./BizCharts.js");
    let start = Instant::now();
    let cm: Lrc<SourceMap> =Arc::new(Default::default());
    let compiler = Compiler::new(cm.clone());
    let fm = cm.clone().load_file(&path).expect("load file failed: {}");
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
        let res = swc::try_with_handler(cm.clone(), Default::default(), |handler| {
            compiler.minify(
                fm,
                handler,
                &swc::config::JsMinifyOptions {
                    source_map: BoolOrDataConfig::from_bool(false),
                    emit_source_map_columns: true,
                    ..Default::default()
                },
            )
        }).unwrap();
    });
    let duration = start.elapsed();
    println!("cost: {:?}",duration ) ;
}
