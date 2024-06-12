import { InvocationContext, Timer, app } from "@azure/functions";

export async function timerTrigger(
  myTimer: Timer,
  context: InvocationContext
): Promise<void> {
  context.log("Timer trigger function executed at:", new Date().toISOString());
}

app.timer("timerTrigger", { schedule: "*/5 * * * *", handler: timerTrigger });
