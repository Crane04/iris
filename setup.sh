#!/bin/bash

echo "Initializing Iris Environment..."

# Download YuNet
if [ ! -f "face_detection_yunet_2023mar.onnx" ]; then
    echo "Downloading YuNet Detection Model..."
    curl -L https://github.com/opencv/opencv_zoo/raw/main/models/face_detection_yunet/face_detection_yunet_2023mar.onnx -o face_detection_yunet_2023mar.onnx
else
    echo "YuNet already exists."
fi

# Download SFace
if [ ! -f "face_recognition_sface_2021dec.onnx" ]; then
    echo "Downloading SFace Recognition Model..."
    curl -L https://github.com/opencv/opencv_zoo/raw/main/models/face_recognition_sface/face_recognition_sface_2021dec.onnx -o face_recognition_sface_2021dec.onnx
else
    echo "SFace already exists."
fi

echo "Setup complete. Run 'cargo run --release' to start Iris."