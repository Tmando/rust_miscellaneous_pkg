(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"bytes/struct.Bytes.html\" title=\"struct bytes::Bytes\">Bytes</a>","synthetic":false,"types":["bytes::bytes::Bytes"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"bytes/struct.BytesMut.html\" title=\"struct bytes::BytesMut\">BytesMut</a>","synthetic":false,"types":["bytes::bytes_mut::BytesMut"]}];
implementors["generic_array"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.62.1/core/primitive.u8.html\">u8</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"generic_array/struct.GenericArray.html\" title=\"struct generic_array::GenericArray\">GenericArray</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.62.1/core/primitive.u8.html\">u8</a>, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;T&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Add.html#associatedtype.Output\" title=\"type core::ops::arith::Add::Output\">Output</a>: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.62.1/core/primitive.u8.html\">u8</a>&gt;,&nbsp;</span>","synthetic":false,"types":["generic_array::GenericArray"]}];
implementors["half"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"half/struct.bf16.html\" title=\"struct half::bf16\">bf16</a>","synthetic":false,"types":["half::bfloat::bf16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"half/struct.f16.html\" title=\"struct half::f16\">f16</a>","synthetic":false,"types":["half::binary16::f16"]}];
implementors["itertools"] = [{"text":"impl&lt;'a, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"itertools/structs/struct.Format.html\" title=\"struct itertools::structs::Format\">Format</a>&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.62.1/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a>,&nbsp;</span>","synthetic":false,"types":["itertools::format::Format"]}];
implementors["nalgebra"] = [{"text":"impl&lt;T, R:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, R, C&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::base::matrix::Matrix"]}];
implementors["num_complex"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> + <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["num_complex::Complex"]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"num_integer/trait.Integer.html\" title=\"trait num_integer::Integer\">Integer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;","synthetic":false,"types":["num_rational::Ratio"]}];
implementors["openssl"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/cms/struct.CMSOptions.html\" title=\"struct openssl::cms::CMSOptions\">CMSOptions</a>","synthetic":false,"types":["openssl::cms::CMSOptions"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ocsp/struct.OcspFlag.html\" title=\"struct openssl::ocsp::OcspFlag\">OcspFlag</a>","synthetic":false,"types":["openssl::ocsp::OcspFlag"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/pkcs7/struct.Pkcs7Flags.html\" title=\"struct openssl::pkcs7::Pkcs7Flags\">Pkcs7Flags</a>","synthetic":false,"types":["openssl::pkcs7::Pkcs7Flags"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslOptions.html\" title=\"struct openssl::ssl::SslOptions\">SslOptions</a>","synthetic":false,"types":["openssl::ssl::SslOptions"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslMode.html\" title=\"struct openssl::ssl::SslMode\">SslMode</a>","synthetic":false,"types":["openssl::ssl::SslMode"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslVerifyMode.html\" title=\"struct openssl::ssl::SslVerifyMode\">SslVerifyMode</a>","synthetic":false,"types":["openssl::ssl::SslVerifyMode"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslSessionCacheMode.html\" title=\"struct openssl::ssl::SslSessionCacheMode\">SslSessionCacheMode</a>","synthetic":false,"types":["openssl::ssl::SslSessionCacheMode"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ssl/struct.ExtensionContext.html\" title=\"struct openssl::ssl::ExtensionContext\">ExtensionContext</a>","synthetic":false,"types":["openssl::ssl::ExtensionContext"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/ssl/struct.ShutdownState.html\" title=\"struct openssl::ssl::ShutdownState\">ShutdownState</a>","synthetic":false,"types":["openssl::ssl::ShutdownState"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/x509/verify/struct.X509CheckFlags.html\" title=\"struct openssl::x509::verify::X509CheckFlags\">X509CheckFlags</a>","synthetic":false,"types":["openssl::x509::verify::X509CheckFlags"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"openssl/x509/verify/struct.X509VerifyFlags.html\" title=\"struct openssl::x509::verify::X509VerifyFlags\">X509VerifyFlags</a>","synthetic":false,"types":["openssl::x509::verify::X509VerifyFlags"]}];
implementors["png"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>","synthetic":false,"types":["png::common::Transformations"]}];
implementors["safe_arch"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"safe_arch/struct.m128.html\" title=\"struct safe_arch::m128\">m128</a>","synthetic":false,"types":["safe_arch::x86_x64::m128_::m128"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"safe_arch/struct.m128d.html\" title=\"struct safe_arch::m128d\">m128d</a>","synthetic":false,"types":["safe_arch::x86_x64::m128d_::m128d"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"safe_arch/struct.m128i.html\" title=\"struct safe_arch::m128i\">m128i</a>","synthetic":false,"types":["safe_arch::x86_x64::m128i_::m128i"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"safe_arch/struct.m256.html\" title=\"struct safe_arch::m256\">m256</a>","synthetic":false,"types":["safe_arch::x86_x64::m256_::m256"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"safe_arch/struct.m256d.html\" title=\"struct safe_arch::m256d\">m256d</a>","synthetic":false,"types":["safe_arch::x86_x64::m256d_::m256d"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"safe_arch/struct.m256i.html\" title=\"struct safe_arch::m256i\">m256i</a>","synthetic":false,"types":["safe_arch::x86_x64::m256i_::m256i"]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"tinyvec/struct.ArrayVec.html\" title=\"struct tinyvec::ArrayVec\">ArrayVec</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a>,&nbsp;</span>","synthetic":false,"types":["tinyvec::arrayvec::ArrayVec"]},{"text":"impl&lt;'s, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"tinyvec/struct.SliceVec.html\" title=\"struct tinyvec::SliceVec\">SliceVec</a>&lt;'s, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a>,&nbsp;</span>","synthetic":false,"types":["tinyvec::slicevec::SliceVec"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"enum\" href=\"tinyvec/enum.TinyVec.html\" title=\"enum tinyvec::TinyVec\">TinyVec</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a>,&nbsp;</span>","synthetic":false,"types":["tinyvec::tinyvec::TinyVec"]}];
implementors["uuid"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>","synthetic":false,"types":["uuid::Uuid"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"uuid/fmt/struct.Hyphenated.html\" title=\"struct uuid::fmt::Hyphenated\">Hyphenated</a>","synthetic":false,"types":["uuid::fmt::Hyphenated"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"uuid/fmt/struct.Simple.html\" title=\"struct uuid::fmt::Simple\">Simple</a>","synthetic":false,"types":["uuid::fmt::Simple"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"uuid/fmt/struct.Urn.html\" title=\"struct uuid::fmt::Urn\">Urn</a>","synthetic":false,"types":["uuid::fmt::Urn"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"uuid/fmt/struct.Braced.html\" title=\"struct uuid::fmt::Braced\">Braced</a>","synthetic":false,"types":["uuid::fmt::Braced"]}];
implementors["wide"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.f32x8.html\" title=\"struct wide::f32x8\">f32x8</a>","synthetic":false,"types":["wide::f32x8_::f32x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.f32x4.html\" title=\"struct wide::f32x4\">f32x4</a>","synthetic":false,"types":["wide::f32x4_::f32x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.f64x4.html\" title=\"struct wide::f64x4\">f64x4</a>","synthetic":false,"types":["wide::f64x4_::f64x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.f64x2.html\" title=\"struct wide::f64x2\">f64x2</a>","synthetic":false,"types":["wide::f64x2_::f64x2"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i8x32.html\" title=\"struct wide::i8x32\">i8x32</a>","synthetic":false,"types":["wide::i8x32_::i8x32"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i8x16.html\" title=\"struct wide::i8x16\">i8x16</a>","synthetic":false,"types":["wide::i8x16_::i8x16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i16x8.html\" title=\"struct wide::i16x8\">i16x8</a>","synthetic":false,"types":["wide::i16x8_::i16x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i16x16.html\" title=\"struct wide::i16x16\">i16x16</a>","synthetic":false,"types":["wide::i16x16_::i16x16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i32x8.html\" title=\"struct wide::i32x8\">i32x8</a>","synthetic":false,"types":["wide::i32x8_::i32x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i32x4.html\" title=\"struct wide::i32x4\">i32x4</a>","synthetic":false,"types":["wide::i32x4_::i32x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i64x2.html\" title=\"struct wide::i64x2\">i64x2</a>","synthetic":false,"types":["wide::i64x2_::i64x2"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.i64x4.html\" title=\"struct wide::i64x4\">i64x4</a>","synthetic":false,"types":["wide::i64x4_::i64x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.u8x16.html\" title=\"struct wide::u8x16\">u8x16</a>","synthetic":false,"types":["wide::u8x16_::u8x16"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.u16x8.html\" title=\"struct wide::u16x8\">u16x8</a>","synthetic":false,"types":["wide::u16x8_::u16x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.u32x8.html\" title=\"struct wide::u32x8\">u32x8</a>","synthetic":false,"types":["wide::u32x8_::u32x8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.u32x4.html\" title=\"struct wide::u32x4\">u32x4</a>","synthetic":false,"types":["wide::u32x4_::u32x4"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.u64x2.html\" title=\"struct wide::u64x2\">u64x2</a>","synthetic":false,"types":["wide::u64x2_::u64x2"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> for <a class=\"struct\" href=\"wide/struct.u64x4.html\" title=\"struct wide::u64x4\">u64x4</a>","synthetic":false,"types":["wide::u64x4_::u64x4"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()