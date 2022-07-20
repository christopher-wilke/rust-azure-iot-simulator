# Motivation
There are several [Azure IoT device simulators](https://docs.microsoft.com/de-de/azure/iot-hub/quickstart-control-device?pivots=programming-language-csharp) written in different languages (e.g., C#, Node.js, Python, etc.) available. However, the embedded industry focuses on performance, binary size, and safety. This might be an issue for classical OOP-oriented languages. 

## Why Rust?
Rust is a low-level statically-typed multi-paradigm programming language that's focused on safety and performance. It solves problems that C/C++ has been struggling with for a long time, such as memory errors and building concurrent programs. According to the [Stack Overflow Developer Survey 2021](https://insights.stackoverflow.com/survey/2021), Rust is the most beloved language.

## Goal of the Project
to do: place a picture here

We will run a temperature simulator and send data to an [gRPC](https://grpc.io/) endpoint via the [OpenTelemetry Protocol](https://opentelemetry.io/docs/reference/specification/protocol/). The receiver converts incoming data and send it via [MQTT](https://mqtt.org/) to an [Azure IoT Hub](https://azure.microsoft.com/en-us/services/iot-hub/) instance. You can  visualize incoming data by running a [web app](https://github.com/christopher-wilke/rust-azure-iot-simulator/tree/main/c2d-visualizer) on your local machine.

The solution is optimized for embedded devices (e.g., `STM32`, `Raspberry Pi`, etc.) as we have an device app with a low footprint (less than 4MB). Rust also supports multiple hardware architectures, you can find `armv7` and `x86_64` binaries in the [release page](https://github.com/christopher-wilke/rust-azure-iot-simulator/releases).

# Run the Device App

## Prerequisites
If you want to run the sample, you need an active [Azure Account](https://azure.microsoft.com/en-us/free/). Please also make sure to install the latest [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli).

### Create an IoT Hub
1. In the first CLI session, run the az extension add command. The command adds the Microsoft Azure IoT Extension for Azure CLI to your CLI shell. The IOT Extension adds IoT Hub, IoT Edge, and IoT Device Provisioning Service (DPS) specific commands to Azure CLI.

```sh
az extension add --name azure-iot
```

2. In the first CLI session, run the `az group create` command to create a resource group. The following command creates a resource group named MyResourceGroup in the eastus location.

```sh
az group create --name MyResourceGroup --location {your region}
```

3. You can then create the IoT hub, this process may take a few minutes to complete.
```sh
az iot hub create --resource-group MyResourceGroup --name {YourIoTHubName}
```

### Create the Azure IoT Hub Device
1. In the CLI session, run the `az iot hub device-identiy create` command which creates a device identitiy.

```sh
az iot hub device-identity create -d simDevice -n {YourIoTHubName}
```

2. When you are ready to set up your device, you need the connection string that links your phyiscal device with its identity in the IoT Hub. Use the `` 


# Visualize Iot Hub Device Data

