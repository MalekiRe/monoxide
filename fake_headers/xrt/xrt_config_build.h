// Copyright 2020-2022, Collabora, Ltd.
// SPDX-License-Identifier: BSL-1.0
/*!
 * @file
 * @brief  What internal components we are building.
 * @author Jakob Bornecrantz <jakob@collabora.com>
 * @ingroup xrt_iface
 */

#pragma once

/* keep sorted */

#define XRT_FEATURE_COLOR_LOG
#define XRT_FEATURE_COMPOSITOR_MAIN
#define XRT_FEATURE_COMPOSITOR_NULL
#define XRT_FEATURE_IPC
#define XRT_FEATURE_OPENXR
/* #undef XRT_FEATURE_OPENXR_DEBUG_UTILS */
#define XRT_FEATURE_OPENXR_LAYER_CUBE
#define XRT_FEATURE_OPENXR_LAYER_CYLINDER
#define XRT_FEATURE_OPENXR_LAYER_DEPTH
#define XRT_FEATURE_OPENXR_LAYER_EQUIRECT1
#define XRT_FEATURE_OPENXR_LAYER_EQUIRECT2
#define XRT_FEATURE_RENDERDOC
#define XRT_FEATURE_SERVICE
/* #undef XRT_FEATURE_SLAM */
/* #undef XRT_FEATURE_TRACING */
/* #undef XRT_FEATURE_CLIENT_DEBUG_GUI */
#define XRT_FEATURE_WINDOW_PEEK
#define XRT_IPC_MSG_SOCK_FILENAME "monado_comp_ipc"
