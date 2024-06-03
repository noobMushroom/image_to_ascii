# Ascii Image generator 

This program converts images to ascii characters and prints them in the terminal.

```
Usage: ascii-image --image <IMAGE> <SIZE>

Arguments:
  <SIZE>
          Size of the output image

          Possible values:
          - fill:     The height of the image will stay same, width will change according to terminal width [Defualt]
          - fit:      Both height and width will change according to the terminal image will maintain the aspect ratio
          - original: Keep the original size of the image

Options:
  -i, --image <IMAGE>
          Path of the image that you want to convert

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


