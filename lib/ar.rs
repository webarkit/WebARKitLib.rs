/*
 *  ar.rs
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

// Does we need these functions??
/*#define arMalloc(V,T,S)  \
{ if( ((V) = (T *)malloc( sizeof(T) * (S) )) == NULL ) \
{ARLOGe("Out of memory!!\n"); exit(1);} }

fn arMalloc<T>(T,size: u32) {
  data = Box<[T; size]>  
}

#define arMallocClear(V,T,S)  \
{ if( ((V) = (T *)calloc( (S), sizeof(T) )) == NULL ) \
{ARLOGe("Out of memory!!\n"); exit(1);} }*/


type ARInt8 = i8;
type ARInt16 = i16;
type ARInt32 = i32;
type ARUint8 = u8;
type ARUint16 = u16;
type ARUint32 = u32;
if ARDOUBLE_IS_FLOAT {
   type ARdouble = f32; 
} else {
   type ARdouble = f64;
}
