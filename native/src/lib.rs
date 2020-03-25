extern crate neon;
extern crate rand;
extern crate unicode_segmentation;
mod charset;

use neon::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct GenOptions {
    length: u32,
    charset: Vec<String>
}

fn get_options(cx: &mut FunctionContext) -> Result<GenOptions, neon::result::Throw> {
    let length = cx.argument::<JsNumber>(0)?.value() as u32;
    let charset = {
        let alphabet = charset::match_charset(&cx.argument::<JsString>(1)?.value());

        if alphabet == charset::Alphabet::CUSTOM {
            let slice = &cx.argument::<JsString>(2)?.value()[..];
            UnicodeSegmentation
                ::graphemes(slice, true)
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        } else {
            charset::get_letters(alphabet)
        }
    };

    Ok(GenOptions {
        length,
        charset: charset,
    })
}

fn gen_random_string(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let o = get_options(&mut cx)?;
    let graphemes = o.charset;
    let charset_len = graphemes.len() as u8;

    let len = o.length;
    let buf_len = o.length * 2;
    let mut buf = cx.buffer(buf_len as u32)?;

    cx.borrow_mut(&mut buf, |b| {
        let slice = b.as_mut_slice::<u8>();
        let mut i: usize = 0;

        for _ in 0..len {
            let idx = rand::random::<u8>() % charset_len;
            let r = graphemes[idx as usize].as_bytes();

            for byte in r.iter() {
                slice[i] = *byte;
                i += 1;
            };
        };
    });

    Ok(buf)
}

register_module!(mut cx, {
    cx.export_function("randomString", gen_random_string)
});
