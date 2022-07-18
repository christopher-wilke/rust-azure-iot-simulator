import { earliestEventPosition, EventHubConsumerClient } from "@azure/event-hubs";

const rcvBtn = document.getElementById("receive")

rcvBtn.addEventListener("click", () => {
    receive_c2d_messages();
})

async function receive_c2d_messages() {
    const connectionString = "Endpoint=sb://iothub-ns-rust-azure-19491203-ed62575ad7.servicebus.windows.net/;SharedAccessKeyName=iothubowner;SharedAccessKey=llYTSfAkHWnHh/T3Xyt58l6e4Zb/+YfRT7e/C8OolsU=;EntityPath=rust-azure-iot-simulator";
    const consumerGroup = "";
    const consumerClient = new EventHubConsumerClient(consumerGroup, connectionString);

    consumerClient.subscribe(
        {
          processEvents: async (events, context) => {
            for (const event of events) {
              let body = JSON.stringify(event.body)
              process_incoming_data(event.body)
            }
          },
          processError: async (err, context) => {
            console.log(`Error on partition "${context.partitionId}": ${err}`);
          },
        },
        { startPosition: earliestEventPosition }
      );
}

const contentContainer = document.getElementById("receiveContent");
function process_incoming_data(body) {
    console.log(body);

    let c2d_message = {
        name: body.name,
        metric: {
            name: body.metric.name,
            description: body.metric.description,
            unit: body.metric.unit,
            data_point: {
                start_time_unix_nano: format_unix_time(body.metric.data_point.start_time_unix_nano)
            }
        }
    }
    contentContainer.value = `${JSON.stringify(c2d_message)}\n`;
}

function format_unix_time(unix_timestamp) {
    const milliseconds = unix_timestamp*1000
    const dateObject = new Date(milliseconds)
    return dateObject.toLocaleDateString()
}