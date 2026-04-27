## 🦀 Ray-Tracer-Implementation

This project is based on the "Ray Tracing" series by Peter Shirley. The original implementation is in C++, but this is my attempt at
creating the project using Rust.

### ❓Information about the project:

This is technically a path finder. Essentially, the program shoots a "ray" of light staight into a pixel on our screen and into the '3D scene'. The ray will either hit an object or not hit anything. When it hits an object, the program calculates how light should bounce off, scatter, or reflect based on the material and object. It will change the color of the pixel depending on whether it hit an object (draw the object) or not (draw the background). This allows us to create some pretty realistic images. (Examples of images in Demo)

### 📽️ Demo

[Link to Demo](https://youtu.be/5jWeXGdOKQ4)

### 🚀 Motivation

The reason that I used Rust rather than the langugage that the project originally used was to strengthen my knowledge with Rust
and understand the quirks that come with it (borrow-checker, ownership, etc.)

## 🏁 Getting Started

### Prerequisites
Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation & Usage
1. **Clone the repository:**
   ```bash
   git clone https://github.com/Saul-AO/Ray-Tracer-Implementation.git
   cd Ray-Tracer-Implementation
2. **Run the program**
    ```
    cargo run --release > output.ppm
- You can use an image viewer to actaully see the result!
