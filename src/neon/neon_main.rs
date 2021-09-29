use neon::prelude::*;
use crate::models::event::Event;


use crate::process::process;
use crate::tests::default_options::DEFAULT_CHINESE_OPT_SINGLE;

fn start_process(mut cx: FunctionContext) -> JsResult<JsArray> {
    let text = cx.argument::<JsString>(0)?;
    let result = process(&text.value(&mut cx), &DEFAULT_CHINESE_OPT_SINGLE);
    Ok(vec_to_array(&result, &mut cx).unwrap())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("process", start_process)?;
    Ok(())
}

fn vec_to_array<'a, C: Context<'a>>(vec: &Vec<Event>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = s.to_neon_object(cx);
        a.set( cx, i as u32, v.unwrap())?;
    }

    Ok(a)
}
