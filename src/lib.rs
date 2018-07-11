#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


pub mod mupdf {
    include!(concat!(env!("OUT_DIR"), "/mupdf_bindings.rs"));
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn page_count() {
        unsafe {
            let context = mupdf::fz_new_context_imp(std::ptr::null(), std::ptr::null(), 0, CString::new("1.13.0").unwrap().as_ptr());
            mupdf::fz_register_document_handlers(context);
            let doc = mupdf::fz_open_document(context, CString::new("test.pdf").unwrap().as_ptr());
            let page_count = mupdf::fz_count_pages(context, doc);
            assert_eq!(page_count, 3);     
        }
    }
}
