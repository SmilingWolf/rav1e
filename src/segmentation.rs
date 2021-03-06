// Copyright (c) 2018, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(safe_extern_statics)]

use crate::context::*;
use crate::header::PRIMARY_REF_NONE;
use crate::util::Pixel;
use crate::FrameInvariants;
use crate::FrameState;

pub fn segmentation_optimize<T: Pixel>(
  fi: &FrameInvariants<T>, fs: &mut FrameState<T>,
) {
  fs.segmentation.enabled = true;
  fs.segmentation.update_map = true;

  // We don't change the values between frames.
  fs.segmentation.update_data = fi.primary_ref_frame == PRIMARY_REF_NONE;

  // A series of AWCY runs with deltas 13, 15, 17, 18, 19, 20, 21, 22, 23
  // showed this to be the optimal one.
  const TEMPORAL_RDO_QI_DELTA: i16 = 21;

  // Fill in 3 slots with 0, delta, -delta. The slot IDs are also used in
  // luma_chroma_mode_rdo() so if you change things here make sure to check
  // that place too.
  for i in 0..3 {
    fs.segmentation.features[i][SegLvl::SEG_LVL_ALT_Q as usize] = true;
    fs.segmentation.data[i][SegLvl::SEG_LVL_ALT_Q as usize] = match i {
      0 => 0,
      1 => TEMPORAL_RDO_QI_DELTA,
      2 => -TEMPORAL_RDO_QI_DELTA,
      _ => unreachable!(),
    };
  }

  /* Figure out parameters */
  fs.segmentation.preskip = false;
  fs.segmentation.last_active_segid = 0;
  if fs.segmentation.enabled {
    for i in 0..8 {
      for j in 0..SegLvl::SEG_LVL_MAX as usize {
        if fs.segmentation.features[i][j] {
          fs.segmentation.last_active_segid = i as u8;
          if j >= SegLvl::SEG_LVL_REF_FRAME as usize {
            fs.segmentation.preskip = true;
          }
        }
      }
    }
  }
}
