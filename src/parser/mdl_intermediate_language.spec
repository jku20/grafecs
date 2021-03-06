This is an intermediate language to be parsed by my graphics engine.
It should be simple.

Each command is composed of an op code followed by a constant amount of arguments.
Each op code is one byte and depending on that op code there will be some more data. The size of the rest of the data will be specified in this spec. The following are the op codes which NEED to be handeled

--------------------------------------------------------
0x01
the "push" command

0x02
the "pop" command

0x03
the "move" command
following this code are three f64 floating point values
these represent the x, y, and z coordinates of the move
following these is one byte, 0x01 if there is a knob followed
the by a null terminated string, or 0x00 if there is none

0x04
the "rotate" command
following it will be one f64 with the value 0 for 'x', 1 for 'y', or 2 for 'z'
representing the axis to rotate around
following that will be a f64 representing the degrees to rotate
following these is one byte, 0x01 if there is a knob followed
the by a null terminated string, or 0x00 if there is none

0x05
the "scale" command
following it will be three f64 floating point values
these represent how much to scale in the x, y, and z directions
following these is one byte, 0x01 if there is a knob followed
the by a null terminated string, or 0x00 if there is none

0x06
the simplist version of box
following the op code are fifteen f64 values
the first six values represent the corner of the box and the dimensions:
'x0', 'y0', 'z0', 'h', 'w', 'd'
The next nine reprsent lighting constants
these are 'Ka_r', 'Ka_g', 'Ka_b', 'Kd_r', 'Kd_g', 'Kd_b', 'Ks_r', 'Ks_g', 'Ks_b'

0x07
the simplist version of the sphere command
following the op code are thirteen f64 values
the first three represent 'x', 'y', 'z', and 'r'
the next nine are the lighting constants
'Ka_r', 'Ka_g', 'Ka_b', 'Kd_r', 'Kd_g', 'Kd_b', 'Ks_r', 'Ks_g', 'Ks_b'

0x08
the simplist version of the torus command following the op code are fourteen f64 values these are 'x', 'y', 'z', 'r0', 'r1'
the next nine are the lighting constants
'Ka_r', 'Ka_g', 'Ka_b', 'Kd_r', 'Kd_g', 'Kd_b', 'Ks_r', 'Ks_g', 'Ks_b'

0x09
the simplist version of the line command
following the op code are size f64 values representing
the start and end points of the line: 'x0', 'y0', 'z0', 'x1', 'y1', 'z1'

0x0A
this is the save command and should save an image as a png
following the png is a null terminated ascii string representing the file name

0x0B
the display command, it displays the image

0x0C
the basename command, following it is a null terminated ascii string representing the base file name

0x0D
if the script represents and animation, this is the number of frames in that animation
that number of frames is represented as a 32bit unsigned integer following the command

0x0E
the vary command
following is a null string representing the name a knob
these ids should be unique to every differnt knob
following that are the frame numbers the animation will go over, both 32bit unsigned integers
these are inclusive on both ends, first the start frame and the nthe end frame
the next two numbers are two 64bit floats between 0 and 1 representing
how far along the animation starts into the transformation and at what point it ends
0 means no transformation has been done, 1 means the entire transformation is completed
first the start value then the end value
