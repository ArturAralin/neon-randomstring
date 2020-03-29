extern crate neon;
extern crate rand;

use neon::prelude::*;
use rand::Rng;

#[derive(Debug)]
struct GenOptions {
    length: usize,
    charset: String
}

fn get_options(cx: &mut FunctionContext) -> Result<GenOptions, neon::result::Throw> {
    let length = cx.argument::<JsNumber>(0)?.value() as usize;
    let charset = cx.argument::<JsString>(1)?.value();

    Ok(GenOptions {
        length,
        charset,
    })
}

fn gen_random_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let opts = get_options(&mut cx)?;

    let charset = opts.charset.chars().collect::<Vec<char>>();
    let charset_len = charset.len();

    let mut rng = rand::thread_rng();
    let result = (0..charset_len)
        .map(|_| {
            let idx = rng.gen_range(0, charset_len);
            charset[idx]
        })
        .collect::<String>();

    Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("randomString", gen_random_string)
});
