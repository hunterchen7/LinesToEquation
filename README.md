Converts lines from an image into parametric equations using Fourier transforms, lines are generated by traversing image with DFS

Currently only supports black line(s).

Usage: pretty rudimentary at the moment, just change "img" input in main.rs to desired image and run it like a normal rust program. generated equations will appear in images/equations.txt. see generated islands in images/lines.png.

**Screenshot:**
- left: generated graph from equations on Desmos
- middle: input image
- right: "islands" generated from DFS
![image](https://github.com/hunterchen7/LinesToEquation/assets/34012681/3a38b715-a0f3-4f57-a484-491794d58a04)

TODO:
- better DFS so more islands become contiguous, there are some issues with pathing so I can't just DFS omnidirectionally towards unexplored nodes from the beginning
- support actual images by converting to lines based on contrast