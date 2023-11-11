# Rust Face Detection

## Overview
This project demonstrates face detection using `Rust` and `OpenCV`. It includes a comparison between the original video and the video with face detection applied.

## Demo
<center>

|Origin|Detection|
|:-:|:-:|
|![demo origin](./resources/demo-origin.gif)|![demo detection](./resources/demo-detection.gif)|

</center>

## Environment

- **Operating System:** MacOS
    - **Dependencies:**
        - OpenCV: ^4.0
- **Programming Language:** Rust
    - **Dependencies:**
        - OpenCV: ^0.85.0

## Execution

### Local

  ```shell
  $ brew install opencv
  $ cargo run
  ```

### Docker

  ```shell
  $ docker build -t rust-face-detection .
  # hint: you need to bind the devices into your own docker container
  $ docker run -idtv $PWD:/app --device="{replace to the path of your device}:/dev/video0" rust-face-detection
  ```

## References

- [YouTube Tutorial](https://www.youtube.com/watch?v=iWficV_pmxY)
- [--device="/dev/video0:/dev/video0"](https://stackoverflow.com/questions/62929645/unable-to-open-camera-using-cv2-videocapture0-in-docker-ubuntu-host)
- [VideoCapture Save as Video](https://stackoverflow.com/questions/30509573/writing-an-mp4-video-using-python-opencv)
- [Haar Cascades](https://github.com/opencv/opencv/tree/master/data/haarcascades)
- [OpenCV Documentation](https://docs.opencv.org/3.4/d4/d15/group__videoio__flags__base.html#gga023786be1ee68a9105bf2e48c700294dacf10e9692c4166f74de62b7d00c377d0)
- [OpenCV-Rust Documentation](https://github.com/twistedfall/opencv-rust#getting-opencv)
- [OpenCV-Rust Documentation](https://docs.rs/opencv/latest/opencv/index.html)
- [Demo Video by Mikhail Nilov from Pexels](https://www.pexels.com/video/a-woman-smelling-the-fragrance-of-an-essential-oil-6707352)
