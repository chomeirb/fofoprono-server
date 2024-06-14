import { Game } from "../models/database/game.js";
import { PrismaClient } from "@prisma/client";

export class SqlRepository {
  prisma: PrismaClient;

  constructor() {
    this.prisma = new PrismaClient();
  }

  public async upsertGames(games: Game[]) {
    try {
      var gamesTuUpdate = await this.prisma.game.findMany({
        where: {
          external_api_id: {
            in: games.map((game) => game.external_api_id),
          },
        },
      });

      gamesTuUpdate.forEach(async (game) => {
        const index = games.findIndex(
          (g) => g.external_api_id === game.external_api_id
        );

        if (index === -1) {
          return game;
        }

        const gameToUpdate = games[index];
        games.splice(index, 1);

        const updatedGame = {
          ...game,
          odds_home: gameToUpdate.odds_home,
          odds_draw: gameToUpdate.odds_draw,
          odds_away: gameToUpdate.odds_away,
        };

        await this.prisma.game.update({
          where: {
            id: game.id,
          },
          data: updatedGame,
        });
      });

      await this.prisma.game.createMany({
        data: games,
      });
    } catch (error) {
      console.error(error);
    }
  }

  public async connect() {
    await this.prisma.$connect();
    console.log("Connected to database");
  }

  public async disconnect() {
    await this.prisma.$disconnect();
    console.log("Disconnected from database");
  }
}
