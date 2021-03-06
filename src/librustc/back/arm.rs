// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use back::target_strs;
use driver::session::sess_os_to_meta_os;
use metadata::loader::meta_section_name;
use syntax::abi;

pub fn get_target_strs(target_triple: ~str, target_os: abi::Os) -> target_strs::t {
    let cc_args = if target_triple.contains("thumb") {
        vec!("-mthumb".to_owned())
    } else {
        vec!("-marm".to_owned())
    };
    return target_strs::t {
        module_asm: "".to_owned(),

        meta_sect_name: meta_section_name(sess_os_to_meta_os(target_os)).to_owned(),

        data_layout: match target_os {
          abi::OsMacos => {
            "e-p:32:32:32".to_owned() +
                "-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64" +
                "-f32:32:32-f64:64:64" +
                "-v64:64:64-v128:64:128" +
                "-a0:0:64-n32"
          }

          abi::OsWin32 => {
            "e-p:32:32:32".to_owned() +
                "-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64" +
                "-f32:32:32-f64:64:64" +
                "-v64:64:64-v128:64:128" +
                "-a0:0:64-n32"
          }

          abi::OsLinux => {
            "e-p:32:32:32".to_owned() +
                "-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64" +
                "-f32:32:32-f64:64:64" +
                "-v64:64:64-v128:64:128" +
                "-a0:0:64-n32"
          }

          abi::OsAndroid => {
            "e-p:32:32:32".to_owned() +
                "-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64" +
                "-f32:32:32-f64:64:64" +
                "-v64:64:64-v128:64:128" +
                "-a0:0:64-n32"
          }

          abi::OsFreebsd => {
            "e-p:32:32:32".to_owned() +
                "-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64" +
                "-f32:32:32-f64:64:64" +
                "-v64:64:64-v128:64:128" +
                "-a0:0:64-n32"
          }
        },

        target_triple: target_triple,

        cc_args: cc_args,
    };
}
