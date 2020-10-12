#[cfg(feature = "contrib")]
pub mod alphamat;
#[cfg(feature = "contrib")]
pub mod aruco;
#[cfg(feature = "contrib")]
pub mod bgsegm;
#[cfg(feature = "contrib")]
pub mod bioinspired;
pub mod calib3d;
#[cfg(feature = "contrib")]
pub mod ccalib;
pub mod core;
#[cfg(feature = "contrib")]
pub mod cudaarithm;
#[cfg(feature = "contrib")]
pub mod cudabgsegm;
#[cfg(feature = "contrib")]
pub mod cudacodec;
#[cfg(feature = "contrib")]
pub mod cudafeatures2d;
#[cfg(feature = "contrib")]
pub mod cudafilters;
#[cfg(feature = "contrib")]
pub mod cudaimgproc;
#[cfg(feature = "contrib")]
pub mod cudaobjdetect;
#[cfg(feature = "contrib")]
pub mod cudaoptflow;
#[cfg(feature = "contrib")]
pub mod cudastereo;
#[cfg(feature = "contrib")]
pub mod cudawarping;
#[cfg(feature = "contrib")]
pub mod cvv;
pub mod dnn;
pub mod dnn_superres;
#[cfg(feature = "contrib")]
pub mod dpm;
#[cfg(feature = "contrib")]
pub mod face;
pub mod features2d;
pub mod flann;
#[cfg(feature = "contrib")]
pub mod freetype;
#[cfg(feature = "contrib")]
pub mod fuzzy;
#[cfg(feature = "contrib")]
pub mod hdf;
#[cfg(feature = "contrib")]
pub mod hfs;
pub mod highgui;
#[cfg(feature = "contrib")]
pub mod img_hash;
pub mod imgcodecs;
pub mod imgproc;
#[cfg(feature = "contrib")]
pub mod intensity_transform;
#[cfg(feature = "contrib")]
pub mod line_descriptor;
#[cfg(feature = "contrib")]
pub mod mcc;
pub mod ml;
pub mod objdetect;
#[cfg(feature = "contrib")]
pub mod optflow;
#[cfg(feature = "contrib")]
pub mod ovis;
#[cfg(feature = "contrib")]
pub mod phase_unwrapping;
pub mod photo;
#[cfg(feature = "contrib")]
pub mod plot;
#[cfg(feature = "contrib")]
pub mod quality;
#[cfg(feature = "contrib")]
pub mod rapid;
#[cfg(feature = "contrib")]
pub mod rgbd;
#[cfg(feature = "contrib")]
pub mod saliency;
#[cfg(feature = "contrib")]
pub mod sfm;
#[cfg(feature = "contrib")]
pub mod shape;
#[cfg(feature = "contrib")]
pub mod stereo;
pub mod stitching;
#[cfg(feature = "contrib")]
pub mod structured_light;
#[cfg(feature = "contrib")]
pub mod superres;
#[cfg(feature = "contrib")]
pub mod surface_matching;
#[cfg(feature = "contrib")]
pub mod text;
#[cfg(feature = "contrib")]
pub mod tracking;
pub mod video;
pub mod videoio;
#[cfg(feature = "contrib")]
pub mod videostab;
pub mod viz;
#[cfg(feature = "contrib")]
pub mod xfeatures2d;
#[cfg(feature = "contrib")]
pub mod ximgproc;
#[cfg(feature = "contrib")]
pub mod xobjdetect;
#[cfg(feature = "contrib")]
pub mod xphoto;
pub mod types;
#[doc(hidden)]
pub mod sys;
pub mod hub_prelude {
	#[cfg(feature = "contrib")]
	pub use super::alphamat::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::aruco::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::bgsegm::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::bioinspired::prelude::*;
	pub use super::calib3d::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::ccalib::prelude::*;
	pub use super::core::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudaarithm::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudabgsegm::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudacodec::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudafeatures2d::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudafilters::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudaimgproc::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudaobjdetect::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudaoptflow::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudastereo::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cudawarping::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::cvv::prelude::*;
	pub use super::dnn::prelude::*;
	pub use super::dnn_superres::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::dpm::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::face::prelude::*;
	pub use super::features2d::prelude::*;
	pub use super::flann::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::freetype::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::fuzzy::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::hdf::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::hfs::prelude::*;
	pub use super::highgui::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::img_hash::prelude::*;
	pub use super::imgcodecs::prelude::*;
	pub use super::imgproc::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::intensity_transform::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::line_descriptor::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::mcc::prelude::*;
	pub use super::ml::prelude::*;
	pub use super::objdetect::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::optflow::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::ovis::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::phase_unwrapping::prelude::*;
	pub use super::photo::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::plot::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::quality::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::rapid::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::rgbd::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::saliency::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::sfm::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::shape::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::stereo::prelude::*;
	pub use super::stitching::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::structured_light::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::superres::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::surface_matching::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::text::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::tracking::prelude::*;
	pub use super::video::prelude::*;
	pub use super::videoio::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::videostab::prelude::*;
	pub use super::viz::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::xfeatures2d::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::ximgproc::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::xobjdetect::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::xphoto::prelude::*;
}
