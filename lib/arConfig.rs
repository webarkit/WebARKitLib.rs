/*
 *  arConfig.rs
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

/* for arDebug */
const  AR_DEBUG_DISABLE: u8 =                     0;
const  AR_DEBUG_ENABLE: u8 =                     1;
const  AR_DEFAULT_DEBUG_MODE: u8 =               AR_DEBUG_DISABLE;

/* for arLabelingMode */
const  AR_LABELING_WHITE_REGION: u8 =            0;
const  AR_LABELING_BLACK_REGION: u8 =            1;
const  AR_DEFAULT_LABELING_MODE: u8 =            AR_LABELING_BLACK_REGION;

/* for arlabelingThresh */
const  AR_DEFAULT_LABELING_THRESH: usize =          100;

/* for arImageProcMode */
const  AR_IMAGE_PROC_FRAME_IMAGE: u8 =           0;
const  AR_IMAGE_PROC_FIELD_IMAGE: u8 =           1;
const  AR_DEFAULT_IMAGE_PROC_MODE: u8 =         AR_IMAGE_PROC_FRAME_IMAGE;

/* for arPatternDetectionMode */
const  AR_TEMPLATE_MATCHING_COLOR: u8 =              0;
const  AR_TEMPLATE_MATCHING_MONO: u8 =               1;
const  AR_MATRIX_CODE_DETECTION: u8 =                2;
const  AR_TEMPLATE_MATCHING_COLOR_AND_MATRIX: u8 =   3;
const  AR_TEMPLATE_MATCHING_MONO_AND_MATRIX: u8 =    4;
const  AR_DEFAULT_PATTERN_DETECTION_MODE: u8 =   AR_TEMPLATE_MATCHING_COLOR;

/* for arMarkerExtractionMode */
const  AR_USE_TRACKING_HISTORY: u8 =             0;
const  AR_NOUSE_TRACKING_HISTORY: u8 =           1;
const  AR_USE_TRACKING_HISTORY_V2: u8 =          2;
const  AR_DEFAULT_MARKER_EXTRACTION_MODE: u8 =   AR_USE_TRACKING_HISTORY_V2;

/* for arGetTransMat */
const  AR_MAX_LOOP_COUNT: u8 =                   5;
const  AR_LOOP_BREAK_THRESH: f32 =               0.5;

/* for arPatt**      */
if AR_ENABLE_MINIMIZE_MEMORY_FOOTPRINT {
    const   AR_PATT_NUM_MAX: usize =                   25;
} else {
    const   AR_PATT_NUM_MAX: usize =                   50; 
}

const   AR_PATT_SIZE1: usize =                     16;		// Default number of rows and columns in pattern when pattern detection mode is not AR_MATRIX_CODE_DETECTION. Must be 16 in order to be compatible with ARToolKit versions 1.0 to 5.1.6.
const   AR_PATT_SIZE1_MAX: usize =                 64;     // Maximum number of rows and columns allowed in pattern when pattern detection mode is not AR_MATRIX_CODE_DETECTION.
const   AR_PATT_SIZE2_MAX: usize =                 32;     // Maximum number of rows and columns allowed in pattern when pattern detection mode is AR_MATRIX_CODE_DETECTION.
const   AR_PATT_SAMPLE_FACTOR1: u8 =            4;     // Maximum number of samples per pattern pixel row / column when pattern detection mode is not AR_MATRIX_CODE_DETECTION.
const   AR_PATT_SAMPLE_FACTOR2: u8 =             3;     // Maximum number of samples per pattern pixel row / column when detection mode is AR_MATRIX_CODE_DETECTION.
const   AR_PATT_CONTRAST_THRESH1: f32 =         15.0;	// Required contrast over pattern space when pattern detection mode is AR_TEMPLATE_MATCHING_MONO or AR_TEMPLATE_MATCHING_COLOR.
const   AR_PATT_CONTRAST_THRESH2: f32 =          30.0;	// Required contrast between black and white barcode segments when pattern detection mode is AR_MATRIX_CODE_DETECTION.
const   AR_PATT_RATIO: f32 =                      0.5;   // Default value for percentage of marker width or height considered to be pattern space. Equal to 1.0 - 2*borderSize. Must be 0.5 in order to be compatible with ARToolKit versions 1.0 to 4.4.



const   AR_AREA_MAX: usize =                  1000000;		// Maximum area (in pixels) of connected regions considered valid candidate for marker detection.
const   AR_AREA_MIN: usize =                      70;		// Minimum area (in pixels) of connected regions considered valid candidate for marker detection.
const   AR_SQUARE_FIT_THRESH: f32 =               1.0;

const   AR_LABELING_32_BIT: u8 =                 0;     // 0 = 16 bits per label, 1 = 32 bits per label.
if AR_LABELING_32_BIT {
    const AR_LABELING_WORK_SIZE: usize =     1024*32*16;
    const AR_LABELING_LABEL_TYPE: type =       ARInt32; // this is for sure wrong but keep it for future references.
} else {
    const AR_LABELING_WORK_SIZE: usize =        1024*32;     // This number may not exceed 65535 when using 16-bits labels.
    const AR_LABELING_LABEL_TYPE: type =       ARInt16; // this is for sure wrong but keep it for future references.
}

if AR_ENABLE_MINIMIZE_MEMORY_FOOTPRINT {
    const   AR_SQUARE_MAX: usize =                     30;     // Maxiumum number of marker squares per frame.
} else {
    const   AR_SQUARE_MAX: usize =                     60;     // Maxiumum number of marker squares per frame.
}

const   AR_CHAIN_MAX: usize =                   10000;

const   AR_LABELING_THRESH_AUTO_INTERVAL_DEFAULT: u8 = 7; // Number of frames between auto-threshold calculations.
const   AR_LABELING_THRESH_MODE_DEFAULT: u8 =    AR_LABELING_THRESH_MODE_MANUAL;
const   AR_LABELING_THRESH_ADAPTIVE_KERNEL_SIZE_DEFAULT: u8 = 9;
const   AR_LABELING_THRESH_ADAPTIVE_BIAS_DEFAULT: i8 = -7;

const   AR_CONFIDENCE_CUTOFF_DEFAULT: f32 =       0.5;
const   AR_MATRIX_CODE_TYPE_DEFAULT: u8 =        AR_MATRIX_CODE_3x3;