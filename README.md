# Motivation
There exists several Azure IoT [device simulators](https://docs.microsoft.com/de-de/azure/iot-hub/quickstart-control-device?pivots=programming-language-csharp) in different languages (e.g., C#, Node.js, Python, and Java). The Azure team provide you well-balanced SDKs for all modern programming languages.

However, the IoT embedded industry needs solutions focused on performance and safety. This might be a challenge for classical OOP-based languages. Rust was created to ensure high performance similar to that offered by C and C++, but with emphasis on code safety. According to the [Stack Overflow Developer Survey 2021](https://insights.stackoverflow.com/survey/2021), Rust is the most beloved programming language.

This project provides you a sample how to connect a Rust-based simulator to the Azure IoT hub. The solutions is optimized for embedded devices (e.g., STM32, Raspberry Pi, etc.) as we will end with a low binary footprint and cross-platform support (AMD64, X84_64, etc.).  

## Prerequisites