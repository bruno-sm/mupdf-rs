#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/mupdf_bindings.rs"));


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn page_count() {
        unsafe {
            let context = fz_new_context_imp(std::ptr::null(), std::ptr::null(), 0, CString::new("1.13.0").unwrap().as_ptr());
            fz_register_document_handlers(context);
            let doc = fz_open_document(context, CString::new("test.pdf").unwrap().as_ptr());
            let page_count = fz_count_pages(context, doc);
            assert_eq!(page_count, 3);     
        }
    }
}
