/*
 * Copyright (c) 2018, 2023 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

//! Encode and decode data in base 256

#[cfg(not(any(
    feature = "encode",
    feature = "decode",
    feature = "wl_eff_encode",
    feature = "wl_pgp_encode"
)))]
compile_error!("Building lib target requires that at least one of the following features is enabled: encode; decode; wl_eff_encode; wl_pgp_encode");

#[cfg(feature = "decode")]
mod decode;
#[cfg(feature = "encode")]
mod encode;

#[cfg(feature = "decode")]
pub use decode::*;
#[cfg(feature = "encode")]
pub use encode::*;
