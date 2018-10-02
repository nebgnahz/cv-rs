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

void calc_optical_flow_df(cv::Mat* from, cv::Mat* to, cv::Mat* out) {

	auto array_from = cv::InputArray(*from);
	auto array_to = cv::InputArray(*to);
	auto array_out = cv::InputOutputArray(*out);

	auto deep_flow = cv::optflow::createOptFlow_DeepFlow();
	deep_flow->calc(array_from, array_to, array_out);
}

void calc_optical_flow_farneback(cv::Mat* from, cv::Mat* to, cv::Mat* out) {

	auto array_from = cv::InputArray(*from);
	auto array_to = cv::InputArray(*to);
	auto array_out = cv::InputOutputArray(*out);

	auto deep_flow = cv::optflow::createOptFlow_Farneback();
	deep_flow->calc(array_from, array_to, array_out);
}

void calc_optical_flow_dis(cv::Mat* from, cv::Mat* to, cv::Mat* out,
		unsigned int preset) {

	auto array_from = cv::InputArray(*from);
	auto array_to = cv::InputArray(*to);
	auto array_out = cv::InputOutputArray(*out);

	auto deep_flow = cv::optflow::createOptFlow_DIS(preset);
	deep_flow->calc(array_from, array_to, array_out);
}

void calc_optical_flow_std(cv::Mat* from, cv::Mat* to, cv::Mat* out) {

	auto array_from = cv::InputArray(*from);
	auto array_to = cv::InputArray(*to);
	auto array_out = cv::InputOutputArray(*out);

	auto deep_flow = cv::optflow::createOptFlow_SparseToDense();
	deep_flow->calc(array_from, array_to, array_out);
}

}
