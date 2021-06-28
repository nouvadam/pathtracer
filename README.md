# pathtracer
Simple toy path tracer, inspired by [Ray Tracing in One Weekend](https://raytracing.github.io/) book series.

## Example images
![pegasus_4096](https://user-images.githubusercontent.com/66559370/123680877-7742a280-d849-11eb-9596-4f64ef553c04.png)
![balls](https://user-images.githubusercontent.com/66559370/104132504-f61c4f80-537d-11eb-9577-020fb06e181d.jpg)
![final_scene_2048](https://user-images.githubusercontent.com/66559370/104132491-e270e900-537d-11eb-8344-a129cb362f14.jpg)
![random_scene_2_256_median_1](https://user-images.githubusercontent.com/66559370/123523318-243ee300-d6c3-11eb-85af-a2f3e150ea0a.jpg)


## Key Features
* Basic support for .obj files
* Custom and procedural textures, like perlin noise and old-school plasma effect
* Importance sampling for faster converge
* Box blur and median filters
* Fully parallel thanks to Rayon library

## Usage: 
If you have [Rust](https://www.rust-lang.org/learn/get-started) installed, download repository, then type in console from top level folder of the project:  
`cargo run --release --example <file from "examples" folder>`  
example:  
`cargo run --release --example pegasus`

## Usage as a library:
To use this path tracer in your project, paste this to `Cargo.toml`:
```toml
[dependencies]
pathtracer = { git = "https://github.com/nouvadam/pathtracer/", branch = "main"}
```
To check out the documentation, generate and open one by:
`cargo doc --open`
