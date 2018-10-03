#ifndef CV_RS_OPTFLOW_H
#define CV_RS_OPTFLOW_H

#include "common.h"
#include <opencv2/core.hpp>
#include <opencv2/optflow.hpp>
#include <opencv2/video/tracking.hpp>

extern "C" {
void calc_optical_flow_sf(cv::Mat* from, cv::Mat* to, cv::Mat* out, int layers,
		int averaging_block_size, int max_flow, double sigma_dist,
		double sigma_color, int postprocess_window, double sigma_dist_fix,
		double sigma_color_fix, double occ_thr, int upscale_averaging_radius,
		double upscale_sigma_dist, double upscale_sigma_color,
		double speed_up_thr);

void calc_optical_flow_df(cv::Mat* from, cv::Mat* to, cv::Mat* out);

void calc_optical_flow_farneback(cv::Mat* from, cv::Mat* to, cv::Mat* out,
		int numLevels = 5, double pyrScale = 0.5, bool fastPyramids = false,
		int winSize = 13, int numIters = 10, int polyN = 5, double polySigma =
				1.1, int flags = 0);

void calc_optical_flow_dtvl1(cv::Mat* from, cv::Mat* to, cv::Mat* out,
		double tau = 0.25, double lambda = 0.15, double theta = 0.3,
		int nscales = 5, int warps = 5, double epsilon = 0.01,
		int innerIterations = 30, int outerIterations = 10, double scaleStep =
				0.8, double gamma = 0.0, int medianFiltering = 5,
		bool useInitialFlow = false);

void calc_optical_flow_std(cv::Mat* from, cv::Mat* to, cv::Mat* out);

void calc_optical_flow_dis(cv::Mat* from, cv::Mat* to, cv::Mat* out,
		unsigned int preset);

}
#endif
