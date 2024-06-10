import {
  app,
  HttpRequest,
  HttpResponseInit,
  InvocationContext,
} from "@azure/functions";

import { ApiFootballHelper } from "../../helpers/api-football-helper.js";

export async function httpTrigger(
  request: HttpRequest,
  context: InvocationContext
): Promise<HttpResponseInit> {
  context.log(`Http function processed request for url "${request.url}"`);

  var games = await ApiFootballHelper.getGames();
  var odds = await ApiFootballHelper.getOdds();

  console.log(games.length);
  console.log(odds.length);

  return { body: "OK" };
}

app.http("httpTrigger", {
  methods: ["GET", "POST"],
  authLevel: "anonymous",
  handler: httpTrigger,
});
