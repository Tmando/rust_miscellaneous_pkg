(function() {var implementors = {};
implementors["half"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"half/struct.bf16.html\" title=\"struct half::bf16\">bf16</a>","synthetic":false,"types":["half::bfloat::bf16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"half/struct.f16.html\" title=\"struct half::f16\">f16</a>","synthetic":false,"types":["half::binary16::f16"]}];
implementors["itertools"] = [{"text":"impl&lt;'a, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"itertools/structs/struct.Format.html\" title=\"struct itertools::structs::Format\">Format</a>&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.62.1/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a>,&nbsp;</span>","synthetic":false,"types":["itertools::format::Format"]}];
implementors["nalgebra"] = [{"text":"impl&lt;T, R:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, R, C&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::base::matrix::Matrix"]}];
implementors["ndarray"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a>, S, D:&nbsp;<a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"ndarray/trait.Data.html\" title=\"trait ndarray::Data\">Data</a>&lt;Elem = A&gt;,&nbsp;</span>","synthetic":false,"types":["ndarray::ArrayBase"]}];
implementors["num_complex"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> + <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["num_complex::Complex"]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"num_integer/trait.Integer.html\" title=\"trait num_integer::Integer\">Integer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;","synthetic":false,"types":["num_rational::Ratio"]}];
implementors["rust_decimal"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"rust_decimal/prelude/struct.Decimal.html\" title=\"struct rust_decimal::prelude::Decimal\">Decimal</a>","synthetic":false,"types":["rust_decimal::decimal::Decimal"]}];
implementors["safe_arch"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"safe_arch/struct.m128.html\" title=\"struct safe_arch::m128\">m128</a>","synthetic":false,"types":["safe_arch::x86_x64::m128_::m128"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"safe_arch/struct.m128d.html\" title=\"struct safe_arch::m128d\">m128d</a>","synthetic":false,"types":["safe_arch::x86_x64::m128d_::m128d"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"safe_arch/struct.m128i.html\" title=\"struct safe_arch::m128i\">m128i</a>","synthetic":false,"types":["safe_arch::x86_x64::m128i_::m128i"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"safe_arch/struct.m256.html\" title=\"struct safe_arch::m256\">m256</a>","synthetic":false,"types":["safe_arch::x86_x64::m256_::m256"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"safe_arch/struct.m256d.html\" title=\"struct safe_arch::m256d\">m256d</a>","synthetic":false,"types":["safe_arch::x86_x64::m256d_::m256d"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"safe_arch/struct.m256i.html\" title=\"struct safe_arch::m256i\">m256i</a>","synthetic":false,"types":["safe_arch::x86_x64::m256i_::m256i"]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"tinyvec/struct.ArrayVec.html\" title=\"struct tinyvec::ArrayVec\">ArrayVec</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a>,&nbsp;</span>","synthetic":false,"types":["tinyvec::arrayvec::ArrayVec"]},{"text":"impl&lt;'s, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"tinyvec/struct.SliceVec.html\" title=\"struct tinyvec::SliceVec\">SliceVec</a>&lt;'s, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a>,&nbsp;</span>","synthetic":false,"types":["tinyvec::slicevec::SliceVec"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"enum\" href=\"tinyvec/enum.TinyVec.html\" title=\"enum tinyvec::TinyVec\">TinyVec</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a>,&nbsp;</span>","synthetic":false,"types":["tinyvec::tinyvec::TinyVec"]}];
implementors["wide"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.f32x8.html\" title=\"struct wide::f32x8\">f32x8</a>","synthetic":false,"types":["wide::f32x8_::f32x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.f32x4.html\" title=\"struct wide::f32x4\">f32x4</a>","synthetic":false,"types":["wide::f32x4_::f32x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.f64x4.html\" title=\"struct wide::f64x4\">f64x4</a>","synthetic":false,"types":["wide::f64x4_::f64x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.f64x2.html\" title=\"struct wide::f64x2\">f64x2</a>","synthetic":false,"types":["wide::f64x2_::f64x2"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i8x32.html\" title=\"struct wide::i8x32\">i8x32</a>","synthetic":false,"types":["wide::i8x32_::i8x32"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i8x16.html\" title=\"struct wide::i8x16\">i8x16</a>","synthetic":false,"types":["wide::i8x16_::i8x16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i16x8.html\" title=\"struct wide::i16x8\">i16x8</a>","synthetic":false,"types":["wide::i16x8_::i16x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i16x16.html\" title=\"struct wide::i16x16\">i16x16</a>","synthetic":false,"types":["wide::i16x16_::i16x16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i32x8.html\" title=\"struct wide::i32x8\">i32x8</a>","synthetic":false,"types":["wide::i32x8_::i32x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i32x4.html\" title=\"struct wide::i32x4\">i32x4</a>","synthetic":false,"types":["wide::i32x4_::i32x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i64x2.html\" title=\"struct wide::i64x2\">i64x2</a>","synthetic":false,"types":["wide::i64x2_::i64x2"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.i64x4.html\" title=\"struct wide::i64x4\">i64x4</a>","synthetic":false,"types":["wide::i64x4_::i64x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.u8x16.html\" title=\"struct wide::u8x16\">u8x16</a>","synthetic":false,"types":["wide::u8x16_::u8x16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.u16x8.html\" title=\"struct wide::u16x8\">u16x8</a>","synthetic":false,"types":["wide::u16x8_::u16x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.u32x8.html\" title=\"struct wide::u32x8\">u32x8</a>","synthetic":false,"types":["wide::u32x8_::u32x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.u32x4.html\" title=\"struct wide::u32x4\">u32x4</a>","synthetic":false,"types":["wide::u32x4_::u32x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.u64x2.html\" title=\"struct wide::u64x2\">u64x2</a>","synthetic":false,"types":["wide::u64x2_::u64x2"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> for <a class=\"struct\" href=\"wide/struct.u64x4.html\" title=\"struct wide::u64x4\">u64x4</a>","synthetic":false,"types":["wide::u64x4_::u64x4"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()