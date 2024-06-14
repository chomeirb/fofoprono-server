import { InvocationContext, Timer, app } from "@azure/functions";

export async function synchronizeScores(
  myTimer: Timer,
  context: InvocationContext
): Promise<void> {
  context.log("Timer trigger function executed at:", new Date().toISOString());
}

app.timer("synchronizeScores", {
  schedule: "*/5 * * * *",
  handler: synchronizeScores,
});
