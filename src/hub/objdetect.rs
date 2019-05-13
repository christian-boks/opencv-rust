//! # Object Detection
//! 
//! Haar Feature-based Cascade Classifier for Object Detection
//! ----------------------------------------------------------
//! 
//! The object detector described below has been initially proposed by Paul Viola [Viola01](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Viola01) and
//! improved by Rainer Lienhart [Lienhart02](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Lienhart02) .
//! 
//! First, a classifier (namely a *cascade of boosted classifiers working with haar-like features*) is
//! trained with a few hundred sample views of a particular object (i.e., a face or a car), called
//! positive examples, that are scaled to the same size (say, 20x20), and negative examples - arbitrary
//! images of the same size.
//! 
//! After a classifier is trained, it can be applied to a region of interest (of the same size as used
//! during the training) in an input image. The classifier outputs a "1" if the region is likely to show
//! the object (i.e., face/car), and "0" otherwise. To search for the object in the whole image one can
//! move the search window across the image and check every location using the classifier. The
//! classifier is designed so that it can be easily "resized" in order to be able to find the objects of
//! interest at different sizes, which is more efficient than resizing the image itself. So, to find an
//! object of an unknown size in the image the scan procedure should be done several times at different
//! scales.
//! 
//! The word "cascade" in the classifier name means that the resultant classifier consists of several
//! simpler classifiers (*stages*) that are applied subsequently to a region of interest until at some
//! stage the candidate is rejected or all the stages are passed. The word "boosted" means that the
//! classifiers at every stage of the cascade are complex themselves and they are built out of basic
//! classifiers using one of four different boosting techniques (weighted voting). Currently Discrete
//! Adaboost, Real Adaboost, Gentle Adaboost and Logitboost are supported. The basic classifiers are
//! decision-tree classifiers with at least 2 leaves. Haar-like features are the input to the basic
//! classifiers, and are calculated as described below. The current algorithm uses the following
//! Haar-like features:
//! 
//! ![image](https://docs.opencv.org/3.4.6/haarfeatures.png)
//! 
//! The feature used in a particular classifier is specified by its shape (1a, 2b etc.), position within
//! the region of interest and the scale (this scale is not the same as the scale used at the detection
//! stage, though these two scales are multiplied). For example, in the case of the third line feature
//! (2c) the response is calculated as the difference between the sum of image pixels under the
//! rectangle covering the whole feature (including the two white stripes and the black stripe in the
//! middle) and the sum of the image pixels under the black stripe multiplied by 3 in order to
//! compensate for the differences in the size of areas. The sums of pixel values over a rectangular
//! regions are calculated rapidly using integral images (see below and the integral description).
//! 
//! To see the object detector at work, have a look at the facedetect demo:
//! <https://github.com/opencv/opencv/tree/3.4/samples/cpp/dbt_face_detection.cpp>
//! 
//! The following reference is for the detection part only. There is a separate application called
//! opencv_traincascade that can train a cascade of boosted classifiers from a set of samples.
//! 
//! 
//! Note: In the new C++ interface it is also possible to use LBP (local binary pattern) features in
//! addition to Haar-like features. .. [Viola01] Paul Viola and Michael J. Jones. Rapid Object Detection
//! using a Boosted Cascade of Simple Features. IEEE CVPR, 2001. The paper is available online at
//! <http://research.microsoft.com/en-us/um/people/viola/Pubs/Detect/violaJones_CVPR2001.pdf>
//! # C API
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const CASCADE_DO_CANNY_PRUNING: i32 = 1;
pub const CASCADE_DO_ROUGH_SEARCH: i32 = 8;
pub const CASCADE_FIND_BIGGEST_OBJECT: i32 = 4;
pub const CASCADE_SCALE_IMAGE: i32 = 2;
pub const CV_HAAR_DO_CANNY_PRUNING: i32 = 1;
pub const CV_HAAR_DO_ROUGH_SEARCH: i32 = 8;
pub const CV_HAAR_FEATURE_MAX: i32 = 3;
pub const CV_HAAR_FIND_BIGGEST_OBJECT: i32 = 4;
pub const CV_HAAR_MAGIC_VAL: i32 = 0x42500000;
pub const CV_HAAR_SCALE_IMAGE: i32 = 2;
pub const CV_HAAR_STAGE_MAX: i32 = 1000;
pub const CV_TYPE_NAME_HAAR: &'static str = "opencv-haar-classifier";
pub const DetectionBasedTracker_DETECTED: i32 = 1;
pub const DetectionBasedTracker_DETECTED_NOT_SHOWN_YET: i32 = 0;
pub const DetectionBasedTracker_DETECTED_TEMPORARY_LOST: i32 = 2;
pub const DetectionBasedTracker_WRONG_OBJECT: i32 = 3;
pub const HOGDescriptor_DEFAULT_NLEVELS: i32 = 64;
pub const HOGDescriptor_L2Hys: i32 = 0;

// identifier: cv_createFaceDetectionMaskGenerator
pub fn create_face_detection_mask_generator() -> Result<types::PtrOfMaskGenerator> {
    unsafe { sys::cv_objdetect_cv_createFaceDetectionMaskGenerator() }.into_result().map(|x| types::PtrOfMaskGenerator { ptr: x })
}

// identifier: cv_detectQRCode_Mat_in_VectorOfPoint_points_double_eps_x_double_eps_y
/// Detect QR code in image and return minimum area of quadrangle that describes QR code.
/// ## Parameters
/// * in:  Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Output vector of vertices of a quadrangle of minimal area that describes QR code.
/// * eps_x: Epsilon neighborhood, which allows you to determine the horizontal pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// * eps_y: Epsilon neighborhood, which allows you to determine the vertical pattern of the scheme 1:1:3:1:1 according to QR code standard.
///
/// ## C++ default parameters:
/// * eps_x: 0.2
/// * eps_y: 0.1
pub fn detect_qr_code(_in: &core::Mat, points: &types::VectorOfPoint, eps_x: f64, eps_y: f64) -> Result<bool> {
    unsafe { sys::cv_objdetect_cv_detectQRCode_Mat_in_VectorOfPoint_points_double_eps_x_double_eps_y(_in.as_raw_Mat(), points.as_raw_VectorOfPoint(), eps_x, eps_y) }.into_result()
}

// identifier: cv_groupRectangles_VectorOfRect_rectList_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_int_groupThreshold_double_eps
/// @overload
///
/// ## C++ default parameters:
/// * eps: 0.2
pub fn group_rectangles_weights_rejects(rect_list: &types::VectorOfRect, reject_levels: &types::VectorOfint, level_weights: &types::VectorOfdouble, group_threshold: i32, eps: f64) -> Result<()> {
    unsafe { sys::cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_int_groupThreshold_double_eps(rect_list.as_raw_VectorOfRect(), reject_levels.as_raw_VectorOfint(), level_weights.as_raw_VectorOfdouble(), group_threshold, eps) }.into_result()
}

// identifier: cv_groupRectangles_VectorOfRect_rectList_VectorOfint_weights_int_groupThreshold_double_eps
/// @overload
///
/// ## C++ default parameters:
/// * eps: 0.2
pub fn group_rectangle_weights(rect_list: &types::VectorOfRect, weights: &types::VectorOfint, group_threshold: i32, eps: f64) -> Result<()> {
    unsafe { sys::cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_VectorOfint_weights_int_groupThreshold_double_eps(rect_list.as_raw_VectorOfRect(), weights.as_raw_VectorOfint(), group_threshold, eps) }.into_result()
}

// identifier: cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps
/// <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>
///  Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// <span lang='latex'>\texttt{eps}\rightarrow +\inf</span> , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
///
/// ## C++ default parameters:
/// * eps: 0.2
pub fn group_rectangle(rect_list: &types::VectorOfRect, group_threshold: i32, eps: f64) -> Result<()> {
    unsafe { sys::cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps(rect_list.as_raw_VectorOfRect(), group_threshold, eps) }.into_result()
}

// identifier: cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps_VectorOfint_weights_VectorOfdouble_levelWeights
/// @overload
pub fn group_rectangle_levelweights(rect_list: &types::VectorOfRect, group_threshold: i32, eps: f64, weights: &types::VectorOfint, level_weights: &types::VectorOfdouble) -> Result<()> {
    unsafe { sys::cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps_VectorOfint_weights_VectorOfdouble_levelWeights(rect_list.as_raw_VectorOfRect(), group_threshold, eps, weights.as_raw_VectorOfint(), level_weights.as_raw_VectorOfdouble()) }.into_result()
}

// identifier: cv_groupRectangles_meanshift_VectorOfRect_rectList_VectorOfdouble_foundWeights_VectorOfdouble_foundScales_double_detectThreshold_Size_winDetSize
/// @overload
///
/// ## C++ default parameters:
/// * detect_threshold: 0.0
/// * win_det_size: Size(64, 128)
pub fn group_rectangles_meanshift(rect_list: &types::VectorOfRect, found_weights: &types::VectorOfdouble, found_scales: &types::VectorOfdouble, detect_threshold: f64, win_det_size: core::Size) -> Result<()> {
    unsafe { sys::cv_objdetect_cv_groupRectangles_meanshift_VectorOfRect_rectList_VectorOfdouble_foundWeights_VectorOfdouble_foundScales_double_detectThreshold_Size_winDetSize(rect_list.as_raw_VectorOfRect(), found_weights.as_raw_VectorOfdouble(), found_scales.as_raw_VectorOfdouble(), detect_threshold, win_det_size) }.into_result()
}

// Generating impl for trait cv::BaseCascadeClassifier (trait)
pub trait BaseCascadeClassifier : core::Algorithm {
    #[doc(hidden)] fn as_raw_BaseCascadeClassifier(&self) -> *mut c_void;
    // identifier: cv_BaseCascadeClassifier_empty_const
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_empty_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_load_String_filename
    fn load(&mut self, filename: &str) -> Result<bool> {
        string_arg!(filename);
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_load_String_filename(self.as_raw_BaseCascadeClassifier(), filename.as_ptr()) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize
    fn detect_multi_scale(&mut self, image: &core::Mat, objects: &types::VectorOfRect, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(self.as_raw_BaseCascadeClassifier(), image.as_raw_Mat(), objects.as_raw_VectorOfRect(), scale_factor, min_neighbors, flags, min_size, max_size) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize
    fn detect_multi_scale_1(&mut self, image: &core::Mat, objects: &types::VectorOfRect, num_detections: &types::VectorOfint, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(self.as_raw_BaseCascadeClassifier(), image.as_raw_Mat(), objects.as_raw_VectorOfRect(), num_detections.as_raw_VectorOfint(), scale_factor, min_neighbors, flags, min_size, max_size) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels
    fn detect_multi_scale_2(&mut self, image: &core::Mat, objects: &types::VectorOfRect, reject_levels: &types::VectorOfint, level_weights: &types::VectorOfdouble, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels(self.as_raw_BaseCascadeClassifier(), image.as_raw_Mat(), objects.as_raw_VectorOfRect(), reject_levels.as_raw_VectorOfint(), level_weights.as_raw_VectorOfdouble(), scale_factor, min_neighbors, flags, min_size, max_size, output_reject_levels) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_isOldFormatCascade_const
    fn is_old_format_cascade(&self) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_isOldFormatCascade_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_getOriginalWindowSize_const
    fn get_original_window_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_getOriginalWindowSize_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_getFeatureType_const
    fn get_feature_type(&self) -> Result<i32> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_getFeatureType_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_getOldCascade
    fn get_old_cascade(&mut self) -> Result<&mut c_void> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_getOldCascade(self.as_raw_BaseCascadeClassifier()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    // identifier: cv_BaseCascadeClassifier_setMaskGenerator_PtrOfMaskGenerator_maskGenerator
    fn set_mask_generator(&mut self, mask_generator: &types::PtrOfMaskGenerator) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_setMaskGenerator_PtrOfMaskGenerator_maskGenerator(self.as_raw_BaseCascadeClassifier(), mask_generator.as_raw_PtrOfMaskGenerator()) }.into_result()
    }
    
    // identifier: cv_BaseCascadeClassifier_getMaskGenerator
    fn get_mask_generator(&mut self) -> Result<types::PtrOfMaskGenerator> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_getMaskGenerator(self.as_raw_BaseCascadeClassifier()) }.into_result().map(|x| types::PtrOfMaskGenerator { ptr: x })
    }
    
}

impl<'a> BaseCascadeClassifier + 'a {

}

// Generating impl for trait cv::BaseCascadeClassifier::MaskGenerator (trait)
pub trait BaseCascadeClassifier_MaskGenerator {
    #[doc(hidden)] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void;
    // identifier: cv_BaseCascadeClassifier_MaskGenerator_generateMask_Mat_src
    fn generate_mask(&mut self, src: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_MaskGenerator_generateMask_Mat_src(self.as_raw_BaseCascadeClassifier_MaskGenerator(), src.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_BaseCascadeClassifier_MaskGenerator_initializeMask_Mat_unnamed_arg
    fn initialize_mask(&mut self, unnamed_arg: &core::Mat) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_BaseCascadeClassifier_MaskGenerator_initializeMask_Mat_unnamed_arg(self.as_raw_BaseCascadeClassifier_MaskGenerator(), unnamed_arg.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> BaseCascadeClassifier_MaskGenerator + 'a {

}

// boxed class cv::CascadeClassifier
/// Cascade classifier class for object detection.
#[allow(dead_code)]
pub struct CascadeClassifier {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::CascadeClassifier {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_CascadeClassifier(self.ptr) };
    }
}
impl crate::objdetect::CascadeClassifier {
    #[doc(hidden)] pub fn as_raw_CascadeClassifier(&self) -> *mut c_void { self.ptr }
}

impl CascadeClassifier {

    // identifier: cv_CascadeClassifier_CascadeClassifier
    pub fn default() -> Result<crate::objdetect::CascadeClassifier> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_CascadeClassifier() }.into_result().map(|x| crate::objdetect::CascadeClassifier { ptr: x })
    }
    
    // identifier: cv_CascadeClassifier_CascadeClassifier_String_filename
    /// Loads a classifier from a file.
    /// 
    /// ## Parameters
    /// * filename: Name of the file from which the classifier is loaded.
    pub fn new(filename: &str) -> Result<crate::objdetect::CascadeClassifier> {
        string_arg!(filename);
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_CascadeClassifier_String_filename(filename.as_ptr()) }.into_result().map(|x| crate::objdetect::CascadeClassifier { ptr: x })
    }
    
    // identifier: cv_CascadeClassifier_empty_const
    /// Checks whether the classifier has been loaded.
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_empty_const(self.as_raw_CascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_load_String_filename
    /// Loads a classifier from a file.
    /// 
    /// ## Parameters
    /// * filename: Name of the file from which the classifier is loaded. The file may contain an old
    /// HAAR classifier trained by the haartraining application or a new cascade classifier trained by the
    /// traincascade application.
    pub fn load(&mut self, filename: &str) -> Result<bool> {
        string_arg!(filename);
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_load_String_filename(self.as_raw_CascadeClassifier(), filename.as_ptr()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize
    /// Detects objects of different sizes in the input image. The detected objects are returned as a list
    /// of rectangles.
    /// 
    /// ## Parameters
    /// * image: Matrix of the type CV_8U containing an image where objects are detected.
    /// * objects: Vector of rectangles where each rectangle contains the detected object, the
    /// rectangles may be partially outside the original image.
    /// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
    /// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
    /// to retain it.
    /// * flags: Parameter with the same meaning for an old cascade as in the function
    /// cvHaarDetectObjects. It is not used for a new cascade.
    /// * minSize: Minimum possible object size. Objects smaller than that are ignored.
    /// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
    /// 
    /// The function is parallelized with the TBB library.
    /// 
    /// 
    /// Note:
    /// *   (Python) A face detection example using cascade classifiers can be found at
    /// opencv_source_code/samples/python/facedetect.py
    ///
    /// ## C++ default parameters:
    /// * scale_factor: 1.1
    /// * min_neighbors: 3
    /// * flags: 0
    /// * min_size: Size()
    /// * max_size: Size()
    pub fn detect_multi_scale(&mut self, image: &core::Mat, objects: &types::VectorOfRect, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(self.as_raw_CascadeClassifier(), image.as_raw_Mat(), objects.as_raw_VectorOfRect(), scale_factor, min_neighbors, flags, min_size, max_size) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize
    /// @overload
    /// ## Parameters
    /// * image: Matrix of the type CV_8U containing an image where objects are detected.
    /// * objects: Vector of rectangles where each rectangle contains the detected object, the
    /// rectangles may be partially outside the original image.
    /// * numDetections: Vector of detection numbers for the corresponding objects. An object's number
    /// of detections is the number of neighboring positively classified rectangles that were joined
    /// together to form the object.
    /// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
    /// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
    /// to retain it.
    /// * flags: Parameter with the same meaning for an old cascade as in the function
    /// cvHaarDetectObjects. It is not used for a new cascade.
    /// * minSize: Minimum possible object size. Objects smaller than that are ignored.
    /// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
    ///
    /// ## C++ default parameters:
    /// * scale_factor: 1.1
    /// * min_neighbors: 3
    /// * flags: 0
    /// * min_size: Size()
    /// * max_size: Size()
    pub fn detect_multi_scale_num(&mut self, image: &core::Mat, objects: &types::VectorOfRect, num_detections: &types::VectorOfint, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(self.as_raw_CascadeClassifier(), image.as_raw_Mat(), objects.as_raw_VectorOfRect(), num_detections.as_raw_VectorOfint(), scale_factor, min_neighbors, flags, min_size, max_size) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels
    /// @overload
    /// This function allows you to retrieve the final stage decision certainty of classification.
    /// For this, one needs to set `outputRejectLevels` on true and provide the `rejectLevels` and `levelWeights` parameter.
    /// For each resulting detection, `levelWeights` will then contain the certainty of classification at the final stage.
    /// This value can then be used to separate strong from weaker classifications.
    /// 
    /// A code sample on how to use it efficiently can be found below:
    /// ```ignore
    /// Mat img;
    /// vector<double> weights;
    /// vector<int> levels;
    /// vector<Rect> detections;
    /// CascadeClassifier model("/path/to/your/model.xml");
    /// model.detectMultiScale(img, detections, levels, weights, 1.1, 3, 0, Size(), Size(), true);
    /// cerr << "Detection " << detections[0] << " with weight " << weights[0] << endl;
    /// ```
    ///
    /// ## C++ default parameters:
    /// * scale_factor: 1.1
    /// * min_neighbors: 3
    /// * flags: 0
    /// * min_size: Size()
    /// * max_size: Size()
    /// * output_reject_levels: false
    pub fn detect_multi_scale_levels(&mut self, image: &core::Mat, objects: &types::VectorOfRect, reject_levels: &types::VectorOfint, level_weights: &types::VectorOfdouble, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels(self.as_raw_CascadeClassifier(), image.as_raw_Mat(), objects.as_raw_VectorOfRect(), reject_levels.as_raw_VectorOfint(), level_weights.as_raw_VectorOfdouble(), scale_factor, min_neighbors, flags, min_size, max_size, output_reject_levels) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_isOldFormatCascade_const
    pub fn is_old_format_cascade(&self) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_isOldFormatCascade_const(self.as_raw_CascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_getOriginalWindowSize_const
    pub fn get_original_window_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_getOriginalWindowSize_const(self.as_raw_CascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_getFeatureType_const
    pub fn get_feature_type(&self) -> Result<i32> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_getFeatureType_const(self.as_raw_CascadeClassifier()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_getOldCascade
    pub fn get_old_cascade(&mut self) -> Result<&mut c_void> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_getOldCascade(self.as_raw_CascadeClassifier()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    // identifier: cv_CascadeClassifier_convert_String_oldcascade_String_newcascade
    pub fn convert(oldcascade: &str, newcascade: &str) -> Result<bool> {
        string_arg!(oldcascade);
        string_arg!(newcascade);
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_convert_String_oldcascade_String_newcascade(oldcascade.as_ptr(), newcascade.as_ptr()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_setMaskGenerator_PtrOfMaskGenerator_maskGenerator
    pub fn set_mask_generator(&mut self, mask_generator: &types::PtrOfMaskGenerator) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_setMaskGenerator_PtrOfMaskGenerator_maskGenerator(self.as_raw_CascadeClassifier(), mask_generator.as_raw_PtrOfMaskGenerator()) }.into_result()
    }
    
    // identifier: cv_CascadeClassifier_getMaskGenerator
    pub fn get_mask_generator(&mut self) -> Result<types::PtrOfMaskGenerator> {
        unsafe { sys::cv_objdetect_cv_CascadeClassifier_getMaskGenerator(self.as_raw_CascadeClassifier()) }.into_result().map(|x| types::PtrOfMaskGenerator { ptr: x })
    }
    
}

// boxed class cv::DetectionBasedTracker
#[allow(dead_code)]
pub struct DetectionBasedTracker {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::DetectionBasedTracker {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DetectionBasedTracker(self.ptr) };
    }
}
impl crate::objdetect::DetectionBasedTracker {
    #[doc(hidden)] pub fn as_raw_DetectionBasedTracker(&self) -> *mut c_void { self.ptr }
}

impl DetectionBasedTracker {

    // identifier: cv_DetectionBasedTracker_run
    pub fn run(&mut self) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_run(self.as_raw_DetectionBasedTracker()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_stop
    pub fn stop(&mut self) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_stop(self.as_raw_DetectionBasedTracker()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_resetTracking
    pub fn reset_tracking(&mut self) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_resetTracking(self.as_raw_DetectionBasedTracker()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_process_Mat_imageGray
    pub fn process(&mut self, image_gray: &core::Mat) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_process_Mat_imageGray(self.as_raw_DetectionBasedTracker(), image_gray.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_setParameters_Parameters_params
    pub fn set_parameters(&mut self, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_setParameters_Parameters_params(self.as_raw_DetectionBasedTracker(), params.as_raw_DetectionBasedTracker_Parameters()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_getParameters_const
    pub fn get_parameters(&self) -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_getParameters_const(self.as_raw_DetectionBasedTracker()) }.into_result().map(|x| crate::objdetect::DetectionBasedTracker_Parameters { ptr: x })
    }
    
    // identifier: cv_DetectionBasedTracker_getObjects_const_VectorOfRect_result
    pub fn get_objects(&self, result: &types::VectorOfRect) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_getObjects_const_VectorOfRect_result(self.as_raw_DetectionBasedTracker(), result.as_raw_VectorOfRect()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_getObjects_const_VectorOfExtObject_result
    pub fn get_objects_1(&self, result: &types::VectorOfExtObject) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_getObjects_const_VectorOfExtObject_result(self.as_raw_DetectionBasedTracker(), result.as_raw_VectorOfExtObject()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_addObject_Rect_location
    pub fn add_object(&mut self, location: core::Rect) -> Result<i32> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_addObject_Rect_location(self.as_raw_DetectionBasedTracker(), location) }.into_result()
    }
    
}

// boxed class cv::DetectionBasedTracker::ExtObject
#[allow(dead_code)]
pub struct DetectionBasedTracker_ExtObject {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::DetectionBasedTracker_ExtObject {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DetectionBasedTracker_ExtObject(self.ptr) };
    }
}
impl crate::objdetect::DetectionBasedTracker_ExtObject {
    #[doc(hidden)] pub fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *mut c_void { self.ptr }
}

// Generating impl for trait cv::DetectionBasedTracker::IDetector (trait)
pub trait DetectionBasedTracker_IDetector {
    #[doc(hidden)] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *mut c_void;
    // identifier: cv_DetectionBasedTracker_IDetector_detect_Mat_image_VectorOfRect_objects
    fn detect(&mut self, image: &core::Mat, objects: &types::VectorOfRect) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_detect_Mat_image_VectorOfRect_objects(self.as_raw_DetectionBasedTracker_IDetector(), image.as_raw_Mat(), objects.as_raw_VectorOfRect()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_setMinObjectSize_Size_min
    fn set_min_object_size(&mut self, min: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_setMinObjectSize_Size_min(self.as_raw_DetectionBasedTracker_IDetector(), min) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_setMaxObjectSize_Size_max
    fn set_max_object_size(&mut self, max: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_setMaxObjectSize_Size_max(self.as_raw_DetectionBasedTracker_IDetector(), max) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_getMinObjectSize_const
    fn get_min_object_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const
    fn get_max_object_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_getScaleFactor
    fn get_scale_factor(&mut self) -> Result<f32> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_getScaleFactor(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_setScaleFactor_float_value
    fn set_scale_factor(&mut self, value: f32) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_setScaleFactor_float_value(self.as_raw_DetectionBasedTracker_IDetector(), value) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_getMinNeighbours
    fn get_min_neighbours(&mut self) -> Result<i32> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_getMinNeighbours(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
    }
    
    // identifier: cv_DetectionBasedTracker_IDetector_setMinNeighbours_int_value
    fn set_min_neighbours(&mut self, value: i32) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_IDetector_setMinNeighbours_int_value(self.as_raw_DetectionBasedTracker_IDetector(), value) }.into_result()
    }
    
}

impl<'a> DetectionBasedTracker_IDetector + 'a {

}

// boxed class cv::DetectionBasedTracker::Parameters
#[allow(dead_code)]
pub struct DetectionBasedTracker_Parameters {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::DetectionBasedTracker_Parameters {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DetectionBasedTracker_Parameters(self.ptr) };
    }
}
impl crate::objdetect::DetectionBasedTracker_Parameters {
    #[doc(hidden)] pub fn as_raw_DetectionBasedTracker_Parameters(&self) -> *mut c_void { self.ptr }
}

impl DetectionBasedTracker_Parameters {

    // identifier: cv_DetectionBasedTracker_Parameters_Parameters
    pub fn new() -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
        unsafe { sys::cv_objdetect_cv_DetectionBasedTracker_Parameters_Parameters() }.into_result().map(|x| crate::objdetect::DetectionBasedTracker_Parameters { ptr: x })
    }
    
}

// boxed class cv::DetectionROI
#[allow(dead_code)]
pub struct DetectionROI {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::DetectionROI {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DetectionROI(self.ptr) };
    }
}
impl crate::objdetect::DetectionROI {
    #[doc(hidden)] pub fn as_raw_DetectionROI(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::HOGDescriptor
/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
/// 
/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Dalal2005) .
/// 
/// useful links:
/// 
/// https://hal.inria.fr/inria-00548512/document/
/// 
/// https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients
/// 
/// https://software.intel.com/en-us/ipp-dev-reference-histogram-of-oriented-gradients-hog-descriptor
/// 
/// http://www.learnopencv.com/histogram-of-oriented-gradients
/// 
/// http://www.learnopencv.com/handwritten-digits-classification-an-opencv-c-python-tutorial
#[allow(dead_code)]
pub struct HOGDescriptor {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::HOGDescriptor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_HOGDescriptor(self.ptr) };
    }
}
impl crate::objdetect::HOGDescriptor {
    #[doc(hidden)] pub fn as_raw_HOGDescriptor(&self) -> *mut c_void { self.ptr }
}

impl HOGDescriptor {

    // identifier: cv_HOGDescriptor_HOGDescriptor_String_filename
    /// @overload
    /// ## Parameters
    /// * filename: the file name containing  HOGDescriptor properties and coefficients of the trained classifier
    pub fn new(filename: &str) -> Result<crate::objdetect::HOGDescriptor> {
        string_arg!(filename);
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_HOGDescriptor_String_filename(filename.as_ptr()) }.into_result().map(|x| crate::objdetect::HOGDescriptor { ptr: x })
    }
    
    // identifier: cv_HOGDescriptor_HOGDescriptor_HOGDescriptor_d
    /// @overload
    /// ## Parameters
    /// * d: the HOGDescriptor which cloned to create a new one.
    pub fn copy(d: &crate::objdetect::HOGDescriptor) -> Result<crate::objdetect::HOGDescriptor> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_HOGDescriptor_HOGDescriptor_d(d.as_raw_HOGDescriptor()) }.into_result().map(|x| crate::objdetect::HOGDescriptor { ptr: x })
    }
    
    // identifier: cv_HOGDescriptor_getDescriptorSize_const
    /// Returns the number of coefficients required for the classification.
    pub fn get_descriptor_size(&self) -> Result<size_t> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_getDescriptorSize_const(self.as_raw_HOGDescriptor()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_checkDetectorSize_const
    /// Checks if detector size equal to descriptor size.
    pub fn check_detector_size(&self) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_checkDetectorSize_const(self.as_raw_HOGDescriptor()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_getWinSigma_const
    /// Returns winSigma value
    pub fn get_win_sigma(&self) -> Result<f64> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_getWinSigma_const(self.as_raw_HOGDescriptor()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_setSVMDetector_Mat__svmdetector
    /// Sets coefficients for the linear SVM classifier.
    /// ## Parameters
    /// * _svmdetector: coefficients for the linear SVM classifier.
    pub fn set_svm_detector(&mut self, _svmdetector: &core::Mat) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_setSVMDetector_Mat__svmdetector(self.as_raw_HOGDescriptor(), _svmdetector.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_load_String_filename_String_objname
    /// loads coefficients for the linear SVM classifier from a file
    /// ## Parameters
    /// * filename: Name of the file to read.
    /// * objname: The optional name of the node to read (if empty, the first top-level node will be used).
    ///
    /// ## C++ default parameters:
    /// * objname: String()
    pub fn load(&mut self, filename: &str, objname: &str) -> Result<bool> {
        string_arg!(filename);
        string_arg!(objname);
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_load_String_filename_String_objname(self.as_raw_HOGDescriptor(), filename.as_ptr(), objname.as_ptr()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_save_const_String_filename_String_objname
    /// saves coefficients for the linear SVM classifier to a file
    /// ## Parameters
    /// * filename: File name
    /// * objname: Object name
    ///
    /// ## C++ default parameters:
    /// * objname: String()
    pub fn save(&self, filename: &str, objname: &str) -> Result<()> {
        string_arg!(filename);
        string_arg!(objname);
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_save_const_String_filename_String_objname(self.as_raw_HOGDescriptor(), filename.as_ptr(), objname.as_ptr()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_copyTo_const_HOGDescriptor_c
    /// clones the HOGDescriptor
    /// ## Parameters
    /// * c: cloned HOGDescriptor
    pub fn copy_to(&self, c: &crate::objdetect::HOGDescriptor) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_copyTo_const_HOGDescriptor_c(self.as_raw_HOGDescriptor(), c.as_raw_HOGDescriptor()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_compute_const_Mat_img_VectorOffloat_descriptors_Size_winStride_Size_padding_VectorOfPoint_locations
    /// Computes HOG descriptors of given image.
    /// ## Parameters
    /// * img: Matrix of the type CV_8U containing an image where HOG features will be calculated.
    /// * descriptors: Matrix of the type CV_32F
    /// * winStride: Window stride. It must be a multiple of block stride.
    /// * padding: Padding
    /// * locations: Vector of Point
    ///
    /// ## C++ default parameters:
    /// * win_stride: Size()
    /// * padding: Size()
    /// * locations: std::vector<Point>()
    pub fn compute(&self, img: &core::Mat, descriptors: &types::VectorOffloat, win_stride: core::Size, padding: core::Size, locations: &types::VectorOfPoint) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_compute_const_Mat_img_VectorOffloat_descriptors_Size_winStride_Size_padding_VectorOfPoint_locations(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), descriptors.as_raw_VectorOffloat(), win_stride, padding, locations.as_raw_VectorOfPoint()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_detect_const_Mat_img_VectorOfPoint_foundLocations_VectorOfdouble_weights_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations
    /// Performs object detection without a multi-scale window.
    /// ## Parameters
    /// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
    /// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
    /// * weights: Vector that will contain confidence values for each detected object.
    /// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
    /// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
    /// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
    /// * winStride: Window stride. It must be a multiple of block stride.
    /// * padding: Padding
    /// * searchLocations: Vector of Point includes set of requested locations to be evaluated.
    ///
    /// ## C++ default parameters:
    /// * hit_threshold: 0
    /// * win_stride: Size()
    /// * padding: Size()
    /// * search_locations: std::vector<Point>()
    pub fn detect_weights(&self, img: &core::Mat, found_locations: &types::VectorOfPoint, weights: &types::VectorOfdouble, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &types::VectorOfPoint) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_detect_const_Mat_img_VectorOfPoint_foundLocations_VectorOfdouble_weights_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfPoint(), weights.as_raw_VectorOfdouble(), hit_threshold, win_stride, padding, search_locations.as_raw_VectorOfPoint()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_detect_const_Mat_img_VectorOfPoint_foundLocations_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations
    /// Performs object detection without a multi-scale window.
    /// ## Parameters
    /// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
    /// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
    /// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
    /// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
    /// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
    /// * winStride: Window stride. It must be a multiple of block stride.
    /// * padding: Padding
    /// * searchLocations: Vector of Point includes locations to search.
    ///
    /// ## C++ default parameters:
    /// * hit_threshold: 0
    /// * win_stride: Size()
    /// * padding: Size()
    /// * search_locations: std::vector<Point>()
    pub fn detect(&self, img: &core::Mat, found_locations: &types::VectorOfPoint, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &types::VectorOfPoint) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_detect_const_Mat_img_VectorOfPoint_foundLocations_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfPoint(), hit_threshold, win_stride, padding, search_locations.as_raw_VectorOfPoint()) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_detectMultiScale_const_Mat_img_VectorOfRect_foundLocations_VectorOfdouble_foundWeights_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping
    /// Detects objects of different sizes in the input image. The detected objects are returned as a list
    /// of rectangles.
    /// ## Parameters
    /// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
    /// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
    /// * foundWeights: Vector that will contain confidence values for each detected object.
    /// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
    /// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
    /// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
    /// * winStride: Window stride. It must be a multiple of block stride.
    /// * padding: Padding
    /// * scale: Coefficient of the detection window increase.
    /// * finalThreshold: Final threshold
    /// * useMeanshiftGrouping: indicates grouping algorithm
    ///
    /// ## C++ default parameters:
    /// * hit_threshold: 0
    /// * win_stride: Size()
    /// * padding: Size()
    /// * scale: 1.05
    /// * final_threshold: 2.0
    /// * use_meanshift_grouping: false
    pub fn detect_multi_scale(&self, img: &core::Mat, found_locations: &types::VectorOfRect, found_weights: &types::VectorOfdouble, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_detectMultiScale_const_Mat_img_VectorOfRect_foundLocations_VectorOfdouble_foundWeights_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfRect(), found_weights.as_raw_VectorOfdouble(), hit_threshold, win_stride, padding, scale, final_threshold, use_meanshift_grouping) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_detectMultiScale_const_Mat_img_VectorOfRect_foundLocations_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping
    /// Detects objects of different sizes in the input image. The detected objects are returned as a list
    /// of rectangles.
    /// ## Parameters
    /// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
    /// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
    /// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
    /// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
    /// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
    /// * winStride: Window stride. It must be a multiple of block stride.
    /// * padding: Padding
    /// * scale: Coefficient of the detection window increase.
    /// * finalThreshold: Final threshold
    /// * useMeanshiftGrouping: indicates grouping algorithm
    ///
    /// ## C++ default parameters:
    /// * hit_threshold: 0
    /// * win_stride: Size()
    /// * padding: Size()
    /// * scale: 1.05
    /// * final_threshold: 2.0
    /// * use_meanshift_grouping: false
    pub fn detect_multi_scale_weights(&self, img: &core::Mat, found_locations: &types::VectorOfRect, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_detectMultiScale_const_Mat_img_VectorOfRect_foundLocations_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfRect(), hit_threshold, win_stride, padding, scale, final_threshold, use_meanshift_grouping) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_computeGradient_const_Mat_img_Mat_grad_Mat_angleOfs_Size_paddingTL_Size_paddingBR
    /// Computes gradients and quantized gradient orientations.
    /// ## Parameters
    /// * img: Matrix contains the image to be computed
    /// * grad: Matrix of type CV_32FC2 contains computed gradients
    /// * angleOfs: Matrix of type CV_8UC2 contains quantized gradient orientations
    /// * paddingTL: Padding from top-left
    /// * paddingBR: Padding from bottom-right
    ///
    /// ## C++ default parameters:
    /// * padding_tl: Size()
    /// * padding_br: Size()
    pub fn compute_gradient(&self, img: &core::Mat, grad: &core::Mat, angle_ofs: &core::Mat, padding_tl: core::Size, padding_br: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_computeGradient_const_Mat_img_Mat_grad_Mat_angleOfs_Size_paddingTL_Size_paddingBR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), grad.as_raw_Mat(), angle_ofs.as_raw_Mat(), padding_tl, padding_br) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_getDefaultPeopleDetector
    /// Returns coefficients of the classifier trained for people detection (for 64x128 windows).
    pub fn get_default_people_detector() -> Result<types::VectorOffloat> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_getDefaultPeopleDetector() }.into_result().map(|x| types::VectorOffloat { ptr: x })
    }
    
    // identifier: cv_HOGDescriptor_getDaimlerPeopleDetector
    /// Returns coefficients of the classifier trained for people detection (for 48x96 windows).
    pub fn get_daimler_people_detector() -> Result<types::VectorOffloat> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_getDaimlerPeopleDetector() }.into_result().map(|x| types::VectorOffloat { ptr: x })
    }
    
    // identifier: cv_HOGDescriptor_detectROI_const_Mat_img_VectorOfPoint_locations_VectorOfPoint_foundLocations_VectorOfdouble_confidences_double_hitThreshold_Size_winStride_Size_padding
    /// evaluate specified ROI and return confidence value for each location
    /// ## Parameters
    /// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
    /// * locations: Vector of Point
    /// * foundLocations: Vector of Point where each Point is detected object's top-left point.
    /// * confidences: confidences
    /// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually
    /// it is 0 and should be specified in the detector coefficients (as the last free coefficient). But if
    /// the free coefficient is omitted (which is allowed), you can specify it manually here
    /// * winStride: winStride
    /// * padding: padding
    ///
    /// ## C++ default parameters:
    /// * hit_threshold: 0
    /// * win_stride: Size()
    /// * padding: Size()
    pub fn detect_roi(&self, img: &core::Mat, locations: &types::VectorOfPoint, found_locations: &types::VectorOfPoint, confidences: &types::VectorOfdouble, hit_threshold: f64, win_stride: core::Size, padding: core::Size) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_detectROI_const_Mat_img_VectorOfPoint_locations_VectorOfPoint_foundLocations_VectorOfdouble_confidences_double_hitThreshold_Size_winStride_Size_padding(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_VectorOfPoint(), confidences.as_raw_VectorOfdouble(), hit_threshold, win_stride, padding) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_detectMultiScaleROI_const_Mat_img_VectorOfRect_foundLocations_VectorOfDetectionROI_locations_double_hitThreshold_int_groupThreshold
    /// evaluate specified ROI and return confidence value for each location in multiple scales
    /// ## Parameters
    /// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
    /// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
    /// * locations: Vector of DetectionROI
    /// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually it is 0 and should be specified
    /// in the detector coefficients (as the last free coefficient). But if the free coefficient is omitted (which is allowed), you can specify it manually here.
    /// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
    ///
    /// ## C++ default parameters:
    /// * hit_threshold: 0
    /// * group_threshold: 0
    pub fn detect_multi_scale_roi(&self, img: &core::Mat, found_locations: &types::VectorOfRect, locations: &types::VectorOfDetectionROI, hit_threshold: f64, group_threshold: i32) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_detectMultiScaleROI_const_Mat_img_VectorOfRect_foundLocations_VectorOfDetectionROI_locations_double_hitThreshold_int_groupThreshold(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfRect(), locations.as_raw_VectorOfDetectionROI(), hit_threshold, group_threshold) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_readALTModel_String_modelfile
    /// read/parse Dalal's alt model file
    /// ## Parameters
    /// * modelfile: Path of Dalal's alt model file.
    pub fn read_alt_model(&mut self, modelfile: &str) -> Result<()> {
        string_arg!(mut modelfile);
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_readALTModel_String_modelfile(self.as_raw_HOGDescriptor(), modelfile.as_ptr() as _) }.into_result()
    }
    
    // identifier: cv_HOGDescriptor_groupRectangles_const_VectorOfRect_rectList_VectorOfdouble_weights_int_groupThreshold_double_eps
    /// Groups the object candidate rectangles.
    /// ## Parameters
    /// * rectList:  Input/output vector of rectangles. Output vector includes retained and grouped rectangles. (The Python list is not modified in place.)
    /// * weights: Input/output vector of weights of rectangles. Output vector includes weights of retained and grouped rectangles. (The Python list is not modified in place.)
    /// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
    /// * eps: Relative difference between sides of the rectangles to merge them into a group.
    pub fn group_rectangles(&self, rect_list: &types::VectorOfRect, weights: &types::VectorOfdouble, group_threshold: i32, eps: f64) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_HOGDescriptor_groupRectangles_const_VectorOfRect_rectList_VectorOfdouble_weights_int_groupThreshold_double_eps(self.as_raw_HOGDescriptor(), rect_list.as_raw_VectorOfRect(), weights.as_raw_VectorOfdouble(), group_threshold, eps) }.into_result()
    }
    
}

// boxed class cv::QRCodeDetector
#[allow(dead_code)]
pub struct QRCodeDetector {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::QRCodeDetector {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_QRCodeDetector(self.ptr) };
    }
}
impl crate::objdetect::QRCodeDetector {
    #[doc(hidden)] pub fn as_raw_QRCodeDetector(&self) -> *mut c_void { self.ptr }
}

impl QRCodeDetector {

    // identifier: cv_QRCodeDetector_QRCodeDetector
    pub fn new() -> Result<crate::objdetect::QRCodeDetector> {
        unsafe { sys::cv_objdetect_cv_QRCodeDetector_QRCodeDetector() }.into_result().map(|x| crate::objdetect::QRCodeDetector { ptr: x })
    }
    
    // identifier: cv_QRCodeDetector_setEpsX_double_epsX
    /// sets the epsilon used during the horizontal scan of QR code stop marker detection.
    /// ## Parameters
    /// * epsX: Epsilon neighborhood, which allows you to determine the horizontal pattern
    /// of the scheme 1:1:3:1:1 according to QR code standard.
    pub fn set_eps_x(&mut self, eps_x: f64) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_QRCodeDetector_setEpsX_double_epsX(self.as_raw_QRCodeDetector(), eps_x) }.into_result()
    }
    
    // identifier: cv_QRCodeDetector_setEpsY_double_epsY
    /// sets the epsilon used during the vertical scan of QR code stop marker detection.
    /// ## Parameters
    /// * epsY: Epsilon neighborhood, which allows you to determine the vertical pattern
    /// of the scheme 1:1:3:1:1 according to QR code standard.
    pub fn set_eps_y(&mut self, eps_y: f64) -> Result<()> {
        unsafe { sys::cv_objdetect_cv_QRCodeDetector_setEpsY_double_epsY(self.as_raw_QRCodeDetector(), eps_y) }.into_result()
    }
    
    // identifier: cv_QRCodeDetector_detect_const_Mat_img_Mat_points
    /// Detects QR code in image and returns the quadrangle containing the code.
    /// ## Parameters
    /// * img: grayscale or color (BGR) image containing (or not) QR code.
    /// * points: Output vector of vertices of the minimum-area quadrangle containing the code.
    pub fn detect(&self, img: &core::Mat, points: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_objdetect_cv_QRCodeDetector_detect_const_Mat_img_Mat_points(self.as_raw_QRCodeDetector(), img.as_raw_Mat(), points.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::SimilarRects
#[allow(dead_code)]
pub struct SimilarRects {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::objdetect::SimilarRects {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SimilarRects(self.ptr) };
    }
}
impl crate::objdetect::SimilarRects {
    #[doc(hidden)] pub fn as_raw_SimilarRects(&self) -> *mut c_void { self.ptr }
}

