# tree-hugger

A rust binary for computing M edge graphs which obtain the largest number of spanning trees possible for an M edge graph.

Currently Implemented
- Command line arguments
- Kirchhoff's Theorem
- Brute force with a very nice heuristic, reducing the computational cost significantly (in fact, it can hardly be called brute force anymore, having much better time complexity 😄)

Possible Future Work
- Tricks using eigenvalue stability
- Possible reduction via edge contraction
