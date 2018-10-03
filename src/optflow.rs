//! This library primarily provides a binding and API for OpenCV 3.x.
//!
//! This is a work-in-progress and modules/functions are implemented as
//! needed. Attempts to use
//! [rust-bindgen](https://github.com/servo/rust-bindgen) or
//! [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust) haven't been very
//! successful (I probably haven't tried hard enough). There is another port
//! [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
//! bindings using a Python script.
use super::*;
use std::os::raw::{c_double, c_int};

#[derive(Copy, Clone, Debug)]
///
pub enum DisPreset {
	///
	ULTRAFAST = 0,
	///
	FAST = 1,
	///
	MEDIUM = 2,
}

#[allow(non_camel_case_types)]
type c_bool = c_int;
extern "C" {
	fn calc_optical_flow_sf(
		from: *const CMat,
		to: *const CMat,
		out: *mut CMat,
		layers: c_int,
		averaging_block_size: c_int,
		max_flow: c_int,
		sigma_dist: c_double,
		sigma_color: c_double,
		postprocess_window: c_int,
		sigma_dist_fix: c_double,
		sigma_color_fix: c_double,
		occ_thr: c_double,
		upscale_averaging_radius: c_int,
		upscale_sigma_dist: c_double,
		upscale_sigma_color: c_double,
		speed_up_thr: c_double,
	);

	fn calc_optical_flow_df(from: *const CMat, to: *const CMat, out: *mut CMat);

	fn calc_optical_flow_farneback(
		from: *const CMat,
		to: *const CMat,
		out: *mut CMat,
		numLevels: c_int,
		pyrScale: c_double,
		fastPyramids: c_bool,
		winSize: c_int,
		numIters: c_int,
		polyN: c_int,
		polySigma: c_double,
		flags: c_int,
	);

	fn calc_optical_flow_dis(from: *const CMat, to: *const CMat, out: *mut CMat, preset: u32);

	fn calc_optical_flow_std(from: *const CMat, to: *const CMat, out: *mut CMat);

	fn calc_optical_flow_dtvl1(
		from: *const CMat,
		to: *const CMat,
		out: *mut CMat,
		tau: c_double,
		lambda: c_double,
		theta: c_double,
		nscales: c_int,
		warps: c_int,
		epsilon: c_double,
		innerIterations: c_int,
		outerIterations: c_int,
		scaleStep: c_double,
		gamma: c_double,
		medianFiltering: c_int,
		useInitialFlow: c_bool,
	);

}

impl Mat {
	/// Fast dense optical flow based on PyrLK sparse matches interpolation.
	///	* `from` First 8-bit 3-channel image.
	/// * `to` Second 8-bit 3-channel image of the same size as prev
	/// * `layers` Number of layers
	/// * `averaging_block_size` Size of block through which we sum up when calculate cost function
	///	for pixel
	/// * `max_flow` maximal flow that we search at each level
	/// * `sigma_dist` vector smooth spatial sigma parameter
	/// * `sigma_color` vector smooth color sigma parameter
	/// * `postprocess`_window window size for postprocess cross bilateral filter
	/// * `sigma_dist_fix` spatial sigma for postprocess cross bilateralf filter
	/// * `sigma_color_fix` color sigma for postprocess cross bilateral filter
	/// * `occ_thr` threshold for detecting occlusions
	/// * `upscale_averaging_radius` window size for bilateral upscale operation
	/// * `upscale_sigma_dist` spatial sigma for bilateral upscale operation
	/// * `upscale_sigma_color` color sigma for bilateral upscale operation
	/// * `speed_up_thr` threshold to detect point with irregular flow - where flow should be
	/// recalculated after upscale
	pub fn from_optical_flow_sf(
		from: &Mat,
		to: &Mat,
		layers: i32,
		averaging_block_size: i32,
		max_flow: i32,
		sigma_dist: f64,
		sigma_color: f64,
		postprocess_window: i32,
		sigma_dist_fix: f64,
		sigma_color_fix: f64,
		occ_thr: f64,
		upscale_averaging_radius: i32,
		upscale_sigma_dist: f64,
		upscale_sigma_color: f64,
		speed_up_thr: f64,
	) -> Mat {
		let out = CMat::new();
		unsafe {
			calc_optical_flow_sf(
				from.inner,
				to.inner,
				out,
				layers,
				averaging_block_size,
				max_flow,
				sigma_dist,
				sigma_color,
				postprocess_window,
				sigma_dist_fix,
				sigma_color_fix,
				occ_thr,
				upscale_averaging_radius,
				upscale_sigma_dist,
				upscale_sigma_color,
				speed_up_thr,
			);
		}
		Mat::from_raw(out)
	}
	///
	pub fn from_optical_flow_df(from: &Mat, to: &Mat) -> Mat {
		let out = CMat::new();
		unsafe {
			calc_optical_flow_df(from.inner, to.inner, out);
		}
		Mat::from_raw(out)
	}

	///
	pub fn from_optical_flow_farneback(
		from: &Mat,
		to: &Mat,
		num_levels: i32,
		pyr_scale: f64,
		fast_pyramids: bool,
		win_size: i32,
		num_iters: i32,
		poly_n: i32,
		poly_sigma: f64,
	) -> Mat {
		let out = CMat::new();
		unsafe {
			calc_optical_flow_farneback(
				from.inner,
				to.inner,
				out,
				num_levels,
				pyr_scale,
				fast_pyramids as c_bool,
				win_size,
				num_iters,
				poly_n,
				poly_sigma,
				0, // no flags, ignore existing
			);
		}
		Mat::from_raw(out)
	}

	///
	pub fn cfrom_optical_flow_dtvl1(
		from: &Mat,
		to: &Mat,
		tau: f64,
		lambda: f64,
		theta: f64,
		nscales: i32,
		warps: i32,
		epsilon: f64,
		inner_iterations: i32,
		outer_iterations: i32,
		scale_step: f64,
		gamma: f64,
		median_filtering: i32,
	) -> Mat {
		let out = CMat::new();
		unsafe {
			calc_optical_flow_dtvl1(
				from.inner,
				to.inner,
				out,
				tau,
				lambda,
				theta,
				nscales,
				warps,
				epsilon,
				inner_iterations,
				outer_iterations,
				scale_step,
				gamma,
				median_filtering,
				0, // false
			);
		}
		Mat::from_raw(out)
	}

	///
	pub fn calc_optical_flow_dis(from: &Mat, to: &Mat, preset: DisPreset) -> Mat {
		let out = CMat::new();
		unsafe {
			calc_optical_flow_dis(from.inner, to.inner, out, preset as u32);
		}
		Mat::from_raw(out)
	}

	///
	pub fn calc_optical_flow_std(from: &Mat, to: &Mat) -> Mat {
		let out = CMat::new();
		unsafe {
			calc_optical_flow_std(from.inner, to.inner, out);
		}
		Mat::from_raw(out)
	}
}
