#[macro_use]
extern crate neon;

use std::fs;
use std::path::Path;

use neon::prelude::*;

fn ls(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = cx.empty_array();
    let arg_path: Handle<JsString> = cx.argument::<JsString>(0)?;
    let path_value = arg_path.value();
    let the_path = Path::new(&path_value);

    let mut i = 0;
    for entry in fs::read_dir(the_path).unwrap() {
        let path = entry.unwrap().path();
        let value = cx.string(&path.to_str().unwrap());
        array.set(&mut cx, i, value)?;
        i += 1;
    }
    Ok(array)
}

register_module!(mut cx, {
    cx.export_function("ls", ls)?;
    Ok(())
});
