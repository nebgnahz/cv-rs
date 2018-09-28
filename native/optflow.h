#ifndef CV_RS_OPTFLOW_H
#define CV_RS_OPTFLOW_H

#include "common.h"
#include <opencv2/core.hpp>
#include <opencv2/optflow.hpp>

extern "C" {
void calc_optical_flow_sf(
		cv::Mat* from,
		cv::Mat* to,
		cv::Mat* out,
		int layers,
		int averaging_block_size,
		int max_flow,
		double sigma_dist,
		double sigma_color,
		int postprocess_window,
		double sigma_dist_fix,
		double sigma_color_fix,
		double occ_thr,
		int upscale_averaging_radius,
		double upscale_sigma_dist,
		double upscale_sigma_color,
		double speed_up_thr
);

}
#endif
