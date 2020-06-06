# strange_painter
    Strange painter 
    
    USAGE:
        strange_painter [OPTIONS] --input <INPUT FILE NAME> --output <OUTPUT FILE NAME> --size <WIDTH:HEIGHT>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
        -d, --depth <NUMBER OF ITERATIONS>    Sets number of iterations in painting [default: 50]
        -i, --input <INPUT FILE NAME>         Sets input file name
        -o, --output <OUTPUT FILE NAME>       Sets output file name
        -p, --palette <PALETTE FILE NAME>     Sets image file name, where first scan line will be used as palette
        -s, --size <WIDTH:HEIGHT>             Sets size of picture in pixels


Input file has a yaml format. Where "left", "top", "right" and "bottom" params (see example.yaml) set borders of picture on complex plane. "transforms" param sets a sequence of transforms where "cut" is boolean, "numerator_coefs" are coefs of numerator of rational transform and "denominator_coefs" are coefs of denominator of rational transform.

![picture](https://github.com/AlEvgeniy/strange_painter/blob/master/grayscale_example.png)
![picture](https://github.com/AlEvgeniy/strange_painter/blob/master/with_palette.png)
