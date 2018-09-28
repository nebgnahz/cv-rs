#include "optflow.h"
#include "utils.h"

extern "C" {

void calc_optical_flow_sf(cv::Mat* from, cv::Mat* to, cv::Mat* out, int layers,
		int averaging_block_size, int max_flow, double sigma_dist,
		double sigma_color, int postprocess_window, double sigma_dist_fix,
		double sigma_color_fix, double occ_thr, int upscale_averaging_radius,
		double upscale_sigma_dist, double upscale_sigma_color,
		double speed_up_thr) {

	auto array_from = cv::InputArray(*from);
	auto array_to = cv::InputArray(*to);
	auto array_out = cv::OutputArray(*out);
	cv::optflow::calcOpticalFlowSF(array_from, array_to, array_out, layers,
			averaging_block_size, max_flow, sigma_dist, sigma_color,
			postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr,
			upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color,
			speed_up_thr);
}

}
