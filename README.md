# Ray Tracer

A simple ray tracer (yet to be) implemented in rust. 

Once the project is done, it should be able to read in a yml or toml file. This file will contain the coordinates of 
different objects, as well as their geometry, refractive index, texture, etc...

The colors at different pixels should be concurrently calculated.  

# Image 

The image generated is [ppm](https://netpbm.sourceforge.net/doc/ppm.html) since it is so easy to generate. Later this could be improved. 

## Later Plans

Cuda could be used to make it faster. 

Use a better image extension. 

# Todo's

Better testing -- Proptest should really be added to improve and simplify testing. 
