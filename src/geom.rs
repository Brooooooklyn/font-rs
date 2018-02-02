// Copyright 2015 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Geometry primitive data structures and manipulations

use lyon::math::{Point, point};

#[derive(Debug)]
pub struct Affine {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl Affine {
    /// Concatenate two affine transforms.
    pub fn concat(t1: &Affine, t2: &Affine) -> Affine {
        Affine {
            a: t1.a * t2.a + t1.c * t2.b,
            b: t1.b * t2.a + t1.d * t2.b,
            c: t1.a * t2.c + t1.c * t2.d,
            d: t1.b * t2.c + t1.d * t2.d,
            e: t1.a * t2.e + t1.c * t2.f + t1.e,
            f: t1.b * t2.e + t1.d * t2.f + t1.f,
        }
    }
}

pub fn affine_pt(z: &Affine, p: &Point) -> Point {
    point(z.a * p.x + z.c * p.y + z.e, z.b * p.x + z.d * p.y + z.f)
}

gen_new!(Affine, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32);
