---
title: ・Minecraft image generator (2024)
date created: 2024-08-13
tags:
  - python
  - pillow
---

### Introduction & Features
This is a Minecraft image generator that I made during summer 2024! It utilizes Python Imaging Library (PIL/Pillow) to turn any inputted image into Minecraft blocks!  For example:  

![[firefly.png]] ![[fireflymc.png]]  

This will also return the amount of all the blocks required to recreate the image (below are the block required to make the image above)...

![[mcblocks.png]]

...as well as customize the detail of the generated image.

![[flower1.png]] ![[flowermchigh1.png]] ![[flowermclow1.png]]  

### Process
After inputting an image and selecting the desired quality, the original image is cut up into little snippets (based on desired quality), and it's RGB value is compared with similarly-colored Minecraft blocks through finding each's euclidean distance on the 3 dimensional RGB Color space. The returned Minecraft block is then placed on it's spot in the original spot (similarly to how it would work in the Minecraft Game).  

### Runtime
It's pretty slow (O(n^2) runtime). Every snippet of pixels is accessed and compared to every single minecraft block. Pictures that are 500x500 px could take up to 5-7 minutes to fully process. My current solutions are to either implement a Median Cut Algorithm and cluster nearby similar colors together for faster runtime.  

You can view this project [here](https://github.com/3nya/mc).
