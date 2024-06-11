import { Game } from "../models/database/game.js";
import { PrismaClient } from "@prisma/client";

export class SqlRepository {
  prisma: PrismaClient;

  constructor() {
    this.prisma = new PrismaClient();
  }

  public async upsertGames(games: Game[]) {
    try {
      await this.prisma.game.createMany({
        data: games,
        skipDuplicates: true,
      });
    } catch (error) {
      console.error(error);
    }
  }
}
