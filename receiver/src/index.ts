import { earliestEventPosition, EventHubConsumerClient } from "@azure/event-hubs";
import * as dotenv from "dotenv";
dotenv.config();

const connectionString = process.env["IOTHUB_EH_COMPATIBLE_CONNECTION_STRING"] || "";
console.log(connectionString)
const consumerGroup = "";

export async function main(): Promise<void> {
    const consumerClient = new EventHubConsumerClient(consumerGroup, connectionString);

    consumerClient.subscribe(
        {
          processEvents: async (events, context) => {
            for (const event of events) {
              console.log(
                `Received event: '${event.body}' from partition: '${context.partitionId}' and consumer group: '${context.consumerGroup}'`
              );
            }
          },
          processError: async (err, context) => {
            console.log(`Error on partition "${context.partitionId}": ${err}`);
          },
        },
        { startPosition: earliestEventPosition }
      );
}

main().catch((error) => {
    console.error("Error running client:", error);
});