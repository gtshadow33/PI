# Monte Carlo Pi Estimator in Rust

A simple Rust program that estimates the value of Pi using the Monte Carlo method with multithreading.

## Description

This project demonstrates how to use **threads** and **channels** in Rust to simulate random points inside a square and count how many fall inside a unit circle. It then calculates an approximation of Pi based on the ratio of points inside the circle to the total points generated.

- Uses multiple threads to generate random points in parallel.
- Communicates batches of points from worker threads to the main thread using channels (`mpsc`).
- Illustrates safe and efficient concurrent programming in Rust.

---

## Features

- Multithreaded computation
- Random point generation
- Batch sending via channels
- Monte Carlo Pi estimation
- Adjustable number of threads and batch sizes


![screen03](https://github.com/user-attachments/assets/3b4ade4c-2021-433a-9133-0769f6762af9)

  
