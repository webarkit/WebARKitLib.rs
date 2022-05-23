/*
 *  param.rs
 *  WebARKitLib.rs
 *
 *  This file is part of WebARKit.
 *
 *  WebARKit is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  WebARKit is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with WebARKit.  If not, see <http://www.gnu.org/licenses/>.
 *
 *  As a special exception, the copyright holders of this library give you
 *  permission to link this library with independent modules to produce an
 *  executable, regardless of the license terms of these independent modules, and to
 *  copy and distribute the resulting executable under terms of your choice,
 *  provided that you also meet, for each linked independent module, the terms and
 *  conditions of the license of that module. An independent module is a module
 *  which is neither derived from nor based on this library. If you modify this
 *  library, you may extend this exception to your version of the library, but you
 *  are not obligated to do so. If you do not wish to do so, delete this exception
 *  statement from your version.
 *
 *  Copyright 2022 WebARKit.org.
 *
 *  Author(s): Walter Perdan @kalwalt
 * 
 *  Derived from ARToolKit5 and ARToolKitX library.
 */

mod ar;

/*!
    @file param.rs
    @brief   WebARKit functions for handling calibrated camera parameters.
    @details
*/
/*!
    @brief   Default version for functions accepting a "distortion function version" parameter.
    @details See function arParamObserv2Ideal() for discussion.
*/
const AR_DIST_FUNCTION_VERSION_DEFAULT: u8 = 4;
/*!
    @brief   Maximum version allowable for functions accepting a "distortion function version" parameter.
    @details See function arParamObserv2Ideal() for discussion.
*/
const AR_DIST_FUNCTION_VERSION_MAX: u8 = 4;
/*!
    @brief   Maximum number of values in a distortion factor array.
    @details See function arParamObserv2Ideal() for discussion.
*/
const AR_DIST_FACTOR_NUM_MAX: u8 = 9;
/*!
    @brief   Default padding added around a lookup-table based camera parameter.
    @details See function arParamLTCreate() for discussion.
*/
const   AR_PARAM_LT_DEFAULT_OFFSET: u8  = 15;
/*!
    @brief   Structure holding camera parameters, including image size, projection matrix and lens distortion parameters.
    @details
        WebARKit's tracking depends on accurate knowledge of the properties of the
        imaging system used to acquire input images. This structure holds the properties
        of an imaging system for internal use in WebARKit. This information is used
        throughout the entire WebARKit pipeline, including knowing the size of images
        being returned by the video library, marker detection and pose estimation,
        and warping of camera images for video-see-through registration.
    @see    arParamClear
    @see    arParamLoad
    @see    arParamSave
 */

 struct ARParam {
    xsize: i32;                 ///< The width in pixels of images returned by arVideoGetImage() for the camera.
    ysize: i32;                 ///< The height in pixels of images returned by arVideoGetImage() for the camera.
    mat[3][4]: ARdouble;             ///< The projection matrix for the calibrated camera parameters. This can be converted to an OpenGL projection matrix by the function arglCameraFrustumRHf().
    dist_factor[AR_DIST_FACTOR_NUM_MAX]: ARdouble; ///< See function arParamObserv2Ideal() for discussion.
    dist_function_version: i32; ///< See function arParamObserv2Ideal() for discussion. Must be last field in structure (as will not be written to disk).
};

// original code from ARToolKit5 AR/param.h 
/*typedef struct {
	int dist_factor_num;
	int ARParam_size;
} arParamVersionInfo_t;*/

struct arParamVersionInfo_t {
    dist_factor_num: i8;
    ARParam_size: i8;
}

// original code from ARToolKit5 AR/param.h
// How we can convert in Rust?
/*!
    @brief   Constant array with parameters applicable to each version of the camera parameter distortion function.
 */
//extern const arParamVersionInfo_t arParamVersionInfo[AR_DIST_FUNCTION_VERSION_MAX];