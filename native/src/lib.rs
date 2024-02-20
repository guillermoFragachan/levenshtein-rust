use levenshtein::levenshtein;
use neon::prelude::*;

fn levenshtein_distance(a: *const u8, b: *const u8) -> usize {
    let a_str = unsafe { std::ffi::CStr::from_ptr(a as *const i8).to_str().unwrap() };
    let b_str = unsafe { std::ffi::CStr::from_ptr(b as *const i8).to_str().unwrap() };
    levenshtein(a_str, b_str)
}

fn levenshtein_func(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsString>(0)?.value(&mut cx);
    let b = cx.argument::<JsString>(1)?.value(&mut cx);

    let distance = levenshtein_distance(a.as_ptr(), b.as_ptr());

    Ok(cx.number(distance as f64))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("levenshtein", levenshtein_func)?;
    Ok(())
}
