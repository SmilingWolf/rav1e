// Copyright (c) 2019, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[cfg(all(target_arch = "x86_64", feature = "nasm"))]
pub mod cdef;
#[cfg(all(target_arch = "x86_64", feature = "nasm"))]
pub mod dist;
#[cfg(all(target_arch = "x86_64", feature = "nasm"))]
pub mod mc;
