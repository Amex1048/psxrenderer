# PSX styled renderer
Small university project for computer graphics course that was later tinkered for PSX styled graphics rendering.
### Features
- [x] Basic GLTF Scene loading
- [x] Mesh rendering
- [x] Texturing
- [x] Rendering into framebuffer
- [ ] Transparency
- [ ] Gouraud shading

### Dependencies
- rust
- sdl
- opengl 3.3

### Build
`cargo run` for debug build and `cargo run --release` for release build

### Affine texturing
PSX hardware supported only affine texturing. This can be reproduced (uncomment `noperspective` attribute in `shaders/texture/frag.glsl`), but assets are not ready to be rendered in these conditions, so major artifacts will uppear on large surfaces with small amount of triangles. This can be solved in different ways (the easiest one is to prepare the assets by manually dividing affected surfaces in more triangles), but I didn't work on it.
