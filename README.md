# CS 517 Project
_CPU Temperature Interpolation and Least-Squares Regression_

## Background

Data is polled from four (4) CPU's at an interval of 30s. Your mission, if you so choose to accept it, is to create a program that will:
1. Read temperature (Farenheit) from plain text files
2. Create functionality to interpolate the temperature arrays
3. Perform a Least Squares Regression over the time for one-core, over every core
4. Output this data into individual files, with a format as follows:

```bash
       0 <= x <=       30 ; y =      61.0000 +       0.6333 x ; interpolation
      30 <= x <=       60 ; y =      98.0000 +      -0.6000 x ; interpolation
      60 <= x <=       90 ; y =      20.0000 +       0.7000 x ; interpolation
      90 <= x <=      120 ; y =     128.0000 +      -0.5000 x ; interpolation
       0 <= x <=      120 ; y =      67.4000 +       0.0567 x ; least-squares
```

## Usage

After installing the program, go to the project root directory. To run without compiling, assuming you have `cargo`:
```bash
cargo run --bin temperature-parser -- <file from the "Data" folder>
```

It will produle four files, one per CPU core:
```
{basename}-core-0.{txt}
{basename}-core-1.{txt}
{basename}-core-2.{txt}
{basename}-core-3.{txt}
```
...where basename is the filename without the extension.

## Input data format

> sample-input.txt
> 
> ```
> 61.0 63.0 50.0 58.0
> 80.0 81.0 68.0 77.0
> 62.0 63.0 52.0 60.0
> 83.0 82.0 70.0 79.0
> 68.0 69.0 58.0 65.0
> ```

## Output data format

> sample-input-core-00.txt
> 
> ```
> 0 <=  x <=       30 ; y =      61.0000 +       0.6333 x ; interpolation
> 30 <= x <=       60 ; y =      98.0000 +      -0.6000 x ; interpolation
> 60 <= x <=       90 ; y =      20.0000 +       0.7000 x ; interpolation
> 90 <= x <=      120 ; y =     128.0000 +      -0.5000 x ; interpolation
> 0 <=  x <=      120 ; y =      67.4000 +       0.0567 x ; least-squares
> ```

## Design choices
- Rust version 1.86.0 was used for this project

#### The `TempMat<N>` Tuple Struct
The core data structure is a `TempMat`, or a Temperature Matrix. It has a 
templated constant, N, which represents the amount of CPUs that are being 
represented in the object.
So, for this project, we are using a `TempMat<4>`.
The underlying structure is an 4x1 array of `Vec<f64>`, which in practive 
amounts to a 4x<num lines in file>. 

#### Implementation of `Deref` trait for ease of vector access

To facilitate access to the vectors, the 
`Deref` trait is implemented. Therefore,

instead of 

> ```rust
> let v0 = tm.0[0];
> ```
You can ...
> ```rust
> let v0 = tm[0];
> ```

#### Creation of `TempMat`

Since the only way that a `TempMat` would be created is via the contents of a
text file, there is only one way in which a `TempMat` is created: via a long
string of text read from a file. 

#### Implementation of `FromStr`

The file contents are loaded into a `String` buffer, upon which the 
`TempMat::from_str()` is called. It creates `N` equal length vectors of 64-bit 
floating point numbers.

## Interpolation

#### The Math 

We use the two-point form:

$$
y = y_0 + \frac{y_1 - y_0}{x_1 - x_0} (x - x_0)
$$

Which we can rewrite as:

$$
y = c_0 + c_1 \cdot x
$$

Where:

$$
c_1 = \frac{y_1 - y_0}{x_1 - x_0}
$$

$$
c_0 = y_0 - c_1 \cdot x_0
$$

##### 🔢 Therefore, for Interval 0 to 30
$$
y_1 = 80, y_0 = 61.0, x_1 = 30, x_0 = 0
$$
Then,
$$
c_1 = \frac{80.0−61.0}{30−0} = 19/30=0.6333 
$$

And...
$$
c_0 = 61.0 - 0.6333 * 0 = 61.000
$$


So:
```bash
0 <= x <= 30; y = 61.000 + 0.6333 x ; interpolation
```

##### 🔢 And for Interval 30-60
$$
y_0 = 80.0, y_1 = 62.0, x_0 = 30, y_1 = 60
$$

$$
c_1 = \frac{62 - 80}{60-30} = -0.6
$$
$$
c_0 = 80 - (-0.6 * 30) = 98
$$

So:
```
30 <= x <= 60; y = 98 - 0.60 ; interpolation
```

#### In Rust...

Rust provides a [helper method over `Slice<T>`, `windows`,](https://doc.rust-lang.org/std/slice/struct.Windows.html)
which yields "an iterator over overlapping subslices of length `size`."
A `fn` function pointer is passed to, `map`, which then returns a vector of 
length, `N - 1`.

```rust
pub const INTERP: fn(f64, f64, f64) -> f64 = |n1, n2, dt| -> f64 { (n2 - n1) / dt };
```

This is called upon every vector within the `TempMat` to create another 
`TempMat` of length = length of original - 1. The code for this is found within
the implementation for `TempMat`, called, `interp`.

```rust
pub fn interp(&self, dt: f64) -> Self {
    let mut this: [Vec<f64>; N] = std::array::from_fn(|_| Vec::new());
    for i in 0..N {
        this[i] = self.0[i]
            .windows(2)
            .map(|n| INTERP(n[0], n[1], dt))
            .collect()
    }
    TempMat(this)
}
```

💭 **THOUGHTS:**
`TempMat` isn't the best name for the object, especially when performing some 
data manipulation on it (i.e. interpolation) likewise creates a `TempMat`. 
However, due to time constraints on this project, `TempMat` it is.


