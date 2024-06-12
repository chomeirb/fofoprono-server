import {
  app,
  HttpRequest,
  HttpResponseInit,
  InvocationContext,
} from "@azure/functions";

import { ApiFootballClient } from "../../helpers/api-football-client.js";
import { EURO_2024_ID } from "../../models/database/const.js";
import { NORDIC_BET_ID } from "../../models/api-football/const.js";
import { SqlRepository } from "../../helpers/sql-repository.js";
import { Game } from "../../models/database/game.js";

export async function httpTrigger(
  request: HttpRequest,
  context: InvocationContext
): Promise<HttpResponseInit> {
  context.log(`Http function processed request for url "${request.url}"`);

  try {
    var stage = request.query.get("stage");
    var fromDate = request.query.get("fromDate");
    var toDate = request.query.get("toDate");

    if (!stage || !fromDate || !toDate) {
      return { status: 400, body: "Missing parameters" };
    }

    if (fromDate < new Date().toISOString()) {
      return { status: 400, body: "fromDate must be in the future" };
    }

    var games = await ApiFootballClient.getGames(
      new Date(fromDate),
      new Date(toDate)
    );
    var odds = await ApiFootballClient.getOdds(NORDIC_BET_ID);

    var entities = games.map((game) => {
      return {
        time: game.fixture.date,
        stage: stage,

        team_home: game.teams.home.name,
        team_away: game.teams.away.name,

        odds_home: Number(
          odds
            .find((odd) => odd.fixture.id === game.fixture.id)
            ?.bookmakers[0].bets[0].values.find(
              (value) => value.value === "Home"
            )?.odd
        ),
        odds_draw: Number(
          odds
            .find((odd) => odd.fixture.id === game.fixture.id)
            ?.bookmakers[0].bets[0].values.find(
              (value) => value.value === "Draw"
            )?.odd
        ),
        odds_away: Number(
          odds
            .find((odd) => odd.fixture.id === game.fixture.id)
            ?.bookmakers[0].bets[0].values.find(
              (value) => value.value === "Away"
            )?.odd
        ),

        competition_id: EURO_2024_ID,
        external_api_id: game.fixture.id,
      } as Game;
    });

    // await new SqlRepository().upsertGames(entities);
  } catch (e) {
    context.error(e);
    return { status: 500, body: e };
  }
}

app.http("httpTrigger", {
  methods: ["GET", "POST"],
  authLevel: "anonymous",
  handler: httpTrigger,
});
