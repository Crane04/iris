# Iris — Biometric Infrastructure

**Iris** is a stateless, high-performance face recognition infrastructure built in Rust. It provides a REST API designed for hospital IT systems to identify unresponsive patients in real-time by comparing emergency captures against secure patient databases.

---

## Core Philosophy

-   **Stateless by Design:** Images are processed in RAM and destroyed immediately after feature extraction. No biometric data ever touches the disk.
-   **Infrastructure, Not Storage:** Iris does not store medical records. It returns a mathematical similarity score between two images, allowing hospitals to link to their own secure EMR systems.
-   **High Performance:** Powered by Rust and ONNX-accelerated models (YuNet and SFace) for sub-100ms inference.

---

## Prerequisites

Before running Iris, ensure you have the following installed:

-   **Rust:** [Install via rustup](https://rustup.rs/)
-   **OpenCV 4.x:** \* _macOS:_ `brew install opencv`
    -   _Linux:_ `sudo apt install libopencv-dev`
    -   _Windows:_ Follow [OpenCV-Rust installation guide](https://github.com/twistedfall/opencv-rust#windows)

---

## Installation & Setup

### Clone the Repository

```bash
git clone [https://github.com/your-username/iris.git](https://github.com/your-username/iris.git)
cd iris
```

# Download AI Models

Iris requires pre-trained ONNX models for detection and recognition. These files are excluded from Git due to size. Run these commands in the project root:

```Bash
chmod +x setup.sh
./setup.sh
```

Or try to do it manually by running the following commands.

```Bash

# Face Detection (YuNet)
curl -L [https://github.com/opencv/opencv_zoo/raw/main/models/face_detection_yunet/face_detection_yunet_2023mar.onnx](https://github.com/opencv/opencv_zoo/raw/main/models/face_detection_yunet/face_detection_yunet_2023mar.onnx) -o face_detection_yunet_2023mar.onnx

# Face Recognition (SFace)
curl -L [https://github.com/opencv/opencv_zoo/raw/main/models/face_recognition_sface/face_recognition_sface_2021dec.onnx](https://github.com/opencv/opencv_zoo/raw/main/models/face_recognition_sface/face_recognition_sface_2021dec.onnx) -o face_recognition_sface_2021dec.onnx
```

# Running the API

Start the Server

```Bash

cargo run --release
```

The API will be available at http://localhost:3002.

# Test the Endpoint

Use curl to verify the comparison engine:

```Bash

curl -X POST http://localhost:3002/compare \
  -H "Content-Type: application/json" \
  -d '{
    "target_url": "[https://hosp.io/er/capture_01.jpg](https://hosp.io/er/capture_01.jpg)",
    "people": [
      {"name": "Patient_A", "image_url": "[https://hosp.io/db/p_001.jpg](https://hosp.io/db/p_001.jpg)"},
      {"name": "Patient_B", "image_url": "[https://hosp.io/db/p_002.jpg](https://hosp.io/db/p_002.jpg)"}
    ]
  }'
```

# Project Structure

```Plaintext

iris/
├── face_detection_yunet_2023mar.onnx   # Detection model
├── face_recognition_sface_2021dec.onnx  # Recognition model
├── Cargo.toml
└── src/
    ├── main.rs    # Server entry & CORS configuration
    ├── models.rs  # Request/Response data structures
    └── face.rs    # OpenCV FaceEngine implementation
```

# Security & Privacy

Zero Persistence: Images are downloaded into volatile memory (RAM), decoded, converted to a 128-dimensional vector, and then purged.

CORS Enabled: The API is pre-configured to allow secure communication with authorized frontends.

Third-Party Isolation: Iris is designed as a biometric engine only. It never stores PII (Personally Identifiable Information) or sensitive medical history.

# License

Distributed under the MIT License.
