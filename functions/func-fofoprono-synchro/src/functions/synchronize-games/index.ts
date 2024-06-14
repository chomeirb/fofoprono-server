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
import { Mapper } from "../../helpers/mapper.js";

export async function synchronizeGames(
  request: HttpRequest,
  context: InvocationContext
): Promise<HttpResponseInit> {
  context.log(`Http function processed request for url "${request.url}"`);

  try {
    var games = await ApiFootballClient.getGames();
    var odds = await ApiFootballClient.getOdds(NORDIC_BET_ID);
    var entities = games.map((game) => {
      return {
        time: game.fixture.date,
        stage: Mapper.fromMatchDateToStage(game.fixture.date),

        team_home: Mapper.fromApiNameToSqlNames(game.teams.home.name),
        team_away: Mapper.fromApiNameToSqlNames(game.teams.away.name),

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

    var repository = new SqlRepository();
    await repository.connect();
    await repository.upsertGames(entities);
    await repository.disconnect();
  } catch (e) {
    context.error(e);
    return { status: 500, body: e };
  }
}

app.http("synchronizeGames", {
  methods: ["GET", "POST"],
  authLevel: "anonymous",
  handler: synchronizeGames,
});
