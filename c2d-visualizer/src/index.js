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
    let c2d_message = {
        name: body.name,
        metric: {
            name: body.metric.name,
            description: body.metric.description,
            unit: body.metric.unit,
            data_point: {
                start_time_unix_nano: body.metric.data_point.start_time_unix_nano,
                value: body.metric.data_point.value,
                arrived_at: new Date(body.metric.data_point.current_time_unix ? body.metric.data_point.current_time_unix : null)
            }
        }
    }
    contentContainer.value = `${JSON.stringify(c2d_message)}`;

    if(JSON.parse(localStorage.getItem("c2d_data")) !== null) {
      let item_list = JSON.parse(localStorage.getItem("c2d_data"))
      item_list.push({
        dateTime: c2d_message.metric.data_point.arrived_at,
        value: c2d_message.metric.data_point.value
      })
      localStorage.setItem("c2d_data", JSON.stringify(item_list))
    } else {
      localStorage.setItem("c2d_data", JSON.stringify([]))
    }
}