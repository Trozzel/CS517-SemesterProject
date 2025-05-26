# CS 517 Project - Doing Stuff with Data

## Background

Data is polled from four (4) CPU's at an interval of 30s. 

## Sample data

> sample-input.txt
> 
> ```
> 61.0 63.0 50.0 58.0
> 80.0 81.0 68.0 77.0
> 62.0 63.0 52.0 60.0
> 83.0 82.0 70.0 79.0
> 68.0 69.0 58.0 65.0
> ```

> sample-input-core-00.txt
> 
> ```
> 0 <=  x <=       30 ; y =      61.0000 +       0.6333 x ; interpolation
> 30 <= x <=       60 ; y =      98.0000 +      -0.6000 x ; interpolation
> 60 <= x <=       90 ; y =      20.0000 +       0.7000 x ; interpolation
> 90 <= x <=      120 ; y =     128.0000 +      -0.5000 x ; interpolation
> 0 <=  x <=      120 ; y =      67.4000 +       0.0567 x ; least-squares
> ```

## To obtain interpolation

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
y_1 = 80, y_0 = 61.0, x_1 = 30  x_0 = 0
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

