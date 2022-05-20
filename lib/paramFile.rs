/*
 *  paramFile.rs
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

mod param;
use std::fs;

struct ARParamd {
    let xsize: i32;
    let ysize: i32;
    let mat[3][4]: f64:
    let dist_factor[param::AR_DIST_FACTOR_NUM_MAX]: f64;
    let dist_function_version: i32;
}

//fn arParamLoad(filename: String, num: i32, param: ARParam)
fn arParamLoad(filename: String)
{
    let mut ret: i8 = 0;
    let dist_function_version: i32;
    let flen: i32;
    //if (num < 1 || !filename || !param) {
    if num < 1 {
        ret =-1;
    }
    let info = fs::read(filename).expect("The file could not be read");
    flen = info.len();
    println!("{}", flen);
    println!("{}", info);
}