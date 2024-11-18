# Elliptic Curve Point Generation, Addition, and Generator Detection over Finite Fields

This Rust program generates points on an elliptic curve defined over a finite field, performs point addition, calculates scalar multiples, and finds generator points on the curve. It showcases fundamental operations on elliptic curves, which are crucial for elliptic curve cryptography (ECC).

## Table of Contents
- [Description](#description)
- [Elliptic Curve Definition](#elliptic-curve-definition)
- [Features](#features)
- [Usage](#usage)
- [Example Output](#example-output)

## Description
This program implements elliptic curve operations over a finite field $\mathbb{Z}_m$. It includes:
- Generating all points on an elliptic curve defined by $y^2 = x^3 + ax + b \mod m$.
- Adding two points on the curve.
- Calculating the scalar multiple of a point.
- Determining the order of points and identifying generators.

### Elliptic Curve Definition
The elliptic curve is defined by:

$y^2 \equiv x^3 + ax + b \pmod{m}$

where:
- `a` and `b` are curve coefficients.
- `m` is a prime number representing the finite field $\mathbb{Z}_m$.

## Features
- **Point Generation**: Generates all valid points on the elliptic curve.
- **Point Addition**: Implements elliptic curve point addition and doubling.
- **Scalar Multiplication**: Multiplies a point by an integer scalar.
- **Order Calculation**: Determines the order of a point.
- **Generator Detection**: Finds generator points that can generate the entire group.

## Usage
### Prerequisites
- Install [Rust](https://www.rust-lang.org/).

### Running the Code
1. Clone the repository:
   ```bash
   git clone https://github.com/cypriansakwa/Elliptic_Curve_Point_Generation_Addition_and_Generator_Detection_over_Finite_Fields.git
   cd Elliptic_Curve_Point_Generation_Addition_and_Generator_Detection_over_Finite_Fields
   ```
2. Compile and run:
```
cargo run
```
## Code Explanation
- **Point Struct**: Represents a point on the curve, with an infinity flag for the point at `infinity`.
- **mod_inv Function**: Computes the modular inverse using the extended Euclidean algorithm.
- **elliptic_add Function**: Handles addition of two points, including point doubling.
- **scalar_mult Function**: Computes scalar multiplication using the double-and-add algorithm.
- **find_all_points Function**: Generates all points on the curve.
- **find_generators Function**: Identifies generator points based on their order.
## Parameters in `main`()
You can modify the curve parameters in the `main`() function:
```
let a = 4;
let b = 4;
let m = 7;
```
## The following is an example of the output for the curve $y^2=x^3+4x+4 \pmod 7$
```
All points on the curve:
(0, 2)
(0, 5)
(1, 3)
(1, 4)
(3, 1)
(3, 6)
(4, 0)
(5, 3)
(5, 4)
Point at infinity
Point (0, 2): Order = 10
Point (0, 5): Order = 10
Point (1, 3): Order = 5
Point (1, 4): Order = 5
Point (3, 1): Order = 10
Point (3, 6): Order = 10
Point (4, 0): Order = 2
Point (5, 3): Order = 5
Point (5, 4): Order = 5
Point (0, 0): Order = 0

Generators:
(0, 2)
(0, 5)
(3, 1)
(3, 6)
