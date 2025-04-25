Dino Image Extension Changer (Dino IEC)

Dino Image Extension Changer (Dino IEC) is a user-friendly, cross-platform desktop application built in Rust that allows you to effortlessly convert image files between various formats. With a sleek GUI powered by eframe and egui, Dino IEC makes image format conversion intuitive and efficient. Whether you're a developer, designer, or casual user, Dino IEC is your go-to tool for quick and reliable image extension changes.
Features

Cross-Platform: Runs seamlessly on Linux, Windows, and macOS.
Intuitive GUI: A modern, easy-to-use interface with file picker and format dropdown.
Wide Format Support: Convert images to JPEG, PNG, BMP, GIF, TIFF, WebP, ICO, PBM, PGM, PPM, TGA, DDS, and HDR.
File Picker: Select images directly from your file system using a native file dialog.
Error Handling: Clear feedback for invalid inputs or unsupported formats.
Lightweight & Fast: Built with Rust for performance and reliability.

Screenshots
Main interface showing file selection, name input, and format dropdown.
Installation
Prerequisites

Rust (version 1.80 or higher)
A C compiler (e.g., gcc or clang for Linux/macOS, or MSVC for Windows)
For Linux:sudo pacman -S libx11 libgl mesa xorg-xwayland  # Arch Linux
sudo apt-get install libx11-dev libgl1-mesa-dev  # Debian/Ubuntu



Steps

Clone the Repository:
git clone https://github.com/natiqmammad/Dino Image Extension Changer.git
cd Dino Image Extension Changer


Build and Run:
cargo run


(Optional) For Linux with Wayland:If you encounter Wayland-related issues, force X11 backend:
WINIT_UNIX_BACKEND=x11 cargo run



Usage

Launch the application using cargo run.
Use the Browse button to select an image file or enter the path manually.
Enter a new name for the output file (without extension).
Choose the desired output format from the dropdown menu.
Click Convert Image to save the image in the new format.
Check the status message for success or error details.

Supported Formats



Format
Extension



JPEG
.jpeg, .jpg


PNG
.png


BMP
.bmp


GIF
.gif


TIFF
.tiff


WebP
.webp


ICO
.ico


PBM
.pbm


PGM
.pgm


PPM
.ppm


TGA
.tga


DDS
.dds


HDR
.hdr


Troubleshooting

Wayland Errors on Linux:

Ensure a Wayland compositor (e.g., sway, weston) is running, or use X11:WINIT_UNIX_BACKEND=x11 cargo run


Install required libraries:sudo pacman -S libx11 libgl mesa xorg-xwayland




Build Errors:

Update dependencies:cargo update


Clean the build cache:cargo clean
cargo run





Contributing
Contributions are welcome! To contribute:

Fork the repository.
Create a new branch (git checkout -b feature/your-feature).
Commit your changes (git commit -m "Add your feature").
Push to the branch (git push origin feature/your-feature).
Open a Pull Request.

Please ensure your code follows Rust's coding standards and includes appropriate tests.
License
This project is licensed under the MIT License. See the LICENSE file for details.
Acknowledgments

Built with Rust, eframe, and image.
File dialogs powered by rfd.
Inspired by the need for a simple, cross-platform image format converter.

Contact
Have questions or suggestions? Open an issue or reach out via GitHub.

⭐ Star this repo if you find Dino IEC useful! ⭐
