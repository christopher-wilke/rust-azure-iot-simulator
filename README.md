# Motivation
There exists several Azure IoT [device simulators](https://docs.microsoft.com/de-de/azure/iot-hub/quickstart-control-device?pivots=programming-language-csharp) in different languages (e.g., C#, Node.js, Python, and Java). The Azure team provide you well-balanced SDKs for all modern programming languages.

However, the IoT embedded industry needs solutions focused on performance and safety. This might be a challenge for classical OOP-based languages. Rust was created to ensure high performance similar to that offered by C and C++, but with emphasis on code safety. According to the [Stack Overflow Developer Survey 2021](https://insights.stackoverflow.com/survey/2021), Rust is the most beloved programming language.

This project provides you a sample how to connect a Rust-based simulator to the Azure IoT hub. The solutions is optimized for embedded devices (e.g., STM32, Raspberry Pi, etc.) as we will end with a low binary footprint and cross-platform support (AMD64, X84_64, etc.).  

## Prerequisites
If you want to run the sample, you need an active [Azure Account](https://azure.microsoft.com/en-us/free/). Please also make sure to install the latest [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli).

### Provision an IoT Hub
1. In the first CLI session, run the az extension add command. The command adds the Microsoft Azure IoT Extension for Azure CLI to your CLI shell. The IOT Extension adds IoT Hub, IoT Edge, and IoT Device Provisioning Service (DPS) specific commands to Azure CLI.

```sh
az extension add --name azure-iot
```

2. In the first CLI session, run the `az group create` command to create a resource group. The following command creates a resource group named MyResourceGroup in the eastus location.

```sh
az group create --name MyResourceGroup --location westeurope
```

3. You can then create the IoT hub, this process may take a few minutes to complete.

```sh
az iot hub create --resource-group MyResourceGroup --name {YourIoTHubName}
```

### Create a device
1. In the CLI session, run the `az iot hub device-identiy create` command which creates a device identitiy.

```sh
az iot hub device-identity create -d simDevice -n {YourIoTHubName}
```