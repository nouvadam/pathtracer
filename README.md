# pathtracer
Simple toy path tracer, inspired by [Ray Tracing in One Weekend](https://raytracing.github.io/) book series.

## Usage: 
If you have [Rust](https://www.rust-lang.org/learn/get-started) installed, download repository, then type in console from top level folder of the project:  
`cargo run --release --example <file from "examples" folder>`  
example:  
`cargo run --release --example pegasus`

## Usage as a library:
To use this path tracer in your project, paste this to `Cargo.toml`:
```toml
[dependencies]
pathtracer = { git = "https://github.com/nouvadam/pathtracer/" }
```
To check out the documentation, first generate one by:
`cargo doc`
then open `target/doc/pathtracer/index.html` file.

## Example images
![mesh_test_2048](https://user-images.githubusercontent.com/66559370/104132481-d38a3680-537d-11eb-8baa-6e11b2caa444.jpg)
![balls](https://user-images.githubusercontent.com/66559370/104132504-f61c4f80-537d-11eb-9577-020fb06e181d.jpg)
![final_scene_2048](https://user-images.githubusercontent.com/66559370/104132491-e270e900-537d-11eb-8344-a129cb362f14.jpg)
