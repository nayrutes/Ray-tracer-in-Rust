A simple CPU-based raytracer built from the ground up in Rust based on https://raytracing.github.io/books/RayTracingInOneWeekend.html.

Objects are currently limited to spheres.
Supported material properties: diffuse, reflection, refraction, emission.
Parallelization is done using Rayon to automatically split up the workload for pixels over all cores. 

![sample_1920_1080_1000_7_day](https://github.com/nayrutes/Ray-tracer-in-Rust/assets/33394281/b593e33c-f685-4145-846c-892f93f8a9bb)

![sample_1920_1080_10000_7_dark](https://github.com/nayrutes/Ray-tracer-in-Rust/assets/33394281/756b05f3-f96a-4b17-81ce-19192394632e)
