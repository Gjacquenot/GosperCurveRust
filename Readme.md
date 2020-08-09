# Gosper Curve implemented in Rust

[![Build Status](https://travis-ci.org/Gjacquenot/GosperCurveRust.svg?branch=master)](https://travis-ci.org/Gjacquenot/GosperCurveRust)

This repository contains a simple pet project in Rust that generates
Gosper curves, that are recursive space filling curves.

Run `cargo run` to build and run program.

Program is run without argument and creates a list of PNG images that
illustrate the recursive creation of the fractal.

It uses [plotters](https://github.com/38/plotters) library to generate PNG images.

A Python implementation is also available
[here](https://github.com/Gjacquenot/GosperCurve).

![Alt text](./gosper_curve_5.png)